//! Security context validation for Kubernetes core internal API.
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{ErrorList, Path, required};
use crate::core::internal::security::{PodSecurityContext, Sysctl};
use crate::core::v1::validation::helpers::validate_nonnegative_field;

/// Validates PodSecurityContext.
pub fn validate_pod_security_context(sec_ctx: &PodSecurityContext, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(value) = sec_ctx.run_as_user {
        all_errs.extend(validate_nonnegative_field(value, &path.child("runAsUser")));
    }

    if let Some(value) = sec_ctx.run_as_group {
        all_errs.extend(validate_nonnegative_field(value, &path.child("runAsGroup")));
    }

    if let Some(value) = sec_ctx.fs_group {
        all_errs.extend(validate_nonnegative_field(value, &path.child("fsGroup")));
    }

    for (i, group) in sec_ctx.supplemental_groups.iter().enumerate() {
        all_errs.extend(validate_nonnegative_field(
            *group,
            &path.child("supplementalGroups").index(i),
        ));
    }

    all_errs.extend(validate_sysctls(&sec_ctx.sysctls, &path.child("sysctls")));

    all_errs
}

/// Validates Sysctl values.
pub fn validate_sysctls(sysctls: &[Sysctl], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, sysctl) in sysctls.iter().enumerate() {
        let idx_path = path.index(i);
        if sysctl.name.is_empty() {
            all_errs.push(required(&idx_path.child("name"), "name is required"));
        }
        if sysctl.value.is_empty() {
            all_errs.push(required(&idx_path.child("value"), "value is required"));
        }
    }

    all_errs
}
