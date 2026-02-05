//! PodLogOptions conversion
//!
//! Upstream: k8s.io/kubernetes/pkg/apis/core/v1/zz_generated.conversion.go

use crate::common::Timestamp;
use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::helper;

impl ToInternal<internal::PodLogOptions> for helper::PodLogOptions {
    fn to_internal(self) -> internal::PodLogOptions {
        internal::PodLogOptions {
            container: self.container.unwrap_or_default(),
            follow: self.follow,
            previous: self.previous,
            since_seconds: self.since_seconds,
            // v1 Timestamp -> internal String (RFC3339)
            since_time: self.since_time.map(|t| t.to_rfc3339()),
            timestamps: self.timestamps,
            tail_lines: self.tail_lines,
            limit_bytes: self.limit_bytes,
            insecure_skip_tls_verify_backend: self.insecure_skip_tls_verify_backend,
            stream: self.stream,
        }
    }
}

impl FromInternal<internal::PodLogOptions> for helper::PodLogOptions {
    fn from_internal(value: internal::PodLogOptions) -> Self {
        Self {
            type_meta: Default::default(),
            container: if value.container.is_empty() {
                None
            } else {
                Some(value.container)
            },
            follow: value.follow,
            previous: value.previous,
            since_seconds: value.since_seconds,
            // internal String (RFC3339) -> v1 Timestamp
            since_time: value.since_time.and_then(|s| Timestamp::from_str(&s).ok()),
            timestamps: value.timestamps,
            tail_lines: value.tail_lines,
            limit_bytes: value.limit_bytes,
            insecure_skip_tls_verify_backend: value.insecure_skip_tls_verify_backend,
            stream: value.stream,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_log_options_to_internal() {
        let v1_opts = helper::PodLogOptions {
            type_meta: Default::default(),
            container: Some("main".to_string()),
            follow: true,
            previous: false,
            since_seconds: Some(3600),
            since_time: None,
            timestamps: true,
            tail_lines: Some(100),
            limit_bytes: Some(1024),
            insecure_skip_tls_verify_backend: true,
            stream: Some("Stdout".to_string()),
        };

        let internal = v1_opts.to_internal();
        assert_eq!(internal.container, "main");
        assert!(internal.follow);
        assert!(!internal.previous);
        assert_eq!(internal.since_seconds, Some(3600));
        assert!(internal.timestamps);
        assert_eq!(internal.tail_lines, Some(100));
        assert_eq!(internal.limit_bytes, Some(1024));
        assert!(internal.insecure_skip_tls_verify_backend);
        assert_eq!(internal.stream, Some("Stdout".to_string()));
    }

    #[test]
    fn test_pod_log_options_from_internal() {
        let internal_opts = internal::PodLogOptions {
            container: "sidecar".to_string(),
            follow: false,
            previous: true,
            since_seconds: None,
            since_time: Some("2024-01-15T10:00:00Z".to_string()),
            timestamps: true,
            tail_lines: Some(50),
            limit_bytes: None,
            insecure_skip_tls_verify_backend: false,
            stream: Some("Stderr".to_string()),
        };

        let v1 = helper::PodLogOptions::from_internal(internal_opts);
        assert_eq!(v1.container, Some("sidecar".to_string()));
        assert!(!v1.follow);
        assert!(v1.previous);
        assert!(v1.since_time.is_some());
        assert!(v1.timestamps);
        assert_eq!(v1.tail_lines, Some(50));
        assert_eq!(v1.limit_bytes, None);
        assert!(!v1.insecure_skip_tls_verify_backend);
        assert_eq!(v1.stream, Some("Stderr".to_string()));
    }

    #[test]
    fn test_pod_log_options_empty_container_roundtrip() {
        let v1_opts = helper::PodLogOptions {
            container: None,
            ..Default::default()
        };

        let internal = v1_opts.to_internal();
        assert!(internal.container.is_empty());

        let v1_back = helper::PodLogOptions::from_internal(internal);
        assert!(v1_back.container.is_none());
    }

    #[test]
    fn test_pod_log_options_roundtrip() {
        let v1_opts = helper::PodLogOptions {
            type_meta: Default::default(),
            container: Some("app".to_string()),
            follow: true,
            previous: true,
            since_seconds: Some(7200),
            since_time: Timestamp::from_str("2024-06-01T12:00:00Z").ok(),
            timestamps: true,
            tail_lines: Some(200),
            limit_bytes: Some(2048),
            insecure_skip_tls_verify_backend: true,
            stream: Some("Stdout".to_string()),
        };

        let internal = v1_opts.clone().to_internal();
        let v1_back = helper::PodLogOptions::from_internal(internal);

        // All fields should roundtrip
        assert_eq!(v1_back.container, v1_opts.container);
        assert_eq!(v1_back.follow, v1_opts.follow);
        assert_eq!(v1_back.previous, v1_opts.previous);
        assert_eq!(v1_back.since_seconds, v1_opts.since_seconds);
        assert_eq!(v1_back.since_time, v1_opts.since_time);
        assert_eq!(v1_back.timestamps, v1_opts.timestamps);
        assert_eq!(v1_back.tail_lines, v1_opts.tail_lines);
        assert_eq!(v1_back.limit_bytes, v1_opts.limit_bytes);
        assert_eq!(
            v1_back.insecure_skip_tls_verify_backend,
            v1_opts.insecure_skip_tls_verify_backend
        );
        assert_eq!(v1_back.stream, v1_opts.stream);
    }
}
