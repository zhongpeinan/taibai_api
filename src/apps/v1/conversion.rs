//! Conversions between apps v1 and internal types
//!
//! Based on k8s.io/kubernetes/pkg/apis/apps/v1/conversion.go

use crate::apps::internal;
use crate::apps::v1::{
    ControllerRevision, ControllerRevisionList, DaemonSet, DaemonSetCondition,
    DaemonSetConditionType, DaemonSetList, DaemonSetSpec, DaemonSetStatus, DaemonSetUpdateStrategy,
    DaemonSetUpdateStrategyType, Deployment, DeploymentCondition, DeploymentConditionType,
    DeploymentList, DeploymentSpec, DeploymentStatus, DeploymentStrategy, DeploymentStrategyType,
    PersistentVolumeClaimRetentionPolicyType, PodManagementPolicyType, ReplicaSet,
    ReplicaSetCondition, ReplicaSetConditionType, ReplicaSetList, ReplicaSetSpec, ReplicaSetStatus,
    RollingUpdateDaemonSet, RollingUpdateDeployment, RollingUpdateStatefulSetStrategy, StatefulSet,
    StatefulSetCondition, StatefulSetConditionType, StatefulSetList, StatefulSetOrdinals,
    StatefulSetPersistentVolumeClaimRetentionPolicy, StatefulSetSpec, StatefulSetStatus,
    StatefulSetUpdateStrategy, StatefulSetUpdateStrategyType,
};
use crate::common::Timestamp;
use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};
use crate::core::internal::ConditionStatus;
use crate::core::v1::{PersistentVolumeClaim, PodTemplateSpec};

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

fn option_string_to_timestamp_or_default(value: Option<String>) -> Timestamp {
    option_string_to_timestamp(value).unwrap_or_default()
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
// Deployment Conversion Helpers
// ============================================================================

fn convert_deployment_condition_type_v1_to_internal(
    value: DeploymentConditionType,
) -> internal::DeploymentConditionType {
    match value {
        DeploymentConditionType::Available => internal::DeploymentConditionType::Available,
        DeploymentConditionType::Progressing => internal::DeploymentConditionType::Progressing,
        DeploymentConditionType::ReplicaFailure => {
            internal::DeploymentConditionType::ReplicaFailure
        }
    }
}

fn convert_deployment_condition_type_internal_to_v1(
    value: internal::DeploymentConditionType,
) -> DeploymentConditionType {
    match value {
        internal::DeploymentConditionType::Available => DeploymentConditionType::Available,
        internal::DeploymentConditionType::Progressing => DeploymentConditionType::Progressing,
        internal::DeploymentConditionType::ReplicaFailure => {
            DeploymentConditionType::ReplicaFailure
        }
    }
}

fn convert_deployment_condition_v1_to_internal(
    value: DeploymentCondition,
) -> internal::DeploymentCondition {
    internal::DeploymentCondition {
        r#type: convert_deployment_condition_type_v1_to_internal(value.r#type),
        status: convert_condition_status_to_internal(value.status),
        last_update_time: option_string_to_timestamp_or_default(value.last_update_time),
        last_transition_time: option_string_to_timestamp_or_default(value.last_transition_time),
        reason: value.reason,
        message: value.message,
    }
}

fn convert_deployment_condition_internal_to_v1(
    value: internal::DeploymentCondition,
) -> DeploymentCondition {
    DeploymentCondition {
        r#type: convert_deployment_condition_type_internal_to_v1(value.r#type),
        status: convert_condition_status_to_v1(value.status),
        last_update_time: Some(value.last_update_time.to_rfc3339()),
        last_transition_time: Some(value.last_transition_time.to_rfc3339()),
        reason: value.reason,
        message: value.message,
    }
}

fn convert_rolling_update_deployment_v1_to_internal(
    value: RollingUpdateDeployment,
) -> internal::RollingUpdateDeployment {
    internal::RollingUpdateDeployment {
        max_unavailable: value.max_unavailable.unwrap_or_default(),
        max_surge: value.max_surge.unwrap_or_default(),
    }
}

fn convert_rolling_update_deployment_internal_to_v1(
    value: internal::RollingUpdateDeployment,
) -> RollingUpdateDeployment {
    RollingUpdateDeployment {
        max_unavailable: Some(value.max_unavailable),
        max_surge: Some(value.max_surge),
    }
}

fn convert_deployment_strategy_v1_to_internal(
    value: DeploymentStrategy,
) -> internal::DeploymentStrategy {
    internal::DeploymentStrategy {
        r#type: match value.r#type.unwrap_or_default() {
            DeploymentStrategyType::Recreate => internal::DeploymentStrategyType::Recreate,
            DeploymentStrategyType::RollingUpdate => {
                internal::DeploymentStrategyType::RollingUpdate
            }
        },
        rolling_update: value
            .rolling_update
            .map(convert_rolling_update_deployment_v1_to_internal),
    }
}

