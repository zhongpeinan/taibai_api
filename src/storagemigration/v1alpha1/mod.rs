//! Kubernetes StorageMigration API v1alpha1 Types
//!
//! This module contains type definitions from k8s.io/api/storagemigration/v1alpha1/types.go
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1alpha1/types.go

use crate::common::time::Timestamp;
use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta,
    UnimplementedConversion, VersionedObject,
};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub mod validation;

// ============================================================================
// MigrationConditionType
// ============================================================================

/// MigrationConditionType represents the type of migration condition.
///
/// Corresponds to [Kubernetes MigrationConditionType](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1alpha1/types.go#L52)
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

/// The names of the group, the version, and the resource.
///
/// Corresponds to [Kubernetes GroupVersionResource](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1alpha1/types.go#L55)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupVersionResource {
    /// The name of the group.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// The name of the version.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// The name of the resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
}

// ============================================================================
// MigrationCondition
// ============================================================================

/// Describes the state of a migration at a certain point.
///
/// Corresponds to [Kubernetes MigrationCondition](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1alpha1/types.go#L69)
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
    /// A human readable message indicating details about the transition.
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
/// Corresponds to [Kubernetes StorageVersionMigrationSpec](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1alpha1/types.go#L44)
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
/// Corresponds to [Kubernetes StorageVersionMigrationStatus](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1alpha1/types.go#L63)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigrationStatus {
    /// The latest available observations of the migration's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<MigrationCondition>,
    /// ResourceVersion to compare with the GC cache for performing the migration.
    /// This is the current resource version of given group, version and resource when
    /// kube-controller-manager first observes this StorageVersionMigration resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_version: String,
}

// ============================================================================
// StorageVersionMigration
// ============================================================================

/// StorageVersionMigration represents a migration of stored data to the latest
/// storage version.
///
/// Corresponds to [Kubernetes StorageVersionMigration](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1alpha1/types.go#L28)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigration {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Specification of the migration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<StorageVersionMigrationSpec>,
    /// Status of the migration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StorageVersionMigrationStatus>,
}

// ============================================================================
// StorageVersionMigrationList
// ============================================================================

/// StorageVersionMigrationList is a collection of storage version migrations.
///
/// Corresponds to [Kubernetes StorageVersionMigrationList](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1alpha1/types.go#L81)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigrationList {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// Items is the list of StorageVersionMigration.
    #[serde(default)]
    pub items: Vec<StorageVersionMigration>,
}

// ============================================================================
// Trait Implementations for StorageVersionMigration and StorageVersionMigrationList
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for StorageVersionMigration {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storagemigration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "StorageVersionMigration"
    }
    fn resource(_: &Self::Meta) -> &str {
        "storageversionmigrations"
    }

    fn group_static() -> &'static str {
        "storagemigration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "StorageVersionMigration"
    }
    fn resource_static() -> &'static str {
        "storageversionmigrations"
    }
}

impl ResourceSchema for StorageVersionMigrationList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "storagemigration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "StorageVersionMigrationList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "storageversionmigrations"
    }

    fn group_static() -> &'static str {
        "storagemigration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "StorageVersionMigrationList"
    }
    fn resource_static() -> &'static str {
        "storageversionmigrations"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for StorageVersionMigration {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for StorageVersionMigrationList {
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

impl VersionedObject for StorageVersionMigration {
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

// Note: StorageVersionMigrationList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for StorageVersionMigration {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storagemigration.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "StorageVersionMigration".to_string();
        }
    }
}

impl ApplyDefault for StorageVersionMigrationList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "storagemigration.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "StorageVersionMigrationList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Placeholder (using UnimplementedConversion)
// ----------------------------------------------------------------------------

impl UnimplementedConversion for StorageVersionMigration {}
impl UnimplementedConversion for StorageVersionMigrationList {}

// ----------------------------------------------------------------------------
// Protobuf Placeholder (using macro)
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(StorageVersionMigration);
impl_unimplemented_prost_message!(StorageVersionMigrationList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}

#[cfg(test)]
mod trait_tests;

// AsRefStr / AsRef<str> implementations for enums
crate::impl_as_str_ref!(MigrationConditionType, {
    Running => migration_condition_type::RUNNING,
    Succeeded => migration_condition_type::SUCCEEDED,
    Failed => migration_condition_type::FAILED,
});
