//! Validation for resource.k8s.io API types
//!
//! This module provides validation functions for the Kubernetes resource.k8s.io API group,
//! closely aligned with upstream validation logic from k8s/pkg/apis/resource/validation/validation.go

use crate::common::validation::{
    dns::*, errors::*, labels::*, object_meta::*, path::*, qualified_name::*,
};
use crate::resource::internal::{
    self, device_class::CELDeviceSelector as DeviceClassCELSelector, device_class::DeviceClass,
    device_class::DeviceClassConfiguration, device_class::DeviceClassSpec,
    device_class::DeviceSelector as DeviceClassSelector,
    device_class::OpaqueDeviceConfiguration as DeviceClassOpaqueConfig, resource_claim::*,
    resource_claim_template::*, resource_slice::*,
};
use std::collections::{BTreeMap, BTreeSet, HashSet};

// ============================================================================
// Constants - Size Limits
// ============================================================================

/// Maximum number of devices in a ResourceSlice
pub const RESOURCE_SLICE_MAX_DEVICES: usize = 128;

/// Maximum number of shared counters across all sets in a ResourceSlice
pub const RESOURCE_SLICE_MAX_SHARED_COUNTERS: usize = 32;

/// Maximum number of attributes and capacities combined per device
pub const RESOURCE_SLICE_MAX_ATTRIBUTES_AND_CAPACITIES_PER_DEVICE: usize = 32;

/// Maximum number of counters per device
pub const RESOURCE_SLICE_MAX_COUNTERS_PER_DEVICE: usize = 32;

/// Maximum total number of device counters per slice
pub const RESOURCE_SLICE_MAX_DEVICE_COUNTERS_PER_SLICE: usize = 1024;

/// Maximum length of pool name (same as DNS subdomain max length)
pub const POOL_NAME_MAX_LENGTH: usize = 253;

/// Maximum number of device requests in a claim
pub const DEVICE_REQUESTS_MAX_SIZE: usize = 32; // Same as AllocationResultsMaxSize

/// Maximum number of device constraints
pub const DEVICE_CONSTRAINTS_MAX_SIZE: usize = 32;

/// Maximum number of device configurations
pub const DEVICE_CONFIG_MAX_SIZE: usize = 32;

/// Maximum number of device selectors
pub const DEVICE_SELECTORS_MAX_SIZE: usize = 32;

/// Maximum number of first available device requests
pub const FIRST_AVAILABLE_DEVICE_REQUEST_MAX_SIZE: usize = 8;

/// Maximum number of allocation results
pub const ALLOCATION_RESULTS_MAX_SIZE: usize = 32;

/// Maximum number of entries in reservedFor
pub const RESOURCE_CLAIM_RESERVED_FOR_MAX_SIZE: usize = 256;

/// Maximum number of binding conditions
pub const BINDING_CONDITIONS_MAX_SIZE: usize = 4;

/// Maximum number of binding failure conditions
pub const BINDING_FAILURE_CONDITIONS_MAX_SIZE: usize = 4;

/// Maximum number of device taints
pub const DEVICE_TAINTS_MAX_LENGTH: usize = 4;

/// Maximum length of device attribute string or version values
pub const DEVICE_ATTRIBUTE_MAX_VALUE_LENGTH: usize = 64;

/// Maximum length of CEL selector expressions
pub const CEL_SELECTOR_EXPRESSION_MAX_LENGTH: usize = 10 * 1024;

/// Maximum length of opaque parameters
pub const OPAQUE_PARAMETERS_MAX_LENGTH: usize = 10 * 1024;

/// Maximum number of conditions on AllocatedDeviceStatus
pub const ALLOCATED_DEVICE_STATUS_MAX_CONDITIONS: usize = 8;

/// Maximum length of AllocatedDeviceStatus.Data
pub const ALLOCATED_DEVICE_STATUS_DATA_MAX_LENGTH: usize = 10 * 1024;

/// Maximum length of NetworkDeviceData.InterfaceName
pub const NETWORK_DEVICE_DATA_INTERFACE_NAME_MAX_LENGTH: usize = 256;

/// Maximum length of NetworkDeviceData.HardwareAddress
pub const NETWORK_DEVICE_DATA_HARDWARE_ADDRESS_MAX_LENGTH: usize = 128;

/// Maximum number of IPs in NetworkDeviceData
pub const NETWORK_DEVICE_DATA_MAX_IPS: usize = 16;

/// Maximum number of discrete capacity request policy options
pub const CAPACITY_REQUEST_POLICY_DISCRETE_MAX_OPTIONS: usize = 10;

/// Maximum length for domain in qualified name
pub const DEVICE_MAX_DOMAIN_LENGTH: usize = 63;

/// Maximum length for ID in qualified name
pub const DEVICE_MAX_ID_LENGTH: usize = 32;

/// Maximum key length for attributes and capacities (domain/ID)
pub const ATTRIBUTE_AND_CAPACITY_MAX_KEY_LENGTH: usize =
    DEVICE_MAX_DOMAIN_LENGTH + 1 + DEVICE_MAX_ID_LENGTH;

// ============================================================================
// Main Validation Entry Points
// ============================================================================

/// Validates a ResourceClaim
pub fn validate_resource_claim(claim: &internal::ResourceClaim) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &claim.metadata,
        true, // namespaced
        name_is_dns_subdomain,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_resource_claim_spec(
        &claim.spec,
        &Path::new("spec"),
        false,
    ));
    all_errs
}

/// Validates a ResourceClaim update
pub fn validate_resource_claim_update(
    claim: &internal::ResourceClaim,
    old_claim: &internal::ResourceClaim,
) -> ErrorList {
    let mut all_errs =
        validate_object_meta_update(&claim.metadata, &old_claim.metadata, &Path::new("metadata"));
    // Spec is immutable
    if claim.spec != old_claim.spec {
        all_errs.push(forbidden(&Path::new("spec"), "spec is immutable"));
    }
    // Validate spec even on update (stored=true since it should have been stored)
    all_errs.extend(validate_resource_claim_spec(
        &claim.spec,
        &Path::new("spec"),
        true,
    ));
    all_errs
}

/// Validates a DeviceClass
pub fn validate_device_class(class: &DeviceClass) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &class.metadata,
        false, // not namespaced
        name_is_dns_subdomain,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_device_class_spec(
        &class.spec,
        None,
        &Path::new("spec"),
    ));
    all_errs
}

