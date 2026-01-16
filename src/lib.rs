//! Taibai API - Kubernetes API types in Rust
//!
//! This library provides Rust representations of Kubernetes API types,
//! supporting both JSON (via serde) and protobuf (via prost) serialization.

pub mod common;
pub mod core;
pub mod node;
pub mod rbac;
pub mod storage;

pub use common::{
    IntOrString, LabelSelector, ListMeta, ObjectMeta, PersistentVolumeReclaimPolicy, Quantity,
    TopologySelectorTerm, TypeMeta,
};
pub use core::Pod;
pub use node::v1::RuntimeClass;
pub use rbac::v1::{ClusterRole, Role, RoleBinding};
pub use storage::v1::{
    CSIDriver, CSINode, CSIStorageCapacity, StorageClass, VolumeAttachment, VolumeAttributesClass,
};
