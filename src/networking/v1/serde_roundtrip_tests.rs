use super::{
    Ingress, IngressList, IngressSpec, IngressStatus, NetworkPolicy, NetworkPolicyList,
    NetworkPolicySpec,
};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{IntOrString, LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use crate::networking::v1::ingress::{
    HTTPIngressPath, HTTPIngressRuleValue, IngressBackend, IngressLoadBalancerIngress,
    IngressLoadBalancerStatus, IngressPortStatus, IngressRule, IngressServiceBackend, IngressTLS,
    PathType, ServiceBackendPort,
};
use crate::networking::v1::network_policy::{
    IPBlock, NetworkPolicyEgressRule, NetworkPolicyIngressRule, NetworkPolicyPeer,
    NetworkPolicyPort, PolicyType,
};

fn ingress_basic() -> Ingress {
    Ingress {
        type_meta: TypeMeta {
            api_version: "networking.k8s.io/v1".to_string(),
            kind: "Ingress".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("example-ingress".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(IngressSpec {
            ingress_class_name: Some("nginx".to_string()),
            default_backend: Some(IngressBackend {
                service: Some(IngressServiceBackend {
                    name: "default-service".to_string(),
                    port: Some(ServiceBackendPort {
                        name: "http".to_string(),
                        number: None,
                    }),
                }),
                resource: None,
            }),
            rules: vec![IngressRule {
                host: "example.com".to_string(),
                http: Some(HTTPIngressRuleValue {
                    paths: vec![HTTPIngressPath {
                        path: "/api".to_string(),
                        path_type: PathType::Prefix,
                        backend: IngressBackend {
                            service: Some(IngressServiceBackend {
                                name: "api-service".to_string(),
                                port: Some(ServiceBackendPort {
                                    name: String::new(),
                                    number: Some(8080),
                                }),
                            }),
                            resource: None,
                        },
                    }],
                }),
            }],
            tls: vec![IngressTLS {
                hosts: vec!["example.com".to_string()],
                secret_name: Some("tls-secret".to_string()),
            }],
        }),
        status: Some(IngressStatus {
            load_balancer: Some(IngressLoadBalancerStatus {
                ingress: vec![IngressLoadBalancerIngress {
                    ip: "10.0.0.1".to_string(),
                    hostname: "lb.example.com".to_string(),
                    ports: vec![IngressPortStatus {
                        port: 443,
                        protocol: "TCP".to_string(),
                        error: None,
                    }],
                }],
            }),
        }),
    }
}

fn ingress_list_basic() -> IngressList {
    IngressList {
        type_meta: TypeMeta {
            api_version: "networking.k8s.io/v1".to_string(),
            kind: "IngressList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![ingress_basic()],
    }
}

fn network_policy_basic() -> NetworkPolicy {
    let mut selector = LabelSelector::default();
    selector
        .match_labels
        .insert("app".to_string(), "web".to_string());

    NetworkPolicy {
        type_meta: TypeMeta {
            api_version: "networking.k8s.io/v1".to_string(),
            kind: "NetworkPolicy".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("web-policy".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(NetworkPolicySpec {
            pod_selector: selector.clone(),
            ingress: vec![NetworkPolicyIngressRule {
                ports: vec![NetworkPolicyPort {
                    protocol: Some("TCP".to_string()),
                    port: Some(IntOrString::Int(80)),
                    end_port: None,
                }],
                from: vec![NetworkPolicyPeer {
                    pod_selector: Some(selector.clone()),
                    namespace_selector: None,
                    ip_block: Some(IPBlock {
                        cidr: "10.0.0.0/24".to_string(),
                        except: vec!["10.0.0.10/32".to_string()],
                    }),
                }],
            }],
            egress: vec![NetworkPolicyEgressRule {
                ports: vec![NetworkPolicyPort {
                    protocol: Some("TCP".to_string()),
                    port: Some(IntOrString::String("https".to_string())),
                    end_port: None,
                }],
                to: vec![NetworkPolicyPeer {
                    pod_selector: None,
                    namespace_selector: Some(selector),
                    ip_block: None,
                }],
            }],
            policy_types: vec![PolicyType::Ingress, PolicyType::Egress],
        }),
    }
}

fn network_policy_list_basic() -> NetworkPolicyList {
    NetworkPolicyList {
        type_meta: TypeMeta {
            api_version: "networking.k8s.io/v1".to_string(),
            kind: "NetworkPolicyList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![network_policy_basic()],
    }
}

#[test]
fn serde_roundtrip_ingress() {
    assert_serde_roundtrip(&ingress_basic());
}

#[test]
fn serde_roundtrip_ingress_list() {
    assert_serde_roundtrip(&ingress_list_basic());
}

#[test]
fn serde_roundtrip_network_policy() {
    assert_serde_roundtrip(&network_policy_basic());
}

#[test]
fn serde_roundtrip_network_policy_list() {
    assert_serde_roundtrip(&network_policy_list_basic());
}
