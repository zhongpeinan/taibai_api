use super::{
    ConfigMap, ConfigMapList, Container, ContainerPort, Namespace, NamespaceList, NamespaceSpec,
    NamespaceStatus, Node, NodeList, NodeSpec, NodeStatus, PersistentVolume, PersistentVolumeClaim,
    PersistentVolumeClaimList, PersistentVolumeClaimSpec, PersistentVolumeClaimStatus,
    PersistentVolumeList, PersistentVolumeSource, PersistentVolumeSpec, PersistentVolumeStatus,
    Pod, PodList, PodSpec, Secret, SecretList, Service, ServiceAccount, ServiceAccountList,
    ServiceList, ServicePort, ServiceSpec, VolumeResourceRequirements,
};
use super::{persistent_volume_access_mode, secret_type};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{IntOrString, ListMeta, ObjectMeta, Quantity, TypeMeta};
use crate::core::internal::ByteString;
use crate::core::internal::{ServiceAffinity, ServiceType};
use crate::core::v1::reference::{LocalObjectReference, ObjectReference};
use crate::core::v1::volume::LocalVolumeSource;
use std::collections::BTreeMap;

fn pod_basic() -> Pod {
    Pod {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "Pod".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("demo-pod".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(PodSpec {
            containers: vec![Container {
                name: "app".to_string(),
                image: Some("nginx:1.25".to_string()),
                ports: vec![ContainerPort {
                    name: Some("http".to_string()),
                    container_port: 80,
                    protocol: Some("TCP".to_string()),
                    host_port: None,
                    host_ip: None,
                }],
                ..Default::default()
            }],
            service_account_name: Some("default".to_string()),
            node_selector: BTreeMap::from([("disk".to_string(), "ssd".to_string())]),
            ..Default::default()
        }),
        status: None,
    }
}

fn pod_list_basic() -> PodList {
    PodList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "PodList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![pod_basic()],
    }
}

fn service_basic() -> Service {
    Service {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "Service".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("web".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(ServiceSpec {
            ports: vec![ServicePort {
                name: "http".to_string(),
                protocol: "TCP".to_string(),
                app_protocol: None,
                port: 80,
                target_port: Some(IntOrString::Int(8080)),
                node_port: None,
            }],
            selector: BTreeMap::from([("app".to_string(), "web".to_string())]),
            type_: Some(ServiceType::ClusterIp),
            session_affinity: ServiceAffinity::None,
            ..Default::default()
        }),
        status: None,
    }
}

fn service_list_basic() -> ServiceList {
    ServiceList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ServiceList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![service_basic()],
    }
}

fn node_basic() -> Node {
    Node {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "Node".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("node-a".to_string()),
            ..Default::default()
        }),
        spec: Some(NodeSpec {
            pod_cidr: Some("10.0.0.0/24".to_string()),
            provider_id: Some("aws:///i-1234567890".to_string()),
            ..Default::default()
        }),
        status: Some(NodeStatus {
            capacity: BTreeMap::from([
                ("cpu".to_string(), Quantity("4".to_string())),
                ("memory".to_string(), Quantity("16Gi".to_string())),
            ]),
            ..Default::default()
        }),
    }
}

fn node_list_basic() -> NodeList {
    NodeList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "NodeList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("3".to_string()),
            ..Default::default()
        }),
        items: vec![node_basic()],
    }
}

fn config_map_basic() -> ConfigMap {
    ConfigMap {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ConfigMap".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("app-config".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        data: BTreeMap::from([("config".to_string(), "port=8080".to_string())]),
        binary_data: BTreeMap::from([("payload".to_string(), ByteString::from(b"data".to_vec()))]),
        immutable: Some(true),
    }
}

fn config_map_list_basic() -> ConfigMapList {
    ConfigMapList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ConfigMapList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("4".to_string()),
            ..Default::default()
        }),
        items: vec![config_map_basic()],
    }
}

fn secret_basic() -> Secret {
    Secret {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "Secret".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("app-secret".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        immutable: Some(false),
        data: BTreeMap::from([("token".to_string(), ByteString::from(b"secret".to_vec()))]),
        string_data: BTreeMap::new(),
        type_: Some(secret_type::OPAQUE.to_string()),
    }
}

fn secret_list_basic() -> SecretList {
    SecretList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "SecretList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("5".to_string()),
            ..Default::default()
        }),
        items: vec![secret_basic()],
    }
}

fn namespace_basic() -> Namespace {
    Namespace {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "Namespace".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("dev".to_string()),
            ..Default::default()
        }),
        spec: Some(NamespaceSpec {
            finalizers: vec!["kubernetes".to_string()],
        }),
        status: Some(NamespaceStatus {
            phase: Some("Active".to_string()),
            ..Default::default()
        }),
    }
}

fn namespace_list_basic() -> NamespaceList {
    NamespaceList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "NamespaceList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("6".to_string()),
            ..Default::default()
        }),
        items: vec![namespace_basic()],
    }
}

