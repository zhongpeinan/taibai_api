//! Pod validation
//!
//! Validates Pod metadata and PodSpec.

use crate::common::validation::{ErrorList, Path, name_is_dns_subdomain, required};
use crate::core::v1::validation::pod_spec;
use crate::core::v1::{Pod, PodSpec};

/// Validates a Pod.
pub fn validate_pod(pod: &Pod) -> ErrorList {
    validate_pod_with_path(pod, &Path::nil())
}

fn validate_pod_with_path(pod: &Pod, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (Pod is namespaced).
    if let Some(ref metadata) = pod.metadata {
        all_errs.extend(crate::common::validation::validate_object_meta(
            metadata,
            true,
            name_is_dns_subdomain,
            &path.child("metadata"),
        ));
    } else {
        all_errs.push(required(&path.child("metadata"), "metadata is required"));
    }

    // Validate spec.
    if let Some(ref spec) = pod.spec {
        all_errs.extend(pod_spec::validate_pod_spec(spec, &path.child("spec")));
    } else {
        all_errs.push(required(&path.child("spec"), "spec is required"));
    }

    all_errs
}

/// Validates a PodSpec.
pub fn validate_pod_spec(spec: &PodSpec, path: &Path) -> ErrorList {
    pod_spec::validate_pod_spec(spec, path)
}

/// Validates Pod update.
pub fn validate_pod_update(new: &Pod, old: &Pod) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let (Some(new_meta), Some(old_meta)) = (&new.metadata, &old.metadata) {
        all_errs.extend(crate::common::validation::validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::nil().child("metadata"),
        ));
    }

    all_errs.extend(validate_pod_with_path(new, &Path::nil()));
    all_errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::v1::pod::Container;

    #[test]
    fn test_validate_pod_requires_metadata_and_spec() {
        let pod = Pod::default();
        let errs = validate_pod(&pod);
        assert!(errs.errors.iter().any(|e| e.field.ends_with("metadata")));
        assert!(errs.errors.iter().any(|e| e.field.ends_with("spec")));
    }

    #[test]
    fn test_validate_pod_wires_spec_validation() {
        let pod = Pod {
            metadata: Some(crate::common::ObjectMeta {
                name: Some("demo".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(PodSpec {
                termination_grace_period_seconds: None,
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
            ..Default::default()
        };

        let errs = validate_pod(&pod);
        assert!(errs.errors.iter().any(|e| {
            e.detail
                .contains("terminationGracePeriodSeconds is required")
        }));
    }
}
