use crate::common::ApplyDefault;

use super::{StorageVersion, StorageVersionList};

impl ApplyDefault for StorageVersion {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apiserverinternal.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "StorageVersion".to_string();
        }
    }
}

impl ApplyDefault for StorageVersionList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apiserverinternal.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "StorageVersionList".to_string();
        }
    }
}
