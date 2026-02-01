//! FlowSchema validation.

use std::collections::HashSet;

use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, forbidden, invalid, name_is_dns_subdomain, not_supported,
    required, validate_object_meta,
};
use crate::flowcontrol::v1 as flowcontrol;

use super::helpers::{
    FLOW_SCHEMA_DEFAULT_MATCHING_PRECEDENCE, FLOW_SCHEMA_MAX_MATCHING_PRECEDENCE, NS_ERR_INTRO,
    SUPPORTED_SUBJECT_KINDS, SUPPORTED_VERBS, has_wildcard, required_field,
    validate_namespace_name, validate_non_resource_url_path, validate_service_account_name,
    validate_supported_list,
};

pub fn validate_flow_schema(flow_schema: &flowcontrol::FlowSchema) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let base_path = Path::nil();
    let default_meta = crate::common::ObjectMeta::default();
    let metadata = flow_schema.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    if let Some(spec) = &flow_schema.spec {
        let name = metadata.name.as_deref().unwrap_or("");
        all_errs.extend(validate_flow_schema_spec(
            name,
            spec,
            &base_path.child("spec"),
        ));
    } else {
        all_errs.push(required(&base_path.child("spec"), "spec is required"));
    }

    if let Some(status) = &flow_schema.status {
        all_errs.extend(validate_flow_schema_status(
            status,
            &base_path.child("status"),
        ));
    }

    all_errs
}

pub fn validate_flow_schema_update(
    _old: &flowcontrol::FlowSchema,
    new: &flowcontrol::FlowSchema,
) -> ErrorList {
    validate_flow_schema(new)
}

pub fn validate_flow_schema_spec(
    flow_schema_name: &str,
    spec: &flowcontrol::FlowSchemaSpec,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let matching_precedence = spec
        .matching_precedence
        .unwrap_or(FLOW_SCHEMA_DEFAULT_MATCHING_PRECEDENCE);

    if matching_precedence <= 0 {
        all_errs.push(invalid(
            &path.child("matchingPrecedence"),
            BadValue::Int(matching_precedence as i64),
            "must be a positive value",
        ));
    }
    if matching_precedence > FLOW_SCHEMA_MAX_MATCHING_PRECEDENCE {
        all_errs.push(invalid(
            &path.child("matchingPrecedence"),
            BadValue::Int(matching_precedence as i64),
            &format!("must not be greater than {FLOW_SCHEMA_MAX_MATCHING_PRECEDENCE}"),
        ));
    }
    if matching_precedence == 1 && flow_schema_name != flowcontrol::flow_schema_names::EXEMPT {
        all_errs.push(invalid(
            &path.child("matchingPrecedence"),
            BadValue::Int(matching_precedence as i64),
            "only the schema named 'exempt' may have matchingPrecedence 1",
        ));
    }

    if let Some(method) = &spec.distinguisher_method {
        let method_str = match method.r#type {
            flowcontrol::FlowDistinguisherMethodType::ByNamespace => {
                flowcontrol::flow_distinguisher_method_type::BY_NAMESPACE
            }
            flowcontrol::FlowDistinguisherMethodType::ByUser => {
                flowcontrol::flow_distinguisher_method_type::BY_USER
            }
        };
        if ![
            flowcontrol::flow_distinguisher_method_type::BY_NAMESPACE,
            flowcontrol::flow_distinguisher_method_type::BY_USER,
        ]
        .contains(&method_str)
        {
            all_errs.push(not_supported(
                &path.child("distinguisherMethod").child("type"),
                BadValue::String(method_str.to_string()),
                &[
                    flowcontrol::flow_distinguisher_method_type::BY_NAMESPACE,
                    flowcontrol::flow_distinguisher_method_type::BY_USER,
                ],
            ));
        }
    }

    if !spec.priority_level_configuration.name.is_empty() {
        for msg in
            crate::common::validation::is_dns1123_subdomain(&spec.priority_level_configuration.name)
        {
            all_errs.push(invalid(
                &path.child("priorityLevelConfiguration").child("name"),
                BadValue::String(spec.priority_level_configuration.name.clone()),
                &msg,
            ));
        }
    } else {
        all_errs.push(required(
            &path.child("priorityLevelConfiguration").child("name"),
            "must reference a priority level",
        ));
    }

    for (i, rule) in spec.rules.iter().enumerate() {
        all_errs.extend(validate_flow_schema_policy_rules_with_subjects(
            rule,
            &path.child("rules").index(i),
        ));
    }

    all_errs
}

