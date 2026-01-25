//! Validation for Kubernetes AdmissionRegistration API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/admissionregistration/validation/validation.go

use std::collections::BTreeSet;

use crate::admissionregistration::internal::{
    MutatingAdmissionPolicy, MutatingAdmissionPolicyBinding,
};
use crate::admissionregistration::v1::{
    FailurePolicyType, MatchCondition, MatchPolicyType, MatchResources, MutatingWebhook,
    MutatingWebhookConfiguration, NamedRuleWithOperations, OperationType, ParamKind, ParamRef,
    ParameterNotFoundActionType, ReinvocationPolicyType, Rule, RuleWithOperations, ScopeType,
    ServiceReference, ValidatingAdmissionPolicy, ValidatingAdmissionPolicyBinding,
    ValidatingAdmissionPolicyBindingSpec, ValidatingAdmissionPolicySpec,
    ValidatingAdmissionPolicyStatus, ValidatingWebhook, ValidatingWebhookConfiguration, Validation,
    ValidationAction, Variable, WebhookClientConfig,
};
use crate::common::ObjectMeta;
use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, invalid, is_dns1035_label, is_dns1123_subdomain,
    not_supported, required, validate_object_meta, validate_qualified_name,
};

// ============================================================================
// Public Validation Entry Points
// ============================================================================

pub fn validate_validating_webhook_configuration(
    obj: &ValidatingWebhookConfiguration,
) -> ErrorList {
    validate_webhook_configuration(&obj.metadata, &obj.webhooks, &Path::new("webhooks"), true)
}

pub fn validate_mutating_webhook_configuration(obj: &MutatingWebhookConfiguration) -> ErrorList {
    validate_webhook_configuration(&obj.metadata, &obj.webhooks, &Path::new("webhooks"), false)
}

pub fn validate_validating_admission_policy(obj: &ValidatingAdmissionPolicy) -> ErrorList {
    let mut all_errs = validate_object_meta(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        false,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_validating_admission_policy_spec(
        &obj.spec,
        &Path::new("spec"),
    ));
    all_errs
}

pub fn validate_validating_admission_policy_binding(
    obj: &ValidatingAdmissionPolicyBinding,
) -> ErrorList {
    let mut all_errs = validate_object_meta(
        obj.metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        false,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_validating_admission_policy_binding_spec(
        &obj.spec,
        &Path::new("spec"),
    ));
    all_errs
}

pub fn validate_validating_admission_policy_status(
    status: &ValidatingAdmissionPolicyStatus,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if let Some(ref type_checking) = status.type_checking {
        for (i, warning) in type_checking.expression_warnings.iter().enumerate() {
            let path = Path::new("typeChecking")
                .child("expressionWarnings")
                .index(i);
            if warning.warning.is_empty() {
                all_errs.push(required(&path.child("warning"), ""));
            }
            if warning.field_ref.is_empty() {
                all_errs.push(required(&path.child("fieldRef"), ""));
            }
        }
    }
    all_errs
}

pub fn validate_mutating_admission_policy(obj: &MutatingAdmissionPolicy) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &obj.metadata,
        false,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_mutating_admission_policy_spec(
        &obj.spec,
        &Path::new("spec"),
    ));
    all_errs
}

pub fn validate_mutating_admission_policy_binding(
    obj: &MutatingAdmissionPolicyBinding,
) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &obj.metadata,
        false,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_mutating_admission_policy_binding_spec(
        &obj.spec,
        &Path::new("spec"),
    ));
    all_errs
}

// ============================================================================
// Webhook Configuration Validation
// ============================================================================

fn validate_webhook_configuration<T>(
    metadata: &Option<ObjectMeta>,
    webhooks: &[T],
    webhooks_path: &Path,
    validating: bool,
) -> ErrorList
where
    T: WebhookAdapter,
{
    let mut all_errs = validate_object_meta(
        metadata.as_ref().unwrap_or(&ObjectMeta::default()),
        false,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    );

    let mut names = BTreeSet::new();
    for (i, webhook) in webhooks.iter().enumerate() {
        let path = webhooks_path.index(i);
        all_errs.extend(validate_webhook(webhook, &path, validating));
        all_errs.extend(validate_admission_review_versions(
            webhook.admission_review_versions(),
            &path.child("admissionReviewVersions"),
        ));
        if !webhook.name().is_empty() && !names.insert(webhook.name().to_string()) {
            all_errs.push(duplicate(
                &path.child("name"),
                BadValue::String(webhook.name().to_string()),
            ));
        }
    }

    all_errs
}

