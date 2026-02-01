//! Validation for Kubernetes RBAC internal API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/rbac/validation/validation.go

mod cluster_role;
mod cluster_role_binding;
mod role;
mod role_binding;

use crate::common::meta::{LabelSelector, label_selector_operator};
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, is_dns1123_subdomain, is_valid_label_value, not_supported,
    required, validate_labels, validate_qualified_name,
};
use crate::rbac::internal::{PolicyRule, Subject};
use crate::rbac::v1::{api_group, subject_kind};

pub use cluster_role::{
    validate_cluster_role, validate_cluster_role_update, ClusterRoleValidationOptions,
};
pub use cluster_role_binding::{
    validate_cluster_role_binding, validate_cluster_role_binding_update,
};
pub use role::{validate_role, validate_role_update};
pub use role_binding::{validate_role_binding, validate_role_binding_update};

/// ValidateRBACName is exported to allow types outside of the RBAC API group
/// to reuse this validation logic.
pub fn validate_rbac_name(name: &str, _prefix: bool) -> Vec<String> {
    let mut errs = Vec::new();
    if name == "." || name == ".." {
        errs.push(format!("may not be '{}'", name));
    }
    if name.contains('/') {
        errs.push("may not contain '/'".to_string());
    }
    if name.contains('%') {
        errs.push("may not contain '%'".to_string());
    }
    errs
}

pub fn validate_policy_rule(rule: &PolicyRule, is_namespaced: bool, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if rule.verbs.is_empty() {
        all_errs.push(required(
            &path.child("verbs"),
            "verbs must contain at least one value",
        ));
    }

    if !rule.non_resource_urls.is_empty() {
        if is_namespaced {
            all_errs.push(invalid(
                &path.child("nonResourceURLs"),
                BadValue::String(format!("{:?}", rule.non_resource_urls)),
                "namespaced rules cannot apply to non-resource URLs",
            ));
        }
        if !rule.api_groups.is_empty() || !rule.resources.is_empty() || !rule.resource_names.is_empty()
        {
            all_errs.push(invalid(
                &path.child("nonResourceURLs"),
                BadValue::String(format!("{:?}", rule.non_resource_urls)),
                "rules cannot apply to both regular resources and non-resource URLs",
            ));
        }
        return all_errs;
    }

    if rule.api_groups.is_empty() {
        all_errs.push(required(
            &path.child("apiGroups"),
            "resource rules must supply at least one api group",
        ));
    }
    if rule.resources.is_empty() {
        all_errs.push(required(
            &path.child("resources"),
            "resource rules must supply at least one resource",
        ));
    }

    all_errs
}

pub fn validate_role_binding_subject(
    subject: &Subject,
    is_namespaced: bool,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if subject.name.is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    }

    match subject.kind.as_str() {
        subject_kind::SERVICE_ACCOUNT => {
            if !subject.name.is_empty() {
                for msg in is_dns1123_subdomain(&subject.name) {
                    all_errs.push(invalid(
                        &path.child("name"),
                        BadValue::String(subject.name.clone()),
                        &msg,
                    ));
                }
            }
            if !subject.api_group.is_empty() {
                all_errs.push(not_supported(
                    &path.child("apiGroup"),
                    BadValue::String(subject.api_group.clone()),
                    &[""],
                ));
            }
            if !is_namespaced && subject.namespace.is_empty() {
                all_errs.push(required(&path.child("namespace"), ""));
            }
        }
        subject_kind::USER | subject_kind::GROUP => {
            if subject.api_group != api_group::RBAC {
                all_errs.push(not_supported(
                    &path.child("apiGroup"),
                    BadValue::String(subject.api_group.clone()),
                    &[api_group::RBAC],
                ));
            }
        }
        _ => {
            all_errs.push(not_supported(
                &path.child("kind"),
                BadValue::String(subject.kind.clone()),
                &[subject_kind::SERVICE_ACCOUNT, subject_kind::USER, subject_kind::GROUP],
            ));
        }
    }

    all_errs
}

