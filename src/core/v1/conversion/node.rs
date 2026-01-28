//! Node conversion implementations
//!
//! Includes: Node, NodeList, NodeSpec, NodeStatus, Taint, NodeCondition, NodeAddress,
//! NodeSystemInfo, NodeDaemonEndpoints, NodeConfigSource, and related types

use super::helpers::*;
use crate::common::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::node;

// ============================================================================
// Node and NodeList
// ============================================================================

impl ToInternal<internal::Node> for node::Node {
    fn to_internal(self) -> internal::Node {
        internal::Node {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(|s| s.to_internal()).unwrap_or_default(),
            status: self.status.map(|s| s.to_internal()).unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::Node> for node::Node {
    fn from_internal(value: internal::Node) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: Some(node::NodeSpec::from_internal(value.spec)),
            status: Some(node::NodeStatus::from_internal(value.status)),
        };
        result.apply_default();
        result
    }
}

impl ToInternal<internal::NodeList> for node::NodeList {
    fn to_internal(self) -> internal::NodeList {
        internal::NodeList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::NodeList> for node::NodeList {
    fn from_internal(value: internal::NodeList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(node::Node::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// NodeSpec
// ============================================================================

impl ToInternal<internal::NodeSpec> for node::NodeSpec {
    fn to_internal(self) -> internal::NodeSpec {
        internal::NodeSpec {
            pod_cidr: self.pod_cidr.unwrap_or_default(),
            pod_cidrs: self.pod_cidrs,
            provider_id: self.provider_id.unwrap_or_default(),
            unschedulable: self.unschedulable,
            taints: self.taints.into_iter().map(|t| t.to_internal()).collect(),
            config_source: self.config_source.map(|c| c.to_internal()),
            external_id: self.external_id,
        }
    }
}

impl FromInternal<internal::NodeSpec> for node::NodeSpec {
    fn from_internal(value: internal::NodeSpec) -> Self {
        Self {
            pod_cidr: if value.pod_cidr.is_empty() {
                None
            } else {
                Some(value.pod_cidr)
            },
            pod_cidrs: value.pod_cidrs,
            provider_id: if value.provider_id.is_empty() {
                None
            } else {
                Some(value.provider_id)
            },
            unschedulable: value.unschedulable,
            taints: value
                .taints
                .into_iter()
                .map(node::Taint::from_internal)
                .collect(),
            config_source: value
                .config_source
                .map(node::NodeConfigSource::from_internal),
            external_id: value.external_id,
        }
    }
}

// ============================================================================
// NodeStatus
// ============================================================================

impl ToInternal<internal::NodeStatus> for node::NodeStatus {
    fn to_internal(self) -> internal::NodeStatus {
        internal::NodeStatus {
            capacity: self.capacity,
            allocatable: self.allocatable,
            phase: option_string_to_node_phase(self.phase),
            conditions: self
                .conditions
                .into_iter()
                .map(|c| c.to_internal())
                .collect(),
            addresses: self
                .addresses
                .into_iter()
                .map(|a| a.to_internal())
                .collect(),
            daemon_endpoints: self
                .daemon_endpoints
                .map(|d| d.to_internal())
                .unwrap_or_default(),
            node_info: self.node_info.map(|n| n.to_internal()).unwrap_or_default(),
            images: self.images.into_iter().map(|i| i.to_internal()).collect(),
            volumes_in_use: self.volumes_in_use,
            volumes_attached: self
                .volumes_attached
                .into_iter()
                .map(|v| v.to_internal())
                .collect(),
            features: self.features.map(|f| f.to_internal()),
            swap: None, // v1 has swap in NodeSystemInfo, not in NodeStatus
                        // config and runtime_handlers are v1-only fields, not in internal
        }
    }
}

impl FromInternal<internal::NodeStatus> for node::NodeStatus {
    fn from_internal(value: internal::NodeStatus) -> Self {
        Self {
            capacity: value.capacity,
            allocatable: value.allocatable,
            phase: node_phase_to_option_string(value.phase),
            conditions: value
                .conditions
                .into_iter()
                .map(node::NodeCondition::from_internal)
                .collect(),
            addresses: value
                .addresses
                .into_iter()
                .map(node::NodeAddress::from_internal)
                .collect(),
            daemon_endpoints: Some(node::NodeDaemonEndpoints::from_internal(
                value.daemon_endpoints,
            )),
            node_info: Some(node::NodeSystemInfo::from_internal(value.node_info)),
            images: value
                .images
                .into_iter()
                .map(node::ContainerImage::from_internal)
                .collect(),
            volumes_in_use: value.volumes_in_use,
            volumes_attached: value
                .volumes_attached
                .into_iter()
                .map(node::AttachedVolume::from_internal)
                .collect(),
            config: None,             // v1-only field, not in internal
            runtime_handlers: vec![], // v1-only field, not in internal
            features: value.features.map(node::NodeFeatures::from_internal),
        }
    }
}

// ============================================================================
// Taint
// ============================================================================

impl ToInternal<internal::Taint> for node::Taint {
    fn to_internal(self) -> internal::Taint {
        internal::Taint {
            key: self.key,
            value: self.value.unwrap_or_default(),
            effect: option_string_to_taint_effect(self.effect),
            time_added: self.time_added,
        }
    }
}

impl FromInternal<internal::Taint> for node::Taint {
    fn from_internal(value: internal::Taint) -> Self {
        Self {
            key: value.key,
            value: if value.value.is_empty() {
                None
            } else {
                Some(value.value)
            },
            effect: taint_effect_to_option_string(value.effect),
            time_added: value.time_added,
        }
    }
}

// ============================================================================
// NodeCondition
// ============================================================================

impl ToInternal<internal::NodeCondition> for node::NodeCondition {
    fn to_internal(self) -> internal::NodeCondition {
        internal::NodeCondition {
            r#type: self.type_,
            status: string_to_condition_status(self.status),
            last_heartbeat_time: self.last_heartbeat_time,
            last_transition_time: self.last_transition_time,
            reason: self.reason.unwrap_or_default(),
            message: self.message.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::NodeCondition> for node::NodeCondition {
    fn from_internal(value: internal::NodeCondition) -> Self {
        Self {
            type_: value.r#type,
            status: condition_status_to_string(value.status),
            last_heartbeat_time: value.last_heartbeat_time,
            last_transition_time: value.last_transition_time,
            reason: if value.reason.is_empty() {
                None
            } else {
                Some(value.reason)
            },
            message: if value.message.is_empty() {
                None
            } else {
                Some(value.message)
            },
        }
    }
}

// ============================================================================
// NodeAddress
// ============================================================================

impl ToInternal<internal::NodeAddress> for node::NodeAddress {
    fn to_internal(self) -> internal::NodeAddress {
        internal::NodeAddress {
            r#type: string_to_node_address_type(self.type_),
            address: self.address,
        }
    }
}

impl FromInternal<internal::NodeAddress> for node::NodeAddress {
    fn from_internal(value: internal::NodeAddress) -> Self {
        Self {
            type_: node_address_type_to_string(value.r#type),
            address: value.address,
        }
    }
}

// ============================================================================
// NodeSystemInfo
// ============================================================================

impl ToInternal<internal::NodeSystemInfo> for node::NodeSystemInfo {
    fn to_internal(self) -> internal::NodeSystemInfo {
        internal::NodeSystemInfo {
            machine_id: self.machine_id.unwrap_or_default(),
            system_uuid: self.system_uuid.unwrap_or_default(),
            boot_id: self.boot_id.unwrap_or_default(),
            kernel_version: self.kernel_version.unwrap_or_default(),
            os_image: self.os_image.unwrap_or_default(),
            container_runtime_version: self.container_runtime_version.unwrap_or_default(),
            kubelet_version: self.kubelet_version.unwrap_or_default(),
            kube_proxy_version: self.kube_proxy_version.unwrap_or_default(),
            operating_system: self.operating_system.unwrap_or_default(),
            architecture: self.architecture.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::NodeSystemInfo> for node::NodeSystemInfo {
    fn from_internal(value: internal::NodeSystemInfo) -> Self {
        Self {
            machine_id: if value.machine_id.is_empty() {
                None
            } else {
                Some(value.machine_id)
            },
            system_uuid: if value.system_uuid.is_empty() {
                None
            } else {
                Some(value.system_uuid)
            },
            boot_id: if value.boot_id.is_empty() {
                None
            } else {
                Some(value.boot_id)
            },
            kernel_version: if value.kernel_version.is_empty() {
                None
            } else {
                Some(value.kernel_version)
            },
            os_image: if value.os_image.is_empty() {
                None
            } else {
                Some(value.os_image)
            },
            container_runtime_version: if value.container_runtime_version.is_empty() {
                None
            } else {
                Some(value.container_runtime_version)
            },
            kubelet_version: if value.kubelet_version.is_empty() {
                None
            } else {
                Some(value.kubelet_version)
            },
            kube_proxy_version: if value.kube_proxy_version.is_empty() {
                None
            } else {
                Some(value.kube_proxy_version)
            },
            operating_system: if value.operating_system.is_empty() {
                None
            } else {
                Some(value.operating_system)
            },
            architecture: if value.architecture.is_empty() {
                None
            } else {
                Some(value.architecture)
            },
            swap: None, // v1-only field, not in internal
        }
    }
}

// ============================================================================
// NodeDaemonEndpoints and DaemonEndpoint
// ============================================================================

impl ToInternal<internal::NodeDaemonEndpoints> for node::NodeDaemonEndpoints {
    fn to_internal(self) -> internal::NodeDaemonEndpoints {
        internal::NodeDaemonEndpoints {
            kubelet_endpoint: self.kubelet_endpoint.map(|e| e.to_internal()),
        }
    }
}

impl FromInternal<internal::NodeDaemonEndpoints> for node::NodeDaemonEndpoints {
    fn from_internal(value: internal::NodeDaemonEndpoints) -> Self {
        Self {
            kubelet_endpoint: value
                .kubelet_endpoint
                .map(node::DaemonEndpoint::from_internal),
        }
    }
}

impl ToInternal<internal::DaemonEndpoint> for node::DaemonEndpoint {
    fn to_internal(self) -> internal::DaemonEndpoint {
        internal::DaemonEndpoint { port: self.port }
    }
}

impl FromInternal<internal::DaemonEndpoint> for node::DaemonEndpoint {
    fn from_internal(value: internal::DaemonEndpoint) -> Self {
        Self { port: value.port }
    }
}

// ============================================================================
// NodeConfigSource and related
// ============================================================================

impl ToInternal<internal::NodeConfigSource> for node::NodeConfigSource {
    fn to_internal(self) -> internal::NodeConfigSource {
        internal::NodeConfigSource {
            config_map: self.config_map.map(|c| c.to_internal()),
        }
    }
}

impl FromInternal<internal::NodeConfigSource> for node::NodeConfigSource {
    fn from_internal(value: internal::NodeConfigSource) -> Self {
        Self {
            config_map: value
                .config_map
                .map(node::ConfigMapNodeConfigSource::from_internal),
        }
    }
}

impl ToInternal<internal::ConfigMapNodeConfigSource> for node::ConfigMapNodeConfigSource {
    fn to_internal(self) -> internal::ConfigMapNodeConfigSource {
        internal::ConfigMapNodeConfigSource {
            namespace: self.namespace.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            uid: self.uid,
            resource_version: self.resource_version,
            kubelet_config_key: self.kubelet_config_key.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::ConfigMapNodeConfigSource> for node::ConfigMapNodeConfigSource {
    fn from_internal(value: internal::ConfigMapNodeConfigSource) -> Self {
        Self {
            namespace: if value.namespace.is_empty() {
                None
            } else {
                Some(value.namespace)
            },
            name: if value.name.is_empty() {
                None
            } else {
                Some(value.name)
            },
            uid: value.uid,
            resource_version: value.resource_version,
            kubelet_config_key: if value.kubelet_config_key.is_empty() {
                None
            } else {
                Some(value.kubelet_config_key)
            },
        }
    }
}

impl ToInternal<internal::NodeConfigStatus> for node::NodeConfigStatus {
    fn to_internal(self) -> internal::NodeConfigStatus {
        internal::NodeConfigStatus {
            assigned: self.assigned.map(|a| a.to_internal()),
            active: self.active.map(|a| a.to_internal()),
            last_known_good: self.last_known_good.map(|l| l.to_internal()),
            error: self.error.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::NodeConfigStatus> for node::NodeConfigStatus {
    fn from_internal(value: internal::NodeConfigStatus) -> Self {
        Self {
            assigned: value.assigned.map(node::NodeConfigSource::from_internal),
            active: value.active.map(node::NodeConfigSource::from_internal),
            last_known_good: value
                .last_known_good
                .map(node::NodeConfigSource::from_internal),
            error: if value.error.is_empty() {
                None
            } else {
                Some(value.error)
            },
        }
    }
}

// ============================================================================
// NodeRuntimeHandler and NodeRuntimeHandlerFeatures
// ============================================================================

impl ToInternal<internal::NodeRuntimeHandler> for node::NodeRuntimeHandler {
    fn to_internal(self) -> internal::NodeRuntimeHandler {
        internal::NodeRuntimeHandler {
            name: self.name,
            features: self.features.map(|f| f.to_internal()),
        }
    }
}

impl FromInternal<internal::NodeRuntimeHandler> for node::NodeRuntimeHandler {
    fn from_internal(value: internal::NodeRuntimeHandler) -> Self {
        Self {
            name: value.name,
            features: value
                .features
                .map(node::NodeRuntimeHandlerFeatures::from_internal),
        }
    }
}

impl ToInternal<internal::NodeRuntimeHandlerFeatures> for node::NodeRuntimeHandlerFeatures {
    fn to_internal(self) -> internal::NodeRuntimeHandlerFeatures {
        internal::NodeRuntimeHandlerFeatures {
            user_namespaces: if self.user_namespaces {
                Some(true)
            } else {
                None
            },
            // recursive_read_only_mounts is v1-only field, not in internal
        }
    }
}

impl FromInternal<internal::NodeRuntimeHandlerFeatures> for node::NodeRuntimeHandlerFeatures {
    fn from_internal(value: internal::NodeRuntimeHandlerFeatures) -> Self {
        Self {
            user_namespaces: value.user_namespaces.unwrap_or(false),
            recursive_read_only_mounts: false, // v1-only field, default to false
        }
    }
}

// ============================================================================
// NodeFeatures
// ============================================================================

impl ToInternal<internal::NodeFeatures> for node::NodeFeatures {
    fn to_internal(self) -> internal::NodeFeatures {
        internal::NodeFeatures {
            runtime_handlers: vec![], // internal-only field, set to empty
                                      // supplemental_groups_policy is v1-only field, not in internal
        }
    }
}

impl FromInternal<internal::NodeFeatures> for node::NodeFeatures {
    fn from_internal(_value: internal::NodeFeatures) -> Self {
        Self {
            supplemental_groups_policy: false, // v1-only field, default to false
                                               // runtime_handlers from internal is dropped
        }
    }
}

// ============================================================================
// NodeSwapStatus
// ============================================================================

impl ToInternal<internal::NodeSwapStatus> for node::NodeSwapStatus {
    fn to_internal(self) -> internal::NodeSwapStatus {
        internal::NodeSwapStatus {
            swap_status: String::new(), // v1 has capacity (i64), internal has swap_status (String); incompatible
        }
    }
}

impl FromInternal<internal::NodeSwapStatus> for node::NodeSwapStatus {
    fn from_internal(_value: internal::NodeSwapStatus) -> Self {
        Self {
            capacity: 0, // internal has swap_status (String), v1 has capacity (i64); incompatible
        }
    }
}

// ============================================================================
// ContainerImage
// ============================================================================

impl ToInternal<internal::ContainerImage> for node::ContainerImage {
    fn to_internal(self) -> internal::ContainerImage {
        internal::ContainerImage {
            names: self.names,
            size_bytes: self.size_bytes,
        }
    }
}

impl FromInternal<internal::ContainerImage> for node::ContainerImage {
    fn from_internal(value: internal::ContainerImage) -> Self {
        Self {
            names: value.names,
            size_bytes: value.size_bytes,
        }
    }
}

// ============================================================================
// AttachedVolume
// ============================================================================

impl ToInternal<internal::AttachedVolume> for node::AttachedVolume {
    fn to_internal(self) -> internal::AttachedVolume {
        internal::AttachedVolume {
            name: self.name,
            device_path: self.device_path.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::AttachedVolume> for node::AttachedVolume {
    fn from_internal(value: internal::AttachedVolume) -> Self {
        Self {
            name: value.name,
            device_path: if value.device_path.is_empty() {
                None
            } else {
                Some(value.device_path)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_roundtrip() {
        let v1_node = node::Node {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("test-node".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(node::NodeSpec {
                pod_cidr: Some("10.0.0.0/24".to_string()),
                pod_cidrs: vec!["10.0.0.0/24".to_string()],
                unschedulable: false,
                taints: vec![],
                ..Default::default()
            }),
            status: Some(node::NodeStatus {
                phase: Some("Running".to_string()),
                conditions: vec![],
                addresses: vec![],
                ..Default::default()
            }),
        };

        let internal_node = v1_node.clone().to_internal();
        assert_eq!(internal_node.metadata.name, Some("test-node".to_string()));
        assert_eq!(internal_node.spec.pod_cidr, "10.0.0.0/24");

        let roundtrip = node::Node::from_internal(internal_node);
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name,
            v1_node.metadata.as_ref().unwrap().name
        );
    }

    #[test]
    fn test_taint_enum_conversion() {
        let v1_taint = node::Taint {
            key: "key1".to_string(),
            value: Some("value1".to_string()),
            effect: Some("NoSchedule".to_string()),
            time_added: None,
        };

        let internal_taint = v1_taint.clone().to_internal();
        assert!(matches!(
            internal_taint.effect,
            internal::TaintEffect::NoSchedule
        ));
        assert_eq!(internal_taint.value, "value1");

        let roundtrip = node::Taint::from_internal(internal_taint);
        assert_eq!(roundtrip.effect, Some("NoSchedule".to_string()));
        assert_eq!(roundtrip.value, Some("value1".to_string()));
    }

    #[test]
    fn test_node_address_type_conversion() {
        let v1_address = node::NodeAddress {
            type_: "InternalIP".to_string(),
            address: "192.168.1.10".to_string(),
        };

        let internal_address = v1_address.clone().to_internal();
        assert!(matches!(
            internal_address.r#type,
            internal::NodeAddressType::InternalIp
        ));

        let roundtrip = node::NodeAddress::from_internal(internal_address);
        assert_eq!(roundtrip.type_, "InternalIP");
    }

    #[test]
    fn test_node_condition_status_conversion() {
        let v1_condition = node::NodeCondition {
            type_: "Ready".to_string(),
            status: "True".to_string(),
            last_heartbeat_time: None,
            last_transition_time: None,
            reason: Some("KubeletReady".to_string()),
            message: Some("kubelet is posting ready status".to_string()),
        };

        let internal_condition = v1_condition.clone().to_internal();
        assert!(matches!(
            internal_condition.status,
            internal::ConditionStatus::True
        ));

        let roundtrip = node::NodeCondition::from_internal(internal_condition);
        assert_eq!(roundtrip.status, "True");
    }

    #[test]
    fn test_node_system_info_option_string_conversion() {
        let v1_info = node::NodeSystemInfo {
            machine_id: Some("12345".to_string()),
            system_uuid: Some("uuid-123".to_string()),
            kernel_version: Some("5.10.0".to_string()),
            os_image: Some("Ubuntu 20.04".to_string()),
            ..Default::default()
        };

        let internal_info = v1_info.clone().to_internal();
        assert_eq!(internal_info.machine_id, "12345");
        assert_eq!(internal_info.kernel_version, "5.10.0");

        let roundtrip = node::NodeSystemInfo::from_internal(internal_info);
        assert_eq!(roundtrip.machine_id, Some("12345".to_string()));
        assert_eq!(roundtrip.kernel_version, Some("5.10.0".to_string()));
    }
}