fn validate_webhook(webhook: &dyn WebhookAdapter, path: &Path, _validating: bool) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if webhook.name().is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    } else {
        all_errs.extend(validate_qualified_name(webhook.name(), &path.child("name")));
    }

    for (i, rule) in webhook.rules().iter().enumerate() {
        all_errs.extend(validate_rule_with_operations(
            rule,
            &path.child("rules").index(i),
            true,
        ));
    }

    if let Some(policy) = webhook.failure_policy() {
        if !matches!(policy, FailurePolicyType::Ignore | FailurePolicyType::Fail) {
            all_errs.push(not_supported(
                &path.child("failurePolicy"),
                BadValue::String(format!("{:?}", policy)),
                &["Ignore", "Fail"],
            ));
        }
    }

    if let Some(policy) = webhook.match_policy() {
        if !matches!(policy, MatchPolicyType::Exact | MatchPolicyType::Equivalent) {
            all_errs.push(not_supported(
                &path.child("matchPolicy"),
                BadValue::String(format!("{:?}", policy)),
                &["Exact", "Equivalent"],
            ));
        }
    }

    match webhook.side_effects() {
        None => {
            all_errs.push(required(&path.child("sideEffects"), ""));
        }
        Some(value) => {
            let value_str = format!("{:?}", value);
            if value_str != "None" && value_str != "NoneOnDryRun" {
                all_errs.push(not_supported(
                    &path.child("sideEffects"),
                    BadValue::String(value_str),
                    &["None", "NoneOnDryRun"],
                ));
            }
        }
    }

    if let Some(timeout) = webhook.timeout_seconds() {
        if !(1..=30).contains(&timeout) {
            all_errs.push(invalid(
                &path.child("timeoutSeconds"),
                BadValue::Int(timeout as i64),
                "the timeout value must be between 1 and 30 seconds",
            ));
        }
    }

    all_errs.extend(validate_webhook_client_config(
        webhook.client_config(),
        &path.child("clientConfig"),
    ));

    for (i, cond) in webhook.match_conditions().iter().enumerate() {
        all_errs.extend(validate_match_condition(
            cond,
            &path.child("matchConditions").index(i),
        ));
    }

    all_errs
}

fn validate_webhook_client_config(config: &WebhookClientConfig, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let has_url = config.url.as_ref().map_or(false, |s| !s.is_empty());
    let has_service = config.service.is_some();
    if has_url == has_service {
        all_errs.push(required(path, "exactly one of url or service is required"));
    } else if let Some(ref url) = config.url {
        if url.is_empty() {
            all_errs.push(required(&path.child("url"), ""));
        }
    } else if let Some(ref svc) = config.service {
        all_errs.extend(validate_service_reference(svc, &path.child("service")));
    }
    all_errs
}

fn validate_service_reference(service: &ServiceReference, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if service.namespace.is_empty() {
        all_errs.push(required(&path.child("namespace"), ""));
    }
    if service.name.is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    }
    if let Some(port) = service.port {
        if port < 1 || port > 65535 {
            all_errs.push(invalid(
                &path.child("port"),
                BadValue::Int(port as i64),
                "port must be between 1 and 65535",
            ));
        }
    }
    all_errs
}

fn validate_admission_review_versions(versions: &[String], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if versions.is_empty() {
        all_errs.push(required(path, "must specify at least one version"));
        return all_errs;
    }

    let mut seen = BTreeSet::new();
    let mut has_recognized = false;
    for (i, version) in versions.iter().enumerate() {
        if !seen.insert(version.clone()) {
            all_errs.push(duplicate(&path.index(i), BadValue::String(version.clone())));
            continue;
        }
        for msg in is_dns1035_label(version) {
            all_errs.push(invalid(
                &path.index(i),
                BadValue::String(version.clone()),
                &msg,
            ));
        }
        if version == "v1" || version == "v1beta1" {
            has_recognized = true;
        }
    }

    if !has_recognized {
        all_errs.push(invalid(
            path,
            BadValue::String(format!("{:?}", versions)),
            "must include at least one of v1, v1beta1",
        ));
    }

    all_errs
}
// ============================================================================
// Policy Validation
// ============================================================================

