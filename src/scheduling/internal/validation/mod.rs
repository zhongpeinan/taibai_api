//! Validation helpers for the scheduling API (internal types).
//!
//! Ported from `k8s/pkg/apis/scheduling/validation/validation.go`.

use crate::common::ObjectMeta;
use crate::common::validation::{
    ErrorList, Path, forbidden, name_is_dns_subdomain, validate_object_meta,
    validate_object_meta_update,
};
use crate::scheduling::internal::{PriorityClass, PriorityClassList};

// ============================================================================
// Constants
// ============================================================================

/// DefaultPriorityWhenNoDefaultClassExists is used to set priority of pods
/// that do not specify any priority class and there is no priority class
/// marked as default.
const DEFAULT_PRIORITY_WHEN_NO_DEFAULT_CLASS_EXISTS: i32 = 0;

/// HighestUserDefinablePriority is the highest priority for user defined
/// priority classes. Priority values larger than 1 billion are reserved
/// for Kubernetes system use.
const HIGHEST_USER_DEFINABLE_PRIORITY: i32 = 1_000_000_000;

/// SystemCriticalPriority is the beginning of the range of priority values
/// for critical system components.
const SYSTEM_CRITICAL_PRIORITY: i32 = 2 * HIGHEST_USER_DEFINABLE_PRIORITY;

/// SystemPriorityClassPrefix is the prefix reserved for system priority class names.
const SYSTEM_PRIORITY_CLASS_PREFIX: &str = "system-";

/// SystemClusterCritical is the system priority class name that represents cluster-critical.
const SYSTEM_CLUSTER_CRITICAL: &str = "system-cluster-critical";

/// SystemNodeCritical is the system priority class name that represents node-critical.
const SYSTEM_NODE_CRITICAL: &str = "system-node-critical";

fn metadata_name(meta: &ObjectMeta) -> &str {
    meta.name.as_deref().unwrap_or("")
}

fn priority_class_value(obj: &PriorityClass) -> i32 {
    if obj.value == 0 {
        DEFAULT_PRIORITY_WHEN_NO_DEFAULT_CLASS_EXISTS
    } else {
        obj.value
    }
}

fn is_known_system_priority_class(
    name: &str,
    value: i32,
    global_default: bool,
) -> Result<(), String> {
    let check = |expected_name: &str, expected_value: i32| -> Result<(), String> {
        if value != expected_value {
            return Err(format!(
                "value of {} PriorityClass must be {}",
                expected_name, expected_value
            ));
        }
        if global_default {
            return Err(format!(
                "globalDefault of {} PriorityClass must be false",
                expected_name
            ));
        }
        Ok(())
    };

    match name {
        SYSTEM_NODE_CRITICAL => check(SYSTEM_NODE_CRITICAL, SYSTEM_CRITICAL_PRIORITY + 1000),
        SYSTEM_CLUSTER_CRITICAL => check(SYSTEM_CLUSTER_CRITICAL, SYSTEM_CRITICAL_PRIORITY),
        _ => Err(format!("{name} is not a known system priority class")),
    }
}

/// Validates a scheduling internal PriorityClass.
pub fn validate_priority_class(obj: &PriorityClass) -> ErrorList {
    validate_priority_class_with_path(obj, &Path::nil())
}

fn validate_priority_class_with_path(obj: &PriorityClass, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let meta = &obj.metadata;

    all_errs.extend(validate_object_meta(
        meta,
        false,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    let name = metadata_name(meta);

    if name.starts_with(SYSTEM_PRIORITY_CLASS_PREFIX) {
        if let Err(err) =
            is_known_system_priority_class(name, priority_class_value(obj), obj.global_default)
        {
            let msg = format!(
                "priority class names with '{}' prefix are reserved for system use only. error: {err}",
                SYSTEM_PRIORITY_CLASS_PREFIX
            );
            all_errs.push(forbidden(&base_path.child("metadata").child("name"), &msg));
        }
    } else if priority_class_value(obj) > HIGHEST_USER_DEFINABLE_PRIORITY {
        let detail = format!(
            "maximum allowed value of a user defined priority is {}",
            HIGHEST_USER_DEFINABLE_PRIORITY
        );
        all_errs.push(forbidden(&base_path.child("value"), &detail));
    }

    all_errs
}

/// Validates a scheduling internal PriorityClassList.
pub fn validate_priority_class_list(obj: &PriorityClassList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in obj.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_priority_class_with_path(item, &item_path));
    }

    all_errs
}

/// Ensures immutable fields are not changed on PriorityClass updates.
pub fn validate_priority_class_update(obj: &PriorityClass, old: &PriorityClass) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let new_meta = &obj.metadata;
    let old_meta = &old.metadata;

    all_errs.extend(validate_object_meta_update(
        new_meta,
        old_meta,
        &Path::new("metadata"),
    ));

    if obj.value != old.value {
        all_errs.push(forbidden(
            &Path::new("value"),
            "may not be changed in an update.",
        ));
    }

    if obj.preemption_policy != old.preemption_policy {
        all_errs.push(forbidden(
            &Path::new("preemptionPolicy"),
            "may not be changed in an update.",
        ));
    }

    all_errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::validation::ErrorType;
    use crate::core::internal::PreemptionPolicy;

    fn base_priority_class() -> PriorityClass {
        PriorityClass {
            type_meta: crate::common::TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("custom".to_string()),
                resource_version: Some("1".to_string()),
                ..Default::default()
            },
            value: 10,
            global_default: false,
            description: String::new(),
            preemption_policy: Some(PreemptionPolicy::PreemptLowerPriority),
        }
    }

    #[test]
    fn test_validate_priority_class_success() {
        let obj = base_priority_class();
        let errs = validate_priority_class(&obj);
        assert!(errs.is_empty());
    }

    #[test]
    fn test_validate_priority_class_system_prefix() {
        let obj = PriorityClass {
            metadata: ObjectMeta {
                name: Some("system-foo".to_string()),
                ..Default::default()
            },
            value: SYSTEM_CRITICAL_PRIORITY,
            global_default: false,
            ..base_priority_class()
        };

        let errs = validate_priority_class(&obj);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.ends_with("metadata.name")),
            "expected metadata.name error but got {errs:?}"
        );
    }

    #[test]
    fn test_validate_priority_class_value_too_high() {
        let obj = PriorityClass {
            value: HIGHEST_USER_DEFINABLE_PRIORITY + 1,
            ..base_priority_class()
        };
        let errs = validate_priority_class(&obj);
        assert!(
            errs.errors
                .iter()
                .any(|e| { e.field.ends_with("value") && e.error_type == ErrorType::Forbidden }),
            "expected forbidden error on value but got {errs:?}"
        );
    }

    #[test]
    fn test_validate_priority_class_update_changes() {
        let mut old = base_priority_class();
        old.preemption_policy = Some(PreemptionPolicy::Never);

        let mut new = base_priority_class();
        new.value = 200;
        new.preemption_policy = Some(PreemptionPolicy::PreemptLowerPriority);

        let errs = validate_priority_class_update(&new, &old);
        assert_eq!(errs.len(), 2);
    }

    #[test]
    fn test_validate_priority_class_update_requires_resource_version() {
        let old = base_priority_class();
        let mut new = base_priority_class();
        new.metadata.resource_version = None;

        let errs = validate_priority_class_update(&new, &old);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.ends_with("metadata.resourceVersion")),
            "expected resourceVersion error but got {errs:?}"
        );
    }
}
