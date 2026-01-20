//! CSIStorageCapacity types
//!
//! CSIStorageCapacity stores the result of one CSI GetCapacity call.
//!
//! Corresponds to [Kubernetes CSIStorageCapacity](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storage/types.go#L648)

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::common::{
    ApplyDefaults, HasTypeMeta, LabelSelector, ListMeta, ObjectMeta, Quantity, ResourceSchema,
    TypeMeta, UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;

/// CSIStorageCapacity stores the result of one CSI GetCapacity call.
///
/// For a given StorageClass, this describes the available capacity in a
/// particular topology segment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIStorageCapacity {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// nodeTopology defines which nodes have access to the storage
    /// for which capacity was reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_topology: Option<LabelSelector>,

    /// storageClassName represents the name of the StorageClass.
    pub storage_class_name: String,

    /// capacity is the value reported by the CSI driver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<Quantity>,

    /// maximumVolumeSize is the largest size that may be used to create a volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_volume_size: Option<Quantity>,
}

/// CSIStorageCapacityList is a collection of CSIStorageCapacity objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CSIStorageCapacityList {
    /// TypeMeta for this resource
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is the list of CSIStorageCapacity objects.
    #[serde(default)]
    pub items: Vec<CSIStorageCapacity>,
}

// ============================================================================
// Trait Implementations for CSIStorageCapacity and CSIStorageCapacityList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for CSIStorageCapacity {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CSIStorageCapacity"
    }
    fn resource(_: &Self::Meta) -> &str {
        "csistoragecapacities"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "CSIStorageCapacity"
    }
    fn resource_static() -> &'static str {
        "csistoragecapacities"
    }
}

impl ResourceSchema for CSIStorageCapacityList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storage.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "CSIStorageCapacityList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "csistoragecapacities"
    }

    fn group_static() -> &'static str {
        "storage.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "CSIStorageCapacityList"
    }
    fn resource_static() -> &'static str {
        "csistoragecapacities"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for CSIStorageCapacity {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for CSIStorageCapacityList {
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

impl VersionedObject for CSIStorageCapacity {
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

// Note: CSIStorageCapacityList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefaults for CSIStorageCapacity {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_none() {
            self.type_meta.api_version = Some("storage.k8s.io/v1".to_string());
        }
        if self.type_meta.kind.is_none() {
            self.type_meta.kind = Some("CSIStorageCapacity".to_string());
        }
    }
}

impl ApplyDefaults for CSIStorageCapacityList {
    fn apply_defaults(&mut self) {
        if self.type_meta.api_version.is_none() {
            self.type_meta.api_version = Some("storage.k8s.io/v1".to_string());
        }
        if self.type_meta.kind.is_none() {
            self.type_meta.kind = Some("CSIStorageCapacityList".to_string());
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for CSIStorageCapacity {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(CSIStorageCapacity);
impl_unimplemented_prost_message!(CSIStorageCapacityList);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_csi_storage_capacity_default() {
        let cap = CSIStorageCapacity::default();
        assert!(cap.metadata.is_none());
        assert!(cap.storage_class_name.is_empty());
    }

    #[test]
    fn test_csi_storage_capacity_with_capacity() {
        let cap = CSIStorageCapacity {
            storage_class_name: "standard".to_string(),
            capacity: Some(Quantity::from("100Gi")),
            ..Default::default()
        };
        assert_eq!(cap.storage_class_name, "standard");
        assert!(cap.capacity.is_some());
    }

    #[test]
    fn test_csi_storage_capacity_with_node_topology() {
        let mut match_labels = BTreeMap::new();
        match_labels.insert(
            "topology.kubernetes.io/zone".to_string(),
            "us-east-1a".to_string(),
        );

        let selector = LabelSelector {
            match_labels,
            ..Default::default()
        };

        let cap = CSIStorageCapacity {
            storage_class_name: "standard".to_string(),
            node_topology: Some(selector),
            ..Default::default()
        };
        assert!(cap.node_topology.is_some());
    }

    #[test]
    fn test_csi_storage_capacity_serialize() {
        let cap = CSIStorageCapacity {
            storage_class_name: "standard".to_string(),
            capacity: Some(Quantity::from("100Gi")),
            ..Default::default()
        };
        let json = serde_json::to_string(&cap).unwrap();
        assert!(json.contains("\"storageClassName\":\"standard\""));
        assert!(json.contains("100Gi"));
    }

    #[test]
    fn test_csi_storage_capacity_deserialize() {
        let json = "{\"storageClassName\":\"standard\",\"capacity\":\"100Gi\"}";
        let cap: CSIStorageCapacity = serde_json::from_str(json).unwrap();
        assert_eq!(cap.storage_class_name, "standard");
        assert!(cap.capacity.is_some());
    }

    #[test]
    fn test_csi_storage_capacity_round_trip() {
        let original = CSIStorageCapacity {
            storage_class_name: "standard".to_string(),
            capacity: Some(Quantity::from("100Gi")),
            maximum_volume_size: Some(Quantity::from("10Gi")),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: CSIStorageCapacity = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_csi_storage_capacity_with_maximum_volume_size() {
        let cap = CSIStorageCapacity {
            storage_class_name: "standard".to_string(),
            capacity: Some(Quantity::from("100Gi")),
            maximum_volume_size: Some(Quantity::from("10Gi")),
            ..Default::default()
        };
        assert_eq!(cap.maximum_volume_size, Some(Quantity::from("10Gi")));
    }

    #[test]
    fn test_csi_storage_capacity_empty_capacity_omitted() {
        let cap = CSIStorageCapacity {
            storage_class_name: "standard".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&cap).unwrap();
        // None capacity should be omitted
        assert!(!json.contains("capacity"));
    }

    #[test]
    fn test_csi_storage_capacity_list() {
        let cap = CSIStorageCapacity {
            storage_class_name: "standard".to_string(),
            ..Default::default()
        };

        let list = CSIStorageCapacityList {
            items: vec![cap],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_csi_storage_capacity_list_serialize() {
        let list = CSIStorageCapacityList {
            items: vec![],
            ..Default::default()
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("\"items\":[]"));
    }

    #[test]
    fn test_csi_storage_capacity_list_with_multiple_items() {
        let cap1 = CSIStorageCapacity {
            storage_class_name: "standard".to_string(),
            capacity: Some(Quantity::from("100Gi")),
            ..Default::default()
        };

        let cap2 = CSIStorageCapacity {
            storage_class_name: "fast".to_string(),
            capacity: Some(Quantity::from("50Gi")),
            ..Default::default()
        };

        let list = CSIStorageCapacityList {
            items: vec![cap1, cap2],
            ..Default::default()
        };
        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_csi_storage_capacity_with_zone_topology() {
        let mut match_labels = BTreeMap::new();
        match_labels.insert(
            "topology.kubernetes.io/zone".to_string(),
            "us-east-1a".to_string(),
        );

        let selector = LabelSelector {
            match_labels,
            ..Default::default()
        };

        let cap = CSIStorageCapacity {
            storage_class_name: "standard".to_string(),
            node_topology: Some(selector),
            capacity: Some(Quantity::from("100Gi")),
            ..Default::default()
        };

        assert_eq!(
            cap.node_topology
                .as_ref()
                .unwrap()
                .match_labels
                .get("topology.kubernetes.io/zone"),
            Some(&"us-east-1a".to_string())
        );
    }
}
