//! StorageClass types
//!
//! StorageClass describes the parameters for a class of storage for
//! which PersistentVolumes can be dynamically provisioned.
//!
//! Corresponds to [Kubernetes StorageClass](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storage/types.go#L30)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::{ListMeta, ObjectMeta, PersistentVolumeReclaimPolicy, TopologySelectorTerm};

/// StorageClass describes the parameters for a class of storage for
/// which PersistentVolumes can be dynamically provisioned.
///
/// StorageClasses are non-namespaced; the name of the storage class
/// according to etcd is in ObjectMeta.Name.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageClass {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// provisioner indicates the type of the provisioner.
    pub provisioner: String,

    /// parameters holds the parameters for the provisioner that should
    /// create volumes of this storage class.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, String>,

    /// reclaimPolicy controls the reclaimPolicy for dynamically provisioned PersistentVolumes.
    /// Defaults to Delete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reclaim_policy: Option<PersistentVolumeReclaimPolicy>,

    /// mountOptions controls the mountOptions for dynamically provisioned PersistentVolumes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mount_options: Vec<String>,

    /// allowVolumeExpansion shows whether the storage class allow volume expand.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_volume_expansion: Option<bool>,

    /// volumeBindingMode indicates how PersistentVolumeClaims should be
    /// provisioned and bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_binding_mode: Option<VolumeBindingMode>,

    /// allowedTopologies restrict the node topologies where volumes can be dynamically provisioned.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_topologies: Vec<TopologySelectorTerm>,
}

/// StorageClassList is a collection of storage classes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageClassList {
    /// Standard list metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// items is the list of StorageClasses
    #[serde(default)]
    pub items: Vec<StorageClass>,
}

/// VolumeBindingMode indicates how PersistentVolumeClaims should be bound.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum VolumeBindingMode {
    /// Immediate indicates that PersistentVolumeClaims should be
    /// immediately provisioned and bound.
    Immediate,

    /// WaitForFirstConsumer indicates that PersistentVolumeClaims
    /// should not be provisioned and bound until the first Pod is created.
    WaitForFirstConsumer,
}

/// VolumeBindingMode constants
pub mod volume_binding_mode {
    /// Immediate indicates that PersistentVolumeClaims should be immediately provisioned
    pub const IMMEDIATE: &str = "Immediate";

    /// WaitForFirstConsumer indicates that PersistentVolumeClaims should wait for first consumer
    pub const WAIT_FOR_FIRST_CONSUMER: &str = "WaitForFirstConsumer";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_class_default() {
        let sc = StorageClass::default();
        assert!(sc.metadata.is_none());
        assert!(sc.parameters.is_empty());
    }

    #[test]
    fn test_storage_class_with_provisioner() {
        let sc = StorageClass {
            provisioner: "kubernetes.io/aws-ebs".to_string(),
            ..Default::default()
        };
        assert_eq!(sc.provisioner, "kubernetes.io/aws-ebs");
    }

    #[test]
    fn test_storage_class_with_parameters() {
        let mut parameters = HashMap::new();
        parameters.insert("type".to_string(), "gp2".to_string());
        parameters.insert("fsType".to_string(), "ext4".to_string());

        let sc = StorageClass {
            provisioner: "kubernetes.io/aws-ebs".to_string(),
            parameters,
            ..Default::default()
        };
        assert_eq!(sc.parameters.len(), 2);
        assert_eq!(sc.parameters.get("type"), Some(&"gp2".to_string()));
    }

    #[test]
    fn test_storage_class_serialize() {
        let mut parameters = HashMap::new();
        parameters.insert("type".to_string(), "gp2".to_string());

        let sc = StorageClass {
            provisioner: "kubernetes.io/aws-ebs".to_string(),
            parameters,
            reclaim_policy: Some(PersistentVolumeReclaimPolicy::Delete),
            ..Default::default()
        };
        let json = serde_json::to_string(&sc).unwrap();
        assert!(json.contains("\"provisioner\":\"kubernetes.io/aws-ebs\""));
        assert!(json.contains("\"type\":\"gp2\""));
    }

