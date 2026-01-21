//! VolumeAttachment types
//!
//! VolumeAttachment captures the intent to attach or detach the specified volume
//! to/from the specified node.
//!
//! Corresponds to [Kubernetes VolumeAttachment](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storage/types.go#L121)

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::common::{
    ApplyDefaults, HasTypeMeta, ListMeta, ObjectMeta, PersistentVolumeSpec, ResourceSchema,
    Timestamp, TypeMeta, UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;

/// VolumeAttachment captures the intent to attach or detach the specified volume
/// to/from the specified node.
///
/// VolumeAttachment objects are non-namespaced.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachment {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// spec represents specification of the desired attach/detach volume behavior.
    pub spec: VolumeAttachmentSpec,

    /// status represents status of the VolumeAttachment request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<VolumeAttachmentStatus>,
}

/// VolumeAttachmentList is a collection of VolumeAttachment objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is the list of VolumeAttachments
    #[serde(default)]
    pub items: Vec<VolumeAttachment>,
}

/// VolumeAttachmentSpec is the specification of a VolumeAttachment request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentSpec {
    /// attacher indicates the name of the volume driver that MUST handle this request.
    pub attacher: String,

    /// source represents the volume that should be attached.
    pub source: VolumeAttachmentSource,

    /// nodeName represents the node that the volume should be attached to.
    pub node_name: String,
}

/// VolumeAttachmentSource represents a volume that should be attached.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentSource {
    /// persistentVolumeName represents the name of the persistent volume to attach.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_name: Option<String>,

    /// inlineVolumeSpec contains all the information necessary to attach
    /// a persistent volume defined by a pod's inline VolumeSource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_volume_spec: Option<PersistentVolumeSpec>,
}

/// VolumeAttachmentStatus is the status of a VolumeAttachment request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentStatus {
    /// attached indicates the volume is successfully attached.
    #[serde(default)]
    pub attached: bool,

    /// attachmentMetadata is populated with any information returned by the attach operation.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attachment_metadata: BTreeMap<String, String>,

    /// attachError represents the last error encountered during attach operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attach_error: Option<VolumeError>,

    /// detachError represents the last error encountered during detach operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detach_error: Option<VolumeError>,
}

/// VolumeError captures an error encountered during a volume operation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeError {
    /// time represents the time the error was encountered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,

    /// message represents the error encountered during Attach or Detach operation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,

    /// errorCode is a numeric gRPC code representing the error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
}

// ============================================================================
// Trait Implementations for VolumeAttachment and VolumeAttachmentList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for VolumeAttachment {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "VolumeAttachment"
    }
    fn resource(_: &Self::Meta) -> &str {
        "volumeattachments"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "VolumeAttachment"
    }
    fn resource_static() -> &'static str {
        "volumeattachments"
    }
}

impl ResourceSchema for VolumeAttachmentList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "VolumeAttachmentList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "volumeattachments"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "VolumeAttachmentList"
    }
    fn resource_static() -> &'static str {
        "volumeattachments"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for VolumeAttachment {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for VolumeAttachmentList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for VolumeAttachment {
    fn metadata(&self) -> &ObjectMeta {
        self.metadata
            .as_ref()
            .unwrap_or_else(|| static_default_object_meta())
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Helper function for static default ObjectMeta
fn static_default_object_meta() -> &'static ObjectMeta {
    static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
    DEFAULT.get_or_init(ObjectMeta::default)
}

// Note: VolumeAttachmentList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefaults for VolumeAttachment {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "VolumeAttachment".to_string();
        }
    }
}

impl ApplyDefaults for VolumeAttachmentList {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "VolumeAttachmentList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for VolumeAttachment {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(VolumeAttachment);
impl_unimplemented_prost_message!(VolumeAttachmentList);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_attachment_default() {
        let va = VolumeAttachment::default();
        assert!(va.metadata.is_none());
        assert!(va.spec.attacher.is_empty());
    }

    #[test]
    fn test_volume_attachment_with_spec() {
        let source = VolumeAttachmentSource {
            persistent_volume_name: Some("pv-1".to_string()),
            ..Default::default()
        };

        let spec = VolumeAttachmentSpec {
            attacher: "csi.example.com".to_string(),
            source,
            node_name: "node-1".to_string(),
        };

        let va = VolumeAttachment {
            type_meta: TypeMeta::default(),
            spec,
            ..Default::default()
        };
        assert_eq!(va.spec.attacher, "csi.example.com");
        assert_eq!(va.spec.node_name, "node-1");
    }

    #[test]
    fn test_volume_attachment_serialize() {
        let source = VolumeAttachmentSource {
            persistent_volume_name: Some("pv-1".to_string()),
            ..Default::default()
        };

        let spec = VolumeAttachmentSpec {
            attacher: "csi.example.com".to_string(),
            source,
            node_name: "node-1".to_string(),
        };

        let va = VolumeAttachment {
            type_meta: TypeMeta::default(),
            spec,
            ..Default::default()
        };
        let json = serde_json::to_string(&va).unwrap();
        assert!(json.contains("\"attacher\":\"csi.example.com\""));
        assert!(json.contains("\"nodeName\":\"node-1\""));
    }

