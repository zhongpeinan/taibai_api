//! ResourceQuota and LimitRange validation for Kubernetes core internal API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::Quantity;
use crate::common::validation::{BadValue, ErrorList, Path, forbidden, invalid, required};
use crate::core::internal::quota::scope_selector_operator;
use crate::core::internal::{
    LimitRange, LimitRangeItem, LimitType, ResourceQuota, ResourceQuotaScope, ResourceQuotaSpec,
    ResourceQuotaStatus, ScopeSelector, ScopeSelectorOperator, ScopedResourceSelectorRequirement,
    limit_type, resource_quota_scope,
};
use std::collections::HashSet;
use std::sync::LazyLock;

const FIELD_IMMUTABLE_ERROR_MSG: &str = "field is immutable";
const IS_INVALID_QUOTA_RESOURCE: &str = "must be a standard resource for quota";
const IS_NEGATIVE_ERROR_MSG: &str = "must be greater than or equal to 0";
const IS_NOT_INTEGER_ERROR_MSG: &str = "must be an integer";

const RESOURCE_CPU: &str = "cpu";
const RESOURCE_MEMORY: &str = "memory";
const RESOURCE_EPHEMERAL_STORAGE: &str = "ephemeral-storage";
const RESOURCE_STORAGE: &str = "storage";
const RESOURCE_PODS: &str = "pods";
const RESOURCE_QUOTAS: &str = "resourcequotas";
const RESOURCE_SERVICES: &str = "services";
const RESOURCE_REPLICATION_CONTROLLERS: &str = "replicationcontrollers";
const RESOURCE_SECRETS: &str = "secrets";
const RESOURCE_PVCS: &str = "persistentvolumeclaims";
const RESOURCE_CONFIGMAPS: &str = "configmaps";
const RESOURCE_SERVICES_NODEPORTS: &str = "services.nodeports";
const RESOURCE_SERVICES_LOADBALANCERS: &str = "services.loadbalancers";

const RESOURCE_REQUESTS_PREFIX: &str = "requests.";
const RESOURCE_REQUESTS_CPU: &str = "requests.cpu";
const RESOURCE_REQUESTS_MEMORY: &str = "requests.memory";
const RESOURCE_REQUESTS_STORAGE: &str = "requests.storage";
const RESOURCE_REQUESTS_EPHEMERAL_STORAGE: &str = "requests.ephemeral-storage";
const RESOURCE_LIMITS_CPU: &str = "limits.cpu";
const RESOURCE_LIMITS_MEMORY: &str = "limits.memory";
const RESOURCE_LIMITS_EPHEMERAL_STORAGE: &str = "limits.ephemeral-storage";
const RESOURCE_HUGEPAGES_PREFIX: &str = "hugepages-";
const RESOURCE_REQUESTS_HUGEPAGES_PREFIX: &str = "requests.hugepages-";

static STANDARD_QUOTA_RESOURCES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        RESOURCE_CPU,
        RESOURCE_MEMORY,
        RESOURCE_EPHEMERAL_STORAGE,
        RESOURCE_REQUESTS_CPU,
        RESOURCE_REQUESTS_MEMORY,
        RESOURCE_REQUESTS_STORAGE,
        RESOURCE_REQUESTS_EPHEMERAL_STORAGE,
        RESOURCE_LIMITS_CPU,
        RESOURCE_LIMITS_MEMORY,
        RESOURCE_LIMITS_EPHEMERAL_STORAGE,
        RESOURCE_PODS,
        RESOURCE_QUOTAS,
        RESOURCE_SERVICES,
        RESOURCE_REPLICATION_CONTROLLERS,
        RESOURCE_SECRETS,
        RESOURCE_PVCS,
        RESOURCE_CONFIGMAPS,
        RESOURCE_SERVICES_NODEPORTS,
        RESOURCE_SERVICES_LOADBALANCERS,
    ])
});

