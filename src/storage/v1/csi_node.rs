//! CSINode types
//!
//! CSINode holds information about all CSI drivers installed on a node.
//!
//! Corresponds to [Kubernetes CSINode](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storage/types.go#L549)

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta,
    UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;

/// CSINode holds information about all CSI drivers installed on a node.
///
/// CSINode has the same name as a node.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSINode {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    /// metadata.name must be the Kubernetes node name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// spec is the specification of CSINode
    pub spec: CSINodeSpec,
}

/// CSINodeList is a collection of CSINode objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSINodeList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is the list of CSINode
    #[serde(default)]
    pub items: Vec<CSINode>,
}

/// CSINodeSpec holds information about the specification of all CSI drivers installed on a node
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSINodeSpec {
    /// drivers is a list of information of all CSI Drivers existing on a node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drivers: Vec<CSINodeDriver>,
}

/// CSINodeDriver holds information about the specification of one CSI driver installed on a node
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSINodeDriver {
    /// name represents the name of the CSI driver that this object refers to.
    pub name: String,

    /// nodeID of the node from the driver point of view.
    pub node_id: String,

    /// topologyKeys is the list of keys supported by the driver.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topology_keys: Vec<String>,

    /// allocatable represents the volume resources of a node that are available for scheduling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<VolumeNodeResources>,
}

/// VolumeNodeResources is a set of resource limits for scheduling of volumes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeNodeResources {
    /// count indicates the maximum number of unique volumes managed by the CSI driver
    /// that can be used on a node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

// ============================================================================
// Trait Implementations for CSINode and CSINodeList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for CSINode {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CSINode"
    }
    fn resource(_: &Self::Meta) -> &str {
        "csinodes"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "CSINode"
    }
    fn resource_static() -> &'static str {
        "csinodes"
    }
}

impl ResourceSchema for CSINodeList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CSINodeList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "csinodes"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "CSINodeList"
    }
    fn resource_static() -> &'static str {
        "csinodes"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for CSINode {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for CSINodeList {
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

impl VersionedObject for CSINode {
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

// Note: CSINodeList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for CSINode {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CSINode".to_string();
        }
    }
}

impl ApplyDefault for CSINodeList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storage.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CSINodeList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for CSINode {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(CSINode);
impl_unimplemented_prost_message!(CSINodeList);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csi_node_default() {
        let node = CSINode::default();
        assert!(node.metadata.is_none());
        assert!(node.spec.drivers.is_empty());
    }

    #[test]
    fn test_csi_node_with_drivers() {
        let driver = CSINodeDriver {
            name: "csi.example.com".to_string(),
            node_id: "node-1".to_string(),
            ..Default::default()
        };

        let spec = CSINodeSpec {
            drivers: vec![driver],
        };

        let node = CSINode {
            spec,
            ..Default::default()
        };
        assert_eq!(node.spec.drivers.len(), 1);
    }