fn validate_validating_admission_policy_spec(
    spec: &ValidatingAdmissionPolicySpec,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if spec.match_constraints.is_none() {
        all_errs.push(required(&path.child("matchConstraints"), ""));
    } else if let Some(ref mc) = spec.match_constraints {
        all_errs.extend(validate_match_resources(
            mc,
            &path.child("matchConstraints"),
        ));
        if mc.resource_rules.is_empty() {
            all_errs.push(required(
                &path.child("matchConstraints").child("resourceRules"),
                "",
            ));
        }
    }

    if spec.validations.is_empty() && spec.audit_annotations.is_empty() {
        all_errs.push(required(
            &path.child("validations"),
            "validations or auditAnnotations must be specified",
        ));
    }

    for (i, validation) in spec.validations.iter().enumerate() {
        all_errs.extend(validate_validation(
            validation,
            &path.child("validations").index(i),
        ));
    }

    for (i, variable) in spec.variables.iter().enumerate() {
        all_errs.extend(validate_variable(
            variable,
            &path.child("variables").index(i),
        ));
    }

    if let Some(ref param_kind) = spec.param_kind {
        all_errs.extend(validate_param_kind(param_kind, &path.child("paramKind")));
    }

    for (i, cond) in spec.match_conditions.iter().enumerate() {
        all_errs.extend(validate_match_condition(
            cond,
            &path.child("matchConditions").index(i),
        ));
    }

    all_errs
}

fn validate_validating_admission_policy_binding_spec(
    spec: &ValidatingAdmissionPolicyBindingSpec,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if spec.policy_name.is_empty() {
        all_errs.push(required(&path.child("policyName"), ""));
    } else {
        for msg in name_is_dns_subdomain(&spec.policy_name, false) {
            all_errs.push(invalid(
                &path.child("policyName"),
                BadValue::String(spec.policy_name.clone()),
                &msg,
            ));
        }
    }
    all_errs.extend(validate_param_ref(
        spec.param_ref.as_ref(),
        &path.child("paramRef"),
    ));
    if let Some(ref mr) = spec.match_resources {
        all_errs.extend(validate_match_resources(mr, &path.child("matchResources")));
    }
    all_errs.extend(validate_validation_actions(
        &spec.validation_actions,
        &path.child("validationActions"),
    ));
    all_errs
}

fn validate_mutating_admission_policy_spec(
    spec: &crate::admissionregistration::internal::MutatingAdmissionPolicySpec,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if spec.failure_policy.is_none() {
        all_errs.push(required(&path.child("failurePolicy"), ""));
    }
    if spec.match_constraints.is_none() {
        all_errs.push(required(&path.child("matchConstraints"), ""));
    } else if let Some(ref mc) = spec.match_constraints {
        all_errs.extend(validate_match_resources(
            mc,
            &path.child("matchConstraints"),
        ));
        if mc.resource_rules.is_empty() {
            all_errs.push(required(
                &path.child("matchConstraints").child("resourceRules"),
                "",
            ));
        }
    }
    if spec.mutations.is_empty() {
        all_errs.push(required(&path.child("mutations"), ""));
    }
    if spec.reinvocation_policy != ReinvocationPolicyType::Never
        && spec.reinvocation_policy != ReinvocationPolicyType::IfNeeded
    {
        all_errs.push(not_supported(
            &path.child("reinvocationPolicy"),
            BadValue::String(format!("{:?}", spec.reinvocation_policy)),
            &["Never", "IfNeeded"],
        ));
    }
    all_errs
}

fn validate_mutating_admission_policy_binding_spec(
    spec: &crate::admissionregistration::internal::MutatingAdmissionPolicyBindingSpec,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if spec.policy_name.is_empty() {
        all_errs.push(required(&path.child("policyName"), ""));
    }
    all_errs.extend(validate_param_ref(
        spec.param_ref.as_ref(),
        &path.child("paramRef"),
    ));
    if let Some(ref mr) = spec.match_resources {
        all_errs.extend(validate_match_resources(mr, &path.child("matchResources")));
    }
    all_errs
}