static STANDARD_RESOURCES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        RESOURCE_CPU,
        RESOURCE_MEMORY,
        RESOURCE_EPHEMERAL_STORAGE,
        RESOURCE_STORAGE,
        RESOURCE_REQUESTS_CPU,
        RESOURCE_REQUESTS_MEMORY,
        RESOURCE_REQUESTS_STORAGE,
        RESOURCE_REQUESTS_EPHEMERAL_STORAGE,
        RESOURCE_LIMITS_CPU,
        RESOURCE_LIMITS_MEMORY,
        RESOURCE_LIMITS_EPHEMERAL_STORAGE,
        RESOURCE_PODS,
        RESOURCE_QUOTAS,
        RESOURCE_SERVICES,
        RESOURCE_REPLICATION_CONTROLLERS,
        RESOURCE_SECRETS,
        RESOURCE_PVCS,
        RESOURCE_CONFIGMAPS,
        RESOURCE_SERVICES_NODEPORTS,
        RESOURCE_SERVICES_LOADBALANCERS,
    ])
});

static STANDARD_CONTAINER_RESOURCES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from([RESOURCE_CPU, RESOURCE_MEMORY, RESOURCE_EPHEMERAL_STORAGE]));

static INTEGER_RESOURCES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        RESOURCE_PODS,
        RESOURCE_QUOTAS,
        RESOURCE_SERVICES,
        RESOURCE_REPLICATION_CONTROLLERS,
        RESOURCE_SECRETS,
        RESOURCE_CONFIGMAPS,
        RESOURCE_PVCS,
        RESOURCE_SERVICES_NODEPORTS,
        RESOURCE_SERVICES_LOADBALANCERS,
    ])
});

// ============================================================================
// ResourceQuota Validation
// ============================================================================

/// Validates a ResourceQuota
pub fn validate_resource_quota(resource_quota: &ResourceQuota) -> ErrorList {
    validate_resource_quota_with_path(resource_quota, &Path::nil())
}

fn validate_resource_quota_with_path(resource_quota: &ResourceQuota, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (ResourceQuota is namespaced)
    all_errs.extend(crate::common::validation::validate_object_meta(
        &resource_quota.metadata,
        true, // ResourceQuota is namespaced
        validate_resource_quota_name,
        &path.child("metadata"),
    ));

    // Validate spec
    if let Some(ref spec) = resource_quota.spec {
        all_errs.extend(validate_resource_quota_spec(spec, &path.child("spec")));
    }

    // Validate status
    all_errs.extend(validate_resource_quota_status(
        &resource_quota.status,
        &path.child("status"),
    ));

    all_errs
}

/// Validates ResourceQuota update
pub fn validate_resource_quota_update(new: &ResourceQuota, old: &ResourceQuota) -> ErrorList {
    validate_resource_quota_update_with_path(new, old, &Path::nil())
}

fn validate_resource_quota_update_with_path(
    new: &ResourceQuota,
    old: &ResourceQuota,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata update
    all_errs.extend(crate::common::validation::validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &path.child("metadata"),
    ));

    // Validate the new resource quota
    all_errs.extend(validate_resource_quota_with_path(new, path));

    // Ensure scopes cannot change
    if let (Some(new_spec), Some(old_spec)) = (&new.spec, &old.spec) {
        // Compare scopes for equality
        let old_scopes: std::collections::HashSet<&str> = old_spec
            .scopes
            .iter()
            .map(resource_quota_scope_to_str)
            .collect();
        let new_scopes: std::collections::HashSet<&str> = new_spec
            .scopes
            .iter()
            .map(resource_quota_scope_to_str)
            .collect();

        if old_scopes != new_scopes {
            all_errs.push(invalid(
                &path.child("spec").child("scopes"),
                BadValue::String(format!(
                    "{:?}",
                    new_spec
                        .scopes
                        .iter()
                        .map(resource_quota_scope_to_str)
                        .collect::<Vec<_>>()
                )),
                FIELD_IMMUTABLE_ERROR_MSG,
            ));
        }
    }

    all_errs
}

/// Validates ResourceQuota status update
///
/// Corresponds to [Kubernetes ValidateResourceQuotaStatusUpdate](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/validation/validation.go#L7842)
pub fn validate_resource_quota_status_update(
    new: &ResourceQuota,
    old: &ResourceQuota,
) -> ErrorList {
    validate_resource_quota_status_update_with_path(new, old, &Path::nil())
}

