//! Resource requirements validation for Kubernetes core/v1 API
//!
//! This module implements validation for container and pod resource requirements.

use crate::common::Quantity;
use crate::common::validation::{BadValue, ErrorList, Path, forbidden, invalid, required};
use crate::core::v1::resource::{ResourceClaim, ResourceRequirements};
use std::collections::HashSet;
use std::sync::LazyLock;

// ============================================================================
// Constants
// ============================================================================

/// Standard compute resources
const RESOURCE_CPU: &str = "cpu";
const RESOURCE_MEMORY: &str = "memory";
const RESOURCE_EPHEMERAL_STORAGE: &str = "ephemeral-storage";

/// Prefix for hugepages resources
const HUGEPAGES_PREFIX: &str = "hugepages-";

/// Resources that support QoS
static QOS_COMPUTE_RESOURCES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from([RESOURCE_CPU, RESOURCE_MEMORY]));

/// Resources that allow overcommit (request < limit)
static OVERCOMMIT_ALLOWED_RESOURCES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from([RESOURCE_CPU, RESOURCE_MEMORY, RESOURCE_EPHEMERAL_STORAGE]));

// ============================================================================
// Resource Requirements Validation
// ============================================================================

/// Validates container resource requirements.
///
/// Validates:
/// - Resource names are valid for containers
/// - Resource quantities are non-negative
/// - Requests <= Limits for overcommitable resources
/// - Requests == Limits for non-overcommitable resources (hugepages)
/// - Hugepages require CPU or memory
pub fn validate_container_resource_requirements(
    requirements: &ResourceRequirements,
    pod_claim_names: &HashSet<String>,
    path: &Path,
) -> ErrorList {
    validate_resource_requirements(
        requirements,
        validate_container_resource_name,
        pod_claim_names,
        path,
    )
}

/// Validates pod-level resource requirements.
///
/// Similar to container validation but with pod-specific resource name validation.
pub fn validate_pod_resource_requirements(
    requirements: &ResourceRequirements,
    pod_claim_names: &HashSet<String>,
    path: &Path,
) -> ErrorList {
    validate_resource_requirements(
        requirements,
        validate_pod_resource_name,
        pod_claim_names,
        path,
    )
}

/// Generic resource requirements validation.
fn validate_resource_requirements<F>(
    requirements: &ResourceRequirements,
    resource_name_fn: F,
    pod_claim_names: &HashSet<String>,
    path: &Path,
) -> ErrorList
where
    F: Fn(&str, &Path) -> ErrorList,
{
    let mut all_errs = ErrorList::new();
    let lim_path = path.child("limits");
    let req_path = path.child("requests");

    let mut lim_contains_cpu_or_memory = false;
    let mut req_contains_cpu_or_memory = false;
    let mut lim_contains_hugepages = false;
    let mut req_contains_hugepages = false;

    // Validate limits
    for (resource_name, quantity) in &requirements.limits {
        let fld_path = lim_path.key(resource_name);

        // Validate resource name
        all_errs.extend(resource_name_fn(resource_name, &fld_path));

        // Validate resource quantity
        all_errs.extend(validate_resource_quantity_value(
            resource_name,
            quantity,
            &fld_path,
        ));

        if is_hugepage_resource(resource_name) {
            lim_contains_hugepages = true;
        }

        if QOS_COMPUTE_RESOURCES.contains(resource_name.as_str()) {
            lim_contains_cpu_or_memory = true;
        }
    }

    // Validate requests
    for (resource_name, quantity) in &requirements.requests {
        let fld_path = req_path.key(resource_name);

        // Validate resource name
        all_errs.extend(resource_name_fn(resource_name, &fld_path));

        // Validate resource quantity
        all_errs.extend(validate_resource_quantity_value(
            resource_name,
            quantity,
            &fld_path,
        ));

        // Check that request <= limit
        if let Some(limit_quantity) = requirements.limits.get(resource_name) {
            // For non-overcommitable resources, request must equal limit
            if !is_overcommit_allowed(resource_name) {
                if quantity != limit_quantity {
                    all_errs.push(invalid(
                        &req_path,
                        BadValue::String(quantity.to_string()),
                        &format!(
                            "must be equal to {} limit of {}",
                            resource_name,
                            limit_quantity.to_string()
                        ),
                    ));
                }
            } else if quantity
                .cmp(limit_quantity)
                .unwrap_or(std::cmp::Ordering::Less)
                .is_gt()
            {
                // For overcommitable resources, request must be <= limit
                all_errs.push(invalid(
                    &req_path,
                    BadValue::String(quantity.to_string()),
                    &format!(
                        "must be less than or equal to {} limit of {}",
                        resource_name,
                        limit_quantity.to_string()
                    ),
                ));
            }
        } else if !is_overcommit_allowed(resource_name) {
            // Non-overcommitable resources require a limit
            all_errs.push(required(
                &lim_path,
                "limit must be set for non-overcommitable resources",
            ));
        }

        if is_hugepage_resource(resource_name) {
            req_contains_hugepages = true;
        }

        if QOS_COMPUTE_RESOURCES.contains(resource_name.as_str()) {
            req_contains_cpu_or_memory = true;
        }
    }

    // Hugepages require CPU or memory
    if !lim_contains_cpu_or_memory
        && !req_contains_cpu_or_memory
        && (req_contains_hugepages || lim_contains_hugepages)
    {
        all_errs.push(forbidden(path, "hugepages require cpu or memory"));
    }

    // Validate resource claims (placeholder for now)
    // TODO: Implement full resource claim validation in Phase 6
    if !requirements.claims.is_empty() {
        all_errs.extend(validate_resource_claim_names(
            &requirements.claims,
            pod_claim_names,
            &path.child("claims"),
        ));
    }

    all_errs
}

