use super::{
    ControllerRevision, ControllerRevisionList, DaemonSet, DaemonSetList, Deployment,
    DeploymentList, ReplicaSet, ReplicaSetList, StatefulSet, StatefulSetList,
};
use crate::apps::v1::{
    DaemonSetSpec, DaemonSetStatus, DaemonSetUpdateStrategy, DaemonSetUpdateStrategyType,
    DeploymentCondition, DeploymentConditionType, DeploymentSpec, DeploymentStatus,
    DeploymentStrategy, DeploymentStrategyType, ReplicaSetSpec, ReplicaSetStatus,
    RollingUpdateDaemonSet, RollingUpdateDeployment, RollingUpdateStatefulSetStrategy,
    StatefulSetSpec, StatefulSetStatus, StatefulSetUpdateStrategy, StatefulSetUpdateStrategyType,
};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{IntOrString, LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use crate::core::v1::PodTemplateSpec;
use std::collections::BTreeMap;

fn type_meta(kind: &str) -> TypeMeta {
    TypeMeta {
        api_version: "apps/v1".to_string(),
        kind: kind.to_string(),
    }
}

fn labeled_selector() -> LabelSelector {
    let mut selector = LabelSelector::default();
    selector
        .match_labels
        .insert("app".to_string(), "demo".to_string());
    selector
}

fn template_with_labels(name: &str, labels: &BTreeMap<String, String>) -> PodTemplateSpec {
    PodTemplateSpec {
        metadata: Some(ObjectMeta {
            name: Some(name.to_string()),
            labels: labels.clone(),
            ..Default::default()
        }),
        spec: None,
    }
}

fn deployment_basic() -> Deployment {
    let selector = labeled_selector();
    let labels = selector.match_labels.clone();

    Deployment {
        type_meta: type_meta("Deployment"),
        metadata: Some(ObjectMeta {
            name: Some("demo-deployment".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(DeploymentSpec {
            replicas: Some(3),
            selector: Some(selector),
            template: Some(template_with_labels("demo-pod", &labels)),
            strategy: Some(DeploymentStrategy {
                r#type: Some(DeploymentStrategyType::RollingUpdate),
                rolling_update: Some(RollingUpdateDeployment {
                    max_unavailable: Some(IntOrString::String("25%".to_string())),
                    max_surge: Some(IntOrString::Int(1)),
                }),
            }),
            min_ready_seconds: Some(5),
            revision_history_limit: Some(10),
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
            terminating_replicas: None,
            conditions: vec![DeploymentCondition {
                r#type: DeploymentConditionType::Progressing,
                status: "True".to_string(),
                last_update_time: Some("2024-01-01T00:00:00Z".to_string()),
                last_transition_time: Some("2024-01-01T00:00:00Z".to_string()),
                reason: "NewReplicaSetAvailable".to_string(),
                message: "Deployment progressing".to_string(),
            }],
            collision_count: None,
        }),
    }
}

fn deployment_list_basic() -> DeploymentList {
    DeploymentList {
        type_meta: type_meta("DeploymentList"),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![deployment_basic()],
    }
}

fn daemon_set_basic() -> DaemonSet {
    let selector = labeled_selector();
    let labels = selector.match_labels.clone();

    DaemonSet {
        type_meta: type_meta("DaemonSet"),
        metadata: Some(ObjectMeta {
            name: Some("demo-daemonset".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(DaemonSetSpec {
            selector: Some(selector),
            template: Some(template_with_labels("daemon-pod", &labels)),
            update_strategy: Some(DaemonSetUpdateStrategy {
                r#type: Some(DaemonSetUpdateStrategyType::RollingUpdate),
                rolling_update: Some(RollingUpdateDaemonSet {
                    max_unavailable: Some(IntOrString::String("10%".to_string())),
                    max_surge: None,
                }),
            }),
            min_ready_seconds: Some(10),
            revision_history_limit: Some(5),
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
            conditions: vec![],
            collision_count: None,
        }),
    }
}

fn daemon_set_list_basic() -> DaemonSetList {
    DaemonSetList {
        type_meta: type_meta("DaemonSetList"),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![daemon_set_basic()],
    }
}

fn replica_set_basic() -> ReplicaSet {
    let selector = labeled_selector();
    let labels = selector.match_labels.clone();

    ReplicaSet {
        type_meta: type_meta("ReplicaSet"),
        metadata: Some(ObjectMeta {
            name: Some("demo-replicaset".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(ReplicaSetSpec {
            replicas: Some(2),
            selector: Some(selector),
            template: Some(template_with_labels("replica-pod", &labels)),
            min_ready_seconds: Some(5),
        }),
        status: Some(ReplicaSetStatus {
            replicas: 2,
            fully_labeled_replicas: Some(2),
            ready_replicas: Some(2),
            available_replicas: Some(2),
            terminating_replicas: None,
            observed_generation: Some(1),
            conditions: vec![],
        }),
    }
}

fn replica_set_list_basic() -> ReplicaSetList {
    ReplicaSetList {
        type_meta: type_meta("ReplicaSetList"),
        metadata: Some(ListMeta {
            resource_version: Some("3".to_string()),
            ..Default::default()
        }),
        items: vec![replica_set_basic()],
    }
}

fn stateful_set_basic() -> StatefulSet {
    let selector = labeled_selector();
    let labels = selector.match_labels.clone();

    StatefulSet {
        type_meta: type_meta("StatefulSet"),
        metadata: Some(ObjectMeta {
            name: Some("demo-statefulset".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(StatefulSetSpec {
            replicas: Some(2),
            selector: Some(selector),
            template: Some(template_with_labels("stateful-pod", &labels)),
            volume_claim_templates: vec![],
            service_name: "demo-service".to_string(),
            pod_management_policy: None,
            update_strategy: Some(StatefulSetUpdateStrategy {
                r#type: Some(StatefulSetUpdateStrategyType::RollingUpdate),
                rolling_update: Some(RollingUpdateStatefulSetStrategy {
                    partition: Some(0),
                    max_unavailable: Some(IntOrString::Int(1)),
                }),
            }),
            revision_history_limit: Some(10),
            min_ready_seconds: Some(5),
            persistent_volume_claim_retention_policy: None,
            ordinals: None,
        }),
        status: Some(StatefulSetStatus {
            observed_generation: Some(1),
            replicas: 2,
            ready_replicas: Some(2),
            current_replicas: Some(2),
            updated_replicas: Some(2),
            current_revision: "rev-1".to_string(),
            update_revision: "rev-1".to_string(),
            collision_count: None,
            conditions: vec![],
            available_replicas: 2,
        }),
    }
}

fn stateful_set_list_basic() -> StatefulSetList {
    StatefulSetList {
        type_meta: type_meta("StatefulSetList"),
        metadata: Some(ListMeta {
            resource_version: Some("4".to_string()),
            ..Default::default()
        }),
        items: vec![stateful_set_basic()],
    }
}

fn controller_revision_basic() -> ControllerRevision {
    ControllerRevision {
        type_meta: type_meta("ControllerRevision"),
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
    ControllerRevisionList {
        type_meta: type_meta("ControllerRevisionList"),
        metadata: Some(ListMeta {
            resource_version: Some("5".to_string()),
            ..Default::default()
        }),
        items: vec![controller_revision_basic()],
    }
}

#[test]
fn serde_roundtrip_deployment() {
    assert_serde_roundtrip(&deployment_basic());
}

#[test]
fn serde_roundtrip_deployment_list() {
    assert_serde_roundtrip(&deployment_list_basic());
}

#[test]
fn serde_roundtrip_daemon_set() {
    assert_serde_roundtrip(&daemon_set_basic());
}

#[test]
fn serde_roundtrip_daemon_set_list() {
    assert_serde_roundtrip(&daemon_set_list_basic());
}

#[test]
fn serde_roundtrip_replica_set() {
    assert_serde_roundtrip(&replica_set_basic());
}

#[test]
fn serde_roundtrip_replica_set_list() {
    assert_serde_roundtrip(&replica_set_list_basic());
}

#[test]
fn serde_roundtrip_stateful_set() {
    assert_serde_roundtrip(&stateful_set_basic());
}

#[test]
fn serde_roundtrip_stateful_set_list() {
    assert_serde_roundtrip(&stateful_set_list_basic());
}

#[test]
fn serde_roundtrip_controller_revision() {
    assert_serde_roundtrip(&controller_revision_basic());
}

#[test]
fn serde_roundtrip_controller_revision_list() {
    assert_serde_roundtrip(&controller_revision_list_basic());
}