/// Validates a DeviceClass update
pub fn validate_device_class_update(class: &DeviceClass, old_class: &DeviceClass) -> ErrorList {
    let mut all_errs =
        validate_object_meta_update(&class.metadata, &old_class.metadata, &Path::new("metadata"));
    all_errs.extend(validate_device_class_spec(
        &class.spec,
        Some(&old_class.spec),
        &Path::new("spec"),
    ));
    all_errs
}

/// Validates a ResourceClaimTemplate
pub fn validate_resource_claim_template(template: &internal::ResourceClaimTemplate) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &template.metadata,
        true, // namespaced
        name_is_dns_subdomain,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_resource_claim_template_spec(
        &template.spec,
        &Path::new("spec"),
        false,
    ));
    all_errs
}

/// Validates a ResourceClaimTemplate update
pub fn validate_resource_claim_template_update(
    template: &internal::ResourceClaimTemplate,
    old_template: &internal::ResourceClaimTemplate,
) -> ErrorList {
    let mut all_errs = validate_object_meta_update(
        &template.metadata,
        &old_template.metadata,
        &Path::new("metadata"),
    );
    // Spec is immutable
    if template.spec != old_template.spec {
        all_errs.push(forbidden(&Path::new("spec"), "spec is immutable"));
    }
    // Validate spec even on update (stored=true since it should have been stored)
    all_errs.extend(validate_resource_claim_template_spec(
        &template.spec,
        &Path::new("spec"),
        true,
    ));
    all_errs
}

/// Validates a ResourceSlice
pub fn validate_resource_slice(slice: &internal::ResourceSlice) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &slice.metadata,
        false, // not namespaced
        name_is_dns_subdomain,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_resource_slice_spec(
        &slice.spec,
        None,
        &Path::new("spec"),
    ));
    all_errs
}

/// Validates a ResourceSlice update
pub fn validate_resource_slice_update(
    slice: &internal::ResourceSlice,
    old_slice: &internal::ResourceSlice,
) -> ErrorList {
    let mut all_errs =
        validate_object_meta_update(&slice.metadata, &old_slice.metadata, &Path::new("metadata"));
    all_errs.extend(validate_resource_slice_spec(
        &slice.spec,
        Some(&old_slice.spec),
        &Path::new("spec"),
    ));
    all_errs
}

// ============================================================================
// Spec Validators
// ============================================================================

fn validate_resource_claim_spec(
    spec: &internal::ResourceClaimSpec,
    fld_path: &Path,
    stored: bool,
) -> ErrorList {
    validate_device_claim(&spec.devices, &fld_path.child("devices"), stored)
}

fn validate_device_claim(claim: &DeviceClaim, fld_path: &Path, stored: bool) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Gather request names for cross-validation
    let request_names = gather_request_names(claim);

    // Validate requests (using validateSet pattern)
    all_errs.extend(validate_set(
        &claim.requests,
        DEVICE_REQUESTS_MAX_SIZE,
        |req, path| validate_device_request(req, path, stored),
        |req| req.name.clone(),
        &fld_path.child("requests"),
        "name",
    ));

    // Validate constraints
    all_errs.extend(validate_slice(
        &claim.constraints,
        DEVICE_CONSTRAINTS_MAX_SIZE,
        |constraint, path| validate_device_constraint(constraint, path, &request_names),
        &fld_path.child("constraints"),
    ));

    // Validate config
    all_errs.extend(validate_slice(
        &claim.config,
        DEVICE_CONFIG_MAX_SIZE,
        |config, path| validate_device_claim_configuration(config, path, &request_names, stored),
        &fld_path.child("config"),
    ));

    all_errs
}

fn validate_resource_claim_template_spec(
    spec: &internal::ResourceClaimTemplateSpec,
    fld_path: &Path,
    stored: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate template metadata (spec.metadata)
    // Note: Template metadata has different validation rules (no resourceVersion, etc.)
    // For simplicity, we skip detailed template metadata validation here
    // In full implementation, this should use ValidateTemplateObjectMeta

    all_errs.extend(validate_resource_claim_spec(
        &spec.spec,
        &fld_path.child("spec"),
        stored,
    ));
    all_errs
}

fn validate_device_class_spec(
    spec: &DeviceClassSpec,
    old_spec: Option<&DeviceClassSpec>,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Determine if selectors should be treated as stored
    let stored = old_spec.map_or(false, |old| spec.selectors == old.selectors);

    // Validate selectors (device_class::DeviceSelector)
    all_errs.extend(validate_slice(
        &spec.selectors,
        DEVICE_SELECTORS_MAX_SIZE,
        |selector, path| validate_device_class_selector(selector, path, stored),
        &fld_path.child("selectors"),
    ));

    // Determine if config should be treated as stored
    let stored = old_spec.map_or(false, |old| spec.config == old.config);

    // Validate config
    all_errs.extend(validate_slice(
        &spec.config,
        DEVICE_CONFIG_MAX_SIZE,
        |config, path| validate_device_class_configuration(config, path, stored),
        &fld_path.child("config"),
    ));

    all_errs
}

