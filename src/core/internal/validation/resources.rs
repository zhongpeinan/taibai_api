//! Resource requirements validation for Kubernetes core internal API.

use crate::common::Quantity;
use crate::common::validation::{BadValue, ErrorList, Path, forbidden, invalid, required};
use crate::core::internal::{ResourceClaim, ResourceRequirements};
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

    for (resource_name, quantity) in &requirements.limits {
        let fld_path = lim_path.key(resource_name);

        all_errs.extend(resource_name_fn(resource_name, &fld_path));
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

    for (resource_name, quantity) in &requirements.requests {
        let fld_path = req_path.key(resource_name);

        all_errs.extend(resource_name_fn(resource_name, &fld_path));
        all_errs.extend(validate_resource_quantity_value(
            resource_name,
            quantity,
            &fld_path,
        ));

        if let Some(limit_quantity) = requirements.limits.get(resource_name) {
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

    if !lim_contains_cpu_or_memory
        && !req_contains_cpu_or_memory
        && (req_contains_hugepages || lim_contains_hugepages)
    {
        all_errs.push(forbidden(path, "hugepages require cpu or memory"));
    }

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

fn validate_container_resource_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if name.is_empty() {
        all_errs.push(required(path, "resource name is required"));
        return all_errs;
    }

    if matches!(
        name,
        RESOURCE_CPU | RESOURCE_MEMORY | RESOURCE_EPHEMERAL_STORAGE
    ) || name.starts_with(HUGEPAGES_PREFIX)
        || name.starts_with("requests.")
        || name.starts_with("limits.")
    {
        return all_errs;
    }

    if name.contains('/') {
        return all_errs;
    }

    all_errs.push(invalid(
        path,
        BadValue::String(name.to_string()),
        "invalid resource name for container",
    ));
    all_errs
}

fn validate_pod_resource_name(name: &str, path: &Path) -> ErrorList {
    validate_container_resource_name(name, path)
}

// ============================================================================
// Resource Quantity Validation
// ============================================================================

pub(crate) fn validate_resource_quantity_value(
    _resource_name: &str,
    quantity: &Quantity,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if quantity.sign().unwrap_or(std::cmp::Ordering::Equal).is_lt() {
        all_errs.push(invalid(
            path,
            BadValue::String(quantity.to_string()),
            "must be non-negative",
        ));
    }

    all_errs
}

#[allow(dead_code)]
pub(crate) fn validate_resource_name_for_node(name: &str, path: &Path) -> ErrorList {
    validate_container_resource_name(name, path)
}

// ============================================================================
// Resource Claim Validation
// ============================================================================

fn validate_resource_claim_names(
    claims: &[ResourceClaim],
    pod_claim_names: &HashSet<String>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut names = HashSet::new();

    for (i, claim) in claims.iter().enumerate() {
        let idx_path = path.index(i);

        if claim.name.is_empty() {
            all_errs.push(required(&idx_path, "claim name is required"));
            continue;
        }

        let key = if !claim.request.is_empty() {
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

        if !pod_claim_names.contains(&claim.name) {
            let mut err = crate::common::validation::not_found(
                &idx_path,
                BadValue::String(claim.name.clone()),
            );
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

fn is_hugepage_resource(name: &str) -> bool {
    name.starts_with(HUGEPAGES_PREFIX)
}

fn is_overcommit_allowed(name: &str) -> bool {
    OVERCOMMIT_ALLOWED_RESOURCES.contains(name)
}
