//! Validation helpers for the scheduling API.
//!
//! Ported from `k8s/pkg/apis/scheduling/validation/validation.go`.

use crate::common::ObjectMeta;
use crate::common::validation::{
    ErrorList, Path, forbidden, name_is_dns_subdomain, validate_object_meta,
    validate_object_meta_update,
};
use crate::scheduling::v1::{
    DEFAULT_PRIORITY_WHEN_NO_DEFAULT_CLASS_EXISTS, HIGHEST_USER_DEFINABLE_PRIORITY, PriorityClass,
    SYSTEM_CLUSTER_CRITICAL, SYSTEM_CRITICAL_PRIORITY, SYSTEM_NODE_CRITICAL,
    SYSTEM_PRIORITY_CLASS_PREFIX,
};

fn metadata_name(meta: &Option<ObjectMeta>) -> &str {
    meta.as_ref().and_then(|m| m.name.as_deref()).unwrap_or("")
}

fn priority_class_value(obj: &PriorityClass) -> i32 {
    obj.value
        .unwrap_or(DEFAULT_PRIORITY_WHEN_NO_DEFAULT_CLASS_EXISTS)
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

/// Validates a scheduling v1 PriorityClass.
pub fn validate_priority_class(obj: &PriorityClass) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = ObjectMeta::default();
    let meta = obj.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        false,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    ));

    let name = metadata_name(&obj.metadata);

    if name.starts_with(SYSTEM_PRIORITY_CLASS_PREFIX) {
        if let Err(err) =
            is_known_system_priority_class(name, priority_class_value(obj), obj.global_default)
        {
            let msg = format!(
                "priority class names with '{}' prefix are reserved for system use only. error: {err}",
                SYSTEM_PRIORITY_CLASS_PREFIX
            );
            all_errs.push(forbidden(&Path::new("metadata").child("name"), &msg));
        }
    } else if priority_class_value(obj) > HIGHEST_USER_DEFINABLE_PRIORITY {
        let detail = format!(
            "maximum allowed value of a user defined priority is {}",
            HIGHEST_USER_DEFINABLE_PRIORITY
        );
        all_errs.push(forbidden(&Path::new("value"), &detail));
    }

    all_errs
}

/// Ensures immutable fields are not changed on PriorityClass updates.
pub fn validate_priority_class_update(obj: &PriorityClass, old: &PriorityClass) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = ObjectMeta::default();
    let new_meta = obj.metadata.as_ref().unwrap_or(&default_meta);
    let old_meta = old.metadata.as_ref().unwrap_or(&default_meta);

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
    use crate::common::ObjectMeta;
    use crate::common::validation::ErrorType;
    use crate::core::internal::PreemptionPolicy;

    fn base_priority_class() -> PriorityClass {
        PriorityClass {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("custom".to_string()),
                resource_version: Some("1".to_string()),
                ..Default::default()
            }),
            value: Some(10),
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
            metadata: Some(ObjectMeta {
                name: Some("system-foo".to_string()),
                ..Default::default()
            }),
            value: Some(SYSTEM_CRITICAL_PRIORITY),
            global_default: false,
            ..base_priority_class()
        };

        let errs = validate_priority_class(&obj);
        assert!(
            errs.errors.iter().any(|e| e.field == "metadata.name"),
            "expected metadata.name error but got {errs:?}"
        );
    }

    #[test]
    fn test_validate_priority_class_value_too_high() {
        let obj = PriorityClass {
            value: Some(HIGHEST_USER_DEFINABLE_PRIORITY + 1),
            ..base_priority_class()
        };
        let errs = validate_priority_class(&obj);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field == "value" && e.error_type == ErrorType::Forbidden),
            "expected forbidden error on value but got {errs:?}"
        );
    }

    #[test]
    fn test_validate_priority_class_update_changes() {
        let mut old = base_priority_class();
        old.preemption_policy = Some(PreemptionPolicy::Never);

        let mut new = base_priority_class();
        new.value = Some(200);
        new.preemption_policy = Some(PreemptionPolicy::PreemptLowerPriority);

        let errs = validate_priority_class_update(&new, &old);
        assert_eq!(errs.len(), 2);
    }

    #[test]
    fn test_validate_priority_class_update_requires_resource_version() {
        let old = base_priority_class();
        let mut new = base_priority_class();
        if let Some(ref mut meta) = new.metadata {
            meta.resource_version = None;
        }

        let errs = validate_priority_class_update(&new, &old);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field == "metadata.resourceVersion"),
            "expected resourceVersion error but got {errs:?}"
        );
    }
}
