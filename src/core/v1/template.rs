//! Kubernetes Pod Template types
//!
//! This module contains pod template-related types from the Kubernetes core/v1 API.
//! Pod templates are used to define pod specifications that can be reused by controllers
//! such as Deployments, StatefulSets, DaemonSets, and Jobs.

use crate::common::meta::{ListMeta, ObjectMeta};
use crate::common::{ApplyDefault, HasTypeMeta, ResourceSchema, TypeMeta};
use crate::core::v1::pod::PodSpec;
use crate::core::v1::selector::object_field_selector_api_version;
use crate::core::v1::volume::{
    ConfigMapVolumeSource, DownwardAPIVolumeSource, EphemeralVolumeSource, HostPathVolumeSource,
    ImageVolumeSource, ProjectedVolumeSource, SecretVolumeSource, ServiceAccountTokenProjection,
    Volume, VolumeProjection, VolumeSource, host_path_type,
};
use crate::{impl_unimplemented_prost_message, impl_versioned_object};
use serde::{Deserialize, Serialize};

/// PodTemplate describes a template for creating copies of a predefined pod.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplate {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Template defines the pods that will be created from this pod template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
}
impl_versioned_object!(PodTemplate);

/// PodTemplateSpec describes the data a pod should have when created from a template.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateSpec {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Specification of the desired behavior of the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PodSpec>,
}
impl_versioned_object!(PodTemplateSpec);

/// PodTemplateList is a list of PodTemplates.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodTemplateList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of pod templates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PodTemplate>,
}

// ============================================================================
// Trait Implementations for PodTemplate Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for PodTemplate {
    type Meta = ();
    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PodTemplate"
    }
    fn resource(_: &Self::Meta) -> &str {
        "podtemplates"
    }
    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PodTemplate"
    }
    fn resource_static() -> &'static str {
        "podtemplates"
    }
}

impl ResourceSchema for PodTemplateList {
    type Meta = ();
    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "PodTemplateList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "podtemplates"
    }
    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "PodTemplateList"
    }
    fn resource_static() -> &'static str {
        "podtemplates"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for PodTemplate {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for PodTemplateList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// Note: VersionedObject for PodTemplate is implemented by impl_versioned_object! macro
// Note: PodTemplateList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for PodTemplate {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PodTemplate".to_string();
        }
        if let Some(ref mut template) = self.template {
            apply_pod_template_spec_defaults(template);
        }
    }
}

