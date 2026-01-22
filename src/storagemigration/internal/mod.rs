//! Kubernetes StorageMigration API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/storagemigration/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/storagemigration/types.go

use crate::common::time::Timestamp;
use crate::impl_has_object_meta;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
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
    use super::*;

    #[test]
    fn test_migration_condition_type_constants() {
        assert_eq!(migration_condition_type::RUNNING, "Running");
        assert_eq!(migration_condition_type::SUCCEEDED, "Succeeded");
        assert_eq!(migration_condition_type::FAILED, "Failed");
    }

    #[test]
    fn test_migration_condition_type_serialize() {
        let condition_type = MigrationConditionType::Running;
        let json = serde_json::to_string(&condition_type).unwrap();
        assert_eq!(json, r#""Running""#);
    }

    #[test]
    fn test_migration_condition_type_deserialize() {
        let json = r#""Succeeded""#;
        let condition_type: MigrationConditionType = serde_json::from_str(json).unwrap();
        assert!(matches!(condition_type, MigrationConditionType::Succeeded));
    }

    #[test]
    fn test_group_version_resource_default() {
        let gvr = GroupVersionResource::default();
        assert_eq!(gvr.group, "");
        assert_eq!(gvr.version, "");
        assert_eq!(gvr.resource, "");
    }

    #[test]
    fn test_group_version_resource_with_fields() {
        let gvr = GroupVersionResource {
            group: "apps".to_string(),
            version: "v1".to_string(),
            resource: "deployments".to_string(),
        };
        assert_eq!(gvr.group, "apps");
        assert_eq!(gvr.version, "v1");
        assert_eq!(gvr.resource, "deployments");
    }

    #[test]
    fn test_group_version_resource_serialize() {
        let gvr = GroupVersionResource {
            group: "apps".to_string(),
            version: "v1".to_string(),
            resource: "deployments".to_string(),
        };
        let json = serde_json::to_string(&gvr).unwrap();
        assert!(json.contains(r#""group":"apps""#));
        assert!(json.contains(r#""version":"v1""#));
        assert!(json.contains(r#""resource":"deployments""#));
    }

    #[test]
    fn test_migration_condition_default() {
        let condition = MigrationCondition {
            type_: MigrationConditionType::Running,
            status: "True".to_string(),
            last_update_time: None,
            reason: None,
            message: None,
        };
        assert_eq!(condition.status, "True");
        assert!(condition.reason.is_none());
    }

    #[test]
    fn test_migration_condition_with_all_fields() {
        let condition = MigrationCondition {
            type_: MigrationConditionType::Running,
            status: "True".to_string(),
            last_update_time: Some(Timestamp::default()),
            reason: Some("MigrationStarted".to_string()),
            message: Some("Migration is in progress".to_string()),
        };
        assert_eq!(condition.reason.as_ref().unwrap(), "MigrationStarted");
        assert_eq!(
            condition.message.as_ref().unwrap(),
            "Migration is in progress"
        );
    }

    #[test]
    fn test_storage_version_migration_spec_default() {
        let spec = StorageVersionMigrationSpec::default();
        assert_eq!(spec.resource.group, "");
        assert!(spec.continue_token.is_none());
    }

    #[test]
    fn test_storage_version_migration_spec_with_resource() {
        let spec = StorageVersionMigrationSpec {
            resource: GroupVersionResource {
                group: "apps".to_string(),
                version: "v1".to_string(),
                resource: "deployments".to_string(),
            },
            continue_token: Some("token123".to_string()),
        };
        assert_eq!(spec.resource.group, "apps");
        assert_eq!(spec.continue_token.unwrap(), "token123");
    }

    #[test]
    fn test_storage_version_migration_status_default() {
        let status = StorageVersionMigrationStatus::default();
        assert!(status.conditions.is_empty());
        assert_eq!(status.resource_version, "");
    }

    #[test]
    fn test_storage_version_migration_status_with_conditions() {
        let status = StorageVersionMigrationStatus {
            conditions: vec![MigrationCondition {
                type_: MigrationConditionType::Running,
                status: "True".to_string(),
                last_update_time: None,
                reason: None,
                message: None,
            }],
            resource_version: "12345".to_string(),
        };
        assert_eq!(status.conditions.len(), 1);
        assert_eq!(status.resource_version, "12345");
    }

    #[test]
    fn test_storage_version_migration_serialize() {
        let svm = StorageVersionMigration {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("my-migration".to_string()),
                ..Default::default()
            },
            spec: StorageVersionMigrationSpec {
                resource: GroupVersionResource {
                    group: "apps".to_string(),
                    version: "v1".to_string(),
                    resource: "deployments".to_string(),
                },
                ..Default::default()
            },
            status: StorageVersionMigrationStatus::default(),
        };
        let json = serde_json::to_string(&svm).unwrap();
        assert!(json.contains(r#""name":"my-migration""#));
        assert!(json.contains(r#""group":"apps""#));
    }

    #[test]
    fn test_storage_version_migration_list_empty() {
        let list = StorageVersionMigrationList {
            type_meta: TypeMeta::default(),
            metadata: ListMeta::default(),
            items: vec![],
        };
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_storage_version_migration_list_with_items() {
        let list = StorageVersionMigrationList {
            type_meta: TypeMeta::default(),
            metadata: ListMeta::default(),
            items: vec![StorageVersionMigration {
                type_meta: TypeMeta {
                    kind: "StorageVersionMigration".to_string(),
                    ..Default::default()
                },
                metadata: ObjectMeta {
                    name: Some("migration-1".to_string()),
                    ..Default::default()
                },
                spec: StorageVersionMigrationSpec::default(),
                status: StorageVersionMigrationStatus::default(),
            }],
        };
        assert_eq!(list.items.len(), 1);
    }
}
