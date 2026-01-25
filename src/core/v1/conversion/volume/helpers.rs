//! Helper functions for volume-related enum conversions

use crate::core::internal;

// ============================================================================
// Enum Conversion Helpers
// ============================================================================

/// Convert Option<String> to Option<internal::MountPropagationMode>
pub(super) fn option_string_to_mount_propagation(
    s: Option<String>,
) -> Option<internal::MountPropagationMode> {
    s.and_then(|s| serde_json::from_value(serde_json::Value::String(s)).ok())
}

/// Convert Option<internal::MountPropagationMode> to Option<String>
pub(super) fn mount_propagation_to_option_string(
    mode: Option<internal::MountPropagationMode>,
) -> Option<String> {
    mode.and_then(|m| {
        serde_json::to_value(m)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
    })
}

/// Convert Option<String> to Option<internal::RecursiveReadOnlyMode>
pub(super) fn option_string_to_recursive_readonly(
    s: Option<String>,
) -> Option<internal::RecursiveReadOnlyMode> {
    s.and_then(|s| serde_json::from_value(serde_json::Value::String(s)).ok())
}

/// Convert Option<internal::RecursiveReadOnlyMode> to Option<String>
pub(super) fn recursive_readonly_to_option_string(
    mode: Option<internal::RecursiveReadOnlyMode>,
) -> Option<String> {
    mode.and_then(|m| {
        serde_json::to_value(m)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
    })
}

/// Convert Option<String> to internal::StorageMedium
pub(super) fn option_string_to_storage_medium(s: Option<String>) -> internal::StorageMedium {
    s.and_then(|s| serde_json::from_value(serde_json::Value::String(s)).ok())
        .unwrap_or_default()
}

/// Convert internal::StorageMedium to Option<String>
pub(super) fn storage_medium_to_option_string(medium: internal::StorageMedium) -> Option<String> {
    serde_json::to_value(medium)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .filter(|s| !s.is_empty())
}

/// Convert Option<String> to internal::PullPolicy
pub(super) fn option_string_to_pull_policy(s: Option<String>) -> internal::PullPolicy {
    s.and_then(|s| serde_json::from_value(serde_json::Value::String(s)).ok())
        .unwrap_or_default()
}

/// Convert internal::PullPolicy to Option<String>
pub(super) fn pull_policy_to_option_string(policy: internal::PullPolicy) -> Option<String> {
    serde_json::to_value(policy)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
}
