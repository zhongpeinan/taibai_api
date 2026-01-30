//! PodTemplate validation for Kubernetes core/v1 API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{ErrorList, Path, forbidden, name_is_dns_subdomain, required};
use crate::core::v1::template::{PodTemplate, PodTemplateSpec};
use crate::core::v1::validation::pod_spec;

/// Validates a PodTemplate resource.
pub fn validate_pod_template(pod: &PodTemplate) -> ErrorList {
    validate_pod_template_with_path(pod, &Path::nil())
}

fn validate_pod_template_with_path(pod: &PodTemplate, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (PodTemplate is namespaced).
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

    // Validate template spec.
    if let Some(ref template) = pod.template {
        all_errs.extend(validate_pod_template_spec(
            template,
            &path.child("template"),
        ));
    } else {
        all_errs.push(required(&path.child("template"), "template is required"));
    }

    all_errs
}

/// Validates a PodTemplateSpec.
pub fn validate_pod_template_spec(spec: &PodTemplateSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref metadata) = spec.metadata {
        if !metadata.labels.is_empty() {
            all_errs.extend(crate::common::validation::validate_labels(
                &metadata.labels,
                &path.child("metadata").child("labels"),
            ));
        }
    }

    if let Some(ref pod_spec) = spec.spec {
        all_errs.extend(pod_spec::validate_pod_spec(pod_spec, &path.child("spec")));
        if !pod_spec.ephemeral_containers.is_empty() {
            all_errs.push(forbidden(
                &path.child("spec").child("ephemeralContainers"),
                "ephemeral containers not allowed in pod template",
            ));
        }
    } else {
        all_errs.push(required(&path.child("spec"), "spec is required"));
    }

    all_errs
}

/// Validates PodTemplate update.
pub fn validate_pod_template_update(new: &PodTemplate, old: &PodTemplate) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let (Some(new_meta), Some(old_meta)) = (&new.metadata, &old.metadata) {
        all_errs.extend(crate::common::validation::validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::nil().child("metadata"),
        ));
    }

    all_errs.extend(validate_pod_template_with_path(new, &Path::nil()));
    all_errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ObjectMeta;
    use crate::core::v1::pod::Container;

    fn base_template() -> PodTemplate {
        PodTemplate {
            metadata: Some(ObjectMeta {
                name: Some("template".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            template: Some(PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: [("app".to_string(), "demo".to_string())].into(),
                    ..Default::default()
                }),
                spec: Some(crate::core::v1::pod::PodSpec {
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
            ..Default::default()
        }
    }

    #[test]
    fn test_validate_pod_template_requires_metadata_and_template() {
        let template = PodTemplate::default();
        let errs = validate_pod_template(&template);
        assert!(errs.errors.iter().any(|e| e.field.ends_with("metadata")));
        assert!(errs.errors.iter().any(|e| e.field.ends_with("template")));
    }

    #[test]
    fn test_validate_pod_template_spec_forbids_ephemeral_containers() {
        let mut template = base_template();
        if let Some(ref mut spec) = template.template.as_mut().unwrap().spec {
            spec.ephemeral_containers.push(Default::default());
        }
        let errs = validate_pod_template(&template);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("ephemeralContainers"))
        );
    }
}