fn convert_deployment_strategy_internal_to_v1(
    value: internal::DeploymentStrategy,
) -> DeploymentStrategy {
    DeploymentStrategy {
        r#type: Some(match value.r#type {
            internal::DeploymentStrategyType::Recreate => DeploymentStrategyType::Recreate,
            internal::DeploymentStrategyType::RollingUpdate => {
                DeploymentStrategyType::RollingUpdate
            }
        }),
        rolling_update: value
            .rolling_update
            .map(convert_rolling_update_deployment_internal_to_v1),
    }
}

fn convert_deployment_spec_v1_to_internal(spec: DeploymentSpec) -> internal::DeploymentSpec {
    internal::DeploymentSpec {
        replicas: spec.replicas.unwrap_or_default(),
        selector: spec.selector,
        template: spec.template.unwrap_or_default().to_internal(),
        strategy: spec
            .strategy
            .map(convert_deployment_strategy_v1_to_internal)
            .unwrap_or_default(),
        min_ready_seconds: spec.min_ready_seconds.unwrap_or_default(),
        revision_history_limit: spec.revision_history_limit,
        paused: spec.paused,
        rollback_to: None,
        progress_deadline_seconds: spec.progress_deadline_seconds,
    }
}

fn convert_deployment_spec_internal_to_v1(spec: internal::DeploymentSpec) -> DeploymentSpec {
    DeploymentSpec {
        replicas: Some(spec.replicas),
        selector: spec.selector,
        template: Some(PodTemplateSpec::from_internal(spec.template)),
        strategy: Some(convert_deployment_strategy_internal_to_v1(spec.strategy)),
        min_ready_seconds: Some(spec.min_ready_seconds),
        revision_history_limit: spec.revision_history_limit,
        paused: spec.paused,
        progress_deadline_seconds: spec.progress_deadline_seconds,
    }
}

fn convert_deployment_status_v1_to_internal(
    status: DeploymentStatus,
) -> internal::DeploymentStatus {
    internal::DeploymentStatus {
        observed_generation: status.observed_generation.unwrap_or_default(),
        replicas: status.replicas.unwrap_or_default(),
        updated_replicas: status.updated_replicas.unwrap_or_default(),
        ready_replicas: status.ready_replicas.unwrap_or_default(),
        available_replicas: status.available_replicas.unwrap_or_default(),
        unavailable_replicas: status.unavailable_replicas.unwrap_or_default(),
        terminating_replicas: status.terminating_replicas,
        conditions: status
            .conditions
            .into_iter()
            .map(convert_deployment_condition_v1_to_internal)
            .collect(),
        collision_count: status.collision_count,
    }
}

fn convert_deployment_status_internal_to_v1(
    status: internal::DeploymentStatus,
) -> DeploymentStatus {
    DeploymentStatus {
        observed_generation: Some(status.observed_generation),
        replicas: Some(status.replicas),
        updated_replicas: Some(status.updated_replicas),
        ready_replicas: Some(status.ready_replicas),
        available_replicas: Some(status.available_replicas),
        unavailable_replicas: Some(status.unavailable_replicas),
        terminating_replicas: status.terminating_replicas,
        conditions: status
            .conditions
            .into_iter()
            .map(convert_deployment_condition_internal_to_v1)
            .collect(),
        collision_count: status.collision_count,
    }
}

// ============================================================================
// DaemonSet Conversion Helpers
// ============================================================================

