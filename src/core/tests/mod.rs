//! Core API group integration tests

// Workload resources
pub mod pod;
pub mod replication_controller;

// Config resources
pub mod configmap;
pub mod secret;

// Service resources
pub mod service;
pub mod endpoints;

// Storage resources
pub mod persistent_volume;
pub mod persistent_volume_claim;

// Quota resources
pub mod resource_quota;
pub mod limit_range;

// Cluster resources
pub mod node;
pub mod namespace;
pub mod service_account;

// Misc resources
pub mod event;
pub mod binding;
pub mod component_status;

// Shared metadata behavior
pub mod metadata;

// Internal resources
pub mod internal;
