use super::*;
use crate::networking::v1::{IPAddress, Ingress, IngressClass, NetworkPolicy, ServiceCIDR};

// Tests
// ============================================================================
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
        HTTPIngressPath, HTTPIngressRuleValue, IngressBackend, IngressRule, IngressServiceBackend,
        IngressSpec, PathType, ServiceBackendPort,
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
            .any(|e| e.detail.contains("cannot set both service and resource")),
        "Expected service/resource exclusivity error"
    );
}

#[test]
fn test_validate_ingress_backend_missing_both() {
    use crate::networking::v1::ingress::{
        HTTPIngressPath, HTTPIngressRuleValue, IngressBackend, IngressRule, IngressSpec, PathType,
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
            .any(|e| e.detail.contains("service or resource backend is required")),
        "Expected error about missing backend"
    );
}

#[test]
fn test_validate_ingress_requires_rules_or_default_backend() {
    use crate::networking::v1::ingress::IngressSpec;

    let ingress = Ingress {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-ingress".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(IngressSpec {
            ingress_class_name: None,
            default_backend: None,
            rules: vec![],
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
            .any(|e| e.detail.contains("defaultBackend") || e.detail.contains("rules")),
        "Expected error about missing rules or default backend"
    );
}

#[test]
fn test_validate_ingress_path_invalid_sequence() {
    use crate::networking::v1::ingress::{
        HTTPIngressPath, HTTPIngressRuleValue, IngressBackend, IngressRule, IngressServiceBackend,
        IngressSpec, PathType, ServiceBackendPort,
    };

    let ingress = Ingress {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-ingress".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(IngressSpec {
            ingress_class_name: None,
            default_backend: None,
            rules: vec![IngressRule {
                host: "example.com".to_string(),
                http: Some(HTTPIngressRuleValue {
                    paths: vec![HTTPIngressPath {
                        path: "/../".to_string(),
                        path_type: PathType::Prefix,
                        backend: IngressBackend {
                            service: Some(IngressServiceBackend {
                                name: "svc".to_string(),
                                port: Some(ServiceBackendPort {
                                    number: Some(80),
                                    name: String::new(),
                                }),
                            }),
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
            .any(|e| e.detail.contains("must not contain")),
        "Expected error about invalid path sequence"
    );
}

#[test]
fn test_validate_ingress_tls_invalid_wildcard() {
    use crate::networking::v1::ingress::IngressSpec;
    use crate::networking::v1::ingress::IngressTLS;

    let ingress = Ingress {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-ingress".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(IngressSpec {
            ingress_class_name: None,
            default_backend: None,
            rules: vec![],
            tls: vec![IngressTLS {
                hosts: vec!["foo.*.example.com".to_string()],
                secret_name: None,
            }],
        }),
        status: None,
    };

    let errors = validate_ingress(&ingress);
    assert!(!errors.is_empty(), "Expected validation errors");
    assert!(
        errors.errors.iter().any(|e| e.detail.contains("wildcard")),
        "Expected wildcard validation error"
    );
}

#[test]
fn test_validate_ingress_update_requires_resource_version() {
    use crate::networking::v1::ingress::IngressSpec;

    let old = Ingress {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-ingress".to_string()),
            namespace: Some("default".to_string()),
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        spec: Some(IngressSpec::default()),
        status: None,
    };

    let mut new = old.clone();
    if let Some(ref mut meta) = new.metadata {
        meta.resource_version = None;
    }

    let errors = validate_ingress_update(&new, &old);
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field.contains("metadata.resourceVersion")),
        "Expected resourceVersion error"
    );
}

#[test]
fn test_validate_ingress_status_update_hostname_ip_rejected() {
    use crate::networking::v1::ingress::{
        IngressLoadBalancerIngress, IngressLoadBalancerStatus, IngressStatus,
    };

    let old = Ingress {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-ingress".to_string()),
            namespace: Some("default".to_string()),
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        spec: None,
        status: None,
    };

    let mut new = old.clone();
    new.status = Some(IngressStatus {
        load_balancer: Some(IngressLoadBalancerStatus {
            ingress: vec![IngressLoadBalancerIngress {
                ip: String::new(),
                hostname: "1.2.3.4".to_string(),
                ports: vec![],
            }],
        }),
    });

    let errors = validate_ingress_status_update(&new, &old);
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field.contains("status.loadBalancer.ingress[0].hostname")),
        "Expected hostname IP validation error"
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
fn test_validate_ingress_class_parameters_scope_required() {
    use crate::networking::v1::ingress_class::{IngressClassParametersReference, IngressClassSpec};

    let ingress_class = IngressClass {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-class".to_string()),
            ..Default::default()
        }),
        spec: IngressClassSpec {
            controller: "example.com/ingress-controller".to_string(),
            parameters: Some(IngressClassParametersReference {
                api_group: Some("example.com".to_string()),
                kind: "BackendConfig".to_string(),
                name: "default".to_string(),
                namespace: None,
                scope: None,
            }),
        },
    };

    let errors = validate_ingress_class(&ingress_class);
    assert!(!errors.is_empty(), "Expected validation errors");
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field.to_string().contains("scope")),
        "Expected error for missing parameters.scope"
    );
}