impl ApplyDefault for PodTemplateList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PodTemplateList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(PodTemplate);
impl_unimplemented_prost_message!(PodTemplateList);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::v1::pod::PodSpec;
    use crate::core::v1::selector::ObjectFieldSelector;
    use crate::core::v1::volume::{
        ConfigMapVolumeSource, DownwardAPIVolumeFile, DownwardAPIVolumeSource,
        HostPathVolumeSource, ImageVolumeSource, ProjectedVolumeSource, SecretVolumeSource,
        ServiceAccountTokenProjection, Volume, VolumeProjection, VolumeSource,
    };

    #[test]
    fn test_pod_template_apply_default_sets_spec_defaults() {
        let mut template = PodTemplate {
            template: Some(PodTemplateSpec {
                spec: Some(PodSpec::default()),
                ..Default::default()
            }),
            ..Default::default()
        };

        template.apply_default();

        let spec = template.template.unwrap().spec.unwrap();
        assert_eq!(spec.dns_policy.as_deref(), Some("ClusterFirst"));
        assert_eq!(spec.restart_policy.as_deref(), Some("Always"));
        assert_eq!(spec.termination_grace_period_seconds, Some(30));
        assert_eq!(spec.scheduler_name.as_deref(), Some("default-scheduler"));
        assert!(spec.security_context.is_some());
    }

    #[test]
    fn test_pod_template_apply_default_sets_volume_defaults() {
        let mut template = PodTemplate {
            template: Some(PodTemplateSpec {
                spec: Some(PodSpec {
                    volumes: vec![
                        Volume {
                            name: "empty".to_string(),
                            volume_source: VolumeSource::default(),
                        },
                        Volume {
                            name: "secret".to_string(),
                            volume_source: VolumeSource {
                                secret: Some(SecretVolumeSource::default()),
                                ..Default::default()
                            },
                        },
                        Volume {
                            name: "config".to_string(),
                            volume_source: VolumeSource {
                                config_map: Some(ConfigMapVolumeSource::default()),
                                ..Default::default()
                            },
                        },
                        Volume {
                            name: "downward".to_string(),
                            volume_source: VolumeSource {
                                downward_api: Some(DownwardAPIVolumeSource {
                                    items: vec![DownwardAPIVolumeFile {
                                        field_ref: Some(ObjectFieldSelector {
                                            api_version: String::new(),
                                            field_path: "metadata.name".to_string(),
                                        }),
                                        ..Default::default()
                                    }],
                                    ..Default::default()
                                }),
                                ..Default::default()
                            },
                        },
                        Volume {
                            name: "projected".to_string(),
                            volume_source: VolumeSource {
                                projected: Some(ProjectedVolumeSource {
                                    sources: vec![VolumeProjection {
                                        service_account_token: Some(
                                            ServiceAccountTokenProjection {
                                                expiration_seconds: None,
                                                path: "token".to_string(),
                                                ..Default::default()
                                            },
                                        ),
                                        ..Default::default()
                                    }],
                                    ..Default::default()
                                }),
                                ..Default::default()
                            },
                        },
                        Volume {
                            name: "image".to_string(),
                            volume_source: VolumeSource {
                                image: Some(ImageVolumeSource {
                                    reference: "nginx:latest".to_string(),
                                    pull_policy: None,
                                }),
                                ..Default::default()
                            },
                        },
                        Volume {
                            name: "hostpath".to_string(),
                            volume_source: VolumeSource {
                                host_path: Some(HostPathVolumeSource {
                                    path: "/data".to_string(),
                                    type_: None,
                                }),
                                ..Default::default()
                            },
                        },
                    ],
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        template.apply_default();

        let spec = template.template.unwrap().spec.unwrap();
        let empty = &spec.volumes[0].volume_source;
        assert!(empty.empty_dir.is_some());

        let secret = &spec.volumes[1].volume_source.secret.as_ref().unwrap();
        assert_eq!(secret.default_mode, Some(0o644));

        let config = &spec.volumes[2].volume_source.config_map.as_ref().unwrap();
        assert_eq!(config.default_mode, Some(0o644));

        let downward = &spec.volumes[3].volume_source.downward_api.as_ref().unwrap();
        assert_eq!(downward.default_mode, Some(0o644));
        let field_ref = downward.items[0].field_ref.as_ref().unwrap();
        assert_eq!(field_ref.api_version, "v1");

        let projected = &spec.volumes[4].volume_source.projected.as_ref().unwrap();
        assert_eq!(projected.default_mode, Some(0o644));
        let token = projected.sources[0].service_account_token.as_ref().unwrap();
        assert_eq!(token.expiration_seconds, Some(3600));

        let image = &spec.volumes[5].volume_source.image.as_ref().unwrap();
        assert_eq!(image.pull_policy.as_deref(), Some("Always"));

        let host_path = &spec.volumes[6].volume_source.host_path.as_ref().unwrap();
        assert_eq!(host_path.type_.as_deref(), Some(host_path_type::UNSET));
    }
}

const DEFAULT_VOLUME_MODE: i32 = 0o644;
const DEFAULT_SERVICE_ACCOUNT_TOKEN_EXPIRATION_SECONDS: i64 = 3600;

fn apply_pod_template_spec_defaults(template: &mut PodTemplateSpec) {
    if let Some(ref mut spec) = template.spec {
        spec.apply_default();
        apply_volume_defaults(&mut spec.volumes);
        apply_ephemeral_container_defaults(spec);
    }
}

fn apply_ephemeral_container_defaults(spec: &mut PodSpec) {
    for container in &mut spec.ephemeral_containers {
        container.apply_default();
    }
}

fn apply_volume_defaults(volumes: &mut [Volume]) {
    for volume in volumes {
        default_volume_source(volume);
        apply_volume_source_defaults(&mut volume.volume_source);
    }
}

fn default_volume_source(volume: &mut Volume) {
    if volume_source_is_empty(&volume.volume_source) {
        volume.volume_source.empty_dir = Some(Default::default());
    }
}

fn volume_source_is_empty(source: &VolumeSource) -> bool {
    source.host_path.is_none()
        && source.empty_dir.is_none()
        && source.gce_persistent_disk.is_none()
        && source.aws_elastic_block_store.is_none()
        && source.git_repo.is_none()
        && source.secret.is_none()
        && source.nfs.is_none()
        && source.iscsi.is_none()
        && source.glusterfs.is_none()
        && source.persistent_volume_claim.is_none()
        && source.rbd.is_none()
        && source.flex_volume.is_none()
        && source.cinder.is_none()
        && source.cephfs.is_none()
        && source.flocker.is_none()
        && source.downward_api.is_none()
        && source.fc.is_none()
        && source.azure_file.is_none()
        && source.config_map.is_none()
        && source.vsphere_volume.is_none()
        && source.quobyte.is_none()
        && source.azure_disk.is_none()
        && source.photon_persistent_disk.is_none()
        && source.projected.is_none()
        && source.portworx_volume.is_none()
        && source.scale_io.is_none()
        && source.storage_os.is_none()
        && source.csi.is_none()
        && source.ephemeral.is_none()
        && source.image.is_none()
}

fn apply_volume_source_defaults(source: &mut VolumeSource) {
    if let Some(ref mut host_path) = source.host_path {
        default_host_path(host_path);
    }
    if let Some(ref mut secret) = source.secret {
        default_secret_volume_source(secret);
    }
    if let Some(ref mut iscsi) = source.iscsi {
        if iscsi.iscsi_interface.is_empty() {
            iscsi.iscsi_interface = "default".to_string();
        }
    }
    if let Some(ref mut downward_api) = source.downward_api {
        default_downward_api_volume_source(downward_api);
    }
    if let Some(ref mut config_map) = source.config_map {
        default_config_map_volume_source(config_map);
    }
    if let Some(ref mut projected) = source.projected {
        default_projected_volume_source(projected);
    }
    if let Some(ref mut ephemeral) = source.ephemeral {
        default_ephemeral_volume_source(ephemeral);
    }
    if let Some(ref mut image) = source.image {
        default_image_volume_source(image);
    }
}

fn default_host_path(host_path: &mut HostPathVolumeSource) {
    if host_path.type_.is_none() {
        host_path.type_ = Some(host_path_type::UNSET.to_string());
    }
}

fn default_secret_volume_source(secret: &mut SecretVolumeSource) {
    if secret.default_mode.is_none() {
        secret.default_mode = Some(DEFAULT_VOLUME_MODE);
    }
}

fn default_config_map_volume_source(config_map: &mut ConfigMapVolumeSource) {
    if config_map.default_mode.is_none() {
        config_map.default_mode = Some(DEFAULT_VOLUME_MODE);
    }
}

fn default_downward_api_volume_source(downward_api: &mut DownwardAPIVolumeSource) {
    if downward_api.default_mode.is_none() {
        downward_api.default_mode = Some(DEFAULT_VOLUME_MODE);
    }
    for item in &mut downward_api.items {
        if let Some(ref mut field_ref) = item.field_ref {
            if field_ref.api_version.is_empty() {
                field_ref.api_version = object_field_selector_api_version::V1.to_string();
            }
        }
    }
}

fn default_projected_volume_source(projected: &mut ProjectedVolumeSource) {
    if projected.default_mode.is_none() {
        projected.default_mode = Some(DEFAULT_VOLUME_MODE);
    }
    for source in &mut projected.sources {
        default_volume_projection(source);
    }
}

fn default_volume_projection(source: &mut VolumeProjection) {
    if let Some(ref mut downward_api) = source.downward_api {
        for item in &mut downward_api.items {
            if let Some(ref mut field_ref) = item.field_ref {
                if field_ref.api_version.is_empty() {
                    field_ref.api_version = object_field_selector_api_version::V1.to_string();
                }
            }
        }
    }
    if let Some(ref mut token) = source.service_account_token {
        default_service_account_token_projection(token);
    }
}

fn default_service_account_token_projection(token: &mut ServiceAccountTokenProjection) {
    if token.expiration_seconds.is_none() {
        token.expiration_seconds = Some(DEFAULT_SERVICE_ACCOUNT_TOKEN_EXPIRATION_SECONDS);
    }
}

fn default_ephemeral_volume_source(ephemeral: &mut EphemeralVolumeSource) {
    if let Some(ref mut template) = ephemeral.volume_claim_template {
        if let Some(ref mut spec) = template.spec {
            spec.apply_default();
        }
    }
}

fn default_image_volume_source(image: &mut ImageVolumeSource) {
    let pull_policy = image.pull_policy.as_deref().unwrap_or("");
    if pull_policy.is_empty() {
        let is_latest = image_tag_or_latest(&image.reference) == "latest";
        image.pull_policy = Some(if is_latest {
            "Always".to_string()
        } else {
            "IfNotPresent".to_string()
        });
    }
}

fn image_tag_or_latest(image: &str) -> &str {
    let (name, _) = image.split_once('@').unwrap_or((image, ""));
    let last_slash = name.rfind('/');
    if let Some(colon) = name.rfind(':') {
        if last_slash.map(|slash| colon > slash).unwrap_or(true) {
            let tag = &name[colon + 1..];
            if !tag.is_empty() {
                return tag;
            }
        }
    }
    "latest"
}
