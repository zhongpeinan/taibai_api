//! Validation for Kubernetes Node API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/node/validation/validation.go

use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, invalid, is_dns1123_label, name_is_dns_subdomain,
    not_supported, validate_label_name, validate_labels, validate_object_meta,
    validate_object_meta_update,
};
use crate::core::v1::taint_effect;
use std::collections::BTreeSet;

use crate::node::v1::{RuntimeClass, RuntimeClassList};

// ============================================================================
// RuntimeClass Validation
// ============================================================================

/// Validates a RuntimeClass object.
pub fn validate_runtime_class(obj: &RuntimeClass) -> ErrorList {
    validate_runtime_class_with_path(obj, &Path::nil())
}

fn validate_runtime_class_with_path(obj: &RuntimeClass, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = obj.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Handler is required
    if obj.handler.is_empty() {
        all_errs.push(invalid(
            &base_path.child("handler"),
            crate::common::validation::BadValue::String(obj.handler.clone()),
            "handler is required",
        ));
    } else {
        // Handler must be a valid DNS 1123 label
        for msg in is_dns1123_label(&obj.handler) {
            all_errs.push(invalid(
                &base_path.child("handler"),
                crate::common::validation::BadValue::String(obj.handler.clone()),
                &msg,
            ));
        }
    }

    // Validate Overhead if present
    if let Some(ref overhead) = obj.overhead {
        all_errs.extend(validate_overhead(overhead, &base_path.child("overhead")));
    }

    // Validate Scheduling if present
    if let Some(ref scheduling) = obj.scheduling {
        all_errs.extend(validate_scheduling(
            scheduling,
            &base_path.child("scheduling"),
        ));
    }

    all_errs
}

/// Validates a RuntimeClassList object.
pub fn validate_runtime_class_list(obj: &RuntimeClassList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in obj.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_runtime_class_with_path(item, &item_path));
    }

    all_errs
}

/// Validates a RuntimeClass update.
pub fn validate_runtime_class_update(obj: &RuntimeClass, old: &RuntimeClass) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let new_meta = obj.metadata.as_ref().unwrap_or(&default_meta);
    let old_meta = old.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta_update(
        new_meta,
        old_meta,
        &Path::new("metadata"),
    ));

    if obj.handler != old.handler {
        all_errs.push(invalid(
            &Path::new("handler"),
            BadValue::String(obj.handler.clone()),
            "field is immutable",
        ));
    }

    all_errs
}

/// Validates Overhead structure.
fn validate_overhead(overhead: &crate::node::v1::Overhead, fld_path: &Path) -> ErrorList {
    validate_resource_list(&overhead.pod_fixed, &fld_path.child("podFixed"))
}

/// Validates Scheduling structure.
fn validate_scheduling(scheduling: &crate::node::v1::Scheduling, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if !scheduling.node_selector.is_empty() {
        all_errs.extend(validate_labels(
            &scheduling.node_selector,
            &fld_path.child("nodeSelector"),
        ));
    }
    all_errs.extend(validate_tolerations(
        &scheduling.tolerations,
        &fld_path.child("tolerations"),
    ));
    all_errs
}

fn validate_resource_list(resources: &crate::core::v1::ResourceList, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for (name, value) in resources {
        all_errs.extend(validate_label_name(name, &fld_path.key(name)));
        if value.0.is_empty() {
            all_errs.push(invalid(
                &fld_path.key(name),
                BadValue::String(value.0.clone()),
                "must be a valid quantity",
            ));
        }
    }
    all_errs
}

