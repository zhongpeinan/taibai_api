//! Common Kubernetes types used across multiple API groups.
//!
//! This module contains fundamental types that are shared across
//! different Kubernetes API versions and groups.

pub mod meta;
pub mod time;
pub mod util;

pub use meta::{Condition, ListMeta, ManagedFieldsEntry, ObjectMeta, TypeMeta};
pub use time::Timestamp;
pub use util::{IntOrString, Quantity};
