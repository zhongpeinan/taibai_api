//! Validation wrappers for Kubernetes StorageMigration API v1alpha1 types.
//!
//! These wrappers convert v1alpha1 types to internal types before validation.

use crate::common::validation::{ErrorList, Path, required};
use crate::common::TypeMeta;
use crate::storagemigration::internal;
use crate::storagemigration::v1alpha1::{
    GroupVersionResource, MigrationCondition, MigrationConditionType, StorageVersionMigration,
    StorageVersionMigrationList, StorageVersionMigrationSpec, StorageVersionMigrationStatus,
};

// ============================================================================
// StorageVersionMigration Validation
// ============================================================================

/// Validates a v1alpha1 StorageVersionMigration by converting to internal and delegating validation.
pub fn validate_storage_version_migration(obj: &StorageVersionMigration) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if obj.spec.is_none() {
        all_errs.push(required(&Path::new("spec"), "spec is required"));
    }

    let internal_obj = to_internal_migration(obj);
    all_errs.extend(internal::validation::validate_storage_version_migration(&internal_obj));

    all_errs
}

/// Validates a v1alpha1 StorageVersionMigrationList by converting to internal and delegating validation.
pub fn validate_storage_version_migration_list(
    obj: &StorageVersionMigrationList,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in obj.items.iter().enumerate() {
        if item.spec.is_none() {
            all_errs.push(required(
                &Path::new("items").index(i).child("spec"),
                "spec is required",
            ));
        }
    }

    let internal_list = internal::StorageVersionMigrationList {
        type_meta: TypeMeta::default(),
        metadata: obj.metadata.clone().unwrap_or_default(),
        items: obj.items.iter().map(to_internal_migration).collect(),
    };

    all_errs.extend(internal::validation::validate_storage_version_migration_list(
        &internal_list,
    ));

    all_errs
}

/// Validates a v1alpha1 StorageVersionMigration update by converting to internal and delegating validation.
pub fn validate_storage_version_migration_update(
    obj: &StorageVersionMigration,
    old: &StorageVersionMigration,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if obj.spec.is_none() {
        all_errs.push(required(&Path::new("spec"), "spec is required"));
    }

    let internal_obj = to_internal_migration(obj);
    let internal_old = to_internal_migration(old);
    all_errs.extend(internal::validation::validate_storage_version_migration_update(
        &internal_obj,
        &internal_old,
    ));

    all_errs
}

/// Validates a v1alpha1 StorageVersionMigration status update by converting to internal and delegating validation.
pub fn validate_storage_version_migration_status_update(
    obj: &StorageVersionMigration,
    old: &StorageVersionMigration,
) -> ErrorList {
    let internal_obj = to_internal_migration(obj);
    let internal_old = to_internal_migration(old);
    internal::validation::validate_storage_version_migration_status_update(
        &internal_obj,
        &internal_old,
    )
}

// ============================================================================
// Conversion Helpers
// ============================================================================

fn to_internal_migration(obj: &StorageVersionMigration) -> internal::StorageVersionMigration {
    internal::StorageVersionMigration {
        type_meta: TypeMeta::default(),
        metadata: obj.metadata.clone().unwrap_or_default(),
        spec: obj
            .spec
            .clone()
            .map(to_internal_spec)
            .unwrap_or_default(),
        status: obj
            .status
            .clone()
            .map(to_internal_status)
            .unwrap_or_default(),
    }
}

fn to_internal_spec(spec: StorageVersionMigrationSpec) -> internal::StorageVersionMigrationSpec {
    internal::StorageVersionMigrationSpec {
        resource: to_internal_gvr(spec.resource),
        continue_token: spec.continue_token,
    }
}

fn to_internal_status(
    status: StorageVersionMigrationStatus,
) -> internal::StorageVersionMigrationStatus {
    internal::StorageVersionMigrationStatus {
        conditions: status
            .conditions
            .into_iter()
            .map(to_internal_condition)
            .collect(),
        resource_version: status.resource_version,
    }
}

fn to_internal_gvr(value: GroupVersionResource) -> internal::GroupVersionResource {
    internal::GroupVersionResource {
        group: value.group,
        version: value.version,
        resource: value.resource,
    }
}

fn to_internal_condition(value: MigrationCondition) -> internal::MigrationCondition {
    internal::MigrationCondition {
        type_: to_internal_condition_type(value.type_),
        status: value.status,
        last_update_time: value.last_update_time,
        reason: value.reason,
        message: value.message,
    }
}

fn to_internal_condition_type(
    value: MigrationConditionType,
) -> internal::MigrationConditionType {
    match value {
        MigrationConditionType::Running => internal::MigrationConditionType::Running,
        MigrationConditionType::Succeeded => internal::MigrationConditionType::Succeeded,
        MigrationConditionType::Failed => internal::MigrationConditionType::Failed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_internal_defaults() {
        let migration = StorageVersionMigration::default();
        let internal = to_internal_migration(&migration);
        assert!(internal.metadata.name.is_none());
    }
}
