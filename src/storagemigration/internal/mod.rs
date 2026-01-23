//! Kubernetes StorageMigration API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/storagemigration/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storagemigration/types.go

use crate::common::time::Timestamp;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// MigrationConditionType
// ============================================================================

/// MigrationConditionType represents the type of migration condition.
///
/// Corresponds to [Kubernetes MigrationConditionType](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storagemigration/types.go#L71)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum MigrationConditionType {
    /// Indicates that the migration is running.
    #[serde(rename = "Running")]
    Running,
    /// Indicates that the migration has completed successfully.
    #[serde(rename = "Succeeded")]
    Succeeded,
    /// Indicates that the migration has failed.
    #[serde(rename = "Failed")]
    Failed,
}

/// MigrationConditionType constants
pub mod migration_condition_type {
    pub const RUNNING: &str = "Running";
    pub const SUCCEEDED: &str = "Succeeded";
    pub const FAILED: &str = "Failed";
}

// ============================================================================
// GroupVersionResource
// ============================================================================

/// GroupVersionResource represents the names of the group, version, and resource.
///
/// Corresponds to [Kubernetes GroupVersionResource](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storagemigration/types.go#L61)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupVersionResource {
    /// The name of the group.
    #[serde(default)]
    pub group: String,
    /// The name of the version.
    #[serde(default)]
    pub version: String,
    /// The name of the resource.
    #[serde(default)]
    pub resource: String,
}

// ============================================================================
// MigrationCondition
// ============================================================================

/// MigrationCondition describes the state of a migration at a certain point.
///
/// Corresponds to [Kubernetes MigrationCondition](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storagemigration/types.go#L83)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MigrationCondition {
    /// Type of the condition.
    #[serde(rename = "type")]
    pub type_: MigrationConditionType,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// The last time this condition was updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<Timestamp>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// ============================================================================
// StorageVersionMigrationSpec
// ============================================================================

/// StorageVersionMigrationSpec defines the specification of a storage version migration.
///
/// Corresponds to [Kubernetes StorageVersionMigrationSpec](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storagemigration/types.go#L45)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigrationSpec {
    /// The resource that is being migrated. The migrator sends requests to
    /// the endpoint serving the resource. Immutable.
    #[serde(default)]
    pub resource: GroupVersionResource,
    /// The token used in the list options to get the next chunk of objects
    /// to migrate. When the .status.conditions indicates the migration is
    /// "Running", users can use this token to check the progress of the
    /// migration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continue_token: Option<String>,
}

// ============================================================================
// StorageVersionMigrationStatus
// ============================================================================

/// StorageVersionMigrationStatus represents the status of a storage version migration.
///
/// Corresponds to [Kubernetes StorageVersionMigrationStatus](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storagemigration/types.go#L99)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigrationStatus {
    /// The latest available observations of the migration's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<MigrationCondition>,
    /// ResourceVersion to compare with the GC cache for performing the migration.
    /// This is the current resource version of given group, version and resource when
    /// kube-controller-manager first observes this StorageVersionMigration resource.
    #[serde(default)]
    pub resource_version: String,
}

// ============================================================================
// StorageVersionMigration
// ============================================================================

/// StorageVersionMigration represents a migration of stored data to the latest
/// storage version.
///
/// Corresponds to [Kubernetes StorageVersionMigration](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storagemigration/types.go#L29)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StorageVersionMigration {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    pub metadata: ObjectMeta,
    /// Specification of the migration.
    pub spec: StorageVersionMigrationSpec,
    /// Status of the migration.
    pub status: StorageVersionMigrationStatus,
}
impl_has_object_meta!(StorageVersionMigration);

// ============================================================================
// StorageVersionMigrationList
// ============================================================================

/// StorageVersionMigrationList is a collection of storage version migrations.
///
/// Corresponds to [Kubernetes StorageVersionMigrationList](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storagemigration/types.go#L117)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StorageVersionMigrationList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    pub metadata: ListMeta,
    /// Items is the list of StorageVersionMigration.
    pub items: Vec<StorageVersionMigration>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
}
