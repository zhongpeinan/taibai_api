//! PodDisruptionBudget conversions

use crate::common::{FromInternal, ToInternal, TypeMeta};
use crate::policy::internal;
use crate::policy::v1::{
    PodDisruptionBudget, PodDisruptionBudgetList, PodDisruptionBudgetSpec,
    PodDisruptionBudgetStatus, UnhealthyPodEvictionPolicyType,
};

// ============================================================================
// UnhealthyPodEvictionPolicyType Conversions
// ============================================================================

impl ToInternal<internal::UnhealthyPodEvictionPolicyType> for UnhealthyPodEvictionPolicyType {
    fn to_internal(self) -> internal::UnhealthyPodEvictionPolicyType {
        match self {
            UnhealthyPodEvictionPolicyType::IfHealthyBudget => {
                internal::UnhealthyPodEvictionPolicyType::IfHealthyBudget
            }
            UnhealthyPodEvictionPolicyType::AlwaysAllow => {
                internal::UnhealthyPodEvictionPolicyType::AlwaysAllow
            }
        }
    }
}

impl FromInternal<internal::UnhealthyPodEvictionPolicyType> for UnhealthyPodEvictionPolicyType {
    fn from_internal(value: internal::UnhealthyPodEvictionPolicyType) -> Self {
        match value {
            internal::UnhealthyPodEvictionPolicyType::IfHealthyBudget => {
                UnhealthyPodEvictionPolicyType::IfHealthyBudget
            }
            internal::UnhealthyPodEvictionPolicyType::AlwaysAllow => {
                UnhealthyPodEvictionPolicyType::AlwaysAllow
            }
        }
    }
}

// ============================================================================
// PodDisruptionBudgetSpec Conversions
// ============================================================================

impl ToInternal<internal::PodDisruptionBudgetSpec> for PodDisruptionBudgetSpec {
    fn to_internal(self) -> internal::PodDisruptionBudgetSpec {
        internal::PodDisruptionBudgetSpec {
            min_available: self.min_available,
            selector: self.selector,
            max_unavailable: self.max_unavailable,
            unhealthy_pod_eviction_policy: self
                .unhealthy_pod_eviction_policy
                .map(|p| p.to_internal()),
        }
    }
}

impl FromInternal<internal::PodDisruptionBudgetSpec> for PodDisruptionBudgetSpec {
    fn from_internal(value: internal::PodDisruptionBudgetSpec) -> Self {
        Self {
            min_available: value.min_available,
            selector: value.selector,
            max_unavailable: value.max_unavailable,
            unhealthy_pod_eviction_policy: value
                .unhealthy_pod_eviction_policy
                .map(UnhealthyPodEvictionPolicyType::from_internal),
        }
    }
}

// ============================================================================
// PodDisruptionBudgetStatus Conversions
// ============================================================================

impl ToInternal<internal::PodDisruptionBudgetStatus> for PodDisruptionBudgetStatus {
    fn to_internal(self) -> internal::PodDisruptionBudgetStatus {
        internal::PodDisruptionBudgetStatus {
            observed_generation: self.observed_generation.unwrap_or_default(),
            disrupted_pods: self.disrupted_pods,
            disruptions_allowed: self.disruptions_allowed.unwrap_or_default(),
            current_healthy: self.current_healthy.unwrap_or_default(),
            desired_healthy: self.desired_healthy.unwrap_or_default(),
            expected_pods: self.expected_pods.unwrap_or_default(),
            conditions: self.conditions,
        }
    }
}

impl FromInternal<internal::PodDisruptionBudgetStatus> for PodDisruptionBudgetStatus {
    fn from_internal(value: internal::PodDisruptionBudgetStatus) -> Self {
        Self {
            observed_generation: if value.observed_generation == 0 {
                None
            } else {
                Some(value.observed_generation)
            },
            disrupted_pods: value.disrupted_pods,
            disruptions_allowed: Some(value.disruptions_allowed),
            current_healthy: Some(value.current_healthy),
            desired_healthy: Some(value.desired_healthy),
            expected_pods: Some(value.expected_pods),
            conditions: value.conditions,
        }
    }
}

// ============================================================================
// PodDisruptionBudget Conversions
// ============================================================================

impl ToInternal<internal::PodDisruptionBudget> for PodDisruptionBudget {
    fn to_internal(self) -> internal::PodDisruptionBudget {
        internal::PodDisruptionBudget {
            type_meta: TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            spec: self.spec.map(|s| s.to_internal()).unwrap_or_default(),
            status: self.status.map(|s| s.to_internal()).unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::PodDisruptionBudget> for PodDisruptionBudget {
    fn from_internal(value: internal::PodDisruptionBudget) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: Some(value.metadata),
            spec: Some(PodDisruptionBudgetSpec::from_internal(value.spec)),
            status: Some(PodDisruptionBudgetStatus::from_internal(value.status)),
        }
    }
}

// ============================================================================
// PodDisruptionBudgetList Conversions
// ============================================================================

impl ToInternal<internal::PodDisruptionBudgetList> for PodDisruptionBudgetList {
    fn to_internal(self) -> internal::PodDisruptionBudgetList {
        internal::PodDisruptionBudgetList {
            type_meta: TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::PodDisruptionBudgetList> for PodDisruptionBudgetList {
    fn from_internal(value: internal::PodDisruptionBudgetList) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: Some(value.metadata),
            items: value
                .items
                .into_iter()
                .map(PodDisruptionBudget::from_internal)
                .collect(),
        }
    }
}
