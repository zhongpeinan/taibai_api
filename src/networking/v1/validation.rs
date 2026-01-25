//! Validation for Kubernetes Networking API v1 types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/networking/validation/validation.go

use crate::common::IntOrString;
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, name_is_dns_subdomain, required, validate_object_meta,
};
use crate::networking::v1::ingress::{Ingress, IngressBackend, IngressList};
use crate::networking::v1::ingress_class::{IngressClass, IngressClassList};
use crate::networking::v1::network_policy::{
    IPBlock, NetworkPolicy, NetworkPolicyList, NetworkPolicyPort,
};

// ============================================================================
// Ingress Validation
// ============================================================================

// ============================================================================
// Helper Functions
// ============================================================================

/// Validates if a string is a valid DNS subdomain
fn is_valid_dns_subdomain(value: &str) -> bool {
    // Basic DNS subdomain validation
    // Must be lowercase alphanumeric with hyphens and dots
    // Max 253 characters
    if value.is_empty() || value.len() > 253 {
        return false;
    }

    // Check each label (separated by dots)
    for label in value.split('.') {
        if label.is_empty() || label.len() > 63 {
            return false;
        }
        if !label
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return false;
        }
        // Label must not start or end with hyphen
        if label.starts_with('-') || label.ends_with('-') {
            return false;
        }
    }

    true
}

/// Validates an IngressBackend
fn validate_ingress_backend(backend: &IngressBackend, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // service and resource are mutually exclusive
    if backend.service.is_some() && backend.resource.is_some() {
        all_errs.push(invalid(
            path,
            BadValue::String("both service and resource specified".to_string()),
            "service and resource are mutually exclusive",
        ));
    }

    // At least one must be specified
    if backend.service.is_none() && backend.resource.is_none() {
        all_errs.push(required(
            path,
            "either service or resource must be specified",
        ));
    }

    // Validate service if present
    if let Some(ref service) = backend.service {
        if service.name.is_empty() {
            all_errs.push(required(
                &path.child("service").child("name"),
                "service name is required",
            ));
        }

        // Validate port if present
        if let Some(ref port) = service.port {
            if port.name.is_empty() && port.number.is_none() {
                all_errs.push(required(
                    &path.child("service").child("port"),
                    "either port name or number must be specified",
                ));
            }

            if !port.name.is_empty() && port.number.is_some() {
                all_errs.push(invalid(
                    &path.child("service").child("port"),
                    BadValue::String("both name and number specified".to_string()),
                    "port name and number are mutually exclusive",
                ));
            }
        }
    }

    // Validate resource if present
    if let Some(ref resource) = backend.resource {
        if resource.kind.as_ref().map_or(true, |k| k.is_empty()) {
            all_errs.push(required(
                &path.child("resource").child("kind"),
                "resource kind is required",
            ));
        }
        if resource.name.as_ref().map_or(true, |n| n.is_empty()) {
            all_errs.push(required(
                &path.child("resource").child("name"),
                "resource name is required",
            ));
        }
    }

    all_errs
}

/// Validates a NetworkPolicyPort
fn validate_network_policy_port(port: &NetworkPolicyPort, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate endPort requirements
    if let Some(end_port) = port.end_port {
        if let Some(ref port_value) = port.port {
            // endPort only valid with numeric ports
            match port_value {
                IntOrString::String(_) => {
                    all_errs.push(invalid(
                        &path.child("endPort"),
                        BadValue::Int(end_port as i64),
                        "endPort cannot be used with named ports",
                    ));
                }
                IntOrString::Int(port_num) => {
                    if end_port < *port_num {
                        all_errs.push(invalid(
                            &path.child("endPort"),
                            BadValue::Int(end_port as i64),
                            "endPort must be greater than or equal to port",
                        ));
                    }
                }
            }
        } else {
            all_errs.push(invalid(
                &path.child("endPort"),
                BadValue::Int(end_port as i64),
                "endPort requires port to be specified",
            ));
        }
    }

    all_errs
}

