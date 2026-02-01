//! Service validation for Kubernetes core/v1 API.

use crate::common::ToInternal;
use crate::common::validation::{ErrorList, Path};
use crate::core::internal::validation::service as internal_service_validation;
use crate::core::v1::service::{Service, ServicePort, ServiceSpec};
use std::collections::HashSet;

/// Validates a Service resource.
pub fn validate_service(service: &Service, path: &Path) -> ErrorList {
    let internal_service = service.clone().to_internal();
    internal_service_validation::validate_service(&internal_service, path)
}

/// Validates a Service spec.
pub fn validate_service_spec(spec: &ServiceSpec, path: &Path) -> ErrorList {
    let internal_spec = spec.clone().to_internal();
    internal_service_validation::validate_service_spec(&internal_spec, path)
}

/// Validates a single Service port.
pub fn validate_service_port(
    port: &ServicePort,
    require_name: bool,
    is_headless: bool,
    all_port_names: &mut HashSet<String>,
    path: &Path,
) -> ErrorList {
    let internal_port = port.clone().to_internal();
    internal_service_validation::validate_service_port(
        &internal_port,
        require_name,
        is_headless,
        all_port_names,
        path,
    )
}

/// Validates a Service update.
pub fn validate_service_update(
    new_service: &Service,
    old_service: &Service,
    path: &Path,
) -> ErrorList {
    let internal_new = new_service.clone().to_internal();
    let internal_old = old_service.clone().to_internal();
    internal_service_validation::validate_service_update(&internal_new, &internal_old, path)
}

/// Validates a Service status update.
pub fn validate_service_status_update(
    new_service: &Service,
    old_service: &Service,
    path: &Path,
) -> ErrorList {
    let internal_new = new_service.clone().to_internal();
    let internal_old = old_service.clone().to_internal();
    internal_service_validation::validate_service_status_update(&internal_new, &internal_old, path)
}