fn validate_resource_slice_spec(
    spec: &ResourceSliceSpec,
    old_spec: Option<&ResourceSliceSpec>,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate driver name
    all_errs.extend(validate_driver_name(
        &spec.driver,
        &fld_path.child("driver"),
    ));

    // Validate pool
    all_errs.extend(validate_resource_pool(&spec.pool, &fld_path.child("pool")));

    // Check immutable fields on update
    if let Some(old) = old_spec {
        if spec.pool.name != old.pool.name {
            all_errs.push(forbidden(
                &fld_path.child("pool").child("name"),
                "pool.name is immutable",
            ));
        }
        if spec.driver != old.driver {
            all_errs.push(forbidden(&fld_path.child("driver"), "driver is immutable"));
        }
        if spec.node_name != old.node_name {
            all_errs.push(forbidden(
                &fld_path.child("nodeName"),
                "nodeName is immutable",
            ));
        }
    }

    // Validate node selection (mutual exclusivity)
    let mut set_fields = Vec::new();

    if let Some(ref node_name) = spec.node_name {
        if node_name.is_empty() {
            all_errs.push(invalid(
                &fld_path.child("nodeName"),
                BadValue::String(node_name.clone()),
                "must be either unset or set to a non-empty string",
            ));
        } else {
            set_fields.push("nodeName");
            all_errs.extend(validate_node_name(node_name, &fld_path.child("nodeName")));
        }
    }

    if spec.node_selector.is_some() {
        set_fields.push("nodeSelector");
        // TODO: Validate NodeSelector once core validation is available
        // all_errs.extend(corevalidation::validate_node_selector(spec.node_selector, false, &fld_path.child("nodeSelector")));
    }

    if let Some(all_nodes) = spec.all_nodes {
        if all_nodes {
            set_fields.push("allNodes");
        } else {
            all_errs.push(invalid(
                &fld_path.child("allNodes"),
                BadValue::Bool(all_nodes),
                "must be either unset or set to true",
            ));
        }
    }

    if let Some(per_device) = spec.per_device_node_selection {
        if per_device {
            set_fields.push("perDeviceNodeSelection");
        } else {
            all_errs.push(invalid(
                &fld_path.child("perDeviceNodeSelection"),
                BadValue::Bool(per_device),
                "must be either unset or set to true",
            ));
        }
    }

    // Check mutual exclusivity
    match set_fields.len() {
        0 => all_errs.push(required(
            fld_path,
            "exactly one of nodeName, nodeSelector, allNodes, perDeviceNodeSelection is required",
        )),
        1 => {}, // OK
        _ => all_errs.push(invalid(
            fld_path,
            BadValue::String(format!("{{{}}}", set_fields.join(", "))),
            "exactly one of nodeName, nodeSelector, allNodes, perDeviceNodeSelection is required, but multiple fields are set",
        )),
    }

    // Gather shared counter names for device validation
    let shared_counter_names = gather_shared_counter_names(&spec.shared_counters);

    // Validate devices
    all_errs.extend(validate_set(
        &spec.devices,
        RESOURCE_SLICE_MAX_DEVICES,
        |device, path| {
            validate_device(
                device,
                path,
                &shared_counter_names,
                spec.per_device_node_selection,
            )
        },
        |device| device.name.clone(),
        &fld_path.child("devices"),
        "name",
    ));

    // Check total device counters limit
    let total_device_counters: usize = spec
        .devices
        .iter()
        .map(|d| {
            d.consumes_counters
                .iter()
                .map(|c| c.counters.len())
                .sum::<usize>()
        })
        .sum();

    if total_device_counters > RESOURCE_SLICE_MAX_DEVICE_COUNTERS_PER_SLICE {
        all_errs.push(invalid(
            &fld_path.child("devices"),
            BadValue::Int(total_device_counters as i64),
            &format!(
                "the total number of counters in devices must not exceed {}",
                RESOURCE_SLICE_MAX_DEVICE_COUNTERS_PER_SLICE
            ),
        ));
    }

    // Check total shared counters limit
    let total_shared_counters: usize = spec.shared_counters.iter().map(|s| s.counters.len()).sum();

    if total_shared_counters > RESOURCE_SLICE_MAX_SHARED_COUNTERS {
        all_errs.push(invalid(
            &fld_path.child("sharedCounters"),
            BadValue::Int(total_shared_counters as i64),
            &format!(
                "the total number of shared counters must not exceed {}",
                RESOURCE_SLICE_MAX_SHARED_COUNTERS
            ),
        ));
    }

    // Validate shared counters
    all_errs.extend(validate_set(
        &spec.shared_counters,
        -1i32 as usize, // No limit on number of sets
        validate_counter_set,
        |set| set.name.clone(),
        &fld_path.child("sharedCounters"),
        "name",
    ));

    all_errs
}

// ============================================================================
// Helper Validators
// ============================================================================

fn validate_device_request(request: &DeviceRequest, fld_path: &Path, stored: bool) -> ErrorList {
    let mut all_errs = validate_request_name(&request.name, &fld_path.child("name"));

    let mut num_device_request_type = 0;

    if !request.first_available.is_empty() {
        num_device_request_type += 1;
        all_errs.extend(validate_set(
            &request.first_available,
            FIRST_AVAILABLE_DEVICE_REQUEST_MAX_SIZE,
            |sub_req, path| validate_device_sub_request(sub_req, path, stored),
            |sub_req| sub_req.name.clone(),
            &fld_path.child("firstAvailable"),
            "name",
        ));
    }

    if request.exactly.is_some() {
        num_device_request_type += 1;
        if let Some(ref exact) = request.exactly {
            all_errs.extend(validate_exact_device_request(
                exact,
                &fld_path.child("exactly"),
                stored,
            ));
        }
    }

    match num_device_request_type {
        0 => all_errs.push(required(
            fld_path,
            "exactly one of `exactly` or `firstAvailable` is required",
        )),
        1 => {} // OK
        _ => all_errs.push(invalid(
            fld_path,
            BadValue::String("multiple".to_string()),
            "exactly one of `exactly` or `firstAvailable` is required, but multiple fields are set",
        )),
    }

    all_errs
}

fn validate_device_sub_request(
    sub_request: &DeviceSubRequest,
    fld_path: &Path,
    stored: bool,
) -> ErrorList {
    let mut all_errs = validate_request_name(&sub_request.name, &fld_path.child("name"));
    all_errs.extend(validate_device_class_string(
        &sub_request.device_class_name,
        &fld_path.child("deviceClassName"),
    ));
    all_errs.extend(validate_selector_slice(
        &sub_request.selectors,
        &fld_path.child("selectors"),
        stored,
    ));
    all_errs.extend(validate_device_allocation_mode(
        &sub_request.allocation_mode,
        sub_request.count,
        &fld_path.child("allocationMode"),
        &fld_path.child("count"),
    ));

    for (i, toleration) in sub_request.tolerations.iter().enumerate() {
        all_errs.extend(validate_device_toleration(
            toleration,
            &fld_path.child("tolerations").index(i),
        ));
    }

    all_errs
}

fn validate_exact_device_request(
    request: &ExactDeviceRequest,
    fld_path: &Path,
    stored: bool,
) -> ErrorList {
    let mut all_errs = validate_device_class_string(
        &request.device_class_name,
        &fld_path.child("deviceClassName"),
    );
    all_errs.extend(validate_selector_slice(
        &request.selectors,
        &fld_path.child("selectors"),
        stored,
    ));
    all_errs.extend(validate_device_allocation_mode(
        &request.allocation_mode,
        request.count,
        &fld_path.child("allocationMode"),
        &fld_path.child("count"),
    ));

    for (i, toleration) in request.tolerations.iter().enumerate() {
        all_errs.extend(validate_device_toleration(
            toleration,
            &fld_path.child("tolerations").index(i),
        ));
    }

    all_errs
}