fn service_account_basic() -> ServiceAccount {
    ServiceAccount {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ServiceAccount".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("builder".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        secrets: vec![ObjectReference {
            name: Some("builder-token".to_string()),
            ..Default::default()
        }],
        image_pull_secrets: vec![LocalObjectReference {
            name: Some("registry".to_string()),
        }],
        automount_service_account_token: Some(true),
    }
}

fn service_account_list_basic() -> ServiceAccountList {
    ServiceAccountList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "ServiceAccountList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("7".to_string()),
            ..Default::default()
        }),
        items: vec![service_account_basic()],
    }
}

fn persistent_volume_basic() -> PersistentVolume {
    PersistentVolume {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "PersistentVolume".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("pv-data".to_string()),
            ..Default::default()
        }),
        spec: Some(PersistentVolumeSpec {
            capacity: BTreeMap::from([("storage".to_string(), Quantity("10Gi".to_string()))]),
            access_modes: vec![persistent_volume_access_mode::READ_WRITE_ONCE.to_string()],
            persistent_volume_source: Some(PersistentVolumeSource {
                local: Some(LocalVolumeSource {
                    path: "/data".to_string(),
                    fs_type: Some("ext4".to_string()),
                }),
                ..Default::default()
            }),
            storage_class_name: Some("standard".to_string()),
            ..Default::default()
        }),
        status: Some(PersistentVolumeStatus {
            phase: Some("Available".to_string()),
            ..Default::default()
        }),
    }
}

fn persistent_volume_list_basic() -> PersistentVolumeList {
    PersistentVolumeList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "PersistentVolumeList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("8".to_string()),
            ..Default::default()
        }),
        items: vec![persistent_volume_basic()],
    }
}

fn persistent_volume_claim_basic() -> PersistentVolumeClaim {
    PersistentVolumeClaim {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "PersistentVolumeClaim".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("data-claim".to_string()),
            namespace: Some("default".to_string()),
            ..Default::default()
        }),
        spec: Some(PersistentVolumeClaimSpec {
            access_modes: vec![persistent_volume_access_mode::READ_WRITE_ONCE.to_string()],
            resources: Some(VolumeResourceRequirements {
                requests: BTreeMap::from([("storage".to_string(), Quantity("1Gi".to_string()))]),
                ..Default::default()
            }),
            storage_class_name: Some("standard".to_string()),
            ..Default::default()
        }),
        status: Some(PersistentVolumeClaimStatus {
            phase: Some("Bound".to_string()),
            ..Default::default()
        }),
    }
}

fn persistent_volume_claim_list_basic() -> PersistentVolumeClaimList {
    PersistentVolumeClaimList {
        type_meta: TypeMeta {
            api_version: "v1".to_string(),
            kind: "PersistentVolumeClaimList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("9".to_string()),
            ..Default::default()
        }),
        items: vec![persistent_volume_claim_basic()],
    }
}

#[test]
fn serde_roundtrip_pod() {
    assert_serde_roundtrip(&pod_basic());
}

#[test]
fn serde_roundtrip_pod_list() {
    assert_serde_roundtrip(&pod_list_basic());
}

#[test]
fn serde_roundtrip_service() {
    assert_serde_roundtrip(&service_basic());
}

#[test]
fn serde_roundtrip_service_list() {
    assert_serde_roundtrip(&service_list_basic());
}

#[test]
fn serde_roundtrip_node() {
    assert_serde_roundtrip(&node_basic());
}

#[test]
fn serde_roundtrip_node_list() {
    assert_serde_roundtrip(&node_list_basic());
}

#[test]
fn serde_roundtrip_config_map() {
    assert_serde_roundtrip(&config_map_basic());
}

#[test]
fn serde_roundtrip_config_map_list() {
    assert_serde_roundtrip(&config_map_list_basic());
}

#[test]
fn serde_roundtrip_secret() {
    assert_serde_roundtrip(&secret_basic());
}

#[test]
fn serde_roundtrip_secret_list() {
    assert_serde_roundtrip(&secret_list_basic());
}

#[test]
fn serde_roundtrip_namespace() {
    assert_serde_roundtrip(&namespace_basic());
}

#[test]
fn serde_roundtrip_namespace_list() {
    assert_serde_roundtrip(&namespace_list_basic());
}

#[test]
fn serde_roundtrip_service_account() {
    assert_serde_roundtrip(&service_account_basic());
}

#[test]
fn serde_roundtrip_service_account_list() {
    assert_serde_roundtrip(&service_account_list_basic());
}

#[test]
fn serde_roundtrip_persistent_volume() {
    assert_serde_roundtrip(&persistent_volume_basic());
}

#[test]
fn serde_roundtrip_persistent_volume_list() {
    assert_serde_roundtrip(&persistent_volume_list_basic());
}

#[test]
fn serde_roundtrip_persistent_volume_claim() {
    assert_serde_roundtrip(&persistent_volume_claim_basic());
}

#[test]
fn serde_roundtrip_persistent_volume_claim_list() {
    assert_serde_roundtrip(&persistent_volume_claim_list_basic());
}