// ============================================================================
// Resource Name Validation
// ============================================================================

/// Validates a container resource name.
fn validate_container_resource_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Basic validation - resource name should not be empty
    if name.is_empty() {
        all_errs.push(required(path, "resource name is required"));
        return all_errs;
    }

    // Check standard resources
    if matches!(
        name,
        RESOURCE_CPU | RESOURCE_MEMORY | RESOURCE_EPHEMERAL_STORAGE
    ) || name.starts_with(HUGEPAGES_PREFIX)
        || name.starts_with("requests.")
        || name.starts_with("limits.")
    {
        return all_errs; // Valid
    }

    // Allow extended resources (with domain prefix)
    if name.contains('/') {
        return all_errs; // Extended resource - valid
    }

    all_errs.push(invalid(
        path,
        BadValue::String(name.to_string()),
        "invalid resource name for container",
    ));
    all_errs
}

/// Validates a pod-level resource name.
fn validate_pod_resource_name(name: &str, path: &Path) -> ErrorList {
    // For now, use same validation as container
    // TODO: Refine pod-specific validation in Phase 6
    validate_container_resource_name(name, path)
}

// ============================================================================
// Resource Quantity Validation
// ============================================================================

/// Validates a resource quantity value.
///
/// Ensures:
/// - Quantity is non-negative
/// - Format is valid
fn validate_resource_quantity_value(
    _resource_name: &str,
    quantity: &Quantity,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Check if quantity is negative
    if quantity.sign().unwrap_or(std::cmp::Ordering::Equal).is_lt() {
        all_errs.push(invalid(
            path,
            BadValue::String(quantity.to_string()),
            "must be non-negative",
        ));
    }

    all_errs
}

// ============================================================================
// Resource Claim Validation
// ============================================================================