/// Validates an IPBlock CIDR
fn validate_ip_block(block: &IPBlock, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate CIDR format
    if block.cidr.is_empty() {
        all_errs.push(required(&path.child("cidr"), "CIDR is required"));
    } else if !is_valid_cidr(&block.cidr) {
        all_errs.push(invalid(
            &path.child("cidr"),
            BadValue::String(block.cidr.clone()),
            "invalid CIDR format (expected format: \"192.168.1.0/24\" or \"2001:db8::/64\")",
        ));
    }

    // Validate except CIDRs
    for (i, except_cidr) in block.except.iter().enumerate() {
        if !is_valid_cidr(except_cidr) {
            all_errs.push(invalid(
                &path.child("except").index(i),
                BadValue::String(except_cidr.clone()),
                "invalid CIDR format",
            ));
        }
    }

    all_errs
}

/// Basic CIDR format validation
fn is_valid_cidr(cidr: &str) -> bool {
    // Simple validation - check for format "IP/prefix"
    // Real implementation should parse and validate IP address
    cidr.contains('/') && !cidr.is_empty()
}

// ============================================================================
// Ingress Validation
// ============================================================================

/// Validates an Ingress object.
pub fn validate_ingress(ingress: &Ingress) -> ErrorList {
    validate_ingress_with_path(ingress, &Path::nil())
}

fn validate_ingress_with_path(ingress: &Ingress, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = ingress.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec if present
    if let Some(ref spec) = ingress.spec {
        // Validate default backend
        if let Some(ref default_backend) = spec.default_backend {
            all_errs.extend(validate_ingress_backend(
                default_backend,
                &base_path.child("spec").child("defaultBackend"),
            ));
        }

        // Validate rules
        for (i, rule) in spec.rules.iter().enumerate() {
            let rule_path = base_path.child("spec").child("rules").index(i);

            // Validate host if present (should be valid DNS subdomain)
            if !rule.host.is_empty() && !is_valid_dns_subdomain(&rule.host) {
                all_errs.push(invalid(
                    &rule_path.child("host"),
                    BadValue::String(rule.host.clone()),
                    "must be a valid DNS subdomain",
                ));
            }

            // Validate HTTP paths
            if let Some(ref http) = rule.http {
                for (j, path) in http.paths.iter().enumerate() {
                    let path_path = rule_path.child("http").child("paths").index(j);

                    // Validate path field
                    if path.path.is_empty() {
                        all_errs.push(required(&path_path.child("path"), "path is required"));
                    } else {
                        // Path must start with '/' for Prefix and Exact types
                        use crate::networking::v1::ingress::PathType;
                        match path.path_type {
                            PathType::Prefix | PathType::Exact => {
                                if !path.path.starts_with('/') {
                                    all_errs.push(invalid(
                                        &path_path.child("path"),
                                        BadValue::String(path.path.clone()),
                                        "must be an absolute path (start with /)",
                                    ));
                                }
                            }
                            PathType::ImplementationSpecific => {
                                // No specific validation for implementation-specific paths
                            }
                        }
                    }

                    // Validate backend
                    all_errs.extend(validate_ingress_backend(
                        &path.backend,
                        &path_path.child("backend"),
                    ));
                }
            }
        }
    }

    all_errs
}

/// Validates an IngressList object.
pub fn validate_ingress_list(list: &IngressList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_ingress_with_path(item, &item_path));
    }

    all_errs
}

// ============================================================================
// IngressClass Validation
// ============================================================================

/// Validates an IngressClass object.
pub fn validate_ingress_class(ingress_class: &IngressClass) -> ErrorList {
    validate_ingress_class_with_path(ingress_class, &Path::nil())
}

fn validate_ingress_class_with_path(ingress_class: &IngressClass, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = ingress_class.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        false, // IngressClass is not namespaced
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec - controller is required
    if ingress_class.spec.controller.is_empty() {
        all_errs.push(required(
            &base_path.child("spec").child("controller"),
            "controller is required",
        ));
    }

    all_errs
}

