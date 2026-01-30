//! Conversions between apps v1 and internal types
//!
//! Based on k8s.io/kubernetes/pkg/apis/apps/v1/conversion.go

use crate::apps::internal;
use crate::apps::v1::{
    ReplicaSet, ReplicaSetCondition, ReplicaSetConditionType, ReplicaSetList, ReplicaSetSpec,
    ReplicaSetStatus,
};
use crate::common::Timestamp;
use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};
use crate::core::internal::ConditionStatus;
use crate::core::v1::PodTemplateSpec;

// ============================================================================
// Metadata helpers
// ============================================================================

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

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

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_list_meta_to_object_meta(meta: Option<ListMeta>) -> ObjectMeta {
    meta.map_or_else(ObjectMeta::default, list_meta_to_object_meta)
}

fn object_meta_to_option_list_meta(meta: ObjectMeta) -> Option<ListMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(object_meta_to_list_meta(meta))
    }
}

fn list_meta_to_object_meta(meta: ListMeta) -> ObjectMeta {
    ObjectMeta {
        resource_version: meta.resource_version,
        self_link: meta.self_link,
        ..Default::default()
    }
}

fn object_meta_to_list_meta(meta: ObjectMeta) -> ListMeta {
    ListMeta {
        resource_version: meta.resource_version,
        self_link: meta.self_link,
        ..Default::default()
    }
}

// ============================================================================
// Conversion helpers
// ============================================================================

fn convert_condition_status_to_internal(status: String) -> ConditionStatus {
    match status.as_str() {
        "True" => ConditionStatus::True,
        "False" => ConditionStatus::False,
        "Unknown" => ConditionStatus::Unknown,
        _ => ConditionStatus::Unknown,
    }
}

fn convert_condition_status_to_v1(status: ConditionStatus) -> String {
    match status {
        ConditionStatus::True => "True".to_string(),
        ConditionStatus::False => "False".to_string(),
        ConditionStatus::Unknown => "Unknown".to_string(),
    }
}

fn option_string_to_timestamp(value: Option<String>) -> Option<Timestamp> {
    value.and_then(|s| Timestamp::from_str(&s).ok())
}

fn option_timestamp_to_string(value: Option<Timestamp>) -> Option<String> {
    value.map(|ts| ts.to_rfc3339())
}

fn convert_replica_set_condition_type_v1_to_internal(
    value: ReplicaSetConditionType,
) -> internal::ReplicaSetConditionType {
    match value {
        ReplicaSetConditionType::ReplicaFailure => {
            internal::ReplicaSetConditionType::ReplicaFailure
        }
    }
}

fn convert_replica_set_condition_type_internal_to_v1(
    value: internal::ReplicaSetConditionType,
) -> ReplicaSetConditionType {
    match value {
        internal::ReplicaSetConditionType::ReplicaFailure => {
            ReplicaSetConditionType::ReplicaFailure
        }
    }
}

fn convert_replica_set_condition_v1_to_internal(
    value: ReplicaSetCondition,
) -> internal::ReplicaSetCondition {
    internal::ReplicaSetCondition {
        r#type: convert_replica_set_condition_type_v1_to_internal(value.r#type),
        status: convert_condition_status_to_internal(value.status),
        last_transition_time: option_string_to_timestamp(value.last_transition_time),
        reason: value.reason,
        message: value.message,
    }
}

fn convert_replica_set_condition_internal_to_v1(
    value: internal::ReplicaSetCondition,
) -> ReplicaSetCondition {
    ReplicaSetCondition {
        r#type: convert_replica_set_condition_type_internal_to_v1(value.r#type),
        status: convert_condition_status_to_v1(value.status),
        last_transition_time: option_timestamp_to_string(value.last_transition_time),
        reason: value.reason,
        message: value.message,
    }
}

fn convert_replica_set_spec_v1_to_internal(spec: ReplicaSetSpec) -> internal::ReplicaSetSpec {
    internal::ReplicaSetSpec {
        replicas: spec.replicas.unwrap_or_default(),
        min_ready_seconds: spec.min_ready_seconds.unwrap_or_default(),
        selector: spec.selector,
        template: spec.template.map(|template| template.to_internal()),
    }
}

fn convert_replica_set_spec_internal_to_v1(spec: internal::ReplicaSetSpec) -> ReplicaSetSpec {
    ReplicaSetSpec {
        replicas: Some(spec.replicas),
        min_ready_seconds: Some(spec.min_ready_seconds),
        selector: spec.selector,
        template: spec.template.map(PodTemplateSpec::from_internal),
    }
}

fn convert_replica_set_status_v1_to_internal(
    status: ReplicaSetStatus,
) -> internal::ReplicaSetStatus {
    internal::ReplicaSetStatus {
        replicas: status.replicas,
        fully_labeled_replicas: status.fully_labeled_replicas.unwrap_or_default(),
        ready_replicas: status.ready_replicas.unwrap_or_default(),
        available_replicas: status.available_replicas.unwrap_or_default(),
        terminating_replicas: status.terminating_replicas,
        observed_generation: status.observed_generation.unwrap_or_default(),
        conditions: status
            .conditions
            .into_iter()
            .map(convert_replica_set_condition_v1_to_internal)
            .collect(),
    }
}

fn convert_replica_set_status_internal_to_v1(
    status: internal::ReplicaSetStatus,
) -> ReplicaSetStatus {
    ReplicaSetStatus {
        replicas: status.replicas,
        fully_labeled_replicas: Some(status.fully_labeled_replicas),
        ready_replicas: Some(status.ready_replicas),
        available_replicas: Some(status.available_replicas),
        terminating_replicas: status.terminating_replicas,
        observed_generation: Some(status.observed_generation),
        conditions: status
            .conditions
            .into_iter()
            .map(convert_replica_set_condition_internal_to_v1)
            .collect(),
    }
}

// ============================================================================
// ReplicaSet Conversions
// ============================================================================

impl ToInternal<internal::ReplicaSet> for ReplicaSet {
    fn to_internal(self) -> internal::ReplicaSet {
        internal::ReplicaSet {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(convert_replica_set_spec_v1_to_internal),
            status: self.status.map(convert_replica_set_status_v1_to_internal),
        }
    }
}

impl FromInternal<internal::ReplicaSet> for ReplicaSet {
    fn from_internal(value: internal::ReplicaSet) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(convert_replica_set_spec_internal_to_v1),
            status: value.status.map(convert_replica_set_status_internal_to_v1),
        };
        result.apply_default();
        result
    }
}

impl ToInternal<internal::ReplicaSetList> for ReplicaSetList {
    fn to_internal(self) -> internal::ReplicaSetList {
        internal::ReplicaSetList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_object_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::ReplicaSetList> for ReplicaSetList {
    fn from_internal(value: internal::ReplicaSetList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: object_meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(ReplicaSet::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// Deployment Conversions (placeholder)
// ============================================================================

// Additional app resource conversions will be added in follow-up batches.
