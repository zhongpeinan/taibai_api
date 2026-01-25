//! Defaulting functions for networking/v1 API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/networking/v1/zz_generated.defaults.go

use super::ingress::*;
use super::network_policy::*;

// ============================================================================
// Ingress Defaults
// ============================================================================

/// Apply defaults to HTTPIngressPath
pub fn set_defaults_http_ingress_path(obj: &mut HTTPIngressPath) {
    // PathType defaults to Prefix if not explicitly set
    // In our model, PathType is always present (not Option), but we check
    // if it's the zero value and set it to Prefix
    // Since PathType enum has #[default] as Prefix, this is already handled
    // but we keep this function for consistency with upstream
}

/// Apply defaults to all Ingress paths recursively
pub fn set_defaults_ingress(obj: &mut Ingress) {
    if let Some(ref mut spec) = obj.spec {
        // Apply defaults to rules
        for rule in &mut spec.rules {
            if let Some(ref mut http) = rule.http {
                for path in &mut http.paths {
                    set_defaults_http_ingress_path(path);
                }
            }
        }
    }
}

// ============================================================================
// NetworkPolicy Defaults
// ============================================================================

/// Apply defaults to NetworkPolicyPort
pub fn set_defaults_network_policy_port(obj: &mut NetworkPolicyPort) {
    // Default protocol to TCP if not specified
    if obj.protocol.is_none() {
        obj.protocol = Some("TCP".to_string());
    }
}

/// Apply defaults to all NetworkPolicy ports recursively
pub fn set_defaults_network_policy(obj: &mut NetworkPolicy) {
    if let Some(ref mut spec) = obj.spec {
        // Apply defaults to ingress rules
        for rule in &mut spec.ingress {
            for port in &mut rule.ports {
                set_defaults_network_policy_port(port);
            }
        }

        // Apply defaults to egress rules
        for rule in &mut spec.egress {
            for port in &mut rule.ports {
                set_defaults_network_policy_port(port);
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{IntOrString, LabelSelector, ObjectMeta, TypeMeta};

    #[test]
    fn test_default_ingress_path_type() {
        let mut ingress = Ingress {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-ingress".to_string()),
                ..Default::default()
            }),
            spec: Some(IngressSpec {
                ingress_class_name: None,
                default_backend: None,
                rules: vec![IngressRule {
                    host: "example.com".to_string(),
                    http: Some(HTTPIngressRuleValue {
                        paths: vec![HTTPIngressPath {
                            path: "/".to_string(),
                            path_type: PathType::Prefix, // Already has default value
                            backend: IngressBackend::default(),
                        }],
                    }),
                }],
                tls: vec![],
            }),
            status: None,
        };

        set_defaults_ingress(&mut ingress);

        // PathType should remain Prefix (already set via enum default)
        assert_eq!(
            ingress.spec.as_ref().unwrap().rules[0]
                .http
                .as_ref()
                .unwrap()
                .paths[0]
                .path_type,
            PathType::Prefix
        );
    }

    #[test]
    fn test_default_network_policy_port_protocol() {
        let mut port = NetworkPolicyPort {
            protocol: None,
            port: None,
            end_port: None,
        };

        set_defaults_network_policy_port(&mut port);

        assert_eq!(port.protocol, Some("TCP".to_string()));
    }

    #[test]
    fn test_default_network_policy_with_ports() {
        let mut policy = NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![NetworkPolicyPort {
                        protocol: None,
                        port: Some(IntOrString::Int(80)),
                        end_port: None,
                    }],
                    from: vec![],
                }],
                egress: vec![NetworkPolicyEgressRule {
                    ports: vec![NetworkPolicyPort {
                        protocol: None,
                        port: Some(IntOrString::Int(443)),
                        end_port: None,
                    }],
                    to: vec![],
                }],
                policy_types: vec![],
            }),
        };

        set_defaults_network_policy(&mut policy);

        // Check that protocol was defaulted to TCP
        assert_eq!(
            policy.spec.as_ref().unwrap().ingress[0].ports[0].protocol,
            Some("TCP".to_string())
        );
        assert_eq!(
            policy.spec.as_ref().unwrap().egress[0].ports[0].protocol,
            Some("TCP".to_string())
        );
    }
}