fn validate_device_allocation_mode(
    mode: &DeviceAllocationMode,
    count: i64,
    alloc_mode_path: &Path,
    count_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    match mode {
        DeviceAllocationMode::All => {
            if count != 0 {
                all_errs.push(invalid(
                    count_path,
                    BadValue::Int(count),
                    "must not be specified when allocationMode is 'All'",
                ));
            }
        }
        DeviceAllocationMode::ExactCount => {
            if count <= 0 {
                all_errs.push(invalid(
                    count_path,
                    BadValue::Int(count),
                    "must be greater than zero",
                ));
            }
        }
    }

    all_errs
}

fn validate_device_class_string(device_class: &str, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if device_class.is_empty() {
        all_errs.push(required(fld_path, "deviceClassName is required"));
    } else {
        all_errs.extend(validate_device_class_name(device_class, fld_path));
    }
    all_errs
}

fn validate_selector_slice(
    selectors: &[DeviceSelector],
    fld_path: &Path,
    stored: bool,
) -> ErrorList {
    validate_slice(
        selectors,
        DEVICE_SELECTORS_MAX_SIZE,
        |selector, path| validate_selector(selector, path, stored),
        fld_path,
    )
}

fn validate_selector(selector: &DeviceSelector, fld_path: &Path, stored: bool) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if selector.cel.is_none() {
        all_errs.push(required(&fld_path.child("cel"), "cel selector is required"));
    } else if let Some(ref cel) = selector.cel {
        all_errs.extend(validate_cel_selector(cel, &fld_path.child("cel"), stored));
    }
    all_errs
}

fn validate_cel_selector(cel: &CELDeviceSelector, fld_path: &Path, stored: bool) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Check length
    if cel.expression.len() > CEL_SELECTOR_EXPRESSION_MAX_LENGTH {
        all_errs.push(too_long(
            &fld_path.child("expression"),
            CEL_SELECTOR_EXPRESSION_MAX_LENGTH,
        ));
        // Don't bother compiling too long expressions
        return all_errs;
    }

    // NOTE: Full CEL compilation and cost checking is not implemented yet
    // In full implementation, this would compile the CEL expression and check:
    // - Syntax validity
    // - Cost limit (CEL_SELECTOR_EXPRESSION_MAX_COST = 1000000)
    // For now, we just check basic requirements
    if cel.expression.is_empty() {
        all_errs.push(required(
            &fld_path.child("expression"),
            "expression is required",
        ));
    }

    all_errs
}

fn validate_device_constraint(
    constraint: &DeviceConstraint,
    fld_path: &Path,
    request_names: &RequestNames,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate requests references
    all_errs.extend(validate_set(
        &constraint.requests,
        DEVICE_REQUESTS_MAX_SIZE,
        |name, path| validate_request_name_ref(name, path, request_names),
        |name| name.clone(),
        &fld_path.child("requests"),
        "",
    ));

    // Check mutual exclusivity of matchAttribute and distinctAttribute
    let has_match = constraint.match_attribute.is_some();
    let has_distinct = constraint.distinct_attribute.is_some();

    if !has_match && !has_distinct {
        all_errs.push(required(
            fld_path,
            "exactly one of matchAttribute or distinctAttribute is required",
        ));
    } else if has_match && has_distinct {
        all_errs.push(invalid(fld_path, BadValue::String("both".to_string()), "exactly one of matchAttribute or distinctAttribute is required, but multiple fields are set"));
    } else if let Some(ref attr) = constraint.match_attribute {
        all_errs.extend(validate_fully_qualified_name(
            attr,
            &fld_path.child("matchAttribute"),
        ));
    } else if let Some(ref attr) = constraint.distinct_attribute {
        all_errs.extend(validate_fully_qualified_name(
            attr,
            &fld_path.child("distinctAttribute"),
        ));
    }

    all_errs
}

fn validate_device_claim_configuration(
    config: &DeviceClaimConfiguration,
    fld_path: &Path,
    request_names: &RequestNames,
    stored: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate requests references
    all_errs.extend(validate_set(
        &config.requests,
        DEVICE_REQUESTS_MAX_SIZE,
        |name, path| validate_request_name_ref(name, path, request_names),
        |name| name.clone(),
        &fld_path.child("requests"),
        "",
    ));

    // Validate opaque configuration
    if config.opaque.is_none() {
        all_errs.push(required(
            &fld_path.child("opaque"),
            "opaque configuration is required",
        ));
    } else if let Some(ref opaque) = config.opaque {
        all_errs.extend(validate_opaque_configuration(
            opaque,
            &fld_path.child("opaque"),
            stored,
        ));
    }

    all_errs
}

fn validate_device_class_configuration(
    config: &DeviceClassConfiguration,
    fld_path: &Path,
    stored: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate opaque configuration
    if config.opaque.is_none() {
        all_errs.push(required(
            &fld_path.child("opaque"),
            "opaque configuration is required",
        ));
    } else if let Some(ref opaque) = config.opaque {
        all_errs.extend(validate_device_class_opaque_configuration(
            opaque,
            &fld_path.child("opaque"),
            stored,
        ));
    }

    all_errs
}

fn validate_device_class_opaque_configuration(
    config: &DeviceClassOpaqueConfig,
    fld_path: &Path,
    stored: bool,
) -> ErrorList {
    let mut all_errs = validate_driver_name(&config.driver, &fld_path.child("driver"));

    // Validate parameters (JSON)
    if let Some(ref params) = config.parameters {
        // Basic JSON validation
        if !stored && params.to_string().len() > OPAQUE_PARAMETERS_MAX_LENGTH {
            all_errs.push(too_long(
                &fld_path.child("parameters"),
                OPAQUE_PARAMETERS_MAX_LENGTH,
            ));
        }

        // Check if it's a valid JSON object (not null, array, or primitive)
        if !params.is_object() {
            all_errs.push(invalid(
                &fld_path.child("parameters"),
                BadValue::String("<value omitted>".to_string()),
                "must be a valid JSON object",
            ));
        }
    }

    all_errs
}

fn validate_opaque_configuration(
    config: &OpaqueDeviceConfiguration,
    fld_path: &Path,
    stored: bool,
) -> ErrorList {
    let mut all_errs = validate_driver_name(&config.driver, &fld_path.child("driver"));

    // Validate parameters (JSON)
    if let Some(ref params) = config.parameters {
        // Basic JSON validation
        if !stored && params.to_string().len() > OPAQUE_PARAMETERS_MAX_LENGTH {
            all_errs.push(too_long(
                &fld_path.child("parameters"),
                OPAQUE_PARAMETERS_MAX_LENGTH,
            ));
        }

        // Check if it's a valid JSON object (not null, array, or primitive)
        if !params.is_object() {
            all_errs.push(invalid(
                &fld_path.child("parameters"),
                BadValue::String("<value omitted>".to_string()),
                "must be a valid JSON object",
            ));
        }
    }

    all_errs
}