// ============================================================================
// Rule/Match Validation
// ============================================================================

fn validate_match_resources(match_resources: &MatchResources, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(policy) = match_resources.match_policy.as_ref() {
        if *policy != MatchPolicyType::Exact && *policy != MatchPolicyType::Equivalent {
            all_errs.push(not_supported(
                &path.child("matchPolicy"),
                BadValue::String(format!("{:?}", policy)),
                &["Exact", "Equivalent"],
            ));
        }
    }

    for (i, rule) in match_resources.resource_rules.iter().enumerate() {
        all_errs.extend(validate_named_rule_with_operations(
            rule,
            &path.child("resourceRules").index(i),
            true,
        ));
    }
    for (i, rule) in match_resources.exclude_resource_rules.iter().enumerate() {
        all_errs.extend(validate_named_rule_with_operations(
            rule,
            &path.child("excludeResourceRules").index(i),
            true,
        ));
    }

    all_errs
}

fn validate_rule(rule: &Rule, path: &Path, allow_subresource: bool) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if rule.api_groups.is_empty() {
        all_errs.push(required(&path.child("apiGroups"), ""));
    }
    if rule.api_groups.len() > 1 && has_wildcard(&rule.api_groups) {
        all_errs.push(invalid(
            &path.child("apiGroups"),
            BadValue::String(format!("{:?}", rule.api_groups)),
            "if '*' is present, must not specify other API groups",
        ));
    }

    if rule.api_versions.is_empty() {
        all_errs.push(required(&path.child("apiVersions"), ""));
    }
    if rule.api_versions.len() > 1 && has_wildcard(&rule.api_versions) {
        all_errs.push(invalid(
            &path.child("apiVersions"),
            BadValue::String(format!("{:?}", rule.api_versions)),
            "if '*' is present, must not specify other API versions",
        ));
    }
    for (i, version) in rule.api_versions.iter().enumerate() {
        if version.is_empty() {
            all_errs.push(required(&path.child("apiVersions").index(i), ""));
        }
    }

    if allow_subresource {
        all_errs.extend(validate_resources(
            &rule.resources,
            &path.child("resources"),
        ));
    } else {
        all_errs.extend(validate_resources_no_subresources(
            &rule.resources,
            &path.child("resources"),
        ));
    }

    if let Some(scope) = rule.scope.as_ref() {
        if *scope != ScopeType::Cluster
            && *scope != ScopeType::Namespaced
            && *scope != ScopeType::AllScopes
        {
            all_errs.push(not_supported(
                &path.child("scope"),
                BadValue::String(format!("{:?}", scope)),
                &["Cluster", "Namespaced", "*"],
            ));
        }
    }

    all_errs
}

fn validate_rule_with_operations(
    rule: &RuleWithOperations,
    path: &Path,
    allow_subresource: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if rule.operations.is_empty() {
        all_errs.push(required(&path.child("operations"), ""));
    } else if rule.operations.len() > 1 && rule.operations.contains(&OperationType::All) {
        all_errs.push(invalid(
            &path.child("operations"),
            BadValue::String(format!("{:?}", rule.operations)),
            "if '*' is present, must not specify other operations",
        ));
    }

    for (i, op) in rule.operations.iter().enumerate() {
        let valid = matches!(
            op,
            OperationType::All
                | OperationType::Create
                | OperationType::Update
                | OperationType::Delete
                | OperationType::Connect
        );
        if !valid {
            all_errs.push(not_supported(
                &path.child("operations").index(i),
                BadValue::String(format!("{:?}", op)),
                &["*", "CREATE", "UPDATE", "DELETE", "CONNECT"],
            ));
        }
    }

    let rule_only = Rule {
        api_groups: rule.api_groups.clone(),
        api_versions: rule.api_versions.clone(),
        resources: rule.resources.clone(),
        scope: rule.scope.clone(),
    };
    all_errs.extend(validate_rule(&rule_only, path, allow_subresource));

    all_errs
}