fn validate_resource_quota_status_update_with_path(
    new: &ResourceQuota,
    old: &ResourceQuota,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata update
    all_errs.extend(crate::common::validation::validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &path.child("metadata"),
    ));

    // ResourceVersion is required for status updates
    if new
        .metadata
        .resource_version
        .as_ref()
        .map_or(true, |v| v.is_empty())
    {
        all_errs.push(required(&path.child("resourceVersion"), ""));
    }

    // Validate status.hard and status.used resources
    all_errs.extend(validate_resource_quota_status(
        &new.status,
        &path.child("status"),
    ));

    all_errs
}

fn validate_resource_quota_spec(spec: &ResourceQuotaSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate hard resources
    let hard_path = path.child("hard");
    for (resource_name, quantity) in &spec.hard {
        let res_path = hard_path.key(resource_name);
        all_errs.extend(validate_resource_quota_resource_name(
            resource_name,
            &res_path,
        ));
        all_errs.extend(validate_resource_quota_quantity_value(
            resource_name,
            quantity,
            &res_path,
        ));
    }

    // Validate scopes
    all_errs.extend(validate_resource_quota_scopes(spec, path));

    // Validate scope selector
    if let Some(ref scope_selector) = spec.scope_selector {
        all_errs.extend(validate_scope_selector(spec, scope_selector, path));
    }

    all_errs
}

fn validate_resource_quota_status(status: &ResourceQuotaStatus, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate hard resources
    let hard_path = path.child("hard");
    for (resource_name, quantity) in &status.hard {
        let res_path = hard_path.key(resource_name);
        all_errs.extend(validate_resource_quota_resource_name(
            resource_name,
            &res_path,
        ));
        all_errs.extend(validate_resource_quota_quantity_value(
            resource_name,
            quantity,
            &res_path,
        ));
    }

    // Validate used resources
    let used_path = path.child("used");
    for (resource_name, quantity) in &status.used {
        let res_path = used_path.key(resource_name);
        all_errs.extend(validate_resource_quota_resource_name(
            resource_name,
            &res_path,
        ));
        all_errs.extend(validate_resource_quota_quantity_value(
            resource_name,
            quantity,
            &res_path,
        ));
    }

    all_errs
}

fn validate_resource_quota_scopes(spec: &ResourceQuotaSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let scopes_path = path.child("scopes");
    let mut scope_set = std::collections::HashSet::new();

    for scope in &spec.scopes {
        if !is_standard_resource_quota_scope(scope) {
            all_errs.push(invalid(
                &scopes_path,
                BadValue::String(format!(
                    "{:?}",
                    spec.scopes
                        .iter()
                        .map(resource_quota_scope_to_str)
                        .collect::<Vec<_>>()
                )),
                "unsupported scope",
            ));
        }

        scope_set.insert(resource_quota_scope_to_str(scope));
    }

    // Check for conflicting scope pairs
    let conflicting_pairs = vec![
        ("BestEffort", "NotBestEffort"),
        ("Terminating", "NotTerminating"),
    ];

    for (scope1, scope2) in conflicting_pairs {
        if scope_set.contains(scope1) && scope_set.contains(scope2) {
            all_errs.push(invalid(
                &scopes_path,
                BadValue::String(format!("{:?}", spec.scopes)),
                "conflicting scopes",
            ));
        }
    }

    // Validate resources against scopes
    for scope in &spec.scopes {
        for (resource_name, quantity) in &spec.hard {
            if !is_resource_quota_scope_valid_for_resource(scope, resource_name) {
                all_errs.push(invalid(
                    &path.child("hard").key(resource_name),
                    BadValue::String(quantity.to_string()),
                    "resource does not match the provided scopes",
                ));
            }
        }
    }

    all_errs
}