// Device Class selector validation (device_class::DeviceSelector)
fn validate_device_class_selector(
    selector: &DeviceClassSelector,
    fld_path: &Path,
    stored: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if selector.cel.is_none() {
        all_errs.push(required(&fld_path.child("cel"), "cel selector is required"));
    } else if let Some(ref cel) = selector.cel {
        all_errs.extend(validate_device_class_cel_selector(
            cel,
            &fld_path.child("cel"),
            stored,
        ));
    }
    all_errs
}

fn validate_device_class_cel_selector(
    cel: &DeviceClassCELSelector,
    fld_path: &Path,
    stored: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Check length
    if cel.expression.len() > CEL_SELECTOR_EXPRESSION_MAX_LENGTH {
        all_errs.push(too_long(
            &fld_path.child("expression"),
            CEL_SELECTOR_EXPRESSION_MAX_LENGTH,
        ));
        // Don't bother compiling too long expressions
        return all_errs;
    }

    // NOTE: Full CEL compilation and cost checking is not implemented yet
    if cel.expression.is_empty() {
        all_errs.push(required(
            &fld_path.child("expression"),
            "expression is required",
        ));
    }

    all_errs
}

fn validate_device(
    device: &Device,
    fld_path: &Path,
    shared_counter_names: &BTreeMap<String, BTreeSet<String>>,
    per_device_node_selection: Option<bool>,
) -> ErrorList {
    let mut all_errs = validate_device_name(&device.name, &fld_path.child("name"));

    let allow_multiple_allocations = device.allow_multiple_allocations.unwrap_or(false);

    // Check total attributes and capacities limit
    let total = device.attributes.len() + device.capacity.len();
    if total > RESOURCE_SLICE_MAX_ATTRIBUTES_AND_CAPACITIES_PER_DEVICE {
        all_errs.push(invalid(
            fld_path,
            BadValue::Int(total as i64),
            &format!(
                "the total number of attributes and capacities must not exceed {}",
                RESOURCE_SLICE_MAX_ATTRIBUTES_AND_CAPACITIES_PER_DEVICE
            ),
        ));
    }

    // Validate attributes
    all_errs.extend(validate_map(
        &device.attributes,
        -1i32 as usize,
        ATTRIBUTE_AND_CAPACITY_MAX_KEY_LENGTH,
        validate_qualified_name_str,
        validate_device_attribute,
        &fld_path.child("attributes"),
    ));

    // Validate capacity (different validation based on allowMultipleAllocations)
    all_errs.extend(validate_map(
        &device.capacity,
        -1i32 as usize,
        ATTRIBUTE_AND_CAPACITY_MAX_KEY_LENGTH,
        validate_qualified_name_str,
        |cap, path| {
            if allow_multiple_allocations {
                validate_multi_allocatable_device_capacity(cap, path)
            } else {
                validate_single_allocatable_device_capacity(cap, path)
            }
        },
        &fld_path.child("capacity"),
    ));

    // Validate taints
    all_errs.extend(validate_slice(
        &device.taints,
        DEVICE_TAINTS_MAX_LENGTH,
        validate_device_taint,
        &fld_path.child("taints"),
    ));

    // Validate consumes_counters
    all_errs.extend(validate_set(
        &device.consumes_counters,
        -1i32 as usize,
        validate_device_counter_consumption,
        |c| c.counter_set.clone(),
        &fld_path.child("consumesCounters"),
        "counterSet",
    ));

    // Check total counters limit
    let total_counters: usize = device
        .consumes_counters
        .iter()
        .map(|c| c.counters.len())
        .sum();

    if total_counters > RESOURCE_SLICE_MAX_COUNTERS_PER_DEVICE {
        all_errs.push(invalid(
            fld_path,
            BadValue::Int(total_counters as i64),
            &format!(
                "the total number of counters must not exceed {}",
                RESOURCE_SLICE_MAX_COUNTERS_PER_DEVICE
            ),
        ));
    }

    // Cross-validate counter references
    for (i, consumption) in device.consumes_counters.iter().enumerate() {
        if let Some(counter_names) = shared_counter_names.get(&consumption.counter_set) {
            for counter_name in consumption.counters.keys() {
                if !counter_names.contains(counter_name) {
                    all_errs.push(invalid(
                        &fld_path
                            .child("consumesCounters")
                            .index(i)
                            .child("counters"),
                        BadValue::String(counter_name.clone()),
                        "must reference a counter defined in the ResourceSlice sharedCounters",
                    ));
                }
            }
        } else {
            all_errs.push(invalid(
                &fld_path
                    .child("consumesCounters")
                    .index(i)
                    .child("counterSet"),
                BadValue::String(consumption.counter_set.clone()),
                "must reference a counterSet defined in the ResourceSlice sharedCounters",
            ));
        }
    }

    // Validate per-device node selection
    if per_device_node_selection == Some(true) {
        let mut set_fields = Vec::new();

        if let Some(ref node_name) = device.node_name {
            if node_name.is_empty() {
                all_errs.push(invalid(
                    &fld_path.child("nodeName"),
                    BadValue::String(node_name.clone()),
                    "must not be empty",
                ));
            } else {
                set_fields.push("nodeName");
                all_errs.extend(validate_node_name(node_name, &fld_path.child("nodeName")));
            }
        }

        if device.node_selector.is_some() {
            set_fields.push("nodeSelector");
            // TODO: Validate NodeSelector
        }

        if let Some(all_nodes) = device.all_nodes {
            if all_nodes {
                set_fields.push("allNodes");
            } else {
                all_errs.push(invalid(
                    &fld_path.child("allNodes"),
                    BadValue::Bool(all_nodes),
                    "must be either unset or set to true",
                ));
            }
        }

        match set_fields.len() {
            0 => all_errs.push(required(
                fld_path,
                "exactly one of nodeName, nodeSelector, or allNodes is required when perDeviceNodeSelection is set to true in the ResourceSlice spec",
            )),
            1 => {},
            _ => all_errs.push(invalid(
                fld_path,
                BadValue::String(format!("{{{}}}", set_fields.join(", "))),
                "exactly one of nodeName, nodeSelector, or allNodes is required when perDeviceNodeSelection is set to true in the ResourceSlice spec",
            )),
        }
    } else if device.node_name.is_some()
        || device.node_selector.is_some()
        || device.all_nodes.is_some()
    {
        all_errs.push(invalid(
            fld_path,
            BadValue::String("node selection fields set".to_string()),
            "nodeName, nodeSelector and allNodes can only be set if perDeviceNodeSelection is set to true in the ResourceSlice spec",
        ));
    }

    // Validate binding parameters
    all_errs.extend(validate_device_binding_parameters(
        &device.binding_conditions,
        &device.binding_failure_conditions,
        fld_path,
    ));

    all_errs
}