pub fn validate_flow_schema_policy_rules_with_subjects(
    rule: &flowcontrol::PolicyRulesWithSubjects,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if !rule.subjects.is_empty() {
        for (i, subject) in rule.subjects.iter().enumerate() {
            all_errs.extend(validate_flow_schema_subject(
                subject,
                &path.child("subjects").index(i),
            ));
        }
    } else {
        all_errs.push(required(
            &path.child("subjects"),
            "subjects must contain at least one value",
        ));
    }

    if rule.resource_rules.is_empty() && rule.non_resource_rules.is_empty() {
        all_errs.push(required(
            path,
            "at least one of resourceRules and nonResourceRules has to be non-empty",
        ));
    }

    for (i, resource_rule) in rule.resource_rules.iter().enumerate() {
        all_errs.extend(validate_flow_schema_resource_policy_rule(
            resource_rule,
            &path.child("resourceRules").index(i),
        ));
    }
    for (i, non_resource_rule) in rule.non_resource_rules.iter().enumerate() {
        all_errs.extend(validate_flow_schema_non_resource_policy_rule(
            non_resource_rule,
            &path.child("nonResourceRules").index(i),
        ));
    }

    all_errs
}

pub fn validate_flow_schema_subject(subject: &flowcontrol::Subject, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    match subject.kind {
        flowcontrol::SubjectKind::ServiceAccount => {
            all_errs.extend(validate_service_account_subject(
                subject.service_account.as_ref(),
                &path.child("serviceAccount"),
            ));
            if subject.user.is_some() {
                all_errs.push(forbidden(
                    &path.child("user"),
                    "user is forbidden when subject kind is not 'User'",
                ));
            }
            if subject.group.is_some() {
                all_errs.push(forbidden(
                    &path.child("group"),
                    "group is forbidden when subject kind is not 'Group'",
                ));
            }
        }
        flowcontrol::SubjectKind::User => {
            all_errs.extend(validate_user_subject(
                subject.user.as_ref(),
                &path.child("user"),
            ));
            if subject.service_account.is_some() {
                all_errs.push(forbidden(
                    &path.child("serviceAccount"),
                    "serviceAccount is forbidden when subject kind is not 'ServiceAccount'",
                ));
            }
            if subject.group.is_some() {
                all_errs.push(forbidden(
                    &path.child("group"),
                    "group is forbidden when subject kind is not 'Group'",
                ));
            }
        }
        flowcontrol::SubjectKind::Group => {
            all_errs.extend(validate_group_subject(
                subject.group.as_ref(),
                &path.child("group"),
            ));
            if subject.service_account.is_some() {
                all_errs.push(forbidden(
                    &path.child("serviceAccount"),
                    "serviceAccount is forbidden when subject kind is not 'ServiceAccount'",
                ));
            }
            if subject.user.is_some() {
                all_errs.push(forbidden(
                    &path.child("user"),
                    "user is forbidden when subject kind is not 'User'",
                ));
            }
        }
    }

    let kind_str = match subject.kind {
        flowcontrol::SubjectKind::ServiceAccount => flowcontrol::subject_kind::SERVICE_ACCOUNT,
        flowcontrol::SubjectKind::Group => flowcontrol::subject_kind::GROUP,
        flowcontrol::SubjectKind::User => flowcontrol::subject_kind::USER,
    };
    if !SUPPORTED_SUBJECT_KINDS.contains(&kind_str) {
        all_errs.push(not_supported(
            &path.child("kind"),
            BadValue::String(kind_str.to_string()),
            &SUPPORTED_SUBJECT_KINDS,
        ));
    }

    all_errs
}

fn validate_service_account_subject(
    subject: Option<&flowcontrol::ServiceAccountSubject>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if subject.is_none() {
        all_errs.push(required(
            path,
            "serviceAccount is required when subject kind is 'ServiceAccount'",
        ));
        return all_errs;
    }

    let subject = subject.unwrap();

    if subject.name.is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    } else if subject.name != flowcontrol::wildcards::NAME_ALL {
        all_errs.extend(validate_service_account_name(
            &subject.name,
            &path.child("name"),
        ));
    }

    if subject.namespace.is_empty() {
        all_errs.push(required(
            &path.child("namespace"),
            "must specify namespace for service account",
        ));
    } else {
        all_errs.extend(validate_namespace_name(
            &subject.namespace,
            &path.child("namespace"),
        ));
    }

    all_errs
}

fn validate_user_subject(subject: Option<&flowcontrol::UserSubject>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if subject.is_none() {
        all_errs.push(required(
            path,
            "user is required when subject kind is 'User'",
        ));
        return all_errs;
    }
    let subject = subject.unwrap();
    if subject.name.is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    }
    all_errs
}

fn validate_group_subject(subject: Option<&flowcontrol::GroupSubject>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if subject.is_none() {
        all_errs.push(required(
            path,
            "group is required when subject kind is 'Group'",
        ));
        return all_errs;
    }
    let subject = subject.unwrap();
    if subject.name.is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    }
    all_errs
}

