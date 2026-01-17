//! Taibai API - Kubernetes API types in Rust
//!
//! This library provides Rust representations of Kubernetes API types,
//! supporting both JSON (via serde) and protobuf (via prost) serialization.

pub mod admission;
pub mod admissionregistration;
pub mod apidiscovery;
pub mod apiserverinternal;
pub mod apps;
pub mod authentication;
pub mod authorization;
pub mod autoscaling;
pub mod batch;
pub mod certificates;
pub mod common;
pub mod coordination;
pub mod core;
pub mod discovery;
pub mod events;
pub mod extensions;
pub mod flowcontrol;
pub mod imagepolicy;
pub mod node;
pub mod policy;
pub mod rbac;
pub mod scheduling;
pub mod storage;
pub mod storagemigration;

pub use admission::{AdmissionRequest, AdmissionResponse, AdmissionReview};
pub use authentication::UserInfo;
pub use common::{
    GroupResource, GroupVersionKind, GroupVersionResource, IntOrString, LabelSelector, ListMeta,
    ObjectMeta, PersistentVolumeReclaimPolicy, Quantity, Status, TopologySelectorTerm, TypeMeta,
};
pub use core::Pod;
pub use node::v1::RuntimeClass;
pub use rbac::v1::{ClusterRole, Role, RoleBinding};
pub use storage::v1::{
    CSIDriver, CSINode, CSIStorageCapacity, StorageClass, VolumeAttachment, VolumeAttributesClass,
};
