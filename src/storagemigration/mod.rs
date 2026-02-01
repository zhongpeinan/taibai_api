//! Kubernetes StorageMigration API types
//!
//! This module contains the storagemigration API types.

pub mod internal;
pub mod v1alpha1;
pub mod validation;

pub use internal::{
    GroupVersionResource as GroupVersionResourceInternal,
    StorageVersionMigration as StorageVersionMigrationInternal,
    StorageVersionMigrationList as StorageVersionMigrationListInternal,
};
pub use v1alpha1::{StorageVersionMigration, StorageVersionMigrationList};

// Re-export constants at module level
pub use v1alpha1::migration_condition_type;