fn validate_scope_selector(
    spec: &ResourceQuotaSpec,
    scope_selector: &ScopeSelector,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let sel_path = path.child("scopeSelector");
    let match_path = sel_path.child("matchExpressions");
    let mut scope_set = std::collections::HashSet::new();

    for (idx, req) in scope_selector.match_expressions.iter().enumerate() {
        all_errs.extend(validate_scoped_resource_selector_requirement(
            req,
            spec,
            &match_path.index(idx),
        ));
        scope_set.insert(resource_quota_scope_to_str(&req.scope_name));
    }

    // Check for conflicting scope pairs
    let conflicting_pairs = vec![
        ("BestEffort", "NotBestEffort"),
        ("Terminating", "NotTerminating"),
    ];

    for (scope1, scope2) in conflicting_pairs {
        if scope_set.contains(scope1) && scope_set.contains(scope2) {
            all_errs.push(invalid(
                &match_path,
                BadValue::String(format!("conflicting scopes: {} and {}", scope1, scope2)),
                "conflicting scopes",
            ));
        }
    }

    all_errs
}

fn validate_scoped_resource_selector_requirement(
    req: &ScopedResourceSelectorRequirement,
    _spec: &ResourceQuotaSpec,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let scope_name_path = path.child("scopeName");
    if !is_standard_resource_quota_scope(&req.scope_name) {
        all_errs.push(invalid(
            &scope_name_path,
            BadValue::String(resource_quota_scope_to_str(&req.scope_name).to_string()),
            "unsupported scope",
        ));
    }

    // Certain scopes must use Exists operator
    let exists_only_scopes = [
        "BestEffort",
        "NotBestEffort",
        "Terminating",
        "NotTerminating",
        "CrossNamespacePodAffinity",
    ];
    if exists_only_scopes.contains(&resource_quota_scope_to_str(&req.scope_name)) {
        if !matches!(req.operator, Some(ScopeSelectorOperator::Exists)) {
            all_errs.push(invalid(
                &path.child("operator"),
                BadValue::String(scope_selector_operator_to_str(req.operator.as_ref()).to_string()),
                "must be 'Exists' when scope is any of ResourceQuotaScopeTerminating, ResourceQuotaScopeNotTerminating, ResourceQuotaScopeBestEffort, ResourceQuotaScopeNotBestEffort or ResourceQuotaScopeCrossNamespacePodAffinity",
            ));
        }
    }

    // Validate operator
    match req.operator {
        Some(ScopeSelectorOperator::In) | Some(ScopeSelectorOperator::NotIn) => {
            if req.values.is_empty() {
                all_errs.push(required(
                    &path.child("values"),
                    "must be at least one value when `operator` is 'In' or 'NotIn' for scope selector",
                ));
            }
        }
        Some(ScopeSelectorOperator::Exists) | Some(ScopeSelectorOperator::DoesNotExist) => {
            if !req.values.is_empty() {
                all_errs.push(invalid(
                    &path.child("values"),
                    BadValue::String(format!("{:?}", req.values)),
                    "must be no value when `operator` is 'Exist' or 'DoesNotExist' for scope selector",
                ));
            }
        }
        _ => {
            all_errs.push(invalid(
                &path.child("operator"),
                BadValue::String(scope_selector_operator_to_str(req.operator.as_ref()).to_string()),
                "not a valid selector operator",
            ));
        }
    }

    all_errs
}

// ============================================================================
// LimitRange Validation
// ============================================================================

/// Validates a LimitRange
pub fn validate_limit_range(limit_range: &LimitRange) -> ErrorList {
    validate_limit_range_with_path(limit_range, &Path::nil())
}