fn validate_named_rule_with_operations(
    rule: &NamedRuleWithOperations,
    path: &Path,
    allow_subresource: bool,
) -> ErrorList {
    let rule_with_ops = RuleWithOperations {
        operations: rule.operations.clone(),
        api_groups: rule.api_groups.clone(),
        api_versions: rule.api_versions.clone(),
        resources: rule.resources.clone(),
        scope: rule.scope.clone(),
    };
    validate_rule_with_operations(&rule_with_ops, path, allow_subresource)
}

fn validate_resources(resources: &[String], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if resources.is_empty() {
        all_errs.push(required(path, ""));
    }

    let mut resources_with_wildcard_sub = BTreeSet::new();
    let mut sub_resources_with_wildcard_res = BTreeSet::new();
    let mut has_double_wildcard = false;
    let mut has_single_wildcard = false;
    let mut has_resource_without_subresource = false;

    for (i, res_sub) in resources.iter().enumerate() {
        if res_sub.is_empty() {
            all_errs.push(required(&path.index(i), ""));
            continue;
        }
        if res_sub == "*/*" {
            has_double_wildcard = true;
        }
        if res_sub == "*" {
            has_single_wildcard = true;
        }
        let parts: Vec<&str> = res_sub.splitn(2, '/').collect();
        if parts.len() == 1 {
            has_resource_without_subresource = res_sub != "*";
            continue;
        }
        let res = parts[0];
        let sub = parts[1];
        if resources_with_wildcard_sub.contains(res) {
            all_errs.push(invalid(
                &path.index(i),
                BadValue::String(res_sub.clone()),
                &format!(
                    "if '{}/{}' is present, must not specify {}",
                    res, "*", res_sub
                ),
            ));
        }
        if sub_resources_with_wildcard_res.contains(sub) {
            all_errs.push(invalid(
                &path.index(i),
                BadValue::String(res_sub.clone()),
                &format!("if '*/{}' is present, must not specify {}", sub, res_sub),
            ));
        }
        if sub == "*" {
            resources_with_wildcard_sub.insert(res.to_string());
        }
        if res == "*" {
            sub_resources_with_wildcard_res.insert(sub.to_string());
        }
    }

    if resources.len() > 1 && has_double_wildcard {
        all_errs.push(invalid(
            path,
            BadValue::String(format!("{:?}", resources)),
            "if '*/*' is present, must not specify other resources",
        ));
    }
    if has_single_wildcard && has_resource_without_subresource {
        all_errs.push(invalid(
            path,
            BadValue::String(format!("{:?}", resources)),
            "if '*' is present, must not specify other resources without subresources",
        ));
    }

    all_errs
}

fn validate_resources_no_subresources(resources: &[String], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if resources.is_empty() {
        all_errs.push(required(path, ""));
    }
    for (i, resource) in resources.iter().enumerate() {
        if resource.is_empty() {
            all_errs.push(required(&path.index(i), ""));
        }
        if resource.contains('/') {
            all_errs.push(invalid(
                &path.index(i),
                BadValue::String(resource.clone()),
                "must not specify subresources",
            ));
        }
    }
    if resources.len() > 1 && has_wildcard(resources) {
        all_errs.push(invalid(
            path,
            BadValue::String(format!("{:?}", resources)),
            "if '*' is present, must not specify other resources",
        ));
    }
    all_errs
}

fn has_wildcard(values: &[String]) -> bool {
    values.iter().any(|v| v == "*")
}
// ============================================================================
// Param/Validation Helpers
// ============================================================================

fn validate_param_ref(param_ref: Option<&ParamRef>, path: &Path) -> ErrorList {
    let Some(param_ref) = param_ref else {
        return ErrorList::new();
    };
    let mut all_errs = ErrorList::new();

    let has_name = !param_ref.name.is_empty();
    let has_selector = param_ref.selector.is_some();
    if has_name && has_selector {
        all_errs.push(invalid(
            &path.child("name"),
            BadValue::String(param_ref.name.clone()),
            "name and selector are mutually exclusive",
        ));
    }
    if !has_name && !has_selector {
        all_errs.push(required(path, "one of name or selector must be specified"));
    }

    if param_ref.parameter_not_found_action.is_none() {
        all_errs.push(required(&path.child("parameterNotFoundAction"), ""));
    } else if let Some(action) = param_ref.parameter_not_found_action.as_ref() {
        if *action != ParameterNotFoundActionType::Allow
            && *action != ParameterNotFoundActionType::Deny
        {
            all_errs.push(not_supported(
                &path.child("parameterNotFoundAction"),
                BadValue::String(format!("{:?}", action)),
                &["Allow", "Deny"],
            ));
        }
    }

    all_errs
}