fn convert_daemon_set_condition_type_v1_to_internal(
    value: DaemonSetConditionType,
) -> internal::DaemonSetConditionType {
    match value {
        DaemonSetConditionType::Unknown => internal::DaemonSetConditionType::Unknown,
    }
}

fn convert_daemon_set_condition_type_internal_to_v1(
    value: internal::DaemonSetConditionType,
) -> DaemonSetConditionType {
    match value {
        internal::DaemonSetConditionType::Unknown => DaemonSetConditionType::Unknown,
    }
}

fn convert_daemon_set_condition_v1_to_internal(
    value: DaemonSetCondition,
) -> internal::DaemonSetCondition {
    internal::DaemonSetCondition {
        r#type: convert_daemon_set_condition_type_v1_to_internal(value.r#type),
        status: convert_condition_status_to_internal(value.status),
        last_transition_time: option_string_to_timestamp_or_default(value.last_transition_time),
        reason: value.reason,
        message: value.message,
    }
}

fn convert_daemon_set_condition_internal_to_v1(
    value: internal::DaemonSetCondition,
) -> DaemonSetCondition {
    DaemonSetCondition {
        r#type: convert_daemon_set_condition_type_internal_to_v1(value.r#type),
        status: convert_condition_status_to_v1(value.status),
        last_transition_time: Some(value.last_transition_time.to_rfc3339()),
        reason: value.reason,
        message: value.message,
    }
}

fn convert_rolling_update_daemon_set_v1_to_internal(
    value: RollingUpdateDaemonSet,
) -> internal::RollingUpdateDaemonSet {
    internal::RollingUpdateDaemonSet {
        max_unavailable: value.max_unavailable.unwrap_or_default(),
        max_surge: value.max_surge.unwrap_or_default(),
    }
}

fn convert_rolling_update_daemon_set_internal_to_v1(
    value: internal::RollingUpdateDaemonSet,
) -> RollingUpdateDaemonSet {
    RollingUpdateDaemonSet {
        max_unavailable: Some(value.max_unavailable),
        max_surge: Some(value.max_surge),
    }
}

fn convert_daemon_set_update_strategy_v1_to_internal(
    value: DaemonSetUpdateStrategy,
) -> internal::DaemonSetUpdateStrategy {
    internal::DaemonSetUpdateStrategy {
        r#type: match value.r#type.unwrap_or_default() {
            DaemonSetUpdateStrategyType::RollingUpdate => {
                internal::DaemonSetUpdateStrategyType::RollingUpdate
            }
            DaemonSetUpdateStrategyType::OnDelete => {
                internal::DaemonSetUpdateStrategyType::OnDelete
            }
        },
        rolling_update: value
            .rolling_update
            .map(convert_rolling_update_daemon_set_v1_to_internal),
    }
}

fn convert_daemon_set_update_strategy_internal_to_v1(
    value: internal::DaemonSetUpdateStrategy,
) -> DaemonSetUpdateStrategy {
    DaemonSetUpdateStrategy {
        r#type: Some(match value.r#type {
            internal::DaemonSetUpdateStrategyType::RollingUpdate => {
                DaemonSetUpdateStrategyType::RollingUpdate
            }
            internal::DaemonSetUpdateStrategyType::OnDelete => {
                DaemonSetUpdateStrategyType::OnDelete
            }
        }),
        rolling_update: value
            .rolling_update
            .map(convert_rolling_update_daemon_set_internal_to_v1),
    }
}

fn convert_daemon_set_spec_v1_to_internal(spec: DaemonSetSpec) -> internal::DaemonSetSpec {
    internal::DaemonSetSpec {
        selector: spec.selector,
        template: spec.template.unwrap_or_default().to_internal(),
        update_strategy: spec
            .update_strategy
            .map(convert_daemon_set_update_strategy_v1_to_internal)
            .unwrap_or_default(),
        min_ready_seconds: spec.min_ready_seconds.unwrap_or_default(),
        template_generation: 0,
        revision_history_limit: spec.revision_history_limit,
    }
}

fn convert_daemon_set_spec_internal_to_v1(spec: internal::DaemonSetSpec) -> DaemonSetSpec {
    DaemonSetSpec {
        selector: spec.selector,
        template: Some(PodTemplateSpec::from_internal(spec.template)),
        update_strategy: Some(convert_daemon_set_update_strategy_internal_to_v1(
            spec.update_strategy,
        )),
        min_ready_seconds: Some(spec.min_ready_seconds),
        revision_history_limit: spec.revision_history_limit,
    }
}