#[test]
fn test_validate_ingress_class_update_controller_immutable() {
    use crate::networking::v1::ingress_class::IngressClassSpec;

    let old = IngressClass {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-class".to_string()),
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        spec: IngressClassSpec {
            controller: "example.com/ingress-controller".to_string(),
            parameters: None,
        },
    };

    let mut new = old.clone();
    new.spec.controller = "example.com/other".to_string();

    let errors = validate_ingress_class_update(&new, &old);
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field.contains("spec.controller")),
        "Expected immutable controller error"
    );
}

#[test]
fn test_validate_ingress_class_parameters_scope_defaulted() {
    use crate::common::traits::ApplyDefault;
    use crate::networking::v1::ingress_class::{IngressClassParametersReference, IngressClassSpec};

    let mut ingress_class = IngressClass {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-class".to_string()),
            ..Default::default()
        }),
        spec: IngressClassSpec {
            controller: "example.com/ingress-controller".to_string(),
            parameters: Some(IngressClassParametersReference {
                api_group: Some("example.com".to_string()),
                kind: "BackendConfig".to_string(),
                name: "default".to_string(),
                namespace: None,
                scope: None,
            }),
        },
    };

    ingress_class.apply_default();

    let errors = validate_ingress_class(&ingress_class);
    assert!(
        errors.is_empty(),
        "Expected validation to pass after defaulting, got: {:?}",
        errors.errors
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
fn test_validate_network_policy_peer_requires_one() {
    use crate::networking::v1::network_policy::{NetworkPolicyIngressRule, NetworkPolicySpec};

    let policy = NetworkPolicy {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-policy".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(NetworkPolicySpec {
            pod_selector: LabelSelector::default(),
            ingress: vec![NetworkPolicyIngressRule {
                ports: vec![],
                from: vec![crate::networking::v1::network_policy::NetworkPolicyPeer::default()],
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
            .any(|e| e.detail.contains("must specify a peer")),
        "Expected peer requirement error"
    );
}

#[test]
fn test_validate_network_policy_invalid_protocol() {
    use crate::networking::v1::network_policy::{
        NetworkPolicyIngressRule, NetworkPolicyPort, NetworkPolicySpec,
    };

    let policy = NetworkPolicy {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-policy".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(NetworkPolicySpec {
            pod_selector: LabelSelector::default(),
            ingress: vec![NetworkPolicyIngressRule {
                ports: vec![NetworkPolicyPort {
                    protocol: Some("ICMP".to_string()),
                    port: Some(IntOrString::Int(80)),
                    end_port: None,
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
            .any(|e| e.field.to_string().contains("protocol")),
        "Expected protocol validation error"
    );
}

#[test]
fn test_validate_network_policy_port_invalid_number() {
    use crate::networking::v1::network_policy::{
        NetworkPolicyIngressRule, NetworkPolicyPort, NetworkPolicySpec,
    };

    let policy = NetworkPolicy {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-policy".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(NetworkPolicySpec {
            pod_selector: LabelSelector::default(),
            ingress: vec![NetworkPolicyIngressRule {
                ports: vec![NetworkPolicyPort {
                    protocol: Some("TCP".to_string()),
                    port: Some(IntOrString::Int(70000)),
                    end_port: None,
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
            .any(|e| e.field.to_string().contains("port")),
        "Expected port number validation error"
    );
}

#[test]
fn test_validate_network_policy_policy_types_limit() {
    use crate::networking::v1::network_policy::{NetworkPolicySpec, PolicyType};

    let policy = NetworkPolicy {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-policy".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(NetworkPolicySpec {
            pod_selector: LabelSelector::default(),
            ingress: vec![],
            egress: vec![],
            policy_types: vec![PolicyType::Ingress, PolicyType::Egress, PolicyType::Ingress],
        }),
    };

    let errors = validate_network_policy(&policy);
    assert!(!errors.is_empty(), "Expected validation errors");
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.detail.contains("policyTypes")),
        "Expected policyTypes length error"
    );
}

#[test]
fn test_validate_network_policy_policy_types_duplicate() {
    use crate::common::validation::ErrorType;
    use crate::networking::v1::network_policy::{NetworkPolicySpec, PolicyType};

    let policy = NetworkPolicy {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-policy".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(NetworkPolicySpec {
            pod_selector: LabelSelector::default(),
            ingress: vec![],
            egress: vec![],
            policy_types: vec![PolicyType::Ingress, PolicyType::Ingress],
        }),
    };

    let errors = validate_network_policy(&policy);
    assert!(!errors.is_empty(), "Expected validation errors");
    assert!(
        errors
            .errors
            .iter()
            .any(|e| { e.error_type == ErrorType::Duplicate && e.field.contains("policyTypes") }),
        "Expected duplicate policyTypes error"
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
            .any(|e| e.detail.contains("non-numeric")),
        "Expected error for endPort with named port"
    );
}

#[test]
fn test_validate_network_policy_update_requires_resource_version() {
    use crate::networking::v1::network_policy::NetworkPolicySpec;

    let old = NetworkPolicy {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("test-policy".to_string()),
            namespace: Some("default".to_string()),
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        spec: Some(NetworkPolicySpec::default()),
    };

    let mut new = old.clone();
    if let Some(ref mut meta) = new.metadata {
        meta.resource_version = None;
    }

    let errors = validate_network_policy_update(&new, &old);
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field.contains("metadata.resourceVersion")),
        "Expected resourceVersion error"
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

#[test]
fn test_validate_ip_address_valid() {
    use crate::networking::v1::ip_address::{IPAddressSpec, ParentReference};

    let ip_address = IPAddress {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("192.168.0.1".to_string()),
            ..Default::default()
        }),
        spec: IPAddressSpec {
            parent_ref: Some(ParentReference {
                group: String::new(),
                resource: "services".to_string(),
                namespace: "default".to_string(),
                name: "kube-dns".to_string(),
            }),
        },
    };

    let errors = validate_ip_address(&ip_address);
    assert!(errors.is_empty(), "Expected no validation errors");
}

#[test]
fn test_validate_ip_address_non_canonical_name() {
    use crate::networking::v1::ip_address::{IPAddressSpec, ParentReference};

    let ip_address = IPAddress {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("2001:0db8::1".to_string()),
            ..Default::default()
        }),
        spec: IPAddressSpec {
            parent_ref: Some(ParentReference {
                group: String::new(),
                resource: "services".to_string(),
                namespace: "default".to_string(),
                name: "kube-dns".to_string(),
            }),
        },
    };

    let errors = validate_ip_address(&ip_address);
    assert!(!errors.is_empty(), "Expected validation errors");
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field.to_string().contains("metadata.name")),
        "Expected metadata.name error"
    );
}