/// Validates an IngressClassList object.
pub fn validate_ingress_class_list(list: &IngressClassList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_ingress_class_with_path(item, &item_path));
    }

    all_errs
}

// ============================================================================
// NetworkPolicy Validation
// ============================================================================

/// Validates a NetworkPolicy object.
pub fn validate_network_policy(policy: &NetworkPolicy) -> ErrorList {
    validate_network_policy_with_path(policy, &Path::nil())
}

fn validate_network_policy_with_path(policy: &NetworkPolicy, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = policy.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec if present
    if let Some(spec) = &policy.spec {
        let spec_path = base_path.child("spec");

        // Validate ingress rules
        for (i, rule) in spec.ingress.iter().enumerate() {
            let rule_path = spec_path.child("ingress").index(i);

            // Validate ports
            for (j, port) in rule.ports.iter().enumerate() {
                let port_path = rule_path.child("ports").index(j);
                all_errs.extend(validate_network_policy_port(port, &port_path));
            }

            // Validate peers
            for (k, peer) in rule.from.iter().enumerate() {
                let peer_path = rule_path.child("from").index(k);
                if let Some(ref ip_block) = peer.ip_block {
                    all_errs.extend(validate_ip_block(ip_block, &peer_path.child("ipBlock")));
                }
            }
        }

        // Validate egress rules
        for (i, rule) in spec.egress.iter().enumerate() {
            let rule_path = spec_path.child("egress").index(i);

            // Validate ports
            for (j, port) in rule.ports.iter().enumerate() {
                let port_path = rule_path.child("ports").index(j);
                all_errs.extend(validate_network_policy_port(port, &port_path));
            }

            // Validate peers
            for (k, peer) in rule.to.iter().enumerate() {
                let peer_path = rule_path.child("to").index(k);
                if let Some(ref ip_block) = peer.ip_block {
                    all_errs.extend(validate_ip_block(ip_block, &peer_path.child("ipBlock")));
                }
            }
        }
    }

    all_errs
}