fn convert_daemon_set_status_v1_to_internal(status: DaemonSetStatus) -> internal::DaemonSetStatus {
    internal::DaemonSetStatus {
        current_number_scheduled: status.current_number_scheduled,
        number_misscheduled: status.number_misscheduled,
        desired_number_scheduled: status.desired_number_scheduled,
        number_ready: status.number_ready,
        observed_generation: status.observed_generation.unwrap_or_default(),
        updated_number_scheduled: status.updated_number_scheduled.unwrap_or_default(),
        number_available: status.number_available.unwrap_or_default(),
        number_unavailable: status.number_unavailable.unwrap_or_default(),
        collision_count: status.collision_count,
        conditions: status
            .conditions
            .into_iter()
            .map(convert_daemon_set_condition_v1_to_internal)
            .collect(),
    }
}

fn convert_daemon_set_status_internal_to_v1(status: internal::DaemonSetStatus) -> DaemonSetStatus {
    DaemonSetStatus {
        current_number_scheduled: status.current_number_scheduled,
        number_misscheduled: status.number_misscheduled,
        desired_number_scheduled: status.desired_number_scheduled,
        number_ready: status.number_ready,
        observed_generation: Some(status.observed_generation),
        updated_number_scheduled: Some(status.updated_number_scheduled),
        number_available: Some(status.number_available),
        number_unavailable: Some(status.number_unavailable),
        collision_count: status.collision_count,
        conditions: status
            .conditions
            .into_iter()
            .map(convert_daemon_set_condition_internal_to_v1)
            .collect(),
    }
}

// ============================================================================
// StatefulSet Conversion Helpers
// ============================================================================

fn convert_pod_management_policy_v1_to_internal(
    value: PodManagementPolicyType,
) -> internal::PodManagementPolicyType {
    match value {
        PodManagementPolicyType::OrderedReady => internal::PodManagementPolicyType::OrderedReady,
        PodManagementPolicyType::Parallel => internal::PodManagementPolicyType::Parallel,
    }
}

fn convert_pod_management_policy_internal_to_v1(
    value: internal::PodManagementPolicyType,
) -> PodManagementPolicyType {
    match value {
        internal::PodManagementPolicyType::OrderedReady => PodManagementPolicyType::OrderedReady,
        internal::PodManagementPolicyType::Parallel => PodManagementPolicyType::Parallel,
    }
}

fn convert_stateful_set_update_strategy_type_v1_to_internal(
    value: StatefulSetUpdateStrategyType,
) -> internal::StatefulSetUpdateStrategyType {
    match value {
        StatefulSetUpdateStrategyType::RollingUpdate => {
            internal::StatefulSetUpdateStrategyType::RollingUpdate
        }
        StatefulSetUpdateStrategyType::OnDelete => {
            internal::StatefulSetUpdateStrategyType::OnDelete
        }
    }
}

fn convert_stateful_set_update_strategy_type_internal_to_v1(
    value: internal::StatefulSetUpdateStrategyType,
) -> StatefulSetUpdateStrategyType {
    match value {
        internal::StatefulSetUpdateStrategyType::RollingUpdate => {
            StatefulSetUpdateStrategyType::RollingUpdate
        }
        internal::StatefulSetUpdateStrategyType::OnDelete => {
            StatefulSetUpdateStrategyType::OnDelete
        }
    }
}

fn convert_pvc_retention_policy_type_v1_to_internal(
    value: PersistentVolumeClaimRetentionPolicyType,
) -> internal::PersistentVolumeClaimRetentionPolicyType {
    match value {
        PersistentVolumeClaimRetentionPolicyType::Retain => {
            internal::PersistentVolumeClaimRetentionPolicyType::Retain
        }
        PersistentVolumeClaimRetentionPolicyType::Delete => {
            internal::PersistentVolumeClaimRetentionPolicyType::Delete
        }
    }
}