fn validate_tolerations(tolerations: &[crate::core::v1::Toleration], fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut seen: BTreeSet<(String, String, String, String)> = BTreeSet::new();
    for (i, toleration) in tolerations.iter().enumerate() {
        let idx_path = fld_path.index(i);

        if !toleration.key.is_empty() {
            all_errs.extend(validate_label_name(&toleration.key, &idx_path.child("key")));
        }

        if toleration.key.is_empty() && toleration.operator != "Exists" {
            all_errs.push(invalid(
                &idx_path.child("operator"),
                BadValue::String(toleration.operator.clone()),
                "operator must be Exists when `key` is empty, which means \"match all values and all keys\"",
            ));
        }

        if toleration.toleration_seconds.is_some() && toleration.effect != taint_effect::NO_EXECUTE
        {
            all_errs.push(invalid(
                &idx_path.child("effect"),
                BadValue::String(toleration.effect.clone()),
                "effect must be 'NoExecute' when `tolerationSeconds` is set",
            ));
        }

        match toleration.operator.as_str() {
            "" | "Equal" => {
                for msg in crate::common::validation::is_valid_label_value(&toleration.value) {
                    all_errs.push(invalid(
                        &idx_path.child("operator"),
                        BadValue::String(toleration.value.clone()),
                        &msg,
                    ));
                }
            }
            "Exists" => {
                if !toleration.value.is_empty() {
                    all_errs.push(invalid(
                        &idx_path.child("operator"),
                        BadValue::String(toleration.value.clone()),
                        "value must be empty when `operator` is 'Exists'",
                    ));
                }
            }
            _ => {
                all_errs.push(not_supported(
                    &idx_path.child("operator"),
                    BadValue::String(toleration.operator.clone()),
                    &["Equal", "Exists"],
                ));
            }
        }

        if !toleration.effect.is_empty()
            && toleration.effect != taint_effect::NO_SCHEDULE
            && toleration.effect != taint_effect::PREFER_NO_SCHEDULE
            && toleration.effect != taint_effect::NO_EXECUTE
        {
            all_errs.push(not_supported(
                &idx_path.child("effect"),
                BadValue::String(toleration.effect.clone()),
                &[
                    taint_effect::NO_SCHEDULE,
                    taint_effect::PREFER_NO_SCHEDULE,
                    taint_effect::NO_EXECUTE,
                ],
            ));
        }

        let key = (
            toleration.key.clone(),
            toleration.operator.clone(),
            toleration.value.clone(),
            toleration.effect.clone(),
        );
        if !seen.insert(key.clone()) {
            all_errs.push(duplicate(&idx_path, BadValue::String(format!("{:?}", key))));
        }
    }
    all_errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;

    #[test]
    fn test_validate_runtime_class_valid() {
        let obj = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClass".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("runtime".to_string()),
                ..Default::default()
            }),
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };

        let errs = validate_runtime_class(&obj);
        assert!(
            errs.is_empty(),
            "expected no errors, got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_runtime_class_empty_handler() {
        let obj = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClass".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("runtime".to_string()),
                ..Default::default()
            }),
            handler: "".to_string(),
            overhead: None,
            scheduling: None,
        };

        let errs = validate_runtime_class(&obj);
        assert!(!errs.is_empty());
        assert!(
            errs.errors.iter().any(|e| e.field.contains("handler")),
            "Expected 'handler', got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_runtime_class_invalid_handler_uppercase() {
        let obj = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClass".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("runtime".to_string()),
                ..Default::default()
            }),
            handler: "Runc".to_string(), // Uppercase is invalid
            overhead: None,
            scheduling: None,
        };

        let errs = validate_runtime_class(&obj);
        assert!(!errs.is_empty());
        assert!(
            errs.errors.iter().any(|e| e.field.contains("handler")),
            "Expected 'handler', got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_runtime_class_invalid_handler_starts_with_hyphen() {
        let obj = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClass".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("runtime".to_string()),
                ..Default::default()
            }),
            handler: "-runc".to_string(), // Can't start with hyphen
            overhead: None,
            scheduling: None,
        };

        let errs = validate_runtime_class(&obj);
        assert!(!errs.is_empty());
        assert!(
            errs.errors.iter().any(|e| e.field.contains("handler")),
            "Expected 'handler', got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_runtime_class_valid_handler_with_hyphen() {
        let obj = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClass".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("runtime".to_string()),
                ..Default::default()
            }),
            handler: "my-runtime".to_string(), // Hyphen in middle is valid
            overhead: None,
            scheduling: None,
        };

        let errs = validate_runtime_class(&obj);
        assert!(
            errs.is_empty(),
            "expected no errors, got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_runtime_class_with_overhead() {
        use crate::common::util::Quantity;
        use crate::core::v1::ResourceList;

        let mut pod_fixed = ResourceList::new();
        pod_fixed.insert("cpu".to_string(), Quantity("100m".to_string()));

        let overhead = crate::node::v1::Overhead { pod_fixed };

        let obj = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClass".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("runtime".to_string()),
                ..Default::default()
            }),
            handler: "runc".to_string(),
            overhead: Some(overhead),
            scheduling: None,
        };

        let errs = validate_runtime_class(&obj);
        assert!(
            errs.is_empty(),
            "expected no errors, got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_runtime_class_with_scheduling() {
        use std::collections::BTreeMap;

        let mut node_selector = BTreeMap::new();
        node_selector.insert("node-role".to_string(), "worker".to_string());

        let scheduling = crate::node::v1::Scheduling {
            node_selector,
            tolerations: vec![],
        };

        let obj = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClass".to_string(),
            },
            metadata: Some(crate::common::ObjectMeta {
                name: Some("runtime".to_string()),
                ..Default::default()
            }),
            handler: "runc".to_string(),
            overhead: None,
            scheduling: Some(scheduling),
        };

        let errs = validate_runtime_class(&obj);
        assert!(
            errs.is_empty(),
            "expected no errors, got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_runtime_class_list_item_index() {
        let obj = RuntimeClassList {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClassList".to_string(),
            },
            metadata: None,
            items: vec![RuntimeClass {
                type_meta: TypeMeta {
                    api_version: "node.k8s.io/v1".to_string(),
                    kind: "RuntimeClass".to_string(),
                },
                metadata: Some(crate::common::ObjectMeta {
                    name: Some("runtime".to_string()),
                    ..Default::default()
                }),
                handler: "".to_string(), // Invalid: empty handler
                overhead: None,
                scheduling: None,
            }],
        };

        let errs = validate_runtime_class_list(&obj);
        assert!(!errs.is_empty());
        // The error should reference items[0].handler, not just handler
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("items[0].handler"))
        );
    }

    #[test]
    fn test_validate_runtime_class_update_handler_immutable() {
        let old = RuntimeClass {
            type_meta: TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("runtime".to_string()),
                resource_version: Some("1".to_string()),
                ..Default::default()
            }),
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };

        let mut new = old.clone();
        new.handler = "alternate".to_string();

        let errs = validate_runtime_class_update(&new, &old);
        assert!(errs.errors.iter().any(|e| e.field == "handler"));
    }

    #[test]
    fn test_validate_runtime_class_update_requires_resource_version() {
        let old = RuntimeClass {
            type_meta: TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("runtime".to_string()),
                resource_version: Some("1".to_string()),
                ..Default::default()
            }),
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };

        let mut new = old.clone();
        if let Some(ref mut meta) = new.metadata {
            meta.resource_version = None;
        }

        let errs = validate_runtime_class_update(&new, &old);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("metadata.resourceVersion")),
            "expected resourceVersion error but got {errs:?}"
        );
    }
}
