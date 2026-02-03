use crate::common::ObjectMeta;
use crate::common::validation::{
    ErrorList, Path, required, validate_object_meta, validate_object_meta_update,
};
use crate::rbac::internal::ClusterRole;

use super::{validate_label_selector, validate_policy_rule, validate_rbac_name};

#[derive(Clone, Copy, Debug, Default)]
pub struct ClusterRoleValidationOptions {
    pub allow_invalid_label_value_in_selector: bool,
}

pub fn validate_cluster_role(role: &ClusterRole, opts: ClusterRoleValidationOptions) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = ObjectMeta::default();
    let meta = role.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        false,
        validate_rbac_name,
        &Path::new("metadata"),
    ));

    // Only validate rules when aggregationRule is not present.
    // When aggregationRule is present, rules can be empty (auto-populated by controller).
    if role.aggregation_rule.is_none() {
        for (i, rule) in role.rules.iter().enumerate() {
            all_errs.extend(validate_policy_rule(
                rule,
                false,
                &Path::new("rules").index(i),
            ));
        }
    }

    if let Some(ref aggregation) = role.aggregation_rule {
        if aggregation.cluster_role_selectors.is_empty() {
            all_errs.push(required(
                &Path::new("aggregationRule").child("clusterRoleSelectors"),
                "at least one clusterRoleSelector required if aggregationRule is non-nil",
            ));
        }
        for (i, selector) in aggregation.cluster_role_selectors.iter().enumerate() {
            let selector_path = Path::new("aggregationRule")
                .child("clusterRoleSelectors")
                .index(i);
            all_errs.extend(validate_label_selector(
                selector,
                opts.allow_invalid_label_value_in_selector,
                &selector_path,
            ));
        }
    }

    all_errs
}

pub fn validate_cluster_role_update(
    role: &ClusterRole,
    old_role: &ClusterRole,
    opts: ClusterRoleValidationOptions,
) -> ErrorList {
    let mut all_errs = validate_cluster_role(role, opts);

    if let (Some(new_meta), Some(old_meta)) = (role.metadata.as_ref(), old_role.metadata.as_ref()) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

    all_errs
}