fn convert_pvc_retention_policy_type_internal_to_v1(
    value: internal::PersistentVolumeClaimRetentionPolicyType,
) -> PersistentVolumeClaimRetentionPolicyType {
    match value {
        internal::PersistentVolumeClaimRetentionPolicyType::Retain => {
            PersistentVolumeClaimRetentionPolicyType::Retain
        }
        internal::PersistentVolumeClaimRetentionPolicyType::Delete => {
            PersistentVolumeClaimRetentionPolicyType::Delete
        }
    }
}

fn convert_rolling_update_stateful_set_v1_to_internal(
    value: RollingUpdateStatefulSetStrategy,
) -> internal::RollingUpdateStatefulSetStrategy {
    internal::RollingUpdateStatefulSetStrategy {
        partition: value.partition.unwrap_or_default(),
        max_unavailable: value.max_unavailable,
    }
}

fn convert_rolling_update_stateful_set_internal_to_v1(
    value: internal::RollingUpdateStatefulSetStrategy,
) -> RollingUpdateStatefulSetStrategy {
    RollingUpdateStatefulSetStrategy {
        partition: Some(value.partition),
        max_unavailable: value.max_unavailable,
    }
}

fn convert_stateful_set_update_strategy_v1_to_internal(
    value: StatefulSetUpdateStrategy,
) -> internal::StatefulSetUpdateStrategy {
    internal::StatefulSetUpdateStrategy {
        r#type: convert_stateful_set_update_strategy_type_v1_to_internal(
            value.r#type.unwrap_or_default(),
        ),
        rolling_update: value
            .rolling_update
            .map(convert_rolling_update_stateful_set_v1_to_internal),
    }
}

fn convert_stateful_set_update_strategy_internal_to_v1(
    value: internal::StatefulSetUpdateStrategy,
) -> StatefulSetUpdateStrategy {
    StatefulSetUpdateStrategy {
        r#type: Some(convert_stateful_set_update_strategy_type_internal_to_v1(
            value.r#type,
        )),
        rolling_update: value
            .rolling_update
            .map(convert_rolling_update_stateful_set_internal_to_v1),
    }
}

fn convert_stateful_set_pvc_retention_policy_v1_to_internal(
    value: StatefulSetPersistentVolumeClaimRetentionPolicy,
) -> internal::StatefulSetPersistentVolumeClaimRetentionPolicy {
    internal::StatefulSetPersistentVolumeClaimRetentionPolicy {
        when_deleted: convert_pvc_retention_policy_type_v1_to_internal(
            value.when_deleted.unwrap_or_default(),
        ),
        when_scaled: convert_pvc_retention_policy_type_v1_to_internal(
            value.when_scaled.unwrap_or_default(),
        ),
    }
}

fn convert_stateful_set_pvc_retention_policy_internal_to_v1(
    value: internal::StatefulSetPersistentVolumeClaimRetentionPolicy,
) -> StatefulSetPersistentVolumeClaimRetentionPolicy {
    StatefulSetPersistentVolumeClaimRetentionPolicy {
        when_deleted: Some(convert_pvc_retention_policy_type_internal_to_v1(
            value.when_deleted,
        )),
        when_scaled: Some(convert_pvc_retention_policy_type_internal_to_v1(
            value.when_scaled,
        )),
    }
}

fn convert_stateful_set_ordinals_v1_to_internal(
    value: StatefulSetOrdinals,
) -> internal::StatefulSetOrdinals {
    internal::StatefulSetOrdinals {
        start: value.start.unwrap_or_default(),
    }
}

fn convert_stateful_set_ordinals_internal_to_v1(
    value: internal::StatefulSetOrdinals,
) -> StatefulSetOrdinals {
    StatefulSetOrdinals {
        start: Some(value.start),
    }
}

fn convert_stateful_set_condition_type_v1_to_internal(
    value: StatefulSetConditionType,
) -> internal::StatefulSetConditionType {
    match value {
        StatefulSetConditionType::Unknown => internal::StatefulSetConditionType::Unknown,
    }
}

fn convert_stateful_set_condition_type_internal_to_v1(
    value: internal::StatefulSetConditionType,
) -> StatefulSetConditionType {
    match value {
        internal::StatefulSetConditionType::Unknown => StatefulSetConditionType::Unknown,
    }
}