fn validate_device_attribute(attribute: &DeviceAttribute, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let mut num_fields = 0;

    if attribute.bool_value.is_some() {
        num_fields += 1;
    }
    if attribute.int_value.is_some() {
        num_fields += 1;
    }
    if let Some(ref string_value) = attribute.string_value {
        num_fields += 1;
        if string_value.len() > DEVICE_ATTRIBUTE_MAX_VALUE_LENGTH {
            all_errs.push(too_long(
                &fld_path.child("string"),
                DEVICE_ATTRIBUTE_MAX_VALUE_LENGTH,
            ));
        }
    }
    if let Some(ref version_value) = attribute.version_value {
        num_fields += 1;
        // TODO: Validate semver format
        if version_value.len() > DEVICE_ATTRIBUTE_MAX_VALUE_LENGTH {
            all_errs.push(too_long(
                &fld_path.child("version"),
                DEVICE_ATTRIBUTE_MAX_VALUE_LENGTH,
            ));
        }
    }

    match num_fields {
        0 => all_errs.push(required(fld_path, "exactly one value must be specified")),
        1 => {} // OK
        _ => all_errs.push(invalid(
            fld_path,
            BadValue::String("multiple".to_string()),
            "exactly one value must be specified",
        )),
    }

    all_errs
}

fn validate_multi_allocatable_device_capacity(
    capacity: &DeviceCapacity,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // requestPolicy is allowed for multi-allocatable devices
    // TODO: Implement full request policy validation

    all_errs
}

fn validate_single_allocatable_device_capacity(
    capacity: &DeviceCapacity,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // requestPolicy is forbidden for single-allocatable devices
    if capacity.request_policy.is_some() {
        all_errs.push(forbidden(
            &fld_path.child("requestPolicy"),
            "allowMultipleAllocations must be true",
        ));
    }

    all_errs
}

fn validate_device_taint(taint: &DeviceTaint, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate key as label name
    if taint.key.is_empty() {
        all_errs.push(required(&fld_path.child("key"), "key is required"));
    } else {
        all_errs.extend(validate_label_name(&taint.key, &fld_path.child("key")));
    }

    // Validate value as label value
    if !taint.value.is_empty() {
        // TODO: Use proper label value validation
        // all_errs.extend(validate_label_value(&taint.value, &fld_path.child("value")));
    }

    // Validate effect
    if taint.effect.is_empty() {
        all_errs.push(required(&fld_path.child("effect"), "effect is required"));
    } else {
        let valid_effects = ["NoSchedule", "NoExecute"];
        if !valid_effects.contains(&taint.effect.as_str()) {
            all_errs.push(not_supported(
                &fld_path.child("effect"),
                BadValue::String(taint.effect.clone()),
                &valid_effects,
            ));
        }
    }

    all_errs
}

fn validate_device_toleration(toleration: &DeviceToleration, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate key (optional, but if present must be valid label name)
    if !toleration.key.is_empty() {
        all_errs.extend(validate_label_name(&toleration.key, &fld_path.child("key")));
    }

    // Validate operator
    match toleration.operator {
        DeviceTolerationOperator::Exists => {
            if !toleration.value.is_empty() {
                all_errs.push(invalid(
                    &fld_path.child("value"),
                    BadValue::String(toleration.value.clone()),
                    "must be empty for operator Exists",
                ));
            }
        }
        DeviceTolerationOperator::Equal => {
            // TODO: Validate value as label value if not empty
        }
    }

    // Validate effect (optional)
    if let Some(ref effect) = toleration.effect {
        let valid_effects = ["NoSchedule", "NoExecute"];
        if !valid_effects.contains(&effect.as_str()) {
            all_errs.push(not_supported(
                &fld_path.child("effect"),
                BadValue::String(effect.clone()),
                &valid_effects,
            ));
        }
    }

    all_errs
}

fn validate_counter_set(counter_set: &CounterSet, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if counter_set.name.is_empty() {
        all_errs.push(required(&fld_path.child("name"), "name is required"));
    } else {
        all_errs.extend(validate_counter_name(
            &counter_set.name,
            &fld_path.child("name"),
        ));
    }

    if counter_set.counters.is_empty() {
        all_errs.push(required(
            &fld_path.child("counters"),
            "counters is required",
        ));
    } else {
        all_errs.extend(validate_map(
            &counter_set.counters,
            -1i32 as usize,
            63, // DNS1123LabelMaxLength
            validate_counter_name,
            |_counter, _path| ErrorList::new(), // Any parsed Quantity is valid
            &fld_path.child("counters"),
        ));
    }

    all_errs
}

fn validate_device_counter_consumption(
    consumption: &DeviceCounterConsumption,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if consumption.counter_set.is_empty() {
        all_errs.push(required(
            &fld_path.child("counterSet"),
            "counterSet is required",
        ));
    }

    if consumption.counters.is_empty() {
        all_errs.push(required(
            &fld_path.child("counters"),
            "counters is required",
        ));
    } else {
        all_errs.extend(validate_map(
            &consumption.counters,
            -1i32 as usize,
            63, // DNS1123LabelMaxLength
            validate_counter_name,
            |_counter, _path| ErrorList::new(), // Any parsed Quantity is valid
            &fld_path.child("counters"),
        ));
    }

    all_errs
}

fn validate_resource_pool(pool: &ResourcePool, fld_path: &Path) -> ErrorList {
    let mut all_errs = validate_pool_name(&pool.name, &fld_path.child("name"));

    if pool.resource_slice_count <= 0 {
        all_errs.push(invalid(
            &fld_path.child("resourceSliceCount"),
            BadValue::Int(pool.resource_slice_count),
            "must be greater than zero",
        ));
    }

    if pool.generation < 0 {
        all_errs.push(invalid(
            &fld_path.child("generation"),
            BadValue::Int(pool.generation),
            "must be greater than or equal to zero",
        ));
    }

    all_errs
}