fn validate_limit_range_with_path(limit_range: &LimitRange, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (LimitRange is namespaced)
    all_errs.extend(crate::common::validation::validate_object_meta(
        &limit_range.metadata,
        true, // LimitRange is namespaced
        validate_limit_range_name,
        &path.child("metadata"),
    ));

    // Validate spec
    if let Some(ref spec) = limit_range.spec {
        let limits_path = path.child("spec").child("limits");
        let mut seen_types = HashSet::new();
        for (i, item) in spec.limits.iter().enumerate() {
            all_errs.extend(validate_limit_range_item(item, &limits_path.index(i)));
            if !seen_types.insert(limit_type_to_str(&item.r#type)) {
                all_errs.push(crate::common::validation::duplicate(
                    &limits_path.index(i).child("type"),
                    BadValue::String(limit_type_to_str(&item.r#type).to_string()),
                ));
            }
        }
    }

    all_errs
}

fn validate_limit_range_item(item: &LimitRangeItem, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate type
    let type_path = path.child("type");
    all_errs.extend(validate_limit_range_type_name(&item.r#type, &type_path));

    // Validate min resources
    for (resource_name, quantity) in &item.min {
        let min_path = path.child("min").key(resource_name);
        all_errs.extend(validate_limit_range_resource_name(
            &item.r#type,
            resource_name,
            &min_path,
        ));
        all_errs.extend(validate_resource_quota_quantity_value(
            resource_name,
            quantity,
            &min_path,
        ));
    }

    // Validate max resources
    for (resource_name, quantity) in &item.max {
        let max_path = path.child("max").key(resource_name);
        all_errs.extend(validate_limit_range_resource_name(
            &item.r#type,
            resource_name,
            &max_path,
        ));
        all_errs.extend(validate_resource_quota_quantity_value(
            resource_name,
            quantity,
            &max_path,
        ));
    }

    if matches!(item.r#type, LimitType::Pod) {
        if !item.default.is_empty() {
            all_errs.push(forbidden(
                &path.child("default"),
                "may not be specified when `type` is 'Pod'",
            ));
        }
        if !item.default_request.is_empty() {
            all_errs.push(forbidden(
                &path.child("defaultRequest"),
                "may not be specified when `type` is 'Pod'",
            ));
        }
    } else {
        for (resource_name, quantity) in &item.default {
            let default_path = path.child("default").key(resource_name);
            all_errs.extend(validate_limit_range_resource_name(
                &item.r#type,
                resource_name,
                &default_path,
            ));
            all_errs.extend(validate_resource_quota_quantity_value(
                resource_name,
                quantity,
                &default_path,
            ));
        }

        for (resource_name, quantity) in &item.default_request {
            let default_request_path = path.child("defaultRequest").key(resource_name);
            all_errs.extend(validate_limit_range_resource_name(
                &item.r#type,
                resource_name,
                &default_request_path,
            ));
            all_errs.extend(validate_resource_quota_quantity_value(
                resource_name,
                quantity,
                &default_request_path,
            ));
        }
    }

    // Validate max_limit_request_ratio
    for (resource_name, quantity) in &item.max_limit_request_ratio {
        let ratio_path = path.child("maxLimitRequestRatio").key(resource_name);
        all_errs.extend(validate_limit_range_resource_name(
            &item.r#type,
            resource_name,
            &ratio_path,
        ));
        all_errs.extend(validate_resource_quota_quantity_value(
            resource_name,
            quantity,
            &ratio_path,
        ));
    }

    if matches!(item.r#type, LimitType::PersistentVolumeClaim) {
        let has_min_storage = item.min.contains_key(RESOURCE_STORAGE);
        let has_max_storage = item.max.contains_key(RESOURCE_STORAGE);
        if !has_min_storage && !has_max_storage {
            all_errs.push(required(
                &path.child("limits"),
                "either minimum or maximum storage value is required, but neither was provided",
            ));
        }
    }

    // Validate relationships: min <= default <= default_request <= max
    // For each resource, check the ordering
    let all_resources: std::collections::HashSet<String> = item
        .min
        .keys()
        .chain(item.max.keys())
        .chain(item.default.keys())
        .chain(item.default_request.keys())
        .cloned()
        .collect();

    for resource_name in all_resources {
        // Min <= Max
        if let (Some(min), Some(max)) = (item.min.get(&resource_name), item.max.get(&resource_name))
        {
            if min.cmp(max).unwrap_or(std::cmp::Ordering::Equal).is_gt() {
                all_errs.push(invalid(
                    &path.child("min").key(&resource_name),
                    BadValue::String(min.to_string()),
                    "min value must be less than or equal to max value",
                ));
            }
        }

        // Default must be between min and max
        if let Some(default) = item.default.get(&resource_name) {
            if let Some(min) = item.min.get(&resource_name) {
                if default
                    .cmp(min)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .is_lt()
                {
                    all_errs.push(invalid(
                        &path.child("default").key(&resource_name),
                        BadValue::String(default.to_string()),
                        "default value must be greater than or equal to min value",
                    ));
                }
            }
            if let Some(max) = item.max.get(&resource_name) {
                if default
                    .cmp(max)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .is_gt()
                {
                    all_errs.push(invalid(
                        &path.child("default").key(&resource_name),
                        BadValue::String(default.to_string()),
                        "default value must be less than or equal to max value",
                    ));
                }
            }
        }

        // DefaultRequest must be between min and max
        if let Some(default_request) = item.default_request.get(&resource_name) {
            if let Some(min) = item.min.get(&resource_name) {
                if default_request
                    .cmp(min)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .is_lt()
                {
                    all_errs.push(invalid(
                        &path.child("defaultRequest").key(&resource_name),
                        BadValue::String(default_request.to_string()),
                        "defaultRequest value must be greater than or equal to min value",
                    ));
                }
            }
            if let Some(max) = item.max.get(&resource_name) {
                if default_request
                    .cmp(max)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .is_gt()
                {
                    all_errs.push(invalid(
                        &path.child("defaultRequest").key(&resource_name),
                        BadValue::String(default_request.to_string()),
                        "defaultRequest value must be less than or equal to max value",
                    ));
                }
            }
        }

        if let (Some(default_request), Some(default)) = (
            item.default_request.get(&resource_name),
            item.default.get(&resource_name),
        ) {
            if default_request
                .cmp(default)
                .unwrap_or(std::cmp::Ordering::Equal)
                .is_gt()
            {
                all_errs.push(invalid(
                    &path.child("defaultRequest").key(&resource_name),
                    BadValue::String(default_request.to_string()),
                    "defaultRequest value must be less than or equal to default value",
                ));
            }
        }

        if let Some(max_ratio) = item.max_limit_request_ratio.get(&resource_name) {
            let ratio = max_ratio.to_f64().unwrap_or(-1.0);
            if ratio < 1.0 {
                all_errs.push(invalid(
                    &path.child("maxLimitRequestRatio").key(&resource_name),
                    BadValue::String(max_ratio.to_string()),
                    "ratio must be greater than or equal to 1",
                ));
            }
            if let (Some(min), Some(max)) =
                (item.min.get(&resource_name), item.max.get(&resource_name))
            {
                if let (Ok(min_val), Ok(max_val)) = (min.to_f64(), max.to_f64()) {
                    if min_val > 0.0 {
                        let max_ratio_allowed = max_val / min_val;
                        if ratio > max_ratio_allowed {
                            all_errs.push(invalid(
                                &path.child("maxLimitRequestRatio").key(&resource_name),
                                BadValue::String(max_ratio.to_string()),
                                &format!(
                                    "ratio {} is greater than max/min = {}",
                                    max_ratio, max_ratio_allowed
                                ),
                            ));
                        }
                    }
                }
            }
        }

        if let (Some(default), Some(default_request)) = (
            item.default.get(&resource_name),
            item.default_request.get(&resource_name),
        ) {
            if !is_overcommit_allowed(&resource_name)
                && default
                    .cmp(default_request)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    != std::cmp::Ordering::Equal
            {
                all_errs.push(invalid(
                    &path.child("defaultRequest").key(&resource_name),
                    BadValue::String(default_request.to_string()),
                    &format!(
                        "default value {} must equal defaultRequest value {}",
                        default, default_request
                    ),
                ));
            }
        }
    }

    all_errs
}

// ============================================================================
// Helper Functions
// ============================================================================

fn validate_resource_quota_name(name: &str, _prefix: bool) -> Vec<String> {
    crate::common::validation::is_dns1123_subdomain(name)
}

fn validate_limit_range_name(name: &str, _prefix: bool) -> Vec<String> {
    crate::common::validation::is_dns1123_subdomain(name)
}

fn resource_quota_scope_to_str(value: &ResourceQuotaScope) -> &'static str {
    match value {
        ResourceQuotaScope::Terminating => resource_quota_scope::TERMINATING,
        ResourceQuotaScope::NotTerminating => resource_quota_scope::NOT_TERMINATING,
        ResourceQuotaScope::BestEffort => resource_quota_scope::BEST_EFFORT,
        ResourceQuotaScope::NotBestEffort => resource_quota_scope::NOT_BEST_EFFORT,
        ResourceQuotaScope::PriorityClass => resource_quota_scope::PRIORITY_CLASS,
        ResourceQuotaScope::CrossNamespacePodAffinity => {
            resource_quota_scope::CROSS_NAMESPACE_POD_AFFINITY
        }
        ResourceQuotaScope::VolumeAttributesClass => resource_quota_scope::VOLUME_ATTRIBUTES_CLASS,
    }
}

fn limit_type_to_str(value: &LimitType) -> &'static str {
    match value {
        LimitType::Pod => limit_type::POD,
        LimitType::Container => limit_type::CONTAINER,
        LimitType::PersistentVolumeClaim => limit_type::PERSISTENT_VOLUME_CLAIM,
    }
}

fn scope_selector_operator_to_str(value: Option<&ScopeSelectorOperator>) -> &'static str {
    match value {
        Some(ScopeSelectorOperator::In) => scope_selector_operator::IN,
        Some(ScopeSelectorOperator::NotIn) => scope_selector_operator::NOT_IN,
        Some(ScopeSelectorOperator::Exists) => scope_selector_operator::EXISTS,
        Some(ScopeSelectorOperator::DoesNotExist) => scope_selector_operator::DOES_NOT_EXIST,
        None => "",
    }
}

fn validate_resource_quota_resource_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for err in crate::common::validation::is_qualified_name(name) {
        all_errs.push(invalid(path, BadValue::String(name.to_string()), &err));
    }

    if !name.contains('/') && !is_standard_quota_resource_name(name) {
        all_errs.push(invalid(
            path,
            BadValue::String(name.to_string()),
            IS_INVALID_QUOTA_RESOURCE,
        ));
    }
    all_errs
}

fn validate_resource_quota_quantity_value(
    resource_name: &str,
    quantity: &Quantity,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if quantity.as_str().is_empty() {
        all_errs.push(invalid(
            path,
            BadValue::String(quantity.to_string()),
            "must be a valid quantity",
        ));
        return all_errs;
    }

    if Quantity::from_str_validated(quantity.as_str()).is_err() {
        all_errs.push(invalid(
            path,
            BadValue::String(quantity.to_string()),
            "must be a valid quantity",
        ));
        return all_errs;
    }

    if quantity.sign().unwrap_or(std::cmp::Ordering::Equal).is_lt() {
        all_errs.push(invalid(
            path,
            BadValue::String(quantity.to_string()),
            IS_NEGATIVE_ERROR_MSG,
        ));
    }

    if is_integer_resource_name(resource_name) {
        if quantity.as_i64().is_err() {
            all_errs.push(invalid(
                path,
                BadValue::String(quantity.to_string()),
                IS_NOT_INTEGER_ERROR_MSG,
            ));
        }
    }

    all_errs
}

fn is_standard_resource_quota_scope(scope: &ResourceQuotaScope) -> bool {
    matches!(
        scope,
        ResourceQuotaScope::Terminating
            | ResourceQuotaScope::NotTerminating
            | ResourceQuotaScope::BestEffort
            | ResourceQuotaScope::NotBestEffort
            | ResourceQuotaScope::PriorityClass
            | ResourceQuotaScope::CrossNamespacePodAffinity
            | ResourceQuotaScope::VolumeAttributesClass
    )
}

fn validate_resource_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for err in crate::common::validation::is_qualified_name(name) {
        all_errs.push(invalid(path, BadValue::String(name.to_string()), &err));
    }

    if !name.contains('/') && !is_standard_resource_name(name) {
        all_errs.push(invalid(
            path,
            BadValue::String(name.to_string()),
            "must be a standard resource type or fully qualified",
        ));
    }
    all_errs
}

