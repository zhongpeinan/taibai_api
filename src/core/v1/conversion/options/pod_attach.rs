//! PodAttachOptions conversion
//!
//! Upstream: k8s.io/kubernetes/pkg/apis/core/v1/zz_generated.conversion.go

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::helper;

impl ToInternal<internal::PodAttachOptions> for helper::PodAttachOptions {
    fn to_internal(self) -> internal::PodAttachOptions {
        internal::PodAttachOptions {
            stdin: self.stdin,
            stdout: self.stdout,
            stderr: self.stderr,
            tty: self.tty,
            container: self.container.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::PodAttachOptions> for helper::PodAttachOptions {
    fn from_internal(value: internal::PodAttachOptions) -> Self {
        Self {
            type_meta: Default::default(),
            stdin: value.stdin,
            stdout: value.stdout,
            stderr: value.stderr,
            tty: value.tty,
            container: if value.container.is_empty() {
                None
            } else {
                Some(value.container)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_attach_options_to_internal() {
        let v1_opts = helper::PodAttachOptions {
            type_meta: Default::default(),
            stdin: true,
            stdout: true,
            stderr: false,
            tty: true,
            container: Some("main".to_string()),
        };

        let internal = v1_opts.to_internal();
        assert!(internal.stdin);
        assert!(internal.stdout);
        assert!(!internal.stderr);
        assert!(internal.tty);
        assert_eq!(internal.container, "main");
    }

    #[test]
    fn test_pod_attach_options_from_internal() {
        let internal_opts = internal::PodAttachOptions {
            stdin: false,
            stdout: true,
            stderr: true,
            tty: false,
            container: "sidecar".to_string(),
        };

        let v1 = helper::PodAttachOptions::from_internal(internal_opts);
        assert!(!v1.stdin);
        assert!(v1.stdout);
        assert!(v1.stderr);
        assert!(!v1.tty);
        assert_eq!(v1.container, Some("sidecar".to_string()));
    }

    #[test]
    fn test_pod_attach_options_empty_container() {
        let v1_opts = helper::PodAttachOptions {
            container: None,
            ..Default::default()
        };

        let internal = v1_opts.to_internal();
        assert!(internal.container.is_empty());

        let v1_back = helper::PodAttachOptions::from_internal(internal);
        assert!(v1_back.container.is_none());
    }

    #[test]
    fn test_pod_attach_options_roundtrip() {
        let v1_opts = helper::PodAttachOptions {
            type_meta: Default::default(),
            stdin: true,
            stdout: true,
            stderr: true,
            tty: true,
            container: Some("app".to_string()),
        };

        let internal = v1_opts.clone().to_internal();
        let v1_back = helper::PodAttachOptions::from_internal(internal);

        assert_eq!(v1_back.stdin, v1_opts.stdin);
        assert_eq!(v1_back.stdout, v1_opts.stdout);
        assert_eq!(v1_back.stderr, v1_opts.stderr);
        assert_eq!(v1_back.tty, v1_opts.tty);
        assert_eq!(v1_back.container, v1_opts.container);
    }
}