/// Validates a NetworkPolicyList object.
pub fn validate_network_policy_list(list: &NetworkPolicyList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_network_policy_with_path(item, &item_path));
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{IntOrString, LabelSelector, ObjectMeta, TypeMeta};

    #[test]
    fn test_validate_ingress_valid() {
        let ingress = Ingress {
            type_meta: TypeMeta {
                api_version: "networking.k8s.io/v1".to_string(),
                kind: "Ingress".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-ingress".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: None,
            status: None,
        };

        let errors = validate_ingress(&ingress);
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors.errors
        );
    }

    #[test]
    fn test_validate_ingress_backend_mutual_exclusion() {
        use crate::networking::v1::ingress::{
            HTTPIngressPath, HTTPIngressRuleValue, IngressBackend, IngressRule,
            IngressServiceBackend, IngressSpec, PathType, ServiceBackendPort,
        };

        let ingress = Ingress {
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
                            path_type: PathType::Prefix,
                            backend: IngressBackend {
                                // Invalid: both service and resource specified
                                service: Some(IngressServiceBackend {
                                    name: "my-service".to_string(),
                                    port: Some(ServiceBackendPort {
                                        number: Some(80),
                                        name: String::new(),
                                    }),
                                }),
                                resource: Some(crate::core::v1::TypedLocalObjectReference {
                                    kind: Some("StorageBucket".to_string()),
                                    name: Some("my-bucket".to_string()),
                                    api_group: Some("example.com".to_string()),
                                }),
                            },
                        }],
                    }),
                }],
                tls: vec![],
            }),
            status: None,
        };

        let errors = validate_ingress(&ingress);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.detail.contains("mutually exclusive")),
            "Expected mutual exclusivity error"
        );
    }

    #[test]
    fn test_validate_ingress_backend_missing_both() {
        use crate::networking::v1::ingress::{
            HTTPIngressPath, HTTPIngressRuleValue, IngressBackend, IngressRule, IngressSpec,
            PathType,
        };

        let ingress = Ingress {
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
                            path_type: PathType::Prefix,
                            backend: IngressBackend {
                                // Invalid: neither service nor resource specified
                                service: None,
                                resource: None,
                            },
                        }],
                    }),
                }],
                tls: vec![],
            }),
            status: None,
        };

        let errors = validate_ingress(&ingress);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.detail.contains("must be specified")),
            "Expected error about missing service or resource"
        );
    }

    #[test]
    fn test_validate_ingress_class_valid() {
        use crate::networking::v1::ingress_class::IngressClassSpec;

        let ingress_class = IngressClass {
            type_meta: TypeMeta {
                api_version: "networking.k8s.io/v1".to_string(),
                kind: "IngressClass".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-class".to_string()),
                ..Default::default()
            }),
            spec: IngressClassSpec {
                controller: "example.com/ingress-controller".to_string(),
                parameters: None,
            },
        };

        let errors = validate_ingress_class(&ingress_class);
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors.errors
        );
    }

    #[test]
    fn test_validate_ingress_class_missing_controller() {
        use crate::networking::v1::ingress_class::IngressClassSpec;

        let ingress_class = IngressClass {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-class".to_string()),
                ..Default::default()
            }),
            spec: IngressClassSpec {
                controller: String::new(), // Invalid: empty
                parameters: None,
            },
        };

        let errors = validate_ingress_class(&ingress_class);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("controller")),
            "Expected error for missing controller"
        );
    }

    #[test]
    fn test_validate_network_policy_valid() {
        use crate::networking::v1::network_policy::NetworkPolicySpec;

        let policy = NetworkPolicy {
            type_meta: TypeMeta {
                api_version: "networking.k8s.io/v1".to_string(),
                kind: "NetworkPolicy".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors.errors
        );
    }

    #[test]
    fn test_validate_network_policy_port_endport_invalid() {
        use crate::networking::v1::network_policy::{
            NetworkPolicyIngressRule, NetworkPolicyPort, NetworkPolicySpec,
        };

        let policy = NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![NetworkPolicyPort {
                        protocol: Some("TCP".to_string()),
                        port: Some(IntOrString::Int(80)),
                        end_port: Some(70), // Invalid: endPort < port
                    }],
                    from: vec![],
                }],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("endPort")),
            "Expected error for invalid endPort"
        );
    }

    #[test]
    fn test_validate_network_policy_port_endport_with_named_port() {
        use crate::networking::v1::network_policy::{
            NetworkPolicyIngressRule, NetworkPolicyPort, NetworkPolicySpec,
        };

        let policy = NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![NetworkPolicyPort {
                        protocol: Some("TCP".to_string()),
                        port: Some(IntOrString::String("http".to_string())),
                        end_port: Some(8080), // Invalid: endPort with named port
                    }],
                    from: vec![],
                }],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.detail.contains("named ports")),
            "Expected error for endPort with named port"
        );
    }

    #[test]
    fn test_validate_ip_block_invalid_cidr() {
        use crate::networking::v1::network_policy::{
            IPBlock, NetworkPolicyIngressRule, NetworkPolicyPeer, NetworkPolicySpec,
        };

        let policy = NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                ..Default::default()
            }),
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector::default(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![],
                    from: vec![NetworkPolicyPeer {
                        pod_selector: None,
                        namespace_selector: None,
                        ip_block: Some(IPBlock {
                            cidr: "invalid-cidr".to_string(), // Invalid: no / separator
                            except: vec![],
                        }),
                    }],
                }],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let errors = validate_network_policy(&policy);
        assert!(!errors.is_empty(), "Expected validation errors");
        assert!(
            errors
                .errors
                .iter()
                .any(|e| e.field.to_string().contains("cidr")),
            "Expected error for invalid CIDR"
        );
    }
}