fn validate_flow_schema_non_resource_policy_rule(
    rule: &flowcontrol::NonResourcePolicyRule,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if rule.verbs.is_empty() {
        all_errs.push(required(
            &path.child("verbs"),
            "verbs must contain at least one value",
        ));
    } else if has_wildcard(&rule.verbs) {
        if rule.verbs.len() > 1 {
            all_errs.push(invalid(
                &path.child("verbs"),
                BadValue::String(format!("{:?}", rule.verbs)),
                "if '*' is present, must not specify other verbs",
            ));
        }
    } else {
        all_errs.extend(validate_supported_list(
            &rule.verbs,
            &SUPPORTED_VERBS,
            &path.child("verbs"),
        ));
    }

    if rule.non_resource_urls.is_empty() {
        all_errs.push(required(
            &path.child("nonResourceURLs"),
            "nonResourceURLs must contain at least one value",
        ));
    } else if has_wildcard(&rule.non_resource_urls) {
        if rule.non_resource_urls.len() > 1 {
            all_errs.push(invalid(
                &path.child("nonResourceURLs"),
                BadValue::String(format!("{:?}", rule.non_resource_urls)),
                "if '*' is present, must not specify other non-resource URLs",
            ));
        }
    } else {
        for (i, url) in rule.non_resource_urls.iter().enumerate() {
            if let Some(err) =
                validate_non_resource_url_path(url, &path.child("nonResourceURLs").index(i))
            {
                all_errs.push(err);
            }
        }
    }

    all_errs
}

fn validate_flow_schema_resource_policy_rule(
    rule: &flowcontrol::ResourcePolicyRule,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if rule.verbs.is_empty() {
        all_errs.push(required(
            &path.child("verbs"),
            "verbs must contain at least one value",
        ));
    } else if has_wildcard(&rule.verbs) {
        if rule.verbs.len() > 1 {
            all_errs.push(invalid(
                &path.child("verbs"),
                BadValue::String(format!("{:?}", rule.verbs)),
                "if '*' is present, must not specify other verbs",
            ));
        }
    } else {
        all_errs.extend(validate_supported_list(
            &rule.verbs,
            &SUPPORTED_VERBS,
            &path.child("verbs"),
        ));
    }

    if rule.api_groups.is_empty() {
        all_errs.push(required(
            &path.child("apiGroups"),
            "resource rules must supply at least one api group",
        ));
    } else if rule.api_groups.len() > 1 && has_wildcard(&rule.api_groups) {
        all_errs.push(invalid(
            &path.child("apiGroups"),
            BadValue::String(format!("{:?}", rule.api_groups)),
            "if '*' is present, must not specify other api groups",
        ));
    }

    if rule.resources.is_empty() {
        all_errs.push(required(
            &path.child("resources"),
            "resource rules must supply at least one resource",
        ));
    } else if rule.resources.len() > 1 && has_wildcard(&rule.resources) {
        all_errs.push(invalid(
            &path.child("resources"),
            BadValue::String(format!("{:?}", rule.resources)),
            "if '*' is present, must not specify other resources",
        ));
    }

    let cluster_scope = rule.cluster_scope.unwrap_or(false);
    if rule.namespaces.is_empty() && !cluster_scope {
        all_errs.push(required(
            &path.child("namespaces"),
            "resource rules that are not cluster scoped must supply at least one namespace",
        ));
    } else if has_wildcard(&rule.namespaces) {
        if rule.namespaces.len() > 1 {
            all_errs.push(invalid(
                &path.child("namespaces"),
                BadValue::String(format!("{:?}", rule.namespaces)),
                "if '*' is present, must not specify other namespaces",
            ));
        }
    } else {
        for (idx, ns) in rule.namespaces.iter().enumerate() {
            let mut errs = validate_namespace_name(ns, &path.child("namespaces").index(idx));
            if !errs.is_empty() {
                for err in errs.errors.iter_mut() {
                    err.detail = format!("{NS_ERR_INTRO}{}", err.detail);
                }
                all_errs.extend(errs);
            }
        }
    }

    all_errs
}

pub fn validate_flow_schema_status(
    status: &flowcontrol::FlowSchemaStatus,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut keys = HashSet::new();

    for (i, condition) in status.conditions.iter().enumerate() {
        let condition_path = path.child("conditions").index(i);
        let key = condition
            .r#type
            .as_ref()
            .map(flow_schema_condition_type_str)
            .unwrap_or("");
        if !key.is_empty() && !keys.insert(key.to_string()) {
            all_errs.push(duplicate(
                &condition_path.child("type"),
                BadValue::String(key.to_string()),
            ));
        }
        all_errs.extend(validate_flow_schema_condition(condition, &condition_path));
    }

    all_errs
}

pub fn validate_flow_schema_status_update(
    _old: &flowcontrol::FlowSchema,
    new_obj: &flowcontrol::FlowSchema,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(status) = &new_obj.status {
        all_errs.extend(validate_flow_schema_status(status, &Path::new("status")));
    }
    all_errs
}

pub fn validate_flow_schema_condition(
    condition: &flowcontrol::FlowSchemaCondition,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    all_errs.extend(required_field(&condition.r#type, &path.child("type"), ""));
    all_errs
}

fn flow_schema_condition_type_str(ty: &flowcontrol::FlowSchemaConditionType) -> &'static str {
    match ty {
        flowcontrol::FlowSchemaConditionType::Dangling => {
            flowcontrol::flow_schema_condition_type::DANGLING
        }
    }
}
