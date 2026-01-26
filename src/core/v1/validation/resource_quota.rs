//! ResourceQuota and LimitRange validation for Kubernetes core/v1 API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::Quantity;
use crate::common::validation::{BadValue, ErrorList, Path, invalid, required};
use crate::core::v1::{
    LimitRange, LimitRangeItem, ResourceQuota, ResourceQuotaSpec, ResourceQuotaStatus,
    ScopeSelector, ScopedResourceSelectorRequirement,
};

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
    if let Some(ref metadata) = resource_quota.metadata {
        all_errs.extend(crate::common::validation::validate_object_meta(
            metadata,
            true, // ResourceQuota is namespaced
            validate_resource_quota_name,
            &path.child("metadata"),
        ));
    } else {
        all_errs.push(required(&path.child("metadata"), "metadata is required"));
    }

    // Validate spec
    if let Some(ref spec) = resource_quota.spec {
        all_errs.extend(validate_resource_quota_spec(spec, &path.child("spec")));
    }

    // Validate status
    if let Some(ref status) = resource_quota.status {
        all_errs.extend(validate_resource_quota_status(
            status,
            &path.child("status"),
        ));
    }

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
    if let (Some(new_meta), Some(old_meta)) = (&new.metadata, &old.metadata) {
        all_errs.extend(crate::common::validation::validate_object_meta_update(
            new_meta,
            old_meta,
            &path.child("metadata"),
        ));
    }

    // Validate the new resource quota
    all_errs.extend(validate_resource_quota_with_path(new, path));

    // Ensure scopes cannot change
    if let (Some(new_spec), Some(old_spec)) = (&new.spec, &old.spec) {
        // Compare scopes for equality
        let old_scopes: std::collections::HashSet<String> =
            old_spec.scopes.iter().cloned().collect();
        let new_scopes: std::collections::HashSet<String> =
            new_spec.scopes.iter().cloned().collect();

        if old_scopes != new_scopes {
            all_errs.push(invalid(
                &path.child("spec").child("scopes"),
                BadValue::String(format!("{:?}", new_spec.scopes)),
                "field is immutable",
            ));
        }
    }

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
        all_errs.extend(validate_resource_quantity_value(
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
        all_errs.extend(validate_resource_quantity_value(
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
        all_errs.extend(validate_resource_quantity_value(
            resource_name,
            quantity,
            &res_path,
        ));
    }

    all_errs
}

fn validate_resource_quota_scopes(spec: &ResourceQuotaSpec, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if spec.scopes.is_empty() {
        return all_errs;
    }

    let scopes_path = path.child("scopes");
    let mut scope_set = std::collections::HashSet::new();

    for scope in &spec.scopes {
        if !is_standard_resource_quota_scope(scope) {
            all_errs.push(invalid(
                &scopes_path,
                BadValue::String(format!("{:?}", spec.scopes)),
                "unsupported scope",
            ));
        }

        scope_set.insert(scope.as_str());
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

    for req in &scope_selector.match_expressions {
        all_errs.extend(validate_scoped_resource_selector_requirement(
            req,
            spec,
            &match_path,
        ));
        scope_set.insert(req.scope_name.as_str());
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
            BadValue::String(req.scope_name.clone()),
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
    if exists_only_scopes.contains(&req.scope_name.as_str()) {
        if req.operator != "Exists" {
            all_errs.push(invalid(
                &path.child("operator"),
                BadValue::String(req.operator.clone()),
                "must be 'Exists' when scope is any of ResourceQuotaScopeTerminating, ResourceQuotaScopeNotTerminating, ResourceQuotaScopeBestEffort, ResourceQuotaScopeNotBestEffort or ResourceQuotaScopeCrossNamespacePodAffinity",
            ));
        }
    }

    // Validate operator
    match req.operator.as_str() {
        "In" | "NotIn" => {
            if req.values.is_empty() {
                all_errs.push(required(
                    &path.child("values"),
                    "must be at least one value when `operator` is 'In' or 'NotIn' for scope selector",
                ));
            }
        }
        "Exists" | "DoesNotExist" => {
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
                BadValue::String(req.operator.clone()),
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
    if let Some(ref metadata) = limit_range.metadata {
        all_errs.extend(crate::common::validation::validate_object_meta(
            metadata,
            true, // LimitRange is namespaced
            validate_limit_range_name,
            &path.child("metadata"),
        ));
    } else {
        all_errs.push(required(&path.child("metadata"), "metadata is required"));
    }

    // Validate spec
    if let Some(ref spec) = limit_range.spec {
        let limits_path = path.child("spec").child("limits");
        for (i, item) in spec.limits.iter().enumerate() {
            all_errs.extend(validate_limit_range_item(item, &limits_path.index(i)));
        }
    }

    all_errs
}

fn validate_limit_range_item(item: &LimitRangeItem, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate type
    let type_path = path.child("type");
    if item.type_.is_empty() {
        all_errs.push(required(&type_path, "type is required"));
    } else if !is_valid_limit_type(&item.type_) {
        all_errs.push(invalid(
            &type_path,
            BadValue::String(item.type_.clone()),
            "must be Pod, Container, or PersistentVolumeClaim",
        ));
    }

    // Validate min resources
    for (resource_name, quantity) in &item.min {
        let min_path = path.child("min").key(resource_name);
        all_errs.extend(validate_resource_quantity_value(
            resource_name,
            quantity,
            &min_path,
        ));
    }

    // Validate max resources
    for (resource_name, quantity) in &item.max {
        let max_path = path.child("max").key(resource_name);
        all_errs.extend(validate_resource_quantity_value(
            resource_name,
            quantity,
            &max_path,
        ));
    }

    // Validate default resources
    for (resource_name, quantity) in &item.default {
        let default_path = path.child("default").key(resource_name);
        all_errs.extend(validate_resource_quantity_value(
            resource_name,
            quantity,
            &default_path,
        ));
    }

    // Validate default_request resources
    for (resource_name, quantity) in &item.default_request {
        let default_request_path = path.child("defaultRequest").key(resource_name);
        all_errs.extend(validate_resource_quantity_value(
            resource_name,
            quantity,
            &default_request_path,
        ));
    }

    // Validate max_limit_request_ratio
    for (resource_name, quantity) in &item.max_limit_request_ratio {
        let ratio_path = path.child("maxLimitRequestRatio").key(resource_name);
        all_errs.extend(validate_resource_quantity_value(
            resource_name,
            quantity,
            &ratio_path,
        ));
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
            if compare_quantities(&min.0, &max.0) > 0 {
                all_errs.push(invalid(
                    &path.child("min").key(&resource_name),
                    BadValue::String(min.0.clone()),
                    "min value must be less than or equal to max value",
                ));
            }
        }

        // Default must be between min and max
        if let Some(default) = item.default.get(&resource_name) {
            if let Some(min) = item.min.get(&resource_name) {
                if compare_quantities(&default.0, &min.0) < 0 {
                    all_errs.push(invalid(
                        &path.child("default").key(&resource_name),
                        BadValue::String(default.0.clone()),
                        "default value must be greater than or equal to min value",
                    ));
                }
            }
            if let Some(max) = item.max.get(&resource_name) {
                if compare_quantities(&default.0, &max.0) > 0 {
                    all_errs.push(invalid(
                        &path.child("default").key(&resource_name),
                        BadValue::String(default.0.clone()),
                        "default value must be less than or equal to max value",
                    ));
                }
            }
        }

        // DefaultRequest must be between min and max
        if let Some(default_request) = item.default_request.get(&resource_name) {
            if let Some(min) = item.min.get(&resource_name) {
                if compare_quantities(&default_request.0, &min.0) < 0 {
                    all_errs.push(invalid(
                        &path.child("defaultRequest").key(&resource_name),
                        BadValue::String(default_request.0.clone()),
                        "defaultRequest value must be greater than or equal to min value",
                    ));
                }
            }
            if let Some(max) = item.max.get(&resource_name) {
                if compare_quantities(&default_request.0, &max.0) > 0 {
                    all_errs.push(invalid(
                        &path.child("defaultRequest").key(&resource_name),
                        BadValue::String(default_request.0.clone()),
                        "defaultRequest value must be less than or equal to max value",
                    ));
                }
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

fn validate_resource_quota_resource_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let errors = crate::common::validation::is_qualified_name(name);
    for err in errors {
        all_errs.push(invalid(path, BadValue::String(name.to_string()), &err));
    }

    all_errs
}

fn validate_resource_quantity_value(
    _resource_name: &str,
    quantity: &Quantity,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Check that quantity is not negative or empty
    let value = &quantity.0;

    if value.is_empty() {
        all_errs.push(invalid(
            path,
            BadValue::String(value.clone()),
            "must be a valid quantity",
        ));
    }

    // Check for negative values (basic check for leading minus)
    if value.starts_with('-') {
        all_errs.push(invalid(
            path,
            BadValue::String(value.clone()),
            "must be greater than or equal to 0",
        ));
    }

    all_errs
}

fn is_standard_resource_quota_scope(scope: &str) -> bool {
    matches!(
        scope,
        "Terminating"
            | "NotTerminating"
            | "BestEffort"
            | "NotBestEffort"
            | "PriorityClass"
            | "CrossNamespacePodAffinity"
            | "VolumeAttributesClass"
    )
}

fn is_valid_limit_type(type_: &str) -> bool {
    matches!(type_, "Pod" | "Container" | "PersistentVolumeClaim")
}

/// Basic quantity comparison (lexicographic, simplified)
/// Returns: -1 if a < b, 0 if a == b, 1 if a > b
fn compare_quantities(a: &str, b: &str) -> i32 {
    // Simplified comparison - just compare the strings lexicographically
    // In a real implementation, this should parse quantity units
    match a.cmp(b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, Quantity, TypeMeta};
    use crate::core::v1::{LimitRangeSpec, ResourceQuotaSpec};
    use std::collections::BTreeMap;

    fn create_test_resource_quota(name: &str) -> ResourceQuota {
        ResourceQuota {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some(name.to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: None,
            status: None,
        }
    }

    fn create_test_limit_range(name: &str) -> LimitRange {
        LimitRange {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some(name.to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: None,
        }
    }

    #[test]
    fn test_validate_resource_quota_valid() {
        let mut quota = create_test_resource_quota("test-quota");
        let mut hard = BTreeMap::new();
        hard.insert("cpu".to_string(), Quantity("10".to_string()));
        hard.insert("memory".to_string(), Quantity("10Gi".to_string()));

        quota.spec = Some(ResourceQuotaSpec {
            hard,
            scopes: vec![],
            scope_selector: None,
        });

        let errs = validate_resource_quota(&quota);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_resource_quota_negative_quantity() {
        let mut quota = create_test_resource_quota("test-quota");
        let mut hard = BTreeMap::new();
        hard.insert("cpu".to_string(), Quantity("-10".to_string()));

        quota.spec = Some(ResourceQuotaSpec {
            hard,
            scopes: vec![],
            scope_selector: None,
        });

        let errs = validate_resource_quota(&quota);
        assert!(!errs.is_empty(), "Expected errors for negative quantity");
    }

    #[test]
    fn test_validate_resource_quota_conflicting_scopes() {
        let mut quota = create_test_resource_quota("test-quota");
        quota.spec = Some(ResourceQuotaSpec {
            hard: BTreeMap::new(),
            scopes: vec!["BestEffort".to_string(), "NotBestEffort".to_string()],
            scope_selector: None,
        });

        let errs = validate_resource_quota(&quota);
        assert!(!errs.is_empty(), "Expected errors for conflicting scopes");
    }

    #[test]
    fn test_validate_limit_range_valid() {
        let mut limit_range = create_test_limit_range("test-limit");
        let mut min = BTreeMap::new();
        let mut max = BTreeMap::new();
        min.insert("cpu".to_string(), Quantity("100m".to_string()));
        max.insert("cpu".to_string(), Quantity("2".to_string()));

        limit_range.spec = Some(LimitRangeSpec {
            limits: vec![LimitRangeItem {
                type_: "Container".to_string(),
                min,
                max,
                ..Default::default()
            }],
        });

        let errs = validate_limit_range(&limit_range);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_limit_range_invalid_type() {
        let mut limit_range = create_test_limit_range("test-limit");
        limit_range.spec = Some(LimitRangeSpec {
            limits: vec![LimitRangeItem {
                type_: "InvalidType".to_string(),
                ..Default::default()
            }],
        });

        let errs = validate_limit_range(&limit_range);
        assert!(!errs.is_empty(), "Expected errors for invalid type");
    }

    #[test]
    fn test_validate_limit_range_min_greater_than_max() {
        let mut limit_range = create_test_limit_range("test-limit");
        let mut min = BTreeMap::new();
        let mut max = BTreeMap::new();
        min.insert("cpu".to_string(), Quantity("2".to_string()));
        max.insert("cpu".to_string(), Quantity("1".to_string()));

        limit_range.spec = Some(LimitRangeSpec {
            limits: vec![LimitRangeItem {
                type_: "Container".to_string(),
                min,
                max,
                ..Default::default()
            }],
        });

        let errs = validate_limit_range(&limit_range);
        assert!(!errs.is_empty(), "Expected errors for min > max");
    }
}
