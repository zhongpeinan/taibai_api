//! PodPortForwardOptions conversion
//!
//! Upstream: k8s.io/kubernetes/pkg/apis/core/v1/zz_generated.conversion.go

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::helper;

impl ToInternal<internal::PodPortForwardOptions> for helper::PodPortForwardOptions {
    fn to_internal(self) -> internal::PodPortForwardOptions {
        internal::PodPortForwardOptions { ports: self.ports }
    }
}

impl FromInternal<internal::PodPortForwardOptions> for helper::PodPortForwardOptions {
    fn from_internal(value: internal::PodPortForwardOptions) -> Self {
        Self {
            type_meta: Default::default(),
            ports: value.ports,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_port_forward_options_to_internal() {
        let v1_opts = helper::PodPortForwardOptions {
            type_meta: Default::default(),
            ports: vec![8080, 9090, 3000],
        };

        let internal = v1_opts.to_internal();
        assert_eq!(internal.ports, vec![8080, 9090, 3000]);
    }

    #[test]
    fn test_pod_port_forward_options_from_internal() {
        let internal_opts = internal::PodPortForwardOptions {
            ports: vec![80, 443],
        };

        let v1 = helper::PodPortForwardOptions::from_internal(internal_opts);
        assert_eq!(v1.ports, vec![80, 443]);
    }

    #[test]
    fn test_pod_port_forward_options_empty_ports() {
        let v1_opts = helper::PodPortForwardOptions {
            ports: vec![],
            ..Default::default()
        };

        let internal = v1_opts.to_internal();
        assert!(internal.ports.is_empty());

        let v1_back = helper::PodPortForwardOptions::from_internal(internal);
        assert!(v1_back.ports.is_empty());
    }

    #[test]
    fn test_pod_port_forward_options_roundtrip() {
        let v1_opts = helper::PodPortForwardOptions {
            type_meta: Default::default(),
            ports: vec![5432, 6379, 27017],
        };

        let internal = v1_opts.clone().to_internal();
        let v1_back = helper::PodPortForwardOptions::from_internal(internal);

        assert_eq!(v1_back.ports, v1_opts.ports);
    }
}
