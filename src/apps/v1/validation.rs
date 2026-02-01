//! Validation for Kubernetes Apps v1 API types
//!
//! Wrapper around internal validation (v1 -> internal -> validate)

use crate::apps::internal::validation as internal_validation;
use crate::common::ToInternal;
use crate::common::validation::ErrorList;

use super::{ControllerRevision, DaemonSet, Deployment, ReplicaSet, StatefulSet};

// =============================================================================
// StatefulSet validation
// =============================================================================

pub fn validate_stateful_set(stateful_set: &StatefulSet) -> ErrorList {
    internal_validation::validate_stateful_set(&stateful_set.clone().to_internal())
}

pub fn validate_stateful_set_update(
    stateful_set: &StatefulSet,
    old_stateful_set: &StatefulSet,
) -> ErrorList {
    internal_validation::validate_stateful_set_update(
        &stateful_set.clone().to_internal(),
        &old_stateful_set.clone().to_internal(),
    )
}

pub fn validate_stateful_set_status_update(
    stateful_set: &StatefulSet,
    old_stateful_set: &StatefulSet,
) -> ErrorList {
    internal_validation::validate_stateful_set_status_update(
        &stateful_set.clone().to_internal(),
        &old_stateful_set.clone().to_internal(),
    )
}

// =============================================================================
// Deployment validation
// =============================================================================

pub fn validate_deployment(deployment: &Deployment) -> ErrorList {
    internal_validation::validate_deployment(&deployment.clone().to_internal())
}

pub fn validate_deployment_update(
    deployment: &Deployment,
    old_deployment: &Deployment,
) -> ErrorList {
    internal_validation::validate_deployment_update(
        &deployment.clone().to_internal(),
        &old_deployment.clone().to_internal(),
    )
}

pub fn validate_deployment_status_update(
    deployment: &Deployment,
    old_deployment: &Deployment,
) -> ErrorList {
    internal_validation::validate_deployment_status_update(
        &deployment.clone().to_internal(),
        &old_deployment.clone().to_internal(),
    )
}

// =============================================================================
// ReplicaSet validation
// =============================================================================

pub fn validate_replica_set(replica_set: &ReplicaSet) -> ErrorList {
    internal_validation::validate_replica_set(&replica_set.clone().to_internal())
}

pub fn validate_replica_set_update(
    replica_set: &ReplicaSet,
    old_replica_set: &ReplicaSet,
) -> ErrorList {
    internal_validation::validate_replica_set_update(
        &replica_set.clone().to_internal(),
        &old_replica_set.clone().to_internal(),
    )
}

pub fn validate_replica_set_status_update(
    replica_set: &ReplicaSet,
    old_replica_set: &ReplicaSet,
) -> ErrorList {
    internal_validation::validate_replica_set_status_update(
        &replica_set.clone().to_internal(),
        &old_replica_set.clone().to_internal(),
    )
}

// =============================================================================
// DaemonSet validation
// =============================================================================

pub fn validate_daemon_set(daemon_set: &DaemonSet) -> ErrorList {
    internal_validation::validate_daemon_set(&daemon_set.clone().to_internal())
}

pub fn validate_daemon_set_update(daemon_set: &DaemonSet, old_daemon_set: &DaemonSet) -> ErrorList {
    internal_validation::validate_daemon_set_update(
        &daemon_set.clone().to_internal(),
        &old_daemon_set.clone().to_internal(),
    )
}

pub fn validate_daemon_set_status_update(
    daemon_set: &DaemonSet,
    old_daemon_set: &DaemonSet,
) -> ErrorList {
    internal_validation::validate_daemon_set_status_update(
        &daemon_set.clone().to_internal(),
        &old_daemon_set.clone().to_internal(),
    )
}

// =============================================================================
// ControllerRevision validation
// =============================================================================

pub fn validate_controller_revision_create(revision: &ControllerRevision) -> ErrorList {
    internal_validation::validate_controller_revision_create(&revision.clone().to_internal())
}

pub fn validate_controller_revision_update(
    new_revision: &ControllerRevision,
    old_revision: &ControllerRevision,
) -> ErrorList {
    internal_validation::validate_controller_revision_update(
        &new_revision.clone().to_internal(),
        &old_revision.clone().to_internal(),
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::v1::{DeploymentSpec, ReplicaSetSpec};
    use crate::common::ObjectMeta;
    use crate::core::v1::PodTemplateSpec;
    use crate::core::v1::pod::{Container, PodSpec, restart_policy};

    fn base_template(labels: std::collections::BTreeMap<String, String>) -> PodTemplateSpec {
        PodTemplateSpec {
            metadata: Some(ObjectMeta {
                labels,
                ..Default::default()
            }),
            spec: Some(PodSpec {
                termination_grace_period_seconds: Some(30),
                restart_policy: Some(restart_policy::ALWAYS.to_string()),
                dns_policy: Some("ClusterFirst".to_string()),
                containers: vec![Container {
                    name: "nginx".to_string(),
                    image: Some("nginx:latest".to_string()),
                    termination_message_policy: Some("File".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            }),
        }
    }

    #[test]
    fn deployment_requires_selector() {
        let deployment = Deployment {
            metadata: Some(ObjectMeta {
                name: Some("demo".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(DeploymentSpec::default()),
            ..Default::default()
        };
        let errs = validate_deployment(&deployment);
        assert!(!errs.is_empty());
    }

    #[test]
    fn replica_set_selector_must_match_template() {
        let selector = crate::common::LabelSelector {
            match_labels: [("app".to_string(), "demo".to_string())].into(),
            match_expressions: Vec::new(),
        };
        let template_labels = [("app".to_string(), "other".to_string())].into();
        let rs = ReplicaSet {
            metadata: Some(ObjectMeta {
                name: Some("rs".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ReplicaSetSpec {
                replicas: Some(1),
                selector: Some(selector),
                template: Some(base_template(template_labels)),
                min_ready_seconds: None,
            }),
            ..Default::default()
        };
        let errs = validate_replica_set(&rs);
        assert!(!errs.is_empty());
    }
}