fn convert_stateful_set_condition_v1_to_internal(
    value: StatefulSetCondition,
) -> internal::StatefulSetCondition {
    internal::StatefulSetCondition {
        r#type: convert_stateful_set_condition_type_v1_to_internal(value.r#type),
        status: convert_condition_status_to_internal(value.status),
        last_transition_time: option_string_to_timestamp_or_default(value.last_transition_time),
        reason: value.reason,
        message: value.message,
    }
}

fn convert_stateful_set_condition_internal_to_v1(
    value: internal::StatefulSetCondition,
) -> StatefulSetCondition {
    StatefulSetCondition {
        r#type: convert_stateful_set_condition_type_internal_to_v1(value.r#type),
        status: convert_condition_status_to_v1(value.status),
        last_transition_time: Some(value.last_transition_time.to_rfc3339()),
        reason: value.reason,
        message: value.message,
    }
}

fn convert_stateful_set_spec_v1_to_internal(spec: StatefulSetSpec) -> internal::StatefulSetSpec {
    internal::StatefulSetSpec {
        replicas: spec.replicas.unwrap_or_default(),
        selector: spec.selector,
        template: spec.template.unwrap_or_default().to_internal(),
        volume_claim_templates: spec
            .volume_claim_templates
            .into_iter()
            .map(PersistentVolumeClaim::to_internal)
            .collect(),
        service_name: spec.service_name,
        pod_management_policy: convert_pod_management_policy_v1_to_internal(
            spec.pod_management_policy.unwrap_or_default(),
        ),
        update_strategy: spec
            .update_strategy
            .map(convert_stateful_set_update_strategy_v1_to_internal)
            .unwrap_or_default(),
        revision_history_limit: spec.revision_history_limit,
        min_ready_seconds: spec.min_ready_seconds.unwrap_or_default(),
        persistent_volume_claim_retention_policy: spec
            .persistent_volume_claim_retention_policy
            .map(convert_stateful_set_pvc_retention_policy_v1_to_internal),
        ordinals: spec
            .ordinals
            .map(convert_stateful_set_ordinals_v1_to_internal),
    }
}

fn convert_stateful_set_spec_internal_to_v1(spec: internal::StatefulSetSpec) -> StatefulSetSpec {
    StatefulSetSpec {
        replicas: Some(spec.replicas),
        selector: spec.selector,
        template: Some(PodTemplateSpec::from_internal(spec.template)),
        volume_claim_templates: spec
            .volume_claim_templates
            .into_iter()
            .map(PersistentVolumeClaim::from_internal)
            .collect(),
        service_name: spec.service_name,
        pod_management_policy: Some(convert_pod_management_policy_internal_to_v1(
            spec.pod_management_policy,
        )),
        update_strategy: Some(convert_stateful_set_update_strategy_internal_to_v1(
            spec.update_strategy,
        )),
        revision_history_limit: spec.revision_history_limit,
        min_ready_seconds: Some(spec.min_ready_seconds),
        persistent_volume_claim_retention_policy: spec
            .persistent_volume_claim_retention_policy
            .map(convert_stateful_set_pvc_retention_policy_internal_to_v1),
        ordinals: spec
            .ordinals
            .map(convert_stateful_set_ordinals_internal_to_v1),
    }
}

fn convert_stateful_set_status_v1_to_internal(
    status: StatefulSetStatus,
) -> internal::StatefulSetStatus {
    internal::StatefulSetStatus {
        observed_generation: status.observed_generation,
        replicas: status.replicas,
        ready_replicas: status.ready_replicas.unwrap_or_default(),
        current_replicas: status.current_replicas.unwrap_or_default(),
        updated_replicas: status.updated_replicas.unwrap_or_default(),
        current_revision: status.current_revision,
        update_revision: status.update_revision,
        collision_count: status.collision_count,
        conditions: status
            .conditions
            .into_iter()
            .map(convert_stateful_set_condition_v1_to_internal)
            .collect(),
        available_replicas: status.available_replicas,
    }
}

