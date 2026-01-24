use crate::common::ResourceSchema;

use super::{StorageVersion, StorageVersionList};

impl ResourceSchema for StorageVersion {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apiserverinternal.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "StorageVersion"
    }
    fn resource(_: &Self::Meta) -> &str {
        "storageversions"
    }

    fn group_static() -> &'static str {
        "apiserverinternal.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "StorageVersion"
    }
    fn resource_static() -> &'static str {
        "storageversions"
    }
}

impl ResourceSchema for StorageVersionList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apiserverinternal.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1alpha1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "StorageVersionList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "storageversions"
    }

    fn group_static() -> &'static str {
        "apiserverinternal.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1alpha1"
    }
    fn kind_static() -> &'static str {
        "StorageVersionList"
    }
    fn resource_static() -> &'static str {
        "storageversions"
    }
}