/// Validates resource claim names.
fn validate_resource_claim_names(
    claims: &[ResourceClaim],
    pod_claim_names: &HashSet<String>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut names = HashSet::new();

    for (i, claim) in claims.iter().enumerate() {
        let idx_path = path.index(i);

        // Name is required
        if claim.name.is_empty() {
            all_errs.push(required(&idx_path, "claim name is required"));
            continue;
        }

        // Check for duplicates
        let key = if !claim.request.is_empty() {
            // Validate request DNS label
            if !crate::common::validation::is_dns1123_label(&claim.request).is_empty() {
                all_errs.push(invalid(
                    &idx_path.child("request"),
                    BadValue::String(claim.request.clone()),
                    "must be a valid DNS label",
                ));
            }
            format!("{}/{}", claim.name, claim.request)
        } else {
            claim.name.clone()
        };

        if names.contains(&key) {
            all_errs.push(crate::common::validation::duplicate(
                &idx_path,
                BadValue::String(key.clone()),
            ));
        } else {
            names.insert(key);
        }

        // Check if claim exists in pod spec
        if !pod_claim_names.contains(&claim.name) {
            let mut err = crate::common::validation::not_found(
                &idx_path,
                BadValue::String(claim.name.clone()),
            );
            // Add helpful detail
            if pod_claim_names.is_empty() {
                err = crate::common::validation::Error {
                    error_type: err.error_type,
                    field: err.field.clone(),
                    bad_value: err.bad_value.clone(),
                    detail: "must be one of the names in pod.spec.resourceClaims which is empty"
                        .to_string(),
                    origin: None,
                    covered_by_declarative: false,
                };
            } else {
                let claims_list: Vec<_> = pod_claim_names.iter().cloned().collect();
                err = crate::common::validation::Error {
                    error_type: err.error_type,
                    field: err.field.clone(),
                    bad_value: err.bad_value.clone(),
                    detail: format!(
                        "must be one of the names in pod.spec.resourceClaims: {}",
                        claims_list.join(", ")
                    ),
                    origin: None,
                    covered_by_declarative: false,
                };
            }
            all_errs.push(err);
        }
    }

    all_errs
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Checks if a resource name is a hugepage resource.
fn is_hugepage_resource(name: &str) -> bool {
    name.starts_with(HUGEPAGES_PREFIX)
}

/// Checks if a resource allows overcommit (request < limit).
fn is_overcommit_allowed(name: &str) -> bool {
    OVERCOMMIT_ALLOWED_RESOURCES.contains(name)
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hugepage_resource() {
        assert!(is_hugepage_resource("hugepages-2Mi"));
        assert!(is_hugepage_resource("hugepages-1Gi"));
        assert!(!is_hugepage_resource("cpu"));
        assert!(!is_hugepage_resource("memory"));
    }

    #[test]
    fn test_is_overcommit_allowed() {
        assert!(is_overcommit_allowed("cpu"));
        assert!(is_overcommit_allowed("memory"));
        assert!(is_overcommit_allowed("ephemeral-storage"));
        assert!(!is_overcommit_allowed("hugepages-2Mi"));
    }

    #[test]
    fn test_validate_container_resource_name() {
        // Valid names
        assert!(validate_container_resource_name("cpu", &Path::nil()).is_empty());
        assert!(validate_container_resource_name("memory", &Path::nil()).is_empty());
        assert!(validate_container_resource_name("hugepages-2Mi", &Path::nil()).is_empty());
        assert!(validate_container_resource_name("nvidia.com/gpu", &Path::nil()).is_empty());

        // Invalid name
        let errs = validate_container_resource_name("invalid", &Path::nil());
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_validate_resource_quantity_value() {
        use crate::common::Quantity;

        // Valid (non-negative) quantity
        let q = Quantity::from_str("1000mi");
        assert!(validate_resource_quantity_value("cpu", &q, &Path::nil()).is_empty());

        // Negative quantity
        let q_neg = Quantity::from_str("-1000mi");
        let errs = validate_resource_quantity_value("cpu", &q_neg, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must be non-negative"))
        );
    }

    #[test]
    fn test_validate_resource_requirements_request_exceeds_limit() {
        let mut requirements = ResourceRequirements::default();
        requirements
            .limits
            .insert("cpu".to_string(), Quantity::from_str("1000mi"));
        requirements
            .requests
            .insert("cpu".to_string(), Quantity::from_str("2000mi")); // Request > Limit

        let errs =
            validate_container_resource_requirements(&requirements, &HashSet::new(), &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must be less than or equal to"))
        );
    }

    #[test]
    fn test_validate_resource_requirements_hugepages_without_cpu_memory() {
        let mut requirements = ResourceRequirements::default();
        requirements
            .requests
            .insert("hugepages-2Mi".to_string(), Quantity::from_str("1Gi"));

        let errs =
            validate_container_resource_requirements(&requirements, &HashSet::new(), &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("hugepages require cpu or memory"))
        );
    }

    #[test]
    fn test_validate_resource_requirements_valid() {
        let mut requirements = ResourceRequirements::default();
        requirements
            .limits
            .insert("cpu".to_string(), Quantity::from_str("2000mi"));
        requirements
            .requests
            .insert("cpu".to_string(), Quantity::from_str("1000mi"));
        requirements
            .limits
            .insert("memory".to_string(), Quantity::from_str("2Gi"));
        requirements
            .requests
            .insert("memory".to_string(), Quantity::from_str("1Gi"));

        let errs =
            validate_container_resource_requirements(&requirements, &HashSet::new(), &Path::nil());
        assert!(
            errs.is_empty(),
            "Valid requirements should not produce errors"
        );
    }
}