fn validate_container_resource_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = validate_resource_name(name, path);
    if !name.contains('/') {
        if !is_standard_container_resource_name(name) {
            all_errs.push(invalid(
                path,
                BadValue::String(name.to_string()),
                "must be a standard resource for containers",
            ));
        }
    } else if !is_native_resource(name) && !is_extended_resource_name(name) {
        all_errs.push(invalid(
            path,
            BadValue::String(name.to_string()),
            "doesn't follow extended resource name standard",
        ));
    }

    all_errs
}

fn validate_limit_range_type_name(value: &LimitType, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let value_str = limit_type_to_str(value);
    for err in crate::common::validation::is_qualified_name(value_str) {
        all_errs.push(invalid(path, BadValue::String(value_str.to_string()), &err));
    }

    if !value_str.contains('/')
        && !matches!(
            value,
            LimitType::Pod | LimitType::Container | LimitType::PersistentVolumeClaim
        )
    {
        all_errs.push(invalid(
            path,
            BadValue::String(value_str.to_string()),
            "must be a standard limit type or fully qualified",
        ));
    }
    all_errs
}

fn validate_limit_range_resource_name(
    limit_type: &LimitType,
    name: &str,
    path: &Path,
) -> ErrorList {
    match limit_type {
        LimitType::Pod | LimitType::Container => validate_container_resource_name(name, path),
        _ => validate_resource_name(name, path),
    }
}

