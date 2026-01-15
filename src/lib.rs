//! Taibai API - Kubernetes API types in Rust
//!
//! This library provides Rust representations of Kubernetes API types,
//! supporting both JSON (via serde) and protobuf (via prost) serialization.

pub mod common;
pub mod core;

pub use common::{IntOrString, ListMeta, ObjectMeta, Quantity, TypeMeta};
pub use core::Pod;
