//! Kubernetes StorageMigration API v1beta1 Types
//!
//! This module contains type definitions from k8s.io/api/storagemigration/v1beta1/types.go
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1beta1/types.go

use crate::common::meta::{Condition, GroupResource};
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use serde::{Deserialize, Serialize};

// ============================================================================
// MigrationConditionType
// ============================================================================

/// MigrationConditionType represents the type of migration condition.
///
/// Corresponds to [Kubernetes MigrationConditionType](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1beta1/types.go#L52)
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
// StorageVersionMigrationSpec
// ============================================================================

/// StorageVersionMigrationSpec defines the specification of a storage version migration.
///
/// Corresponds to [Kubernetes StorageVersionMigrationSpec](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1beta1/types.go#L44)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigrationSpec {
    /// The resource that is being migrated. The migrator sends requests to
    /// the endpoint serving the resource. Immutable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<GroupResource>,
}

// ============================================================================
// StorageVersionMigrationStatus
// ============================================================================

/// StorageVersionMigrationStatus represents the status of a storage version migration.
///
/// Corresponds to [Kubernetes StorageVersionMigrationStatus](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1beta1/types.go#L63)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigrationStatus {
    /// The latest available observations of the migration's current state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
    /// ResourceVersion to compare with the GC cache for performing the migration.
    /// This is the current resource version of given group, version and resource when
    /// kube-controller-manager first observes this StorageVersionMigration resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,
}

// ============================================================================
// StorageVersionMigration
// ============================================================================

/// StorageVersionMigration represents a migration of stored data to the latest
/// storage version.
///
/// Corresponds to [Kubernetes StorageVersionMigration](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1beta1/types.go#L28)
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
/// Corresponds to [Kubernetes StorageVersionMigrationList](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/api/storagemigration/v1beta1/types.go#L81)
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    fn test_storage_version_migration_spec_default() {
        let spec = StorageVersionMigrationSpec::default();
        assert!(spec.resource.is_none());
    }

    #[test]
    fn test_storage_version_migration_spec_with_resource() {
        let spec = StorageVersionMigrationSpec {
            resource: Some(GroupResource {
                group: "apps".to_string(),
                resource: "deployments".to_string(),
            }),
        };
        assert!(spec.resource.is_some());
        let resource = spec.resource.unwrap();
        assert_eq!(resource.group, "apps");
        assert_eq!(resource.resource, "deployments");
    }

    #[test]
    fn test_storage_version_migration_spec_serialize() {
        let spec = StorageVersionMigrationSpec {
            resource: Some(GroupResource {
                group: "apps".to_string(),
                resource: "deployments".to_string(),
            }),
        };
        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains(r#""group":"apps""#));
        assert!(json.contains(r#""resource":"deployments""#));
    }

    #[test]
    fn test_storage_version_migration_spec_deserialize() {
        let json = r#"{"resource":{"group":"apps","resource":"deployments"}}"#;
        let spec: StorageVersionMigrationSpec = serde_json::from_str(json).unwrap();
        assert!(spec.resource.is_some());
        let resource = spec.resource.unwrap();
        assert_eq!(resource.group, "apps");
        assert_eq!(resource.resource, "deployments");
    }

    #[test]
    fn test_storage_version_migration_status_default() {
        let status = StorageVersionMigrationStatus::default();
        assert!(status.conditions.is_empty());
        assert!(status.resource_version.is_none());
    }

    #[test]
    fn test_storage_version_migration_status_with_conditions() {
        let status = StorageVersionMigrationStatus {
            conditions: vec![Condition {
                type_: "Running".to_string(),
                status: "True".to_string(),
                observed_generation: None,
                last_transition_time: None,
                reason: Some("MigrationStarted".to_string()),
                message: Some("Migration is in progress".to_string()),
            }],
            resource_version: Some("12345".to_string()),
        };
        assert_eq!(status.conditions.len(), 1);
        assert_eq!(status.resource_version.unwrap(), "12345");
    }

    #[test]
    fn test_storage_version_migration_default() {
        let svm = StorageVersionMigration::default();
        assert!(svm.metadata.is_none());
        assert!(svm.spec.is_none());
        assert!(svm.status.is_none());
    }

    #[test]
    fn test_storage_version_migration_serialize() {
        let svm = StorageVersionMigration {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("my-migration".to_string()),
                ..Default::default()
            }),
            spec: Some(StorageVersionMigrationSpec {
                resource: Some(GroupResource {
                    group: "apps".to_string(),
                    resource: "deployments".to_string(),
                }),
            }),
            status: None,
        };
        let json = serde_json::to_string(&svm).unwrap();
        assert!(json.contains(r#""name":"my-migration""#));
        assert!(json.contains(r#""group":"apps""#));
    }

    #[test]
    fn test_storage_version_migration_deserialize() {
        let json = r#"{
            "kind": "StorageVersionMigration",
            "metadata": {"name": "my-migration"},
            "spec": {
                "resource": {
                    "group": "apps",
                    "resource": "deployments"
                }
            }
        }"#;
        let svm: StorageVersionMigration = serde_json::from_str(json).unwrap();
        assert_eq!(svm.metadata.unwrap().name.unwrap(), "my-migration");
        assert!(svm.spec.is_some());
    }

    #[test]
    fn test_storage_version_migration_list_default() {
        let list = StorageVersionMigrationList::default();
        assert!(list.metadata.is_none());
        assert!(list.items.is_empty());
    }

    #[test]
    fn test_storage_version_migration_list_serialize() {
        let list = StorageVersionMigrationList {
            type_meta: TypeMeta::default(),
            metadata: Some(ListMeta {
                resource_version: Some("1".to_string()),
                ..Default::default()
            }),
            items: vec![StorageVersionMigration {
                type_meta: TypeMeta {
                    kind: Some("StorageVersionMigration".to_string()),
                    ..Default::default()
                },
                metadata: Some(ObjectMeta {
                    name: Some("migration-1".to_string()),
                    ..Default::default()
                }),
                spec: None,
                status: None,
            }],
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains(r#""resourceVersion":"1""#));
        assert!(json.contains(r#""name":"migration-1""#));
    }

    #[test]
    fn test_storage_version_migration_list_deserialize() {
        let json = r#"{
            "kind": "StorageVersionMigrationList",
            "metadata": {"resourceVersion": "1"},
            "items": [{
                "kind": "StorageVersionMigration",
                "metadata": {"name": "migration-1"}
            }]
        }"#;
        let list: StorageVersionMigrationList = serde_json::from_str(json).unwrap();
        assert_eq!(list.items.len(), 1);
        assert_eq!(
            list.items[0]
                .metadata
                .as_ref()
                .unwrap()
                .name
                .as_ref()
                .unwrap(),
            "migration-1"
        );
    }
}
