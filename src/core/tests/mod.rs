//! Core API group integration tests

// Workload resources
pub mod pod;
pub mod replication_controller;

// Config resources
pub mod configmap;
pub mod secret;

// Service resources
pub mod endpoints;
pub mod service;

// Storage resources
pub mod persistent_volume;
pub mod persistent_volume_claim;

// Quota resources
pub mod limit_range;
pub mod resource_quota;

// Cluster resources
pub mod namespace;
pub mod node;
pub mod service_account;

// Misc resources
pub mod binding;
pub mod component_status;
pub mod event;

// Shared metadata behavior
pub mod metadata;

// Internal resources
pub mod internal;