fn validate_param_kind(param_kind: &ParamKind, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if param_kind.api_version.is_empty() {
        all_errs.push(required(&path.child("apiVersion"), ""));
    } else {
        let parts: Vec<&str> = param_kind.api_version.split('/').collect();
        if parts.len() != 2 {
            all_errs.push(invalid(
                &path.child("apiVersion"),
                BadValue::String(param_kind.api_version.clone()),
                "apiVersion must be in the form group/version",
            ));
        } else {
            for msg in is_dns1123_subdomain(parts[0]) {
                all_errs.push(invalid(
                    &path.child("apiVersion"),
                    BadValue::String(param_kind.api_version.clone()),
                    &msg,
                ));
            }
            for msg in is_dns1035_label(parts[1]) {
                all_errs.push(invalid(
                    &path.child("apiVersion"),
                    BadValue::String(param_kind.api_version.clone()),
                    &msg,
                ));
            }
        }
    }

    if param_kind.kind.is_empty() {
        all_errs.push(required(&path.child("kind"), ""));
    }

    all_errs
}

fn validate_validation(validation: &Validation, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if validation.expression.is_empty() {
        all_errs.push(required(&path.child("expression"), ""));
    }

    if has_newlines(&validation.expression)
        && validation.message.is_empty()
        && validation.message_expression.is_empty()
    {
        all_errs.push(required(
            &path.child("message"),
            "message or messageExpression is required when expression contains newlines",
        ));
    }

    if has_newlines(&validation.message) {
        all_errs.push(invalid(
            &path.child("message"),
            BadValue::String(validation.message.clone()),
            "message must not contain line breaks",
        ));
    }
    if has_newlines(&validation.message_expression) {
        all_errs.push(invalid(
            &path.child("messageExpression"),
            BadValue::String(validation.message_expression.clone()),
            "messageExpression must not contain line breaks",
        ));
    }

    all_errs
}

fn validate_variable(variable: &Variable, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if variable.name.is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    } else if !is_cel_identifier(&variable.name) {
        all_errs.push(invalid(
            &path.child("name"),
            BadValue::String(variable.name.clone()),
            "must be a valid CEL identifier",
        ));
    }
    if variable.expression.is_empty() {
        all_errs.push(required(&path.child("expression"), ""));
    }
    all_errs
}

fn validate_match_condition(condition: &MatchCondition, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if condition.name.is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    } else {
        all_errs.extend(validate_qualified_name(
            &condition.name,
            &path.child("name"),
        ));
    }
    if condition.expression.is_empty() {
        all_errs.push(required(&path.child("expression"), ""));
    }
    all_errs
}

fn validate_validation_actions(actions: &[ValidationAction], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if actions.is_empty() {
        all_errs.push(required(path, ""));
        return all_errs;
    }
    let mut seen = BTreeSet::new();
    for (i, action) in actions.iter().enumerate() {
        let value = format!("{:?}", action);
        if !matches!(
            action,
            ValidationAction::Deny | ValidationAction::Warn | ValidationAction::Audit
        ) {
            all_errs.push(not_supported(
                &path.index(i),
                BadValue::String(value.clone()),
                &["Deny", "Warn", "Audit"],
            ));
        }
        if !seen.insert(value.clone()) {
            all_errs.push(duplicate(
                &path.index(i),
                BadValue::String(format!("{:?}", action)),
            ));
        }
    }
    all_errs
}

fn name_is_dns_subdomain(name: &str, prefix: bool) -> Vec<String> {
    crate::common::validation::name_is_dns_subdomain(name, prefix)
}

fn is_cel_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    if !chars.all(|c| c == '_' || c.is_ascii_alphanumeric()) {
        return false;
    }
    !is_cel_reserved(name)
}

