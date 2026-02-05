//! PodProxyOptions conversion
//!
//! Upstream: k8s.io/kubernetes/pkg/apis/core/v1/zz_generated.conversion.go

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::helper;

impl ToInternal<internal::PodProxyOptions> for helper::PodProxyOptions {
    fn to_internal(self) -> internal::PodProxyOptions {
        internal::PodProxyOptions {
            path: self.path.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::PodProxyOptions> for helper::PodProxyOptions {
    fn from_internal(value: internal::PodProxyOptions) -> Self {
        Self {
            type_meta: Default::default(),
            path: if value.path.is_empty() {
                None
            } else {
                Some(value.path)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_proxy_options_to_internal() {
        let v1_opts = helper::PodProxyOptions {
            type_meta: Default::default(),
            path: Some("/api/v1/health".to_string()),
        };

        let internal = v1_opts.to_internal();
        assert_eq!(internal.path, "/api/v1/health");
    }

    #[test]
    fn test_pod_proxy_options_from_internal() {
        let internal_opts = internal::PodProxyOptions {
            path: "/metrics".to_string(),
        };

        let v1 = helper::PodProxyOptions::from_internal(internal_opts);
        assert_eq!(v1.path, Some("/metrics".to_string()));
    }

    #[test]
    fn test_pod_proxy_options_empty_path() {
        let v1_opts = helper::PodProxyOptions {
            path: None,
            ..Default::default()
        };

        let internal = v1_opts.to_internal();
        assert!(internal.path.is_empty());

        let v1_back = helper::PodProxyOptions::from_internal(internal);
        assert!(v1_back.path.is_none());
    }

    #[test]
    fn test_pod_proxy_options_roundtrip() {
        let v1_opts = helper::PodProxyOptions {
            type_meta: Default::default(),
            path: Some("/debug/pprof".to_string()),
        };

        let internal = v1_opts.clone().to_internal();
        let v1_back = helper::PodProxyOptions::from_internal(internal);

        assert_eq!(v1_back.path, v1_opts.path);
    }
}
