//! Pod validation for core internal API types.

use crate::common::validation::{ErrorList, Path, name_is_dns_subdomain};
use crate::core::internal::validation::pod_spec;
use crate::core::internal::{Pod, PodSpec};

/// Validates a Pod.
pub fn validate_pod(pod: &Pod) -> ErrorList {
    validate_pod_with_path(pod, &Path::nil())
}

fn validate_pod_with_path(pod: &Pod, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (Pod is namespaced).
    all_errs.extend(crate::common::validation::validate_object_meta(
        &pod.metadata,
        true,
        name_is_dns_subdomain,
        &path.child("metadata"),
    ));

    // Validate spec.
    all_errs.extend(pod_spec::validate_pod_spec(&pod.spec, &path.child("spec")));

    all_errs
}

/// Validates a PodSpec.
pub fn validate_pod_spec(spec: &PodSpec, path: &Path) -> ErrorList {
    pod_spec::validate_pod_spec(spec, path)
}

/// Validates Pod update.
pub fn validate_pod_update(new: &Pod, old: &Pod) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(crate::common::validation::validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &Path::nil().child("metadata"),
    ));

    all_errs.extend(validate_pod_with_path(new, &Path::nil()));
    all_errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::internal::InternalContainer;

    #[test]
    fn test_validate_pod_wires_spec_validation() {
        let pod = Pod {
            metadata: crate::common::ObjectMeta {
                name: Some("demo".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            spec: PodSpec {
                termination_grace_period_seconds: None,
                containers: vec![InternalContainer {
                    name: "nginx".to_string(),
                    image: Some("nginx:latest".to_string()),
                    termination_message_policy: Some("File".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        };

        let errs = validate_pod(&pod);
        assert!(errs.errors.iter().any(|e| {
            e.detail
                .contains("terminationGracePeriodSeconds is required")
        }));
    }
}
