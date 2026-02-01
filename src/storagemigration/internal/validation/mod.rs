//! Validation for Kubernetes StorageMigration API internal types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/storagemigration/validation/validation.go

use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, is_dns1035_label, is_dns1123_subdomain, not_supported,
    required, validate_object_meta, validate_object_meta_update,
};
use crate::storagemigration::internal::{
    GroupVersionResource, MigrationCondition, StorageVersionMigration, StorageVersionMigrationList,
    StorageVersionMigrationSpec, StorageVersionMigrationStatus,
};

// ============================================================================
// StorageVersionMigration Validation
// ============================================================================

/// Validates a StorageVersionMigration.
pub fn validate_storage_version_migration(obj: &StorageVersionMigration) -> ErrorList {
    validate_storage_version_migration_with_path(obj, &Path::nil())
}

fn validate_storage_version_migration_with_path(
    obj: &StorageVersionMigration,
    base_path: &Path,
) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &obj.metadata,
        false,
        crate::common::validation::name_is_dns_subdomain,
        &base_path.child("metadata"),
    );

    all_errs.extend(validate_storage_version_migration_spec(
        &obj.spec,
        &base_path.child("spec"),
    ));

    all_errs
}

/// Validates a StorageVersionMigrationList.
pub fn validate_storage_version_migration_list(obj: &StorageVersionMigrationList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in obj.items.iter().enumerate() {
        all_errs.extend(validate_storage_version_migration_with_path(
            item,
            &Path::new("items").index(i),
        ));
    }

    all_errs
}

/// Validates StorageVersionMigration updates.
pub fn validate_storage_version_migration_update(
    obj: &StorageVersionMigration,
    old: &StorageVersionMigration,
) -> ErrorList {
    let mut all_errs =
        validate_object_meta_update(&obj.metadata, &old.metadata, &Path::new("metadata"));

    if obj.spec.resource != old.spec.resource {
        all_errs.push(invalid(
            &Path::new("spec").child("resource"),
            BadValue::String(format!("{:?}", obj.spec.resource)),
            "field is immutable",
        ));
    }

    all_errs.extend(validate_storage_version_migration_spec(
        &obj.spec,
        &Path::new("spec"),
    ));

    all_errs
}

/// Validates StorageVersionMigration status updates.
pub fn validate_storage_version_migration_status_update(
    obj: &StorageVersionMigration,
    old: &StorageVersionMigration,
) -> ErrorList {
    let mut all_errs =
        validate_object_meta_update(&obj.metadata, &old.metadata, &Path::new("metadata"));

    all_errs.extend(validate_storage_version_migration_status(
        &obj.status,
        &Path::new("status"),
    ));

    all_errs
}

// ============================================================================
// Spec/Status Validation
// ============================================================================

fn validate_storage_version_migration_spec(
    spec: &StorageVersionMigrationSpec,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_group_version_resource(
        &spec.resource,
        &path.child("resource"),
    ));

    all_errs
}

fn validate_storage_version_migration_status(
    status: &StorageVersionMigrationStatus,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, condition) in status.conditions.iter().enumerate() {
        all_errs.extend(validate_migration_condition(
            condition,
            &path.child("conditions").index(i),
        ));
    }

    all_errs
}

fn validate_group_version_resource(value: &GroupVersionResource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if !value.group.is_empty() {
        for msg in is_dns1123_subdomain(&value.group) {
            all_errs.push(invalid(
                &path.child("group"),
                BadValue::String(value.group.clone()),
                &msg,
            ));
        }
    }

    if value.version.is_empty() {
        all_errs.push(required(&path.child("version"), ""));
    } else {
        for msg in is_dns1035_label(&value.version) {
            all_errs.push(invalid(
                &path.child("version"),
                BadValue::String(value.version.clone()),
                &msg,
            ));
        }
    }

    if value.resource.is_empty() {
        all_errs.push(required(&path.child("resource"), ""));
    } else {
        for msg in is_dns1035_label(&value.resource) {
            all_errs.push(invalid(
                &path.child("resource"),
                BadValue::String(value.resource.clone()),
                &msg,
            ));
        }
    }

    all_errs
}

fn validate_migration_condition(condition: &MigrationCondition, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    match condition.status.as_str() {
        "True" | "False" | "Unknown" => {}
        _ => {
            all_errs.push(not_supported(
                &path.child("status"),
                BadValue::String(condition.status.clone()),
                &["True", "False", "Unknown"],
            ));
        }
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ListMeta;
    use crate::common::TypeMeta;

    fn base_migration() -> StorageVersionMigration {
        StorageVersionMigration {
            type_meta: TypeMeta::default(),
            metadata: crate::common::ObjectMeta {
                name: Some("migrate".to_string()),
                ..Default::default()
            },
            spec: StorageVersionMigrationSpec {
                resource: GroupVersionResource {
                    group: "apps".to_string(),
                    version: "v1".to_string(),
                    resource: "deployments".to_string(),
                },
                continue_token: None,
            },
            status: StorageVersionMigrationStatus::default(),
        }
    }

    #[test]
    fn test_validate_storage_version_migration_valid() {
        let obj = base_migration();
        let errs = validate_storage_version_migration(&obj);
        assert!(errs.is_empty(), "expected no errors, got {errs:?}");
    }

    #[test]
    fn test_validate_storage_version_migration_missing_version() {
        let mut obj = base_migration();
        obj.spec.resource.version = "".to_string();
        let errs = validate_storage_version_migration(&obj);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.ends_with("spec.resource.version"))
        );
    }

    #[test]
    fn test_validate_storage_version_migration_status_invalid_condition() {
        let mut obj = base_migration();
        obj.status.conditions.push(MigrationCondition {
            status: "Bogus".to_string(),
            ..Default::default()
        });
        let errs = validate_storage_version_migration_status_update(&obj, &base_migration());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("status.conditions[0].status"))
        );
    }

    #[test]
    fn test_validate_storage_version_migration_list_item_index() {
        let mut list = StorageVersionMigrationList {
            type_meta: TypeMeta::default(),
            metadata: ListMeta::default(),
            items: vec![base_migration()],
        };
        list.items[0].spec.resource.resource = "".to_string();
        let errs = validate_storage_version_migration_list(&list);
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("items[0].spec.resource.resource"))
        );
    }
}