    #[test]
    fn test_storage_class_deserialize() {
        let json = "{\"provisioner\":\"kubernetes.io/aws-ebs\",\"parameters\":{\"type\":\"gp2\"}}";
        let sc: StorageClass = serde_json::from_str(json).unwrap();
        assert_eq!(sc.provisioner, "kubernetes.io/aws-ebs");
        assert_eq!(sc.parameters.get("type"), Some(&"gp2".to_string()));
    }

    #[test]
    fn test_storage_class_round_trip() {
        let mut parameters = HashMap::new();
        parameters.insert("type".to_string(), "gp2".to_string());

        let original = StorageClass {
            provisioner: "kubernetes.io/aws-ebs".to_string(),
            parameters,
            reclaim_policy: Some(PersistentVolumeReclaimPolicy::Delete),
            allow_volume_expansion: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: StorageClass = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_storage_class_empty_parameters_omitted() {
        let sc = StorageClass {
            provisioner: "kubernetes.io/aws-ebs".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&sc).unwrap();
        // Empty parameters should be omitted
        assert!(!json.contains("parameters"));
    }

    #[test]
    fn test_storage_class_list_default() {
        let scl = StorageClassList::default();
        assert!(scl.metadata.is_none());
        assert!(scl.items.is_empty());
    }

    #[test]
    fn test_storage_class_list_with_items() {
        let sc1 = StorageClass {
            provisioner: "kubernetes.io/aws-ebs".to_string(),
            ..Default::default()
        };
        let sc2 = StorageClass {
            provisioner: "kubernetes.io/gce-pd".to_string(),
            ..Default::default()
        };

        let scl = StorageClassList {
            items: vec![sc1, sc2],
            ..Default::default()
        };
        assert_eq!(scl.items.len(), 2);
    }

    #[test]
    fn test_storage_class_list_serialize() {
        let sc = StorageClass {
            provisioner: "kubernetes.io/aws-ebs".to_string(),
            ..Default::default()
        };

        let scl = StorageClassList {
            items: vec![sc],
            ..Default::default()
        };
        let json = serde_json::to_string(&scl).unwrap();
        assert!(json.contains("\"items\""));
    }

    #[test]
    fn test_storage_class_list_round_trip() {
        let sc = StorageClass {
            provisioner: "kubernetes.io/aws-ebs".to_string(),
            parameters: HashMap::new(),
            ..Default::default()
        };

        let original = StorageClassList {
            items: vec![sc],
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: StorageClassList = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_volume_binding_mode_serialize() {
        let mode = VolumeBindingMode::Immediate;
        let json = serde_json::to_string(&mode).unwrap();
        assert_eq!(json, r#""Immediate""#);
    }

    #[test]
    fn test_volume_binding_mode_deserialize() {
        let json = r#""WaitForFirstConsumer""#;
        let mode: VolumeBindingMode = serde_json::from_str(json).unwrap();
        assert_eq!(mode, VolumeBindingMode::WaitForFirstConsumer);
    }

    #[test]
    fn test_volume_binding_mode_round_trip() {
        let modes = vec![
            VolumeBindingMode::Immediate,
            VolumeBindingMode::WaitForFirstConsumer,
        ];
        for mode in modes {
            let json = serde_json::to_string(&mode).unwrap();
            let deserialized: VolumeBindingMode = serde_json::from_str(&json).unwrap();
            assert_eq!(mode, deserialized);
        }
    }

    #[test]
    fn test_volume_binding_mode_constants() {
        assert_eq!(volume_binding_mode::IMMEDIATE, "Immediate");
        assert_eq!(
            volume_binding_mode::WAIT_FOR_FIRST_CONSUMER,
            "WaitForFirstConsumer"
        );
    }
}
