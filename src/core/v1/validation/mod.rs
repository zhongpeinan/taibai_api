//! Validation for Kubernetes Core v1 API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go
//!
//! This module provides comprehensive validation for all core/v1 API types including:
//! - Pod and container validation
//! - Volume and storage validation
//! - Service validation
//! - Node validation
//! - Namespace validation
//! - ConfigMap and Secret validation
//! - ResourceQuota and LimitRange validation
//!
//! The validation functions follow Kubernetes upstream behavior and return
//! `ErrorList` for all validation errors found.

pub mod config;
pub mod constants;
pub mod container;
pub mod container_ports;
pub mod dns;
pub mod endpoints;
pub mod env;
pub mod events;
pub mod helpers;
pub mod namespace;
pub mod node;
pub mod pod;
pub mod pod_spec;
pub mod probe;
pub mod replication_controller;
pub mod resource_quota;
pub mod resources;
pub mod service;
pub mod storage;
pub mod template;
pub mod volume;

// Re-export public API
pub use config::{validate_config_map, validate_secret, validate_service_account};
pub use endpoints::validate_endpoints;
pub use events::{EventRequestVersion, validate_event_create, validate_event_update};
pub use namespace::{validate_namespace, validate_namespace_update};
pub use node::{validate_node, validate_node_update};
pub use pod::{validate_pod, validate_pod_spec, validate_pod_update};
pub use replication_controller::{
    validate_replication_controller, validate_replication_controller_status_update,
    validate_replication_controller_update,
};
pub use resource_quota::{validate_limit_range, validate_resource_quota};
pub use service::{validate_service, validate_service_spec, validate_service_update};
pub use storage::{
    validate_persistent_volume, validate_persistent_volume_claim,
    validate_persistent_volume_claim_update, validate_persistent_volume_update,
};
pub use template::{
    validate_pod_template, validate_pod_template_spec, validate_pod_template_update,
};
pub use volume::{validate_volume, validate_volumes};
