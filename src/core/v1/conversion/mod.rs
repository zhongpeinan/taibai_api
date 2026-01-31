//! Modular conversion implementations for core v1 ↔ internal API
//!
//! Based on k8s.io/kubernetes/pkg/apis/core/v1/conversion.go
//! and k8s.io/kubernetes/pkg/apis/core/v1/zz_generated.conversion.go

mod binding;
mod component_status;
mod config;
mod events;
mod helpers;
mod namespace;
mod node;
mod persistent_volume;
mod pod;
mod pod_status_result;
mod reference;
mod replication_controller;
mod resource;
mod scheduling;
mod service;
mod volume;

// Re-export everything for backward compatibility
pub use helpers::*;
