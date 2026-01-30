use super::{
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
use crate::apps::internal;
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, IntOrString, LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::{PersistentVolumeClaim, PodSpec, PodTemplateSpec};

fn replica_set_basic() -> ReplicaSet {
    let mut selector = LabelSelector::default();
    selector
        .match_labels
        .insert("app".to_string(), "demo".to_string());

    ReplicaSet {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("demo-replicaset".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(ReplicaSetSpec {
            replicas: Some(3),
            selector: Some(selector.clone()),
            template: Some(PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: selector.match_labels.clone(),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    restart_policy: Some("Always".to_string()),
                    dns_policy: Some("ClusterFirst".to_string()),
                    ..Default::default()
                }),
            }),
            min_ready_seconds: Some(10),
        }),
        status: Some(ReplicaSetStatus {
            replicas: 3,
            fully_labeled_replicas: Some(3),
            ready_replicas: Some(2),
            available_replicas: Some(2),
            terminating_replicas: Some(0),
            observed_generation: Some(2),
            conditions: vec![ReplicaSetCondition {
                r#type: ReplicaSetConditionType::ReplicaFailure,
                status: "True".to_string(),
                last_transition_time: Some("2024-01-01T00:00:00Z".to_string()),
                reason: "ReplicaFailure".to_string(),
                message: "replica failed".to_string(),
            }],
        }),
    }
}

fn replica_set_list_basic() -> ReplicaSetList {
    let mut item = replica_set_basic();
    item.apply_default();

    ReplicaSetList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn deployment_basic() -> Deployment {
    let mut selector = LabelSelector::default();
    selector
        .match_labels
        .insert("app".to_string(), "demo".to_string());

    Deployment {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("demo-deployment".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(DeploymentSpec {
            replicas: Some(3),
            selector: Some(selector.clone()),
            template: Some(PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: selector.match_labels.clone(),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    restart_policy: Some("Always".to_string()),
                    dns_policy: Some("ClusterFirst".to_string()),
                    ..Default::default()
                }),
            }),
            strategy: Some(DeploymentStrategy {
                r#type: Some(DeploymentStrategyType::RollingUpdate),
                rolling_update: Some(RollingUpdateDeployment {
                    max_unavailable: Some(crate::common::IntOrString::Int(1)),
                    max_surge: Some(crate::common::IntOrString::Int(2)),
                }),
            }),
            min_ready_seconds: Some(10),
            revision_history_limit: Some(5),
            paused: false,
            progress_deadline_seconds: Some(600),
        }),
        status: Some(DeploymentStatus {
            observed_generation: Some(2),
            replicas: Some(3),
            updated_replicas: Some(2),
            ready_replicas: Some(2),
            available_replicas: Some(2),
            unavailable_replicas: Some(1),
            terminating_replicas: Some(0),
            conditions: vec![DeploymentCondition {
                r#type: DeploymentConditionType::Progressing,
                status: "True".to_string(),
                last_update_time: Some("2024-01-01T00:00:00Z".to_string()),
                last_transition_time: Some("2024-01-01T00:00:00Z".to_string()),
                reason: "NewReplicaSetAvailable".to_string(),
                message: "Deployment progressing".to_string(),
            }],
            collision_count: Some(0),
        }),
    }
}

fn deployment_list_basic() -> DeploymentList {
    let mut item = deployment_basic();
    item.apply_default();

    DeploymentList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn daemon_set_basic() -> DaemonSet {
    let mut selector = LabelSelector::default();
    selector
        .match_labels
        .insert("app".to_string(), "daemon".to_string());

    DaemonSet {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("demo-daemonset".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(DaemonSetSpec {
            selector: Some(selector.clone()),
            template: Some(PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: selector.match_labels.clone(),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    restart_policy: Some("Always".to_string()),
                    dns_policy: Some("ClusterFirst".to_string()),
                    ..Default::default()
                }),
            }),
            update_strategy: Some(DaemonSetUpdateStrategy {
                r#type: Some(DaemonSetUpdateStrategyType::RollingUpdate),
                rolling_update: Some(RollingUpdateDaemonSet {
                    max_unavailable: Some(IntOrString::Int(1)),
                    max_surge: Some(IntOrString::Int(0)),
                }),
            }),
            min_ready_seconds: Some(5),
            revision_history_limit: Some(10),
        }),
        status: Some(DaemonSetStatus {
            current_number_scheduled: 2,
            number_misscheduled: 0,
            desired_number_scheduled: 2,
            number_ready: 2,
            observed_generation: Some(1),
            updated_number_scheduled: Some(2),
            number_available: Some(2),
            number_unavailable: Some(0),
            collision_count: Some(0),
            conditions: vec![DaemonSetCondition {
                r#type: DaemonSetConditionType::Unknown,
                status: "True".to_string(),
                last_transition_time: Some("2024-01-01T00:00:00Z".to_string()),
                reason: "Ready".to_string(),
                message: "DaemonSet is ready".to_string(),
            }],
        }),
    }
}