    #[test]
    fn test_csi_node_serialize() {
        let driver = CSINodeDriver {
            name: "csi.example.com".to_string(),
            node_id: "node-1".to_string(),
            ..Default::default()
        };

        let spec = CSINodeSpec {
            drivers: vec![driver],
        };

        let node = CSINode {
            spec,
            ..Default::default()
        };
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"name\":\"csi.example.com\""));
        assert!(json.contains("\"nodeId\":\"node-1\""));
    }

    #[test]
    fn test_csi_node_deserialize() {
        let json =
            "{\"spec\":{\"drivers\":[{\"name\":\"csi.example.com\",\"nodeId\":\"node-1\"}]}}";
        let node: CSINode = serde_json::from_str(json).unwrap();
        assert_eq!(node.spec.drivers.len(), 1);
        assert_eq!(node.spec.drivers[0].name, "csi.example.com");
    }

    #[test]
    fn test_csi_node_round_trip() {
        let driver = CSINodeDriver {
            name: "csi.example.com".to_string(),
            node_id: "node-1".to_string(),
            topology_keys: vec!["topology.kubernetes.io/zone".to_string()],
            ..Default::default()
        };

        let original = CSINode {
            spec: CSINodeSpec {
                drivers: vec![driver],
            },
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: CSINode = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_csi_node_driver() {
        let driver = CSINodeDriver {
            name: "csi.example.com".to_string(),
            node_id: "node-1".to_string(),
            topology_keys: vec!["topology.kubernetes.io/zone".to_string()],
            allocatable: Some(VolumeNodeResources { count: Some(10) }),
        };
        assert_eq!(driver.name, "csi.example.com");
        assert_eq!(driver.node_id, "node-1");
        assert_eq!(driver.topology_keys.len(), 1);
        assert_eq!(driver.allocatable.as_ref().unwrap().count, Some(10));
    }

    #[test]
    fn test_csi_node_driver_serialize() {
        let driver = CSINodeDriver {
            name: "csi.example.com".to_string(),
            node_id: "node-1".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&driver).unwrap();
        assert!(json.contains("\"name\":\"csi.example.com\""));
        assert!(json.contains("\"nodeId\":\"node-1\""));
    }

    #[test]
    fn test_csi_node_driver_deserialize() {
        let json = "{\"name\":\"csi.example.com\",\"nodeId\":\"node-1\"}";
        let driver: CSINodeDriver = serde_json::from_str(json).unwrap();
        assert_eq!(driver.name, "csi.example.com");
        assert_eq!(driver.node_id, "node-1");
    }

    #[test]
    fn test_csi_node_driver_round_trip() {
        let original = CSINodeDriver {
            name: "csi.example.com".to_string(),
            node_id: "node-1".to_string(),
            topology_keys: vec!["topology.kubernetes.io/zone".to_string()],
            allocatable: Some(VolumeNodeResources { count: Some(10) }),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: CSINodeDriver = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_volume_node_resources() {
        let resources = VolumeNodeResources { count: Some(10) };
        assert_eq!(resources.count, Some(10));
    }

    #[test]
    fn test_volume_node_resources_default() {
        let resources = VolumeNodeResources::default();
        assert!(resources.count.is_none());
    }

    #[test]
    fn test_volume_node_resources_serialize() {
        let resources = VolumeNodeResources { count: Some(10) };
        let json = serde_json::to_string(&resources).unwrap();
        assert!(json.contains("\"count\":10"));
    }

    #[test]
    fn test_volume_node_resources_deserialize() {
        let json = "{\"count\":10}";
        let resources: VolumeNodeResources = serde_json::from_str(json).unwrap();
        assert_eq!(resources.count, Some(10));
    }

    #[test]
    fn test_volume_node_resources_round_trip() {
        let original = VolumeNodeResources { count: Some(10) };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: VolumeNodeResources = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_csi_node_list() {
        let node = CSINode {
            spec: CSINodeSpec::default(),
            ..Default::default()
        };

        let list = CSINodeList {
            items: vec![node],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_csi_node_list_serialize() {
        let list = CSINodeList {
            items: vec![],
            ..Default::default()
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("\"items\":[]"));
    }

    #[test]
    fn test_csi_node_driver_with_topology_keys() {
        let driver = CSINodeDriver {
            name: "csi.example.com".to_string(),
            node_id: "node-1".to_string(),
            topology_keys: vec![
                "topology.kubernetes.io/zone".to_string(),
                "topology.kubernetes.io/region".to_string(),
            ],
            ..Default::default()
        };
        assert_eq!(driver.topology_keys.len(), 2);
    }

    #[test]
    fn test_csi_node_driver_empty_topology_keys_omitted() {
        let driver = CSINodeDriver {
            name: "csi.example.com".to_string(),
            node_id: "node-1".to_string(),
            topology_keys: vec![],
            ..Default::default()
        };
        let json = serde_json::to_string(&driver).unwrap();
        // Empty topology_keys should be omitted
        assert!(!json.contains("topologyKeys"));
    }

    #[test]
    fn test_csi_node_spec_with_multiple_drivers() {
        let driver1 = CSINodeDriver {
            name: "csi.example.com".to_string(),
            node_id: "node-1".to_string(),
            ..Default::default()
        };

        let driver2 = CSINodeDriver {
            name: "csi.other.com".to_string(),
            node_id: "node-1".to_string(),
            ..Default::default()
        };

        let spec = CSINodeSpec {
            drivers: vec![driver1, driver2],
        };
        assert_eq!(spec.drivers.len(), 2);
    }
}
