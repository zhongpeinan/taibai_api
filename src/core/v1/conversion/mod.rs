//! Modular conversion implementations for core v1 ↔ internal API
//!
//! Based on k8s.io/kubernetes/pkg/apis/core/v1/conversion.go
//! and k8s.io/kubernetes/pkg/apis/core/v1/zz_generated.conversion.go

mod binding;
mod component_status;
mod config;
mod env;
mod events;
mod helpers;
mod namespace;
mod node;
mod options;
mod persistent_volume;
mod pod;
mod pod_status_result;
mod probe;
mod reference;
mod replication_controller;
mod resource;
mod scheduling;
mod selector;
mod service;
mod volume;

mod preconditions;
mod range_allocation;
mod serialized_reference;

// Re-export everything for backward compatibility
pub use helpers::*;
