//! ServiceProxyOptions conversion
//!
//! Upstream: k8s.io/kubernetes/pkg/apis/core/v1/zz_generated.conversion.go

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::helper;

impl ToInternal<internal::ServiceProxyOptions> for helper::ServiceProxyOptions {
    fn to_internal(self) -> internal::ServiceProxyOptions {
        internal::ServiceProxyOptions {
            path: self.path.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::ServiceProxyOptions> for helper::ServiceProxyOptions {
    fn from_internal(value: internal::ServiceProxyOptions) -> Self {
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
    fn test_service_proxy_options_to_internal() {
        let v1_opts = helper::ServiceProxyOptions {
            type_meta: Default::default(),
            path: Some("/api/v1/namespaces/default/services".to_string()),
        };

        let internal = v1_opts.to_internal();
        assert_eq!(internal.path, "/api/v1/namespaces/default/services");
    }

    #[test]
    fn test_service_proxy_options_from_internal() {
        let internal_opts = internal::ServiceProxyOptions {
            path: "/healthz".to_string(),
        };

        let v1 = helper::ServiceProxyOptions::from_internal(internal_opts);
        assert_eq!(v1.path, Some("/healthz".to_string()));
    }

    #[test]
    fn test_service_proxy_options_empty_path() {
        let v1_opts = helper::ServiceProxyOptions {
            path: None,
            ..Default::default()
        };

        let internal = v1_opts.to_internal();
        assert!(internal.path.is_empty());

        let v1_back = helper::ServiceProxyOptions::from_internal(internal);
        assert!(v1_back.path.is_none());
    }

    #[test]
    fn test_service_proxy_options_roundtrip() {
        let v1_opts = helper::ServiceProxyOptions {
            type_meta: Default::default(),
            path: Some("/ready".to_string()),
        };

        let internal = v1_opts.clone().to_internal();
        let v1_back = helper::ServiceProxyOptions::from_internal(internal);

        assert_eq!(v1_back.path, v1_opts.path);
    }
}