fn is_cel_reserved(name: &str) -> bool {
    matches!(
        name,
        "true"
            | "false"
            | "null"
            | "in"
            | "as"
            | "break"
            | "const"
            | "continue"
            | "else"
            | "for"
            | "function"
            | "if"
            | "import"
            | "let"
            | "loop"
            | "package"
            | "namespace"
            | "return"
            | "var"
            | "void"
            | "while"
    )
}

fn has_newlines(value: &str) -> bool {
    value.contains('\n') || value.contains('\r')
}

// ============================================================================
// Webhook Adapter
// ============================================================================

trait WebhookAdapter {
    fn name(&self) -> &str;
    fn rules(&self) -> &[RuleWithOperations];
    fn failure_policy(&self) -> Option<crate::admissionregistration::v1::FailurePolicyType>;
    fn match_policy(&self) -> Option<MatchPolicyType>;
    fn side_effects(&self) -> Option<crate::admissionregistration::v1::SideEffectClass>;
    fn timeout_seconds(&self) -> Option<i32>;
    fn admission_review_versions(&self) -> &[String];
    fn client_config(&self) -> &WebhookClientConfig;
    fn match_conditions(&self) -> &[MatchCondition];
}

impl WebhookAdapter for ValidatingWebhook {
    fn name(&self) -> &str {
        &self.name
    }
    fn rules(&self) -> &[RuleWithOperations] {
        &self.rules
    }
    fn failure_policy(&self) -> Option<crate::admissionregistration::v1::FailurePolicyType> {
        self.failure_policy.clone()
    }
    fn match_policy(&self) -> Option<MatchPolicyType> {
        self.match_policy.clone()
    }
    fn side_effects(&self) -> Option<crate::admissionregistration::v1::SideEffectClass> {
        self.side_effects.clone()
    }
    fn timeout_seconds(&self) -> Option<i32> {
        self.timeout_seconds
    }
    fn admission_review_versions(&self) -> &[String] {
        &self.admission_review_versions
    }
    fn client_config(&self) -> &WebhookClientConfig {
        &self.client_config
    }
    fn match_conditions(&self) -> &[MatchCondition] {
        &self.match_conditions
    }
}

impl WebhookAdapter for MutatingWebhook {
    fn name(&self) -> &str {
        &self.name
    }
    fn rules(&self) -> &[RuleWithOperations] {
        &self.rules
    }
    fn failure_policy(&self) -> Option<crate::admissionregistration::v1::FailurePolicyType> {
        self.failure_policy.clone()
    }
    fn match_policy(&self) -> Option<MatchPolicyType> {
        self.match_policy.clone()
    }
    fn side_effects(&self) -> Option<crate::admissionregistration::v1::SideEffectClass> {
        self.side_effects.clone()
    }
    fn timeout_seconds(&self) -> Option<i32> {
        self.timeout_seconds
    }
    fn admission_review_versions(&self) -> &[String] {
        &self.admission_review_versions
    }
    fn client_config(&self) -> &WebhookClientConfig {
        &self.client_config
    }
    fn match_conditions(&self) -> &[MatchCondition] {
        &self.match_conditions
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;

    #[test]
    fn test_validate_validating_webhook_configuration_requires_side_effects() {
        let obj = ValidatingWebhookConfiguration {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("cfg".to_string()),
                ..Default::default()
            }),
            webhooks: vec![ValidatingWebhook {
                name: "example.com/hook".to_string(),
                client_config: WebhookClientConfig {
                    url: Some("https://example.com".to_string()),
                    service: None,
                    ca_bundle: Default::default(),
                },
                admission_review_versions: vec!["v1".to_string()],
                ..Default::default()
            }],
        };

        let errs = validate_validating_webhook_configuration(&obj);
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_validate_validating_admission_policy_requires_match_constraints() {
        let obj = ValidatingAdmissionPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("policy".to_string()),
                ..Default::default()
            }),
            spec: ValidatingAdmissionPolicySpec::default(),
            status: ValidatingAdmissionPolicyStatus::default(),
        };

        let errs = validate_validating_admission_policy(&obj);
        assert!(!errs.is_empty());
    }
}