#[test]
fn test_validate_ip_address_parent_ref_required() {
    use crate::networking::v1::ip_address::IPAddressSpec;

    let ip_address = IPAddress {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("192.168.0.1".to_string()),
            ..Default::default()
        }),
        spec: IPAddressSpec { parent_ref: None },
    };

    let errors = validate_ip_address(&ip_address);
    assert!(!errors.is_empty(), "Expected validation errors");
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field.to_string().contains("spec.parentRef")),
        "Expected parentRef required error"
    );
}

#[test]
fn test_validate_service_cidr_requires_cidrs() {
    use crate::networking::v1::service_cidr::ServiceCIDRSpec;

    let service_cidr = ServiceCIDR {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("service-cidr".to_string()),
            ..Default::default()
        }),
        spec: ServiceCIDRSpec { cidrs: vec![] },
        status: None,
    };

    let errors = validate_service_cidr(&service_cidr);
    assert!(!errors.is_empty(), "Expected validation errors");
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field.to_string().contains("spec.cidrs")),
        "Expected cidrs required error"
    );
}

#[test]
fn test_validate_service_cidr_dual_stack_same_family() {
    use crate::networking::v1::service_cidr::ServiceCIDRSpec;

    let service_cidr = ServiceCIDR {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("service-cidr".to_string()),
            ..Default::default()
        }),
        spec: ServiceCIDRSpec {
            cidrs: vec!["10.0.0.0/24".to_string(), "10.1.0.0/24".to_string()],
        },
        status: None,
    };

    let errors = validate_service_cidr(&service_cidr);
    assert!(!errors.is_empty(), "Expected validation errors");
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.detail.contains("one IP for each IP family")),
        "Expected dual stack family error"
    );
}
