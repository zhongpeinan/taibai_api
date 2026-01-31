use super::{
    ConfigMap, ConfigMapList, Container, ContainerPort, Namespace, NamespaceList, NamespaceSpec,
    NamespaceStatus, Node, NodeDaemonEndpoints, NodeList, NodeSpec, NodeStatus, NodeSystemInfo,
    PersistentVolume, PersistentVolumeClaim, PersistentVolumeClaimList, PersistentVolumeClaimSpec,
    PersistentVolumeClaimStatus, PersistentVolumeList, PersistentVolumeSource,
    PersistentVolumeSpec, PersistentVolumeStatus, Pod, PodList, PodSpec, PodStatus, Secret,
    SecretList, Service, ServiceAccount, ServiceAccountList, ServiceList, ServicePort, ServiceSpec,
    ServiceStatus, VolumeResourceRequirements,
};
use super::{persistent_volume_access_mode, secret_type};
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{ApplyDefault, IntOrString, ListMeta, ObjectMeta, Quantity, TypeMeta};
use crate::core::internal::{self, ByteString, ServiceAffinity, ServiceType};
use crate::core::v1::reference::{LocalObjectReference, ObjectReference};
use crate::core::v1::volume::LocalVolumeSource;
use std::collections::BTreeMap;

fn pod_basic() -> Pod {
    Pod {
        type_meta: TypeMeta::default(),
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
        status: Some(PodStatus {
            phase: Some("Pending".to_string()),
            ..Default::default()
        }),
    }
}

fn pod_list_basic() -> PodList {
    let mut item = pod_basic();
    item.apply_default();
    PodList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn service_basic() -> Service {
    Service {
        type_meta: TypeMeta::default(),
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
        status: Some(ServiceStatus::default()),
    }
}

fn service_list_basic() -> ServiceList {
    let mut item = service_basic();
    item.apply_default();
    ServiceList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn node_basic() -> Node {
    Node {
        type_meta: TypeMeta::default(),
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
            phase: Some("Pending".to_string()),
            daemon_endpoints: Some(NodeDaemonEndpoints::default()),
            node_info: Some(NodeSystemInfo::default()),
            ..Default::default()
        }),
    }
}

fn node_list_basic() -> NodeList {
    let mut item = node_basic();
    item.apply_default();
    NodeList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("3".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn config_map_basic() -> ConfigMap {
    ConfigMap {
        type_meta: TypeMeta::default(),
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
    let mut item = config_map_basic();
    item.apply_default();
    ConfigMapList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("4".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn secret_basic() -> Secret {
    Secret {
        type_meta: TypeMeta::default(),
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
    let mut item = secret_basic();
    item.apply_default();
    SecretList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("5".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn namespace_basic() -> Namespace {
    Namespace {
        type_meta: TypeMeta::default(),
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
    let mut item = namespace_basic();
    item.apply_default();
    NamespaceList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("6".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn service_account_basic() -> ServiceAccount {
    ServiceAccount {
        type_meta: TypeMeta::default(),
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
    let mut item = service_account_basic();
    item.apply_default();
    ServiceAccountList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("7".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn persistent_volume_basic() -> PersistentVolume {
    PersistentVolume {
        type_meta: TypeMeta::default(),
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
    let mut item = persistent_volume_basic();
    item.apply_default();
    PersistentVolumeList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("8".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

fn persistent_volume_claim_basic() -> PersistentVolumeClaim {
    PersistentVolumeClaim {
        type_meta: TypeMeta::default(),
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
    let mut item = persistent_volume_claim_basic();
    item.apply_default();
    PersistentVolumeClaimList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("9".to_string()),
            ..Default::default()
        }),
        items: vec![item],
    }
}

#[test]
fn conversion_roundtrip_pod() {
    assert_conversion_roundtrip::<Pod, internal::pod::Pod>(pod_basic());
}

#[test]
fn conversion_roundtrip_pod_list() {
    assert_conversion_roundtrip::<PodList, internal::pod::PodList>(pod_list_basic());
}

#[test]
fn conversion_roundtrip_service() {
    assert_conversion_roundtrip::<Service, internal::service::Service>(service_basic());
}

#[test]
fn conversion_roundtrip_service_list() {
    assert_conversion_roundtrip::<ServiceList, internal::service::ServiceList>(service_list_basic());
}

#[test]
fn conversion_roundtrip_node() {
    assert_conversion_roundtrip::<Node, internal::node::Node>(node_basic());
}

#[test]
fn conversion_roundtrip_node_list() {
    assert_conversion_roundtrip::<NodeList, internal::node::NodeList>(node_list_basic());
}

#[test]
fn conversion_roundtrip_config_map() {
    assert_conversion_roundtrip::<ConfigMap, internal::config::ConfigMap>(config_map_basic());
}

#[test]
fn conversion_roundtrip_config_map_list() {
    assert_conversion_roundtrip::<ConfigMapList, internal::config::ConfigMapList>(
        config_map_list_basic(),
    );
}

#[test]
fn conversion_roundtrip_secret() {
    assert_conversion_roundtrip::<Secret, internal::config::Secret>(secret_basic());
}

#[test]
fn conversion_roundtrip_secret_list() {
    assert_conversion_roundtrip::<SecretList, internal::config::SecretList>(secret_list_basic());
}

#[test]
fn conversion_roundtrip_namespace() {
    assert_conversion_roundtrip::<Namespace, internal::namespace::Namespace>(namespace_basic());
}

#[test]
fn conversion_roundtrip_namespace_list() {
    assert_conversion_roundtrip::<NamespaceList, internal::namespace::NamespaceList>(
        namespace_list_basic(),
    );
}

#[test]
fn conversion_roundtrip_service_account() {
    assert_conversion_roundtrip::<ServiceAccount, internal::config::ServiceAccount>(
        service_account_basic(),
    );
}

#[test]
fn conversion_roundtrip_service_account_list() {
    assert_conversion_roundtrip::<ServiceAccountList, internal::config::ServiceAccountList>(
        service_account_list_basic(),
    );
}

#[test]
fn conversion_roundtrip_persistent_volume() {
    assert_conversion_roundtrip::<PersistentVolume, internal::persistent_volume::PersistentVolume>(
        persistent_volume_basic(),
    );
}

#[test]
fn conversion_roundtrip_persistent_volume_list() {
    assert_conversion_roundtrip::<
        PersistentVolumeList,
        internal::persistent_volume::PersistentVolumeList,
    >(persistent_volume_list_basic());
}

#[test]
fn conversion_roundtrip_persistent_volume_claim() {
    assert_conversion_roundtrip::<
        PersistentVolumeClaim,
        internal::persistent_volume::PersistentVolumeClaim,
    >(persistent_volume_claim_basic());
}

#[test]
fn conversion_roundtrip_persistent_volume_claim_list() {
    assert_conversion_roundtrip::<
        PersistentVolumeClaimList,
        internal::persistent_volume::PersistentVolumeClaimList,
    >(persistent_volume_claim_list_basic());
}
