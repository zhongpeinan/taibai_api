//! PodTemplate validation for Kubernetes core internal API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{
    ErrorList, Path, forbidden, name_is_dns_subdomain, validate_labels, validate_object_meta,
    validate_object_meta_update,
};
use crate::core::internal::pod::{PodTemplate, PodTemplateSpec};
use crate::core::internal::validation::pod_spec;

/// Validates a PodTemplate resource.
///
/// Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go `ValidatePodTemplate`
pub fn validate_pod_template(pod: &PodTemplate) -> ErrorList {
    validate_pod_template_with_path(pod, &Path::nil())
}

fn validate_pod_template_with_path(pod: &PodTemplate, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (PodTemplate is namespaced).
    all_errs.extend(validate_object_meta(
        &pod.metadata,
        true,
        name_is_dns_subdomain,
        &path.child("metadata"),
    ));

    // Validate template spec.
    all_errs.extend(validate_pod_template_spec(
        &pod.template,
        &path.child("template"),
    ));

    all_errs
}

/// Validates a PodTemplateSpec.
///
/// Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go `ValidatePodTemplateSpec`
pub fn validate_pod_template_spec(spec: &PodTemplateSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if !spec.metadata.labels.is_empty() {
        all_errs.extend(validate_labels(
            &spec.metadata.labels,
            &path.child("metadata").child("labels"),
        ));
    }

    all_errs.extend(pod_spec::validate_pod_spec(&spec.spec, &path.child("spec")));
    if !spec.spec.ephemeral_containers.is_empty() {
        all_errs.push(forbidden(
            &path.child("spec").child("ephemeralContainers"),
            "ephemeral containers not allowed in pod template",
        ));
    }

    all_errs
}

/// Validates PodTemplate update.
pub fn validate_pod_template_update(new: &PodTemplate, old: &PodTemplate) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &Path::nil().child("metadata"),
    ));

    all_errs.extend(validate_pod_template_with_path(new, &Path::nil()));
    all_errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ObjectMeta;
    use crate::core::internal::pod::{Container, PodSpec};

    fn base_template() -> PodTemplate {
        use crate::common::TypeMeta;
        PodTemplate {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("template".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: ObjectMeta {
                    labels: [("app".to_string(), "demo".to_string())].into(),
                    ..Default::default()
                },
                spec: PodSpec {
                    termination_grace_period_seconds: Some(30),
                    restart_policy: crate::core::internal::RestartPolicy::Always,
                    dns_policy: crate::core::internal::DNSPolicy::ClusterFirst,
                    containers: vec![Container {
                        name: "nginx".to_string(),
                        image: Some("nginx:latest".to_string()),
                        termination_message_policy: Some("File".to_string()),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            },
        }
    }

    #[test]
    fn test_validate_pod_template_valid() {
        let template = base_template();
        let errs = validate_pod_template(&template);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_pod_template_missing_name() {
        let mut template = base_template();
        template.metadata.name = None;
        let errs = validate_pod_template(&template);
        assert!(
            errs.errors.iter().any(|e| e.field.contains("metadata")),
            "Expected metadata error for missing name"
        );
    }

    #[test]
    fn test_validate_pod_template_spec_forbids_ephemeral_containers() {
        let mut template = base_template();
        template
            .template
            .spec
            .ephemeral_containers
            .push(Default::default());
        let errs = validate_pod_template(&template);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("ephemeralContainers")),
            "Expected error for ephemeral containers in template"
        );
    }

    #[test]
    fn test_validate_pod_template_update_valid() {
        let mut old = base_template();
        old.metadata.resource_version = Some("123".to_string());
        let mut new = base_template();
        new.metadata.resource_version = Some("123".to_string());

        let errs = validate_pod_template_update(&new, &old);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_pod_template_invalid_labels() {
        let mut template = base_template();
        template.template.metadata.labels = [("invalid key!".to_string(), "v".to_string())].into();
        let errs = validate_pod_template(&template);
        assert!(
            errs.errors.iter().any(|e| e.field.contains("labels")),
            "Expected error for invalid label key"
        );
    }
}