fn validate_device_binding_parameters(
    binding_conditions: &[String],
    binding_failure_conditions: &[String],
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate size limits
    all_errs.extend(validate_slice(
        binding_conditions,
        BINDING_CONDITIONS_MAX_SIZE,
        |name, path| validate_label_name(name.as_str(), path),
        &fld_path.child("bindingConditions"),
    ));

    all_errs.extend(validate_slice(
        binding_failure_conditions,
        BINDING_FAILURE_CONDITIONS_MAX_SIZE,
        |name, path| validate_label_name(name.as_str(), path),
        &fld_path.child("bindingFailureConditions"),
    ));

    // Check for duplicates within each list
    let mut conditions_set = HashSet::new();
    for (i, condition) in binding_conditions.iter().enumerate() {
        if conditions_set.contains(condition) {
            all_errs.push(duplicate(
                &fld_path.child("bindingConditions").index(i),
                BadValue::String(condition.clone()),
            ));
        } else {
            conditions_set.insert(condition.clone());
        }
    }

    let mut failure_conditions_set = HashSet::new();
    for (i, condition) in binding_failure_conditions.iter().enumerate() {
        if failure_conditions_set.contains(condition) {
            all_errs.push(duplicate(
                &fld_path.child("bindingFailureConditions").index(i),
                BadValue::String(condition.clone()),
            ));
        } else {
            failure_conditions_set.insert(condition.clone());
        }

        // Check for overlap with bindingConditions
        if conditions_set.contains(condition) {
            all_errs.push(invalid(
                &fld_path.child("bindingFailureConditions").index(i),
                BadValue::String(condition.clone()),
                "bindingFailureConditions must not overlap with bindingConditions",
            ));
        }
    }

    // Check mutual requirements
    if binding_conditions.is_empty() && !binding_failure_conditions.is_empty() {
        all_errs.push(invalid(
            &fld_path.child("bindingConditions"),
            BadValue::String("empty".to_string()),
            "bindingConditions are required to use bindingFailureConditions",
        ));
    }

    if !binding_conditions.is_empty() && binding_failure_conditions.is_empty() {
        all_errs.push(invalid(
            &fld_path.child("bindingFailureConditions"),
            BadValue::String("empty".to_string()),
            "bindingFailureConditions are required to use bindingConditions",
        ));
    }

    all_errs
}

// ============================================================================
// Name Validators
// ============================================================================

