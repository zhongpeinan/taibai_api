//! Conversions between v1 and internal scheduling types

use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};

use super::{PriorityClass, PriorityClassList};

// Internal types are re-exports from v1 (as per scheduling/internal/mod.rs)
// Since internal and v1 are the same, we create internal wrapper types for conversion

mod internal {
    use super::*;

    /// Internal PriorityClass type (identical to v1 but without TypeMeta)
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct PriorityClassInternal {
        pub metadata: ObjectMeta,
        pub value: Option<i32>,
        pub global_default: bool,
        pub description: String,
        pub preemption_policy: Option<crate::core::internal::PreemptionPolicy>,
    }

    /// Internal PriorityClassList type (identical to v1 but without TypeMeta)
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct PriorityClassListInternal {
        pub metadata: ListMeta,
        pub items: Vec<PriorityClassInternal>,
    }
}

// Re-export internal types for public API
pub use internal::{PriorityClassInternal, PriorityClassListInternal};

// ============================================================================
// Conversion Helper Functions
// ============================================================================

fn is_empty_object_meta(meta: &ObjectMeta) -> bool {
    meta.name.is_none()
        && meta.generate_name.is_none()
        && meta.namespace.is_none()
        && meta.uid.is_none()
        && meta.resource_version.is_none()
        && meta.generation.is_none()
        && meta.self_link.is_none()
        && meta.labels.is_empty()
        && meta.annotations.is_empty()
        && meta.owner_references.is_empty()
        && meta.finalizers.is_empty()
        && meta.managed_fields.is_empty()
        && meta.creation_timestamp.is_none()
        && meta.deletion_timestamp.is_none()
        && meta.deletion_grace_period_seconds.is_none()
}

fn is_empty_list_meta(meta: &ListMeta) -> bool {
    meta.continue_.is_none()
        && meta.remaining_item_count.is_none()
        && meta.resource_version.is_none()
        && meta.self_link.is_none()
}

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_list_meta_to_meta(meta: Option<ListMeta>) -> ListMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_list_meta(meta: ListMeta) -> Option<ListMeta> {
    if is_empty_list_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

// ============================================================================
// PriorityClass Conversions
// ============================================================================

impl ToInternal<PriorityClassInternal> for PriorityClass {
    fn to_internal(self) -> PriorityClassInternal {
        PriorityClassInternal {
            metadata: option_object_meta_to_meta(self.metadata),
            value: self.value,
            global_default: self.global_default,
            description: self.description,
            preemption_policy: self.preemption_policy.map(Into::into),
        }
    }
}

impl FromInternal<PriorityClassInternal> for PriorityClass {
    fn from_internal(value: PriorityClassInternal) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            value: value.value,
            global_default: value.global_default,
            description: value.description,
            preemption_policy: value.preemption_policy.map(Into::into),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// PriorityClassList Conversions
// ============================================================================

impl ToInternal<PriorityClassListInternal> for PriorityClassList {
    fn to_internal(self) -> PriorityClassListInternal {
        PriorityClassListInternal {
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<PriorityClassListInternal> for PriorityClassList {
    fn from_internal(value: PriorityClassListInternal) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(PriorityClass::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_class_round_trip() {
        let v1 = PriorityClass {
            type_meta: TypeMeta {
                api_version: "scheduling.k8s.io/v1".to_string(),
                kind: "PriorityClass".to_string(),
            },
            metadata: None,
            value: Some(1000),
            global_default: false,
            description: "test priority class".to_string(),
            preemption_policy: None,
        };

        let internal = v1.clone().to_internal();
        let back = PriorityClass::from_internal(internal);

        assert_eq!(back.value, Some(1000));
        assert_eq!(back.description, "test priority class");
        // Verify that apply_default was called
        assert_eq!(back.type_meta.api_version, "scheduling.k8s.io/v1");
        assert_eq!(back.type_meta.kind, "PriorityClass");
    }
}
