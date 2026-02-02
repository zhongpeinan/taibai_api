//! Conversions between v1 and internal node types

use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};
use crate::core::internal::{TaintEffect, TolerationOperator};
use crate::node::internal;

use super::{Overhead, RuntimeClass, RuntimeClassList, Scheduling};

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

fn to_internal_toleration(value: crate::core::v1::Toleration) -> crate::core::internal::Toleration {
    let operator = match value.operator.as_str() {
        "" => None,
        "Exists" => Some(TolerationOperator::Exists),
        "Equal" => Some(TolerationOperator::Equal),
        _ => None,
    };
    let effect = match value.effect.as_str() {
        "" => None,
        "NoSchedule" => Some(TaintEffect::NoSchedule),
        "PreferNoSchedule" => Some(TaintEffect::PreferNoSchedule),
        "NoExecute" => Some(TaintEffect::NoExecute),
        _ => None,
    };
    crate::core::internal::Toleration {
        key: value.key,
        operator,
        value: value.value,
        effect,
        toleration_seconds: value.toleration_seconds,
    }
}

fn from_internal_toleration(
    value: crate::core::internal::Toleration,
) -> crate::core::v1::Toleration {
    let operator = match value.operator {
        Some(TolerationOperator::Exists) => "Exists".to_string(),
        Some(TolerationOperator::Equal) => "Equal".to_string(),
        None => String::new(),
    };
    let effect = match value.effect {
        Some(TaintEffect::NoSchedule) => "NoSchedule".to_string(),
        Some(TaintEffect::PreferNoSchedule) => "PreferNoSchedule".to_string(),
        Some(TaintEffect::NoExecute) => "NoExecute".to_string(),
        None => String::new(),
    };
    crate::core::v1::Toleration {
        key: value.key,
        operator,
        value: value.value,
        effect,
        toleration_seconds: value.toleration_seconds,
    }
}

// ============================================================================
// RuntimeClass Conversions
// ============================================================================

impl ToInternal<internal::RuntimeClass> for RuntimeClass {
    fn to_internal(self) -> internal::RuntimeClass {
        internal::RuntimeClass {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            handler: self.handler,
            overhead: self.overhead.map(|value| internal::Overhead {
                pod_fixed: value.pod_fixed,
            }),
            scheduling: self.scheduling.map(|value| internal::Scheduling {
                node_selector: value.node_selector,
                tolerations: value
                    .tolerations
                    .into_iter()
                    .map(to_internal_toleration)
                    .collect(),
            }),
        }
    }
}

impl FromInternal<internal::RuntimeClass> for RuntimeClass {
    fn from_internal(value: internal::RuntimeClass) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            handler: value.handler,
            overhead: value.overhead.map(|value| Overhead {
                pod_fixed: value.pod_fixed,
            }),
            scheduling: value.scheduling.map(|value| Scheduling {
                node_selector: value.node_selector,
                tolerations: value
                    .tolerations
                    .into_iter()
                    .map(from_internal_toleration)
                    .collect(),
            }),
        };

        result
    }
}

// ============================================================================
// RuntimeClassList Conversions
// ============================================================================

impl ToInternal<internal::RuntimeClassList> for RuntimeClassList {
    fn to_internal(self) -> internal::RuntimeClassList {
        internal::RuntimeClassList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(ToInternal::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::RuntimeClassList> for RuntimeClassList {
    fn from_internal(value: internal::RuntimeClassList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(RuntimeClass::from_internal)
                .collect(),
        };

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{FromInternal, ToInternal};

    #[test]
    fn test_runtime_class_round_trip() {
        let v1 = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClass".to_string(),
            },
            metadata: None,
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };

        let internal = v1.clone().to_internal();
        let mut back = RuntimeClass::from_internal(internal);
        back.apply_default();

        assert_eq!(back.handler, "runc");
        // Verify that apply_default was called - TypeMeta should be populated
        assert_eq!(back.type_meta.api_version, "node.k8s.io/v1");
        assert_eq!(back.type_meta.kind, "RuntimeClass");
    }

    #[test]
    fn test_runtime_class_with_overhead_round_trip() {
        use crate::common::util::Quantity;

        let mut overhead = Overhead::default();
        overhead
            .pod_fixed
            .insert("cpu".to_string(), Quantity("100m".to_string()));

        let v1 = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClass".to_string(),
            },
            metadata: None,
            handler: "runc".to_string(),
            overhead: Some(overhead.clone()),
            scheduling: None,
        };

        let internal = v1.clone().to_internal();
        let mut back = RuntimeClass::from_internal(internal);
        back.apply_default();

        assert_eq!(back.handler, "runc");
        assert!(back.overhead.is_some());
        let back_overhead = back.overhead.as_ref().unwrap();
        assert_eq!(
            back_overhead.pod_fixed.get("cpu").map(|q| q.0.as_str()),
            Some("100m")
        );
    }

    #[test]
    fn test_runtime_class_list_round_trip() {
        let v1 = RuntimeClassList {
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
                metadata: None,
                handler: "runc".to_string(),
                overhead: None,
                scheduling: None,
            }],
        };

        let internal = v1.clone().to_internal();
        let mut back = RuntimeClassList::from_internal(internal);
        back.apply_default();

        assert_eq!(back.items.len(), 1);
        assert_eq!(back.items[0].handler, "runc");
        // Verify that apply_default was called
        assert_eq!(back.type_meta.api_version, "node.k8s.io/v1");
        assert_eq!(back.type_meta.kind, "RuntimeClassList");
    }

    #[test]
    fn test_runtime_class_list_apply_default() {
        let internal = internal::RuntimeClassList {
            type_meta: TypeMeta::default(),
            metadata: ListMeta::default(),
            items: vec![],
        };

        let mut v1: RuntimeClassList = RuntimeClassList::from_internal(internal);
        v1.apply_default();
        // Verify that apply_default was called
        assert_eq!(v1.type_meta.api_version, "node.k8s.io/v1");
        assert_eq!(v1.type_meta.kind, "RuntimeClassList");
    }

    #[test]
    fn test_runtime_class_empty_metadata_conversion() {
        let v1 = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClass".to_string(),
            },
            metadata: None,
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };

        let internal = v1.to_internal();
        let mut back = RuntimeClass::from_internal(internal);
        back.apply_default();

        // Empty metadata should stay None
        assert!(back.metadata.is_none());
    }

    #[test]
    fn test_runtime_class_with_metadata_conversion() {
        use crate::common::ObjectMeta;

        let metadata = ObjectMeta {
            name: Some("test-runtime-class".to_string()),
            ..Default::default()
        };

        let v1 = RuntimeClass {
            type_meta: TypeMeta {
                api_version: "node.k8s.io/v1".to_string(),
                kind: "RuntimeClass".to_string(),
            },
            metadata: Some(metadata.clone()),
            handler: "runc".to_string(),
            overhead: None,
            scheduling: None,
        };

        let internal = v1.to_internal();
        let mut back = RuntimeClass::from_internal(internal);
        back.apply_default();

        // Non-empty metadata should be preserved
        assert!(back.metadata.is_some());
        assert_eq!(
            back.metadata.as_ref().unwrap().name,
            Some("test-runtime-class".to_string())
        );
    }
}