fn is_resource_quota_scope_valid_for_resource(scope: &ResourceQuotaScope, resource: &str) -> bool {
    match scope {
        ResourceQuotaScope::Terminating
        | ResourceQuotaScope::NotTerminating
        | ResourceQuotaScope::NotBestEffort
        | ResourceQuotaScope::PriorityClass
        | ResourceQuotaScope::CrossNamespacePodAffinity => {
            is_pod_object_count_quota_resource(resource) || is_pod_compute_quota_resource(resource)
        }
        ResourceQuotaScope::BestEffort => is_pod_object_count_quota_resource(resource),
        ResourceQuotaScope::VolumeAttributesClass => {
            is_pvc_object_count_quota_resource(resource) || is_pvc_storage_quota_resource(resource)
        }
    }
}

fn is_pod_object_count_quota_resource(resource: &str) -> bool {
    resource == RESOURCE_PODS
}

fn is_pod_compute_quota_resource(resource: &str) -> bool {
    matches!(
        resource,
        RESOURCE_CPU
            | RESOURCE_MEMORY
            | RESOURCE_LIMITS_CPU
            | RESOURCE_LIMITS_MEMORY
            | RESOURCE_REQUESTS_CPU
            | RESOURCE_REQUESTS_MEMORY
    )
}

fn is_pvc_object_count_quota_resource(resource: &str) -> bool {
    resource == RESOURCE_PVCS
}