pub(crate) fn validate_label_selector(
    selector: &LabelSelector,
    allow_invalid_label_value: bool,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_labels(
        &selector.match_labels,
        &path.child("matchLabels"),
    ));

    for (i, requirement) in selector.match_expressions.iter().enumerate() {
        let req_path = path.child("matchExpressions").index(i);
        if requirement.key.is_empty() {
            all_errs.push(required(&req_path.child("key"), "key is required"));
        } else {
            all_errs.extend(validate_qualified_name(
                &requirement.key,
                &req_path.child("key"),
            ));
        }

        let operator = requirement.operator.as_str();
        match operator {
            label_selector_operator::IN | label_selector_operator::NOT_IN => {
                if requirement.values.is_empty() {
                    all_errs.push(required(
                        &req_path.child("values"),
                        "values must be non-empty for In/NotIn operators",
                    ));
                }
            }
            label_selector_operator::EXISTS | label_selector_operator::DOES_NOT_EXIST => {
                if !requirement.values.is_empty() {
                    all_errs.push(invalid(
                        &req_path.child("values"),
                        BadValue::String(format!("{:?}", requirement.values)),
                        "values must be empty for Exists/DoesNotExist operators",
                    ));
                }
            }
            _ => {
                all_errs.push(not_supported(
                    &req_path.child("operator"),
                    BadValue::String(requirement.operator.clone()),
                    &[
                        label_selector_operator::IN,
                        label_selector_operator::NOT_IN,
                        label_selector_operator::EXISTS,
                        label_selector_operator::DOES_NOT_EXIST,
                    ],
                ));
            }
        }

        if !allow_invalid_label_value {
            for (j, value) in requirement.values.iter().enumerate() {
                for msg in is_valid_label_value(value) {
                    all_errs.push(invalid(
                        &req_path.child("values").index(j),
                        BadValue::String(value.clone()),
                        &msg,
                    ));
                }
            }
        }
    }

    all_errs
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, TypeMeta};
    use crate::common::validation::ErrorType;
    use crate::rbac::internal::{ClusterRoleBinding, Role, RoleBinding, RoleRef};
    use crate::rbac::v1::subject_kind;

    #[test]
    fn role_requires_rule_verbs() {
        let role = Role {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("demo".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            rules: vec![PolicyRule::default()],
        };

        let errs = validate_role(&role);
        assert!(errs.errors.iter().any(|e| {
            e.error_type == ErrorType::Required && e.detail.contains("verbs must contain")
        }));
    }

    #[test]
    fn role_binding_rejects_invalid_role_ref_kind() {
        let role_binding = RoleBinding {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("binding".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            role_ref: RoleRef {
                api_group: api_group::RBAC.to_string(),
                kind: "InvalidKind".to_string(),
                name: "role".to_string(),
            },
            subjects: vec![],
        };

        let errs = validate_role_binding(&role_binding);
        assert!(errs.errors.iter().any(|e| {
            e.error_type == ErrorType::NotSupported && e.field.contains("roleRef.kind")
        }));
    }

    #[test]
    fn cluster_role_binding_service_account_requires_namespace() {
        let role_binding = ClusterRoleBinding {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("cluster-binding".to_string()),
                ..Default::default()
            }),
            role_ref: RoleRef {
                api_group: api_group::RBAC.to_string(),
                kind: "ClusterRole".to_string(),
                name: "cluster-admin".to_string(),
            },
            subjects: vec![Subject {
                kind: subject_kind::SERVICE_ACCOUNT.to_string(),
                api_group: "".to_string(),
                name: "default".to_string(),
                namespace: "".to_string(),
            }],
        };

        let errs = validate_cluster_role_binding(&role_binding);
        assert!(errs.errors.iter().any(|e| {
            e.error_type == ErrorType::Required && e.field.contains("subjects[0].namespace")
        }));
    }
}