fn daemon_set_list_basic() -> DaemonSetList {
    let mut item = daemon_set_basic();
    item.apply_default();

    DaemonSetList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("3".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn stateful_set_basic() -> StatefulSet {
    let mut selector = LabelSelector::default();
    selector
        .match_labels
        .insert("app".to_string(), "stateful".to_string());

    StatefulSet {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("demo-statefulset".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(StatefulSetSpec {
            replicas: Some(2),
            selector: Some(selector.clone()),
            template: Some(PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: selector.match_labels.clone(),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    restart_policy: Some("Always".to_string()),
                    dns_policy: Some("ClusterFirst".to_string()),
                    ..Default::default()
                }),
            }),
            volume_claim_templates: vec![PersistentVolumeClaim {
                type_meta: TypeMeta {
                    api_version: "v1".to_string(),
                    kind: "PersistentVolumeClaim".to_string(),
                },
                metadata: Some(ObjectMeta {
                    name: Some("data".to_string()),
                    ..Default::default()
                }),
                spec: None,
                status: None,
            }],
            service_name: "demo-service".to_string(),
            pod_management_policy: Some(PodManagementPolicyType::OrderedReady),
            update_strategy: Some(StatefulSetUpdateStrategy {
                r#type: Some(StatefulSetUpdateStrategyType::RollingUpdate),
                rolling_update: Some(RollingUpdateStatefulSetStrategy {
                    partition: Some(0),
                    max_unavailable: Some(IntOrString::Int(1)),
                }),
            }),
            revision_history_limit: Some(10),
            min_ready_seconds: Some(5),
            persistent_volume_claim_retention_policy: Some(
                StatefulSetPersistentVolumeClaimRetentionPolicy {
                    when_deleted: Some(PersistentVolumeClaimRetentionPolicyType::Retain),
                    when_scaled: Some(PersistentVolumeClaimRetentionPolicyType::Delete),
                },
            ),
            ordinals: Some(StatefulSetOrdinals { start: Some(0) }),
        }),
        status: Some(StatefulSetStatus {
            observed_generation: Some(1),
            replicas: 2,
            ready_replicas: Some(2),
            current_replicas: Some(2),
            updated_replicas: Some(2),
            current_revision: "rev-1".to_string(),
            update_revision: "rev-1".to_string(),
            collision_count: Some(0),
            conditions: vec![StatefulSetCondition {
                r#type: StatefulSetConditionType::Unknown,
                status: "True".to_string(),
                last_transition_time: Some("2024-01-01T00:00:00Z".to_string()),
                reason: "Ready".to_string(),
                message: "StatefulSet is ready".to_string(),
            }],
            available_replicas: 2,
        }),
    }
}

fn stateful_set_list_basic() -> StatefulSetList {
    let mut item = stateful_set_basic();
    item.apply_default();

    StatefulSetList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("4".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn controller_revision_basic() -> ControllerRevision {
    ControllerRevision {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("demo-revision".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        data: Some(serde_json::json!({"config": {"replicas": 3}})),
        revision: 1,
    }
}

fn controller_revision_list_basic() -> ControllerRevisionList {
    let mut item = controller_revision_basic();
    item.apply_default();

    ControllerRevisionList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("5".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_replica_set() {
    assert_conversion_roundtrip::<ReplicaSet, internal::ReplicaSet>(replica_set_basic());
}

#[test]
fn conversion_roundtrip_replica_set_list() {
    assert_conversion_roundtrip::<ReplicaSetList, internal::ReplicaSetList>(
        replica_set_list_basic(),
    );
}

#[test]
fn conversion_roundtrip_deployment() {
    assert_conversion_roundtrip::<Deployment, internal::Deployment>(deployment_basic());
}

#[test]
fn conversion_roundtrip_deployment_list() {
    assert_conversion_roundtrip::<DeploymentList, internal::DeploymentList>(deployment_list_basic());
}

#[test]
fn conversion_roundtrip_daemon_set() {
    assert_conversion_roundtrip::<DaemonSet, internal::DaemonSet>(daemon_set_basic());
}

#[test]
fn conversion_roundtrip_daemon_set_list() {
    assert_conversion_roundtrip::<DaemonSetList, internal::DaemonSetList>(daemon_set_list_basic());
}

#[test]
fn conversion_roundtrip_stateful_set() {
    assert_conversion_roundtrip::<StatefulSet, internal::StatefulSet>(stateful_set_basic());
}

#[test]
fn conversion_roundtrip_stateful_set_list() {
    assert_conversion_roundtrip::<StatefulSetList, internal::StatefulSetList>(
        stateful_set_list_basic(),
    );
}

#[test]
fn conversion_roundtrip_controller_revision() {
    assert_conversion_roundtrip::<ControllerRevision, internal::ControllerRevision>(
        controller_revision_basic(),
    );
}

#[test]
fn conversion_roundtrip_controller_revision_list() {
    assert_conversion_roundtrip::<ControllerRevisionList, internal::ControllerRevisionList>(
        controller_revision_list_basic(),
    );
}
