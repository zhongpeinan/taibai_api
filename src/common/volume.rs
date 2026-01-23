//! Kubernetes volume-related common types
//!
//! This module contains volume-related types that are shared across
//! different Kubernetes API versions and groups.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// TopologySelectorTerm is a selector that matches given label.
///
/// Corresponds to [Kubernetes TopologySelectorTerm](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L3820)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TopologySelectorTerm {
    /// A list of topology selector requirements by labels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_label_expressions: Vec<TopologySelectorLabelRequirement>,
}

/// TopologySelectorLabelRequirement is a selector that matches given label.
///
/// Corresponds to [Kubernetes TopologySelectorLabelRequirement](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L3831)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TopologySelectorLabelRequirement {
    /// The label key that the selector applies to.
    pub key: String,

    /// An array of string values. One value must match the label to be selected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// PersistentVolumeReclaimPolicy describes a policy for end-of-life maintenance of persistent volumes.
///
/// Corresponds to [Kubernetes PersistentVolumeReclaimPolicy](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/types.go#L452)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PersistentVolumeReclaimPolicy {
    /// Recycle means the volume will be recycled back into the pool of unbound persistent volumes.
    Recycle,

    /// Delete means the volume will be deleted from Kubernetes on release from its claim.
    Delete,

    /// Retain means the volume will be left in its current phase for manual reclamation.
    Retain,
}

/// PersistentVolumeReclaimPolicy constants
pub mod persistent_volume_reclaim_policy {
    /// Recycle means the volume will be recycled back into the pool
    pub const RECYCLE: &str = "Recycle";

    /// Delete means the volume will be deleted from Kubernetes
    pub const DELETE: &str = "Delete";

    /// Retain means the volume will be left in its current phase
    pub const RETAIN: &str = "Retain";
}

/// PersistentVolumeSpec is the specification of a persistent volume.
///
/// Corresponds to [Kubernetes PersistentVolumeSpec](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/types.go#L388)
///
/// Note: This is a minimal representation for storage/v1 usage.
/// The full implementation would be in core/v1.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeSpec {
    /// capacity is the description of the persistent volume's resources and capacity.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub capacity: BTreeMap<String, crate::Quantity>,

    /// accessModes contains all ways the volume can be mounted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,

    /// persistentVolumeReclaimPolicy defines what happens to a persistent volume when released.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent_volume_reclaim_policy: Option<PersistentVolumeReclaimPolicy>,

    /// storageClassName is the name of StorageClass to which this persistent volume belongs.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub storage_class_name: String,

    /// mountOptions is the list of mount options.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mount_options: Vec<String>,

    /// volumeMode defines if a volume is intended to be used with a formatted filesystem
    /// or to remain in raw block state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<String>,

    /// nodeAffinity defines constraints that limit what nodes this volume can be accessed from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<serde_json::Value>,

    /// volumeAttributesClassName is the name of VolumeAttributesClass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_attributes_class_name: Option<String>,
}

#[cfg(test)]
mod tests {
}