fn is_pvc_storage_quota_resource(resource: &str) -> bool {
    resource == RESOURCE_REQUESTS_STORAGE
}

fn is_standard_quota_resource_name(name: &str) -> bool {
    STANDARD_QUOTA_RESOURCES.contains(name) || is_quota_hugepage_resource_name(name)
}

fn is_quota_hugepage_resource_name(name: &str) -> bool {
    name.starts_with(RESOURCE_HUGEPAGES_PREFIX)
        || name.starts_with(RESOURCE_REQUESTS_HUGEPAGES_PREFIX)
}

fn is_standard_resource_name(name: &str) -> bool {
    STANDARD_RESOURCES.contains(name) || is_quota_hugepage_resource_name(name)
}

fn is_standard_container_resource_name(name: &str) -> bool {
    STANDARD_CONTAINER_RESOURCES.contains(name) || name.starts_with(RESOURCE_HUGEPAGES_PREFIX)
}

fn is_native_resource(name: &str) -> bool {
    !name.contains('/') || name.contains("kubernetes.io/")
}

fn is_extended_resource_name(name: &str) -> bool {
    if is_native_resource(name) || name.starts_with(RESOURCE_REQUESTS_PREFIX) {
        return false;
    }
    let name_for_quota = format!("{}{}", RESOURCE_REQUESTS_PREFIX, name);
    crate::common::validation::is_qualified_name(&name_for_quota).is_empty()
}

fn is_integer_resource_name(name: &str) -> bool {
    INTEGER_RESOURCES.contains(name) || is_extended_resource_name(name)
}

fn is_overcommit_allowed(name: &str) -> bool {
    is_native_resource(name) && !name.starts_with(RESOURCE_HUGEPAGES_PREFIX)
}
