//! PodExecOptions conversion
//!
//! Upstream: k8s.io/kubernetes/pkg/apis/core/v1/zz_generated.conversion.go

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::helper;

impl ToInternal<internal::PodExecOptions> for helper::PodExecOptions {
    fn to_internal(self) -> internal::PodExecOptions {
        internal::PodExecOptions {
            stdin: self.stdin,
            stdout: self.stdout,
            stderr: self.stderr,
            tty: self.tty,
            container: self.container.unwrap_or_default(),
            command: self.command,
        }
    }
}

impl FromInternal<internal::PodExecOptions> for helper::PodExecOptions {
    fn from_internal(value: internal::PodExecOptions) -> Self {
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
            command: value.command,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_exec_options_to_internal() {
        let v1_opts = helper::PodExecOptions {
            type_meta: Default::default(),
            stdin: true,
            stdout: true,
            stderr: false,
            tty: true,
            container: Some("main".to_string()),
            command: vec!["sh".to_string(), "-c".to_string(), "echo hello".to_string()],
        };

        let internal = v1_opts.to_internal();
        assert!(internal.stdin);
        assert!(internal.stdout);
        assert!(!internal.stderr);
        assert!(internal.tty);
        assert_eq!(internal.container, "main");
        assert_eq!(internal.command, vec!["sh", "-c", "echo hello"]);
    }

    #[test]
    fn test_pod_exec_options_from_internal() {
        let internal_opts = internal::PodExecOptions {
            stdin: false,
            stdout: true,
            stderr: true,
            tty: false,
            container: "debug".to_string(),
            command: vec!["ls".to_string(), "-la".to_string()],
        };

        let v1 = helper::PodExecOptions::from_internal(internal_opts);
        assert!(!v1.stdin);
        assert!(v1.stdout);
        assert!(v1.stderr);
        assert!(!v1.tty);
        assert_eq!(v1.container, Some("debug".to_string()));
        assert_eq!(v1.command, vec!["ls", "-la"]);
    }

    #[test]
    fn test_pod_exec_options_empty_container() {
        let v1_opts = helper::PodExecOptions {
            container: None,
            command: vec!["cat".to_string(), "/etc/hosts".to_string()],
            ..Default::default()
        };

        let internal = v1_opts.to_internal();
        assert!(internal.container.is_empty());

        let v1_back = helper::PodExecOptions::from_internal(internal);
        assert!(v1_back.container.is_none());
    }

    #[test]
    fn test_pod_exec_options_roundtrip() {
        let v1_opts = helper::PodExecOptions {
            type_meta: Default::default(),
            stdin: true,
            stdout: true,
            stderr: true,
            tty: true,
            container: Some("app".to_string()),
            command: vec!["/bin/bash".to_string()],
        };

        let internal = v1_opts.clone().to_internal();
        let v1_back = helper::PodExecOptions::from_internal(internal);

        assert_eq!(v1_back.stdin, v1_opts.stdin);
        assert_eq!(v1_back.stdout, v1_opts.stdout);
        assert_eq!(v1_back.stderr, v1_opts.stderr);
        assert_eq!(v1_back.tty, v1_opts.tty);
        assert_eq!(v1_back.container, v1_opts.container);
        assert_eq!(v1_back.command, v1_opts.command);
    }
}