fn validate_driver_name(name: &str, fld_path: &Path) -> ErrorList {
    // Driver names follow CSI driver name rules (DNS subdomain)
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_subdomain(name) {
        all_errs.push(invalid(fld_path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

fn validate_device_name(name: &str, fld_path: &Path) -> ErrorList {
    // Device names are DNS labels
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_label(name) {
        all_errs.push(invalid(fld_path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

fn validate_device_class_name(name: &str, fld_path: &Path) -> ErrorList {
    // Device class names are DNS subdomains
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_subdomain(name) {
        all_errs.push(invalid(fld_path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

fn validate_request_name(name: &str, fld_path: &Path) -> ErrorList {
    // Request names are DNS labels
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_label(name) {
        all_errs.push(invalid(fld_path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

fn validate_counter_name(name: &str, fld_path: &Path) -> ErrorList {
    // Counter names are DNS labels
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_label(name) {
        all_errs.push(invalid(fld_path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

fn validate_pool_name(name: &str, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if name.is_empty() {
        all_errs.push(required(fld_path, "pool name is required"));
    } else {
        if name.len() > POOL_NAME_MAX_LENGTH {
            all_errs.push(too_long(fld_path, POOL_NAME_MAX_LENGTH));
        }

        // Each part separated by / must be a DNS subdomain
        for part in name.split('/') {
            for msg in is_dns1123_subdomain(part) {
                all_errs.push(invalid(fld_path, BadValue::String(name.to_string()), &msg));
            }
        }
    }

    all_errs
}

fn validate_node_name(name: &str, fld_path: &Path) -> ErrorList {
    // Node names are DNS subdomains
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_subdomain(name) {
        all_errs.push(invalid(fld_path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

fn validate_qualified_name_str(name: &str, fld_path: &Path) -> ErrorList {
    validate_qualified_name(name, fld_path)
}

fn validate_fully_qualified_name(name: &str, fld_path: &Path) -> ErrorList {
    let mut all_errs = validate_qualified_name(name, fld_path);

    // Fully qualified names MUST include a domain (contain a '/')
    if !name.is_empty() && !name.contains('/') {
        all_errs.push(invalid(
            fld_path,
            BadValue::String(name.to_string()),
            "must include a domain",
        ));
    }

    all_errs
}

fn validate_request_name_ref(
    name: &str,
    fld_path: &Path,
    request_names: &RequestNames,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let segments: Vec<&str> = name.split('/').collect();

    if segments.len() > 2 {
        all_errs.push(invalid(
            fld_path,
            BadValue::String(name.to_string()),
            "must be the name of a request in the claim or the name of a request and a subrequest separated by '/'",
        ));
        return all_errs;
    }

    // Validate each segment
    for segment in &segments {
        all_errs.extend(validate_request_name(segment, fld_path));
    }

    // Check if the reference exists
    if !request_names.has(name) {
        all_errs.push(invalid(
            fld_path,
            BadValue::String(name.to_string()),
            "must be the name of a request in the claim or the name of a request and a subrequest separated by '/'",
        ));
    }

    all_errs
}

// ============================================================================
// Helper Functions and Types
// ============================================================================

/// RequestNames tracks request and subrequest names for validation
#[derive(Debug)]
struct RequestNames {
    // Maps request name to optional set of subrequest names
    names: BTreeMap<String, Option<BTreeSet<String>>>,
}

impl RequestNames {
    fn has(&self, name: &str) -> bool {
        let segments: Vec<&str> = name.split('/').collect();

        if segments.len() > 2 {
            return false;
        }

        // Check if the first segment exists
        if let Some(sub_names) = self.names.get(segments[0]) {
            if segments.len() == 1 {
                return true;
            }

            // Check if the subrequest exists
            if let Some(subs) = sub_names {
                return subs.contains(segments[1]);
            }
        }

        false
    }
}

fn gather_request_names(claim: &DeviceClaim) -> RequestNames {
    let mut names = BTreeMap::new();

    for request in &claim.requests {
        if request.first_available.is_empty() {
            names.insert(request.name.clone(), None);
        } else {
            let mut sub_names = BTreeSet::new();
            for sub_request in &request.first_available {
                sub_names.insert(sub_request.name.clone());
            }
            names.insert(request.name.clone(), Some(sub_names));
        }
    }

    RequestNames { names }
}

fn gather_shared_counter_names(
    shared_counters: &[CounterSet],
) -> BTreeMap<String, BTreeSet<String>> {
    let mut result = BTreeMap::new();

    for counter_set in shared_counters {
        let counter_names: BTreeSet<String> = counter_set.counters.keys().cloned().collect();
        result.insert(counter_set.name.clone(), counter_names);
    }

    result
}

// ============================================================================
// Generic Validation Helpers
// ============================================================================

/// Validates a slice with maximum size limit
fn validate_slice<T, F>(
    slice: &[T],
    max_size: usize,
    validate_item: F,
    fld_path: &Path,
) -> ErrorList
where
    F: Fn(&T, &Path) -> ErrorList,
{
    let mut all_errs = ErrorList::new();

    for (i, item) in slice.iter().enumerate() {
        all_errs.extend(validate_item(item, &fld_path.index(i)));
    }

    if max_size != (-1i32 as usize) && slice.len() > max_size {
        all_errs.push(too_many(fld_path, Some(slice.len()), max_size));
    }

    all_errs
}

/// Validates a slice ensuring uniqueness of elements
fn validate_set<T, F, K>(
    slice: &[T],
    max_size: usize,
    validate_item: F,
    item_key: K,
    fld_path: &Path,
    field_name: &str,
) -> ErrorList
where
    F: Fn(&T, &Path) -> ErrorList,
    K: Fn(&T) -> String,
{
    let mut all_errs = validate_slice(slice, max_size, validate_item, fld_path);

    let mut seen = BTreeSet::new();
    for (i, item) in slice.iter().enumerate() {
        let key = item_key(item);
        let child_path = if field_name.is_empty() {
            fld_path.index(i)
        } else {
            fld_path.index(i).child(field_name)
        };

        if seen.contains(&key) {
            all_errs.push(duplicate(&child_path, BadValue::String(key.clone())));
        } else {
            seen.insert(key);
        }
    }

    all_errs
}

/// Validates a map with maximum size and key length limits
fn validate_map<K, V, FK, FV>(
    map: &BTreeMap<K, V>,
    max_size: usize,
    truncate_key_len: usize,
    validate_key: FK,
    validate_item: FV,
    fld_path: &Path,
) -> ErrorList
where
    K: AsRef<str> + std::fmt::Display,
    FK: Fn(&str, &Path) -> ErrorList,
    FV: Fn(&V, &Path) -> ErrorList,
{
    let mut all_errs = ErrorList::new();

    if max_size != (-1i32 as usize) && map.len() > max_size {
        all_errs.push(too_many(fld_path, Some(map.len()), max_size));
    }

    for (key, value) in map {
        let key_str = key.as_ref();
        let truncated_key = truncate_if_too_long(key_str, truncate_key_len);
        let key_path = fld_path.key(&truncated_key);

        all_errs.extend(validate_key(key_str, &key_path));
        all_errs.extend(validate_item(value, &key_path));
    }

    all_errs
}

fn truncate_if_too_long(s: &str, max_len: usize) -> String {
    let max_len = if max_len < 16 { 16 } else { max_len };

    if s.len() <= max_len {
        return s.to_string();
    }

    let ellipsis = "...";
    let remaining = max_len - ellipsis.len();
    let first_half = (remaining + 1) / 2;
    let second_half = remaining / 2;

    format!(
        "{}{}{}",
        &s[..first_half],
        ellipsis,
        &s[s.len() - second_half..]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ObjectMeta;

    #[test]
    fn test_validate_resource_claim_valid() {
        let claim = internal::ResourceClaim {
            type_meta: Default::default(),
            metadata: ObjectMeta {
                name: Some("test-claim".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            spec: internal::ResourceClaimSpec {
                devices: DeviceClaim {
                    requests: vec![],
                    constraints: vec![],
                    config: vec![],
                },
            },
            status: None,
        };

        let errors = validate_resource_claim(&claim);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_device_class_valid() {
        let class = DeviceClass {
            type_meta: Default::default(),
            metadata: ObjectMeta {
                name: Some("test-class".to_string()),
                ..Default::default()
            },
            spec: DeviceClassSpec {
                selectors: vec![],
                config: vec![],
                extended_resource_name: None,
            },
        };

        let errors = validate_device_class(&class);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_validate_resource_slice_valid() {
        let slice = internal::ResourceSlice {
            type_meta: Default::default(),
            metadata: ObjectMeta {
                name: Some("test-slice".to_string()),
                ..Default::default()
            },
            spec: ResourceSliceSpec {
                driver: "test.driver.io".to_string(),
                pool: ResourcePool {
                    name: "test-pool".to_string(),
                    generation: 0,
                    resource_slice_count: 1,
                },
                node_name: Some("node-1".to_string()),
                node_selector: None,
                all_nodes: None,
                devices: vec![],
                per_device_node_selection: None,
                shared_counters: vec![],
            },
        };

        let errors = validate_resource_slice(&slice);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_request_names_has() {
        let claim = DeviceClaim {
            requests: vec![
                DeviceRequest {
                    name: "req1".to_string(),
                    exactly: None,
                    first_available: vec![DeviceSubRequest {
                        name: "sub1".to_string(),
                        device_class_name: "class1".to_string(),
                        selectors: vec![],
                        allocation_mode: DeviceAllocationMode::ExactCount,
                        count: 1,
                        tolerations: vec![],
                        capacity: None,
                    }],
                },
                DeviceRequest {
                    name: "req2".to_string(),
                    exactly: Some(ExactDeviceRequest {
                        device_class_name: "class1".to_string(),
                        selectors: vec![],
                        allocation_mode: DeviceAllocationMode::ExactCount,
                        count: 1,
                        admin_access: None,
                        tolerations: vec![],
                        capacity: None,
                    }),
                    first_available: vec![],
                },
            ],
            constraints: vec![],
            config: vec![],
        };

        let names = gather_request_names(&claim);

        assert!(names.has("req1"));
        assert!(names.has("req1/sub1"));
        assert!(names.has("req2"));
        assert!(!names.has("req2/sub1"));
        assert!(!names.has("req3"));
        assert!(!names.has("req1/sub2"));
    }
}