    #[test]
    fn test_volume_attachment_deserialize() {
        let json =
            "{\"spec\":{\"attacher\":\"csi.example.com\",\"source\":{},\"nodeName\":\"node-1\"}}";
        let va: VolumeAttachment = serde_json::from_str(json).unwrap();
        assert_eq!(va.spec.attacher, "csi.example.com");
        assert_eq!(va.spec.node_name, "node-1");
    }

    #[test]
    fn test_volume_attachment_round_trip() {
        let source = VolumeAttachmentSource {
            persistent_volume_name: Some("pv-1".to_string()),
            ..Default::default()
        };

        let spec = VolumeAttachmentSpec {
            attacher: "csi.example.com".to_string(),
            source,
            node_name: "node-1".to_string(),
        };

        let status = VolumeAttachmentStatus {
            attached: true,
            ..Default::default()
        };

        let original = VolumeAttachment {
            spec,
            status: Some(status),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: VolumeAttachment = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_volume_attachment_source_with_persistent_volume() {
        let source = VolumeAttachmentSource {
            persistent_volume_name: Some("pv-1".to_string()),
            inline_volume_spec: None,
        };
        assert_eq!(source.persistent_volume_name, Some("pv-1".to_string()));
        assert!(source.inline_volume_spec.is_none());
    }

    #[test]
    fn test_volume_attachment_source_serialize() {
        let source = VolumeAttachmentSource {
            persistent_volume_name: Some("pv-1".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&source).unwrap();
        assert!(json.contains("persistentVolumeName"));
        assert!(json.contains("pv-1"));
    }

    #[test]
    fn test_volume_attachment_source_deserialize() {
        let json = "{\"persistentVolumeName\":\"pv-1\"}";
        let source: VolumeAttachmentSource = serde_json::from_str(json).unwrap();
        assert_eq!(source.persistent_volume_name, Some("pv-1".to_string()));
    }

    #[test]
    fn test_volume_attachment_source_round_trip() {
        let original = VolumeAttachmentSource {
            persistent_volume_name: Some("pv-1".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: VolumeAttachmentSource = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_volume_attachment_status() {
        let mut metadata = BTreeMap::new();
        metadata.insert("device".to_string(), "/dev/sdx".to_string());

        let status = VolumeAttachmentStatus {
            attached: true,
            attachment_metadata: metadata,
            ..Default::default()
        };
        assert!(status.attached);
        assert_eq!(status.attachment_metadata.len(), 1);
    }

    #[test]
    fn test_volume_attachment_status_serialize() {
        let status = VolumeAttachmentStatus {
            attached: true,
            ..Default::default()
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"attached\":true"));
    }

    #[test]
    fn test_volume_attachment_status_with_error() {
        let error = VolumeError {
            message: "failed to attach volume".to_string(),
            ..Default::default()
        };

        let status = VolumeAttachmentStatus {
            attached: false,
            attach_error: Some(error),
            ..Default::default()
        };
        assert!(!status.attached);
        assert!(status.attach_error.is_some());
    }

    #[test]
    fn test_volume_error() {
        let error = VolumeError {
            time: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            message: "attachment failed".to_string(),
            error_code: Some(5),
        };
        assert_eq!(error.message, "attachment failed");
        assert_eq!(error.error_code, Some(5));
    }

    #[test]
    fn test_volume_error_serialize() {
        let error = VolumeError {
            message: "attachment failed".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("attachment failed"));
    }

    #[test]
    fn test_volume_error_deserialize() {
        let json = "{\"message\":\"attachment failed\",\"errorCode\":5}";
        let error: VolumeError = serde_json::from_str(json).unwrap();
        assert_eq!(error.message, "attachment failed");
        assert_eq!(error.error_code, Some(5));
    }

    #[test]
    fn test_volume_error_round_trip() {
        let original = VolumeError {
            time: Some(Timestamp::from_str("2024-01-15T10:00:00Z").unwrap()),
            message: "attachment failed".to_string(),
            error_code: Some(5),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: VolumeError = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_volume_attachment_list() {
        let spec = VolumeAttachmentSpec {
            attacher: "csi.example.com".to_string(),
            source: VolumeAttachmentSource::default(),
            node_name: "node-1".to_string(),
        };

        let va = VolumeAttachment {
            type_meta: TypeMeta::default(),
            spec,
            ..Default::default()
        };

        let list = VolumeAttachmentList {
            items: vec![va],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_volume_attachment_list_serialize() {
        let list = VolumeAttachmentList {
            items: vec![],
            ..Default::default()
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("\"items\":[]"));
    }
}
