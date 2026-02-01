//! ReplicationController validation for Kubernetes core/v1 API.
//!
//! Delegates to internal validation for consistency.

use crate::common::ToInternal;
use crate::common::validation::ErrorList;
use crate::core::internal::validation::replication_controller as internal_rc_validation;
use crate::core::v1::replication_controller::{
    ReplicationController, ReplicationControllerSpec, ReplicationControllerStatus,
};

/// Validates a ReplicationController resource.
pub fn validate_replication_controller(controller: &ReplicationController) -> ErrorList {
    let internal_controller = controller.clone().to_internal();
    internal_rc_validation::validate_replication_controller(&internal_controller)
}

/// Validates ReplicationController update.
pub fn validate_replication_controller_update(
    new: &ReplicationController,
    old: &ReplicationController,
) -> ErrorList {
    let internal_new = new.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal_rc_validation::validate_replication_controller_update(&internal_new, &internal_old)
}

/// Validates ReplicationController status update.
pub fn validate_replication_controller_status_update(
    new: &ReplicationController,
    old: &ReplicationController,
) -> ErrorList {
    let internal_new = new.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal_rc_validation::validate_replication_controller_status_update(
        &internal_new,
        &internal_old,
    )
}

/// Validates ReplicationControllerStatus.
pub fn validate_replication_controller_status(
    status: &ReplicationControllerStatus,
    path: &crate::common::validation::Path,
) -> ErrorList {
    let internal_status = status.clone().to_internal();
    internal_rc_validation::validate_replication_controller_status(&internal_status, path)
}

/// Validates ReplicationControllerSpec.
pub fn validate_replication_controller_spec(
    spec: &ReplicationControllerSpec,
    path: &crate::common::validation::Path,
) -> ErrorList {
    let internal_spec = spec.clone().to_internal();
    internal_rc_validation::validate_replication_controller_spec(&internal_spec, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ObjectMeta;
    use crate::core::v1::pod::{Container, PodSpec};
    use crate::core::v1::template::PodTemplateSpec;
    use std::collections::BTreeMap;

    fn base_rc() -> ReplicationController {
        ReplicationController {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("demo".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(ReplicationControllerSpec {
                replicas: Some(1),
                min_ready_seconds: Some(0),
                selector: BTreeMap::from([("app".to_string(), "demo".to_string())]),
                template: Some(PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        labels: BTreeMap::from([("app".to_string(), "demo".to_string())]),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        termination_grace_period_seconds: Some(30),
                        restart_policy: Some("Always".to_string()),
                        dns_policy: Some("ClusterFirst".to_string()),
                        containers: vec![Container {
                            name: "nginx".to_string(),
                            image: Some("nginx:latest".to_string()),
                            termination_message_policy: Some("File".to_string()),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                }),
            }),
            status: None,
        }
    }

    #[test]
    fn test_validate_replication_controller_requires_metadata_and_spec() {
        let rc = ReplicationController::default();
        let errs = validate_replication_controller(&rc);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.ends_with("metadata.name")
                    || e.field.ends_with("metadata.namespace"))
        );
        assert!(errs.errors.iter().any(|e| e.field.ends_with("spec")));
    }

    #[test]
    fn test_validate_replication_controller_requires_selector() {
        let mut rc = base_rc();
        rc.spec.as_mut().unwrap().selector.clear();
        let errs = validate_replication_controller(&rc);
        assert!(errs.errors.iter().any(|e| e.field.ends_with("selector")));
    }

    #[test]
    fn test_validate_replication_controller_selector_mismatch() {
        let mut rc = base_rc();
        rc.spec
            .as_mut()
            .unwrap()
            .selector
            .insert("other".to_string(), "value".to_string());
        let errs = validate_replication_controller(&rc);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("selector` does not match"))
        );
    }

    #[test]
    fn test_validate_replication_controller_restart_policy_must_be_always() {
        let mut rc = base_rc();
        if let Some(ref mut spec) = rc.spec.as_mut().unwrap().template.as_mut().unwrap().spec {
            spec.restart_policy = Some("OnFailure".to_string());
        }
        let errs = validate_replication_controller(&rc);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("restartPolicy"))
        );
    }

    #[test]
    fn test_validate_replication_controller_active_deadline_seconds_forbidden() {
        let mut rc = base_rc();
        if let Some(ref mut spec) = rc.spec.as_mut().unwrap().template.as_mut().unwrap().spec {
            spec.active_deadline_seconds = Some(10);
        }
        let errs = validate_replication_controller(&rc);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("activeDeadlineSeconds"))
        );
    }

    #[test]
    fn test_validate_replication_controller_status_invalid_counts() {
        let status = ReplicationControllerStatus {
            replicas: 1,
            ready_replicas: 2,
            available_replicas: 3,
            fully_labeled_replicas: 4,
            observed_generation: Some(1),
            conditions: Vec::new(),
        };

        let errs = validate_replication_controller_status(
            &status,
            &crate::common::validation::Path::nil(),
        );
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("readyReplicas"))
        );
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("availableReplicas"))
        );
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("fullyLabeledReplicas"))
        );
    }
}
