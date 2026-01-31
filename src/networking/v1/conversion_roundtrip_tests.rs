use super::{
    Ingress, IngressList, IngressSpec, NetworkPolicy, NetworkPolicyList, NetworkPolicySpec,
};
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, IntOrString, LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use crate::networking::internal;
use crate::networking::v1::ingress::{
    HTTPIngressPath, HTTPIngressRuleValue, IngressBackend, IngressRule, IngressServiceBackend,
    PathType, ServiceBackendPort,
};
use crate::networking::v1::network_policy::{
    NetworkPolicyEgressRule, NetworkPolicyIngressRule, NetworkPolicyPeer, NetworkPolicyPort,
    PolicyType,
};

fn ingress_basic() -> Ingress {
    Ingress {
        type_meta: TypeMeta::default(),
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
            tls: vec![],
        }),
        status: None,
    }
}

fn ingress_list_basic() -> IngressList {
    let mut item = ingress_basic();
    item.apply_default();

    IngressList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn network_policy_basic() -> NetworkPolicy {
    let mut selector = LabelSelector::default();
    selector
        .match_labels
        .insert("app".to_string(), "web".to_string());

    NetworkPolicy {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("web-policy".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(NetworkPolicySpec {
            pod_selector: selector.clone(),
            ingress: vec![NetworkPolicyIngressRule {
                ports: vec![NetworkPolicyPort {
                    protocol: None,
                    port: Some(IntOrString::Int(80)),
                    end_port: None,
                }],
                from: vec![NetworkPolicyPeer {
                    pod_selector: Some(selector.clone()),
                    namespace_selector: None,
                    ip_block: None,
                }],
            }],
            egress: vec![NetworkPolicyEgressRule {
                ports: vec![NetworkPolicyPort {
                    protocol: None,
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
    let mut item = network_policy_basic();
    item.apply_default();

    NetworkPolicyList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_ingress() {
    assert_conversion_roundtrip::<Ingress, internal::Ingress>(ingress_basic());
}

#[test]
fn conversion_roundtrip_ingress_list() {
    assert_conversion_roundtrip::<IngressList, internal::IngressList>(ingress_list_basic());
}

#[test]
fn conversion_roundtrip_network_policy() {
    assert_conversion_roundtrip::<NetworkPolicy, internal::NetworkPolicy>(network_policy_basic());
}

#[test]
fn conversion_roundtrip_network_policy_list() {
    assert_conversion_roundtrip::<NetworkPolicyList, internal::NetworkPolicyList>(
        network_policy_list_basic(),
    );
}
