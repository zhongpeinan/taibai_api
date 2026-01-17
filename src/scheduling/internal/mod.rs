//! Scheduling internal API types
//!
//! This module contains the internal types for the Kubernetes Scheduling API.
//!
//! The scheduling API provides support for configuring pod priority and preemption.
//!
//! Source: k8s-pkg/apis/scheduling/types.go

// Re-export all v1 types (internal and external types are essentially the same)
pub use crate::scheduling::v1::{
    DEFAULT_PRIORITY_WHEN_NO_DEFAULT_CLASS_EXISTS, HIGHEST_USER_DEFINABLE_PRIORITY, PriorityClass,
    PriorityClassList, SYSTEM_CLUSTER_CRITICAL, SYSTEM_CRITICAL_PRIORITY, SYSTEM_NODE_CRITICAL,
    SYSTEM_PRIORITY_CLASS_PREFIX,
};

// ============================================================================
// Additional Internal Constants
// ============================================================================

/// DefaultPriorityWhenNoDefaultClassExists is used to set priority of pods
/// that do not specify any priority class and there is no priority class
/// marked as default.
///
/// Note: This constant has the same value as DEFAULT_PRIORITY_WHEN_NO_DEFAULT_CLASS_EXISTS
/// but is named according to the Go convention.
pub const DEFAULT_PRIORITY_WHEN_NO_DEFAULT_CLASS_EXISTS_INTERNAL: i32 =
    DEFAULT_PRIORITY_WHEN_NO_DEFAULT_CLASS_EXISTS;

/// HighestUserDefinablePriority is the highest priority for user defined
/// priority classes. Priority values larger than 1 billion are reserved
/// for Kubernetes system use.
pub const HIGHEST_USER_DEFINABLE_PRIORITY_INTERNAL: i32 = HIGHEST_USER_DEFINABLE_PRIORITY;

/// SystemCriticalPriority is the beginning of the range of priority values
/// for critical system components.
pub const SYSTEM_CRITICAL_PRIORITY_INTERNAL: i32 = SYSTEM_CRITICAL_PRIORITY;

/// SystemPriorityClassPrefix is the prefix reserved for system priority class names.
pub const SYSTEM_PRIORITY_CLASS_PREFIX_INTERNAL: &str = SYSTEM_PRIORITY_CLASS_PREFIX;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_constants() {
        assert_eq!(DEFAULT_PRIORITY_WHEN_NO_DEFAULT_CLASS_EXISTS_INTERNAL, 0);
        assert_eq!(HIGHEST_USER_DEFINABLE_PRIORITY_INTERNAL, 1_000_000_000);
        assert_eq!(SYSTEM_CRITICAL_PRIORITY_INTERNAL, 2_000_000_000);
        assert_eq!(SYSTEM_PRIORITY_CLASS_PREFIX_INTERNAL, "system-");
    }
}
