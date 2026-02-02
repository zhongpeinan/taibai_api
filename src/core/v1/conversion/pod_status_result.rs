//! PodStatusResult conversion implementations
//!
//! Includes: PodStatusResult

use super::helpers::*;
use crate::common::traits::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::pod_status_result;

// ============================================================================
// PodStatusResult
// ============================================================================

impl ToInternal<internal::PodStatusResult> for pod_status_result::PodStatusResult {
    fn to_internal(self) -> internal::PodStatusResult {
        internal::PodStatusResult {
            metadata: option_object_meta_to_meta(self.metadata),
            status: self.status.map(|status| status.to_internal()),
            ..Default::default()
        }
    }
}

impl FromInternal<internal::PodStatusResult> for pod_status_result::PodStatusResult {
    fn from_internal(value: internal::PodStatusResult) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            status: value.status.map(crate::core::v1::PodStatus::from_internal),
        };

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::v1::PodStatus;

    #[test]
    fn test_pod_status_result_roundtrip() {
        let status = PodStatus {
            message: Some("ready".to_string()),
            ..Default::default()
        };

        let pod_status_result = pod_status_result::PodStatusResult {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("status-pod".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            status: Some(status),
        };

        let internal = pod_status_result.clone().to_internal();
        assert_eq!(internal.metadata.name.as_deref(), Some("status-pod"));
        assert_eq!(
            internal.status.as_ref().map(|s| s.message.as_str()),
            Some("ready")
        );

        let mut roundtrip = pod_status_result::PodStatusResult::from_internal(internal);
        roundtrip.apply_default();
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name.as_deref(),
            Some("status-pod")
        );
        assert_eq!(
            roundtrip.status.as_ref().and_then(|s| s.message.as_deref()),
            Some("ready")
        );
    }
}
