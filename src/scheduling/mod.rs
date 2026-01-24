//! Kubernetes Scheduling API types
//!
//! This module provides Rust representations of Kubernetes Scheduling API types.
//!
//! The Scheduling API is used to configure pod priority and preemption policies,
//! allowing cluster administrators to control the order in which pods are scheduled
//! and how they can preempt each other.
//!
//! ## Priority Classes
//!
//! PriorityClass objects define the mapping from a priority class name to an
//! integer priority value. Pods reference a PriorityClass using the
//! `priorityClassName` field in their `spec.priorityClassName`.
//!
//! ## Priority Values
//!
//! - **0 to 1,000,000,000**: User-definable priority range
//! - **> 1,000,000,000**: Reserved for Kubernetes system use
//! - **2,000,000,000**: System-critical priority (`system-cluster-critical`)
//! - **Default**: 0 (when no default PriorityClass exists)
//!
//! ## Preemption Policies
//!
//! - `PreemptLowerPriority`: Pods can preempt lower priority pods (default)
//! - `Never`: Pods never preempt other pods
//!
//! ## System Priority Classes
//!
//! Kubernetes includes two built-in system priority classes:
//! - `system-cluster-critical`: For cluster-critical components
//! - `system-node-critical`: For node-critical components
//!
//! System priority classes are reserved and must not be used for user workloads.
//!
//! The module is organized into:
//! - `v1`: The v1 version of the Scheduling API
//! - `internal`: Internal types used within Kubernetes

pub mod internal;
pub mod v1;
pub mod validation;

// Re-export commonly used v1 types
pub use v1::{PriorityClass, PriorityClassList};
