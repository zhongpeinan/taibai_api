//! Common Kubernetes types used across multiple API groups.
//!
//! This module contains fundamental types that are shared across
//! different Kubernetes API versions and groups.

pub mod meta;
#[cfg(test)]
pub mod test_fixtures;
#[cfg(test)]
pub mod test_utils;
pub mod time;
pub mod traits;
pub mod util;
pub mod validation;
pub mod volume;

pub use meta::{
    Condition, FieldSelectorRequirement, GroupResource, GroupVersionKind, GroupVersionResource,
    LabelSelector, LabelSelectorRequirement, ListMeta, ManagedFieldsEntry, ObjectMeta,
    OwnerReference, Status, StatusCause, StatusDetails, TypeMeta,
};
pub use time::{MicroTime, Timestamp};
pub use traits::*;
pub use util::{IntOrString, Quantity, is_false, is_zero_i32};
pub use volume::{
    PersistentVolumeReclaimPolicy, PersistentVolumeSpec, TopologySelectorLabelRequirement,
    TopologySelectorTerm,
};