fn convert_stateful_set_status_internal_to_v1(
    status: internal::StatefulSetStatus,
) -> StatefulSetStatus {
    StatefulSetStatus {
        observed_generation: status.observed_generation,
        replicas: status.replicas,
        ready_replicas: Some(status.ready_replicas),
        current_replicas: Some(status.current_replicas),
        updated_replicas: Some(status.updated_replicas),
        current_revision: status.current_revision,
        update_revision: status.update_revision,
        collision_count: status.collision_count,
        conditions: status
            .conditions
            .into_iter()
            .map(convert_stateful_set_condition_internal_to_v1)
            .collect(),
        available_replicas: status.available_replicas,
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

        result
    }
}

// ============================================================================
// Deployment Conversions
// ============================================================================

impl ToInternal<internal::Deployment> for Deployment {
    fn to_internal(self) -> internal::Deployment {
        internal::Deployment {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(convert_deployment_spec_v1_to_internal),
            status: self.status.map(convert_deployment_status_v1_to_internal),
        }
    }
}

impl FromInternal<internal::Deployment> for Deployment {
    fn from_internal(value: internal::Deployment) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(convert_deployment_spec_internal_to_v1),
            status: value.status.map(convert_deployment_status_internal_to_v1),
        };

        result
    }
}

impl ToInternal<internal::DeploymentList> for DeploymentList {
    fn to_internal(self) -> internal::DeploymentList {
        internal::DeploymentList {
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

impl FromInternal<internal::DeploymentList> for DeploymentList {
    fn from_internal(value: internal::DeploymentList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: object_meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(Deployment::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// StatefulSet Conversions
// ============================================================================

impl ToInternal<internal::StatefulSet> for StatefulSet {
    fn to_internal(self) -> internal::StatefulSet {
        internal::StatefulSet {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(convert_stateful_set_spec_v1_to_internal),
            status: self.status.map(convert_stateful_set_status_v1_to_internal),
        }
    }
}

impl FromInternal<internal::StatefulSet> for StatefulSet {
    fn from_internal(value: internal::StatefulSet) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(convert_stateful_set_spec_internal_to_v1),
            status: value.status.map(convert_stateful_set_status_internal_to_v1),
        };

        result
    }
}

impl ToInternal<internal::StatefulSetList> for StatefulSetList {
    fn to_internal(self) -> internal::StatefulSetList {
        internal::StatefulSetList {
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

impl FromInternal<internal::StatefulSetList> for StatefulSetList {
    fn from_internal(value: internal::StatefulSetList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: object_meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(StatefulSet::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// ControllerRevision Conversions
// ============================================================================

impl ToInternal<internal::ControllerRevision> for ControllerRevision {
    fn to_internal(self) -> internal::ControllerRevision {
        internal::ControllerRevision {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            data: self.data.unwrap_or(serde_json::Value::Null),
            revision: self.revision,
        }
    }
}

impl FromInternal<internal::ControllerRevision> for ControllerRevision {
    fn from_internal(value: internal::ControllerRevision) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            data: Some(value.data),
            revision: value.revision,
        };

        result
    }
}

impl ToInternal<internal::ControllerRevisionList> for ControllerRevisionList {
    fn to_internal(self) -> internal::ControllerRevisionList {
        internal::ControllerRevisionList {
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

impl FromInternal<internal::ControllerRevisionList> for ControllerRevisionList {
    fn from_internal(value: internal::ControllerRevisionList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: object_meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(ControllerRevision::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// DaemonSet Conversions
// ============================================================================

impl ToInternal<internal::DaemonSet> for DaemonSet {
    fn to_internal(self) -> internal::DaemonSet {
        internal::DaemonSet {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(convert_daemon_set_spec_v1_to_internal),
            status: self.status.map(convert_daemon_set_status_v1_to_internal),
        }
    }
}

impl FromInternal<internal::DaemonSet> for DaemonSet {
    fn from_internal(value: internal::DaemonSet) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(convert_daemon_set_spec_internal_to_v1),
            status: value.status.map(convert_daemon_set_status_internal_to_v1),
        };

        result
    }
}

impl ToInternal<internal::DaemonSetList> for DaemonSetList {
    fn to_internal(self) -> internal::DaemonSetList {
        internal::DaemonSetList {
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

impl FromInternal<internal::DaemonSetList> for DaemonSetList {
    fn from_internal(value: internal::DaemonSetList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: object_meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(DaemonSet::from_internal)
                .collect(),
        };

        result
    }
}
