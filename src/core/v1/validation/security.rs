//! Security context validation for Kubernetes core/v1 API.
//!
//! Delegates to internal validation for consistency.

use crate::common::ToInternal;
use crate::common::validation::{ErrorList, Path};
use crate::core::internal::validation::security as internal_security_validation;
use crate::core::v1::security::{PodSecurityContext, Sysctl};

/// Validates PodSecurityContext.
pub fn validate_pod_security_context(context: &PodSecurityContext, path: &Path) -> ErrorList {
    let internal_context = context.clone().to_internal();
    internal_security_validation::validate_pod_security_context(&internal_context, path)
}

/// Validates Sysctl values.
pub fn validate_sysctls(sysctls: &[Sysctl], path: &Path) -> ErrorList {
    let internal_sysctls: Vec<crate::core::internal::Sysctl> =
        sysctls.iter().cloned().map(|s| s.to_internal()).collect();
    internal_security_validation::validate_sysctls(&internal_sysctls, path)
}
