//! Conversions between networking v1 and internal types
//!
//! Based on k8s.io/kubernetes/pkg/apis/networking/v1/conversion.go

#[allow(unused_imports)]
use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};
use crate::networking::internal;
use crate::networking::v1::{ingress, ingress_class, network_policy};

// ============================================================================
// Helper Functions
// ============================================================================

fn is_empty_object_meta(meta: &ObjectMeta) -> bool {
    meta.name.is_none()
        && meta.generate_name.is_none()
        && meta.namespace.is_none()
        && meta.uid.is_none()
        && meta.resource_version.is_none()
        && meta.generation.is_none()
        && meta.self_link.is_none()
        && meta.labels.is_empty()
        && meta.annotations.is_empty()
        && meta.owner_references.is_empty()
        && meta.finalizers.is_empty()
        && meta.managed_fields.is_empty()
        && meta.creation_timestamp.is_none()
        && meta.deletion_timestamp.is_none()
        && meta.deletion_grace_period_seconds.is_none()
}

fn is_empty_list_meta(meta: &ListMeta) -> bool {
    meta.continue_.is_none()
        && meta.remaining_item_count.is_none()
        && meta.resource_version.is_none()
        && meta.self_link.is_none()
}

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_list_meta_to_meta(meta: Option<ListMeta>) -> ListMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_list_meta(meta: ListMeta) -> Option<ListMeta> {
    if is_empty_list_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

// ============================================================================
// Spec/Status Conversion Helpers
// ============================================================================

fn convert_ingress_spec_v1_to_internal(
    spec: ingress::IngressSpec,
) -> internal::ingress::IngressSpec {
    internal::ingress::IngressSpec {
        ingress_class_name: spec.ingress_class_name,
        default_backend: spec
            .default_backend
            .map(convert_ingress_backend_v1_to_internal),
        rules: spec
            .rules
            .into_iter()
            .map(convert_ingress_rule_v1_to_internal)
            .collect(),
        tls: spec
            .tls
            .into_iter()
            .map(convert_ingress_tls_v1_to_internal)
            .collect(),
    }
}

fn convert_ingress_spec_internal_to_v1(
    spec: internal::ingress::IngressSpec,
) -> ingress::IngressSpec {
    ingress::IngressSpec {
        ingress_class_name: spec.ingress_class_name,
        default_backend: spec
            .default_backend
            .map(convert_ingress_backend_internal_to_v1),
        rules: spec
            .rules
            .into_iter()
            .map(convert_ingress_rule_internal_to_v1)
            .collect(),
        tls: spec
            .tls
            .into_iter()
            .map(convert_ingress_tls_internal_to_v1)
            .collect(),
    }
}

fn convert_ingress_status_v1_to_internal(
    status: ingress::IngressStatus,
) -> internal::ingress::IngressStatus {
    internal::ingress::IngressStatus {
        load_balancer: status
            .load_balancer
            .map(convert_ingress_lb_status_v1_to_internal),
    }
}

fn convert_ingress_status_internal_to_v1(
    status: internal::ingress::IngressStatus,
) -> ingress::IngressStatus {
    ingress::IngressStatus {
        load_balancer: status
            .load_balancer
            .map(convert_ingress_lb_status_internal_to_v1),
    }
}

fn convert_ingress_backend_v1_to_internal(
    backend: ingress::IngressBackend,
) -> internal::ingress::IngressBackend {
    internal::ingress::IngressBackend {
        service: backend
            .service
            .map(convert_ingress_service_backend_v1_to_internal),
        resource: backend
            .resource
            .map(convert_typed_local_object_reference_v1_to_internal),
    }
}

fn convert_ingress_backend_internal_to_v1(
    backend: internal::ingress::IngressBackend,
) -> ingress::IngressBackend {
    ingress::IngressBackend {
        service: backend
            .service
            .map(convert_ingress_service_backend_internal_to_v1),
        resource: backend
            .resource
            .map(convert_typed_local_object_reference_internal_to_v1),
    }
}

fn convert_typed_local_object_reference_v1_to_internal(
    resource: crate::core::v1::TypedLocalObjectReference,
) -> crate::core::internal::TypedLocalObjectReference {
    crate::core::internal::TypedLocalObjectReference {
        api_group: resource.api_group,
        kind: resource.kind.unwrap_or_default(),
        name: resource.name.unwrap_or_default(),
        namespace: None,
    }
}

fn convert_typed_local_object_reference_internal_to_v1(
    resource: crate::core::internal::TypedLocalObjectReference,
) -> crate::core::v1::TypedLocalObjectReference {
    crate::core::v1::TypedLocalObjectReference {
        api_group: resource.api_group,
        kind: if resource.kind.is_empty() {
            None
        } else {
            Some(resource.kind)
        },
        name: if resource.name.is_empty() {
            None
        } else {
            Some(resource.name)
        },
    }
}

fn convert_ingress_service_backend_v1_to_internal(
    backend: ingress::IngressServiceBackend,
) -> internal::ingress::IngressServiceBackend {
    internal::ingress::IngressServiceBackend {
        name: backend.name,
        port: backend
            .port
            .map(convert_service_backend_port_v1_to_internal),
    }
}

fn convert_ingress_service_backend_internal_to_v1(
    backend: internal::ingress::IngressServiceBackend,
) -> ingress::IngressServiceBackend {
    ingress::IngressServiceBackend {
        name: backend.name,
        port: backend
            .port
            .map(convert_service_backend_port_internal_to_v1),
    }
}

fn convert_service_backend_port_v1_to_internal(
    port: ingress::ServiceBackendPort,
) -> internal::ingress::ServiceBackendPort {
    internal::ingress::ServiceBackendPort {
        name: port.name,
        number: port.number,
    }
}

fn convert_service_backend_port_internal_to_v1(
    port: internal::ingress::ServiceBackendPort,
) -> ingress::ServiceBackendPort {
    ingress::ServiceBackendPort {
        name: port.name,
        number: port.number,
    }
}

fn convert_path_type_v1_to_internal(path_type: ingress::PathType) -> internal::ingress::PathType {
    match path_type {
        ingress::PathType::Exact => internal::ingress::PathType::Exact,
        ingress::PathType::Prefix => internal::ingress::PathType::Prefix,
        ingress::PathType::ImplementationSpecific => {
            internal::ingress::PathType::ImplementationSpecific
        }
    }
}

fn convert_path_type_internal_to_v1(path_type: internal::ingress::PathType) -> ingress::PathType {
    match path_type {
        internal::ingress::PathType::Exact => ingress::PathType::Exact,
        internal::ingress::PathType::Prefix => ingress::PathType::Prefix,
        internal::ingress::PathType::ImplementationSpecific => {
            ingress::PathType::ImplementationSpecific
        }
    }
}

fn convert_ingress_rule_v1_to_internal(
    rule: ingress::IngressRule,
) -> internal::ingress::IngressRule {
    internal::ingress::IngressRule {
        host: rule.host,
        http: rule
            .http
            .map(convert_http_ingress_rule_value_v1_to_internal),
    }
}

fn convert_ingress_rule_internal_to_v1(
    rule: internal::ingress::IngressRule,
) -> ingress::IngressRule {
    ingress::IngressRule {
        host: rule.host,
        http: rule
            .http
            .map(convert_http_ingress_rule_value_internal_to_v1),
    }
}

fn convert_http_ingress_rule_value_v1_to_internal(
    http: ingress::HTTPIngressRuleValue,
) -> internal::ingress::HTTPIngressRuleValue {
    internal::ingress::HTTPIngressRuleValue {
        paths: http
            .paths
            .into_iter()
            .map(convert_http_ingress_path_v1_to_internal)
            .collect(),
    }
}

fn convert_http_ingress_rule_value_internal_to_v1(
    http: internal::ingress::HTTPIngressRuleValue,
) -> ingress::HTTPIngressRuleValue {
    ingress::HTTPIngressRuleValue {
        paths: http
            .paths
            .into_iter()
            .map(convert_http_ingress_path_internal_to_v1)
            .collect(),
    }
}

fn convert_http_ingress_path_v1_to_internal(
    path: ingress::HTTPIngressPath,
) -> internal::ingress::HTTPIngressPath {
    internal::ingress::HTTPIngressPath {
        path: path.path,
        path_type: convert_path_type_v1_to_internal(path.path_type),
        backend: convert_ingress_backend_v1_to_internal(path.backend),
    }
}

fn convert_http_ingress_path_internal_to_v1(
    path: internal::ingress::HTTPIngressPath,
) -> ingress::HTTPIngressPath {
    ingress::HTTPIngressPath {
        path: path.path,
        path_type: convert_path_type_internal_to_v1(path.path_type),
        backend: convert_ingress_backend_internal_to_v1(path.backend),
    }
}

fn convert_ingress_tls_v1_to_internal(tls: ingress::IngressTLS) -> internal::ingress::IngressTLS {
    internal::ingress::IngressTLS {
        hosts: tls.hosts,
        secret_name: tls.secret_name,
    }
}

fn convert_ingress_tls_internal_to_v1(tls: internal::ingress::IngressTLS) -> ingress::IngressTLS {
    ingress::IngressTLS {
        hosts: tls.hosts,
        secret_name: tls.secret_name,
    }
}

fn convert_ingress_lb_status_v1_to_internal(
    status: ingress::IngressLoadBalancerStatus,
) -> internal::ingress::IngressLoadBalancerStatus {
    internal::ingress::IngressLoadBalancerStatus {
        ingress: status
            .ingress
            .into_iter()
            .map(convert_ingress_lb_ingress_v1_to_internal)
            .collect(),
    }
}

fn convert_ingress_lb_status_internal_to_v1(
    status: internal::ingress::IngressLoadBalancerStatus,
) -> ingress::IngressLoadBalancerStatus {
    ingress::IngressLoadBalancerStatus {
        ingress: status
            .ingress
            .into_iter()
            .map(convert_ingress_lb_ingress_internal_to_v1)
            .collect(),
    }
}

fn convert_ingress_lb_ingress_v1_to_internal(
    ing: ingress::IngressLoadBalancerIngress,
) -> internal::ingress::IngressLoadBalancerIngress {
    internal::ingress::IngressLoadBalancerIngress {
        ip: ing.ip,
        hostname: ing.hostname,
        ports: ing
            .ports
            .into_iter()
            .map(convert_ingress_port_status_v1_to_internal)
            .collect(),
    }
}

fn convert_ingress_lb_ingress_internal_to_v1(
    ing: internal::ingress::IngressLoadBalancerIngress,
) -> ingress::IngressLoadBalancerIngress {
    ingress::IngressLoadBalancerIngress {
        ip: ing.ip,
        hostname: ing.hostname,
        ports: ing
            .ports
            .into_iter()
            .map(convert_ingress_port_status_internal_to_v1)
            .collect(),
    }
}

fn convert_ingress_port_status_v1_to_internal(
    port: ingress::IngressPortStatus,
) -> internal::ingress::IngressPortStatus {
    internal::ingress::IngressPortStatus {
        port: port.port,
        protocol: port.protocol,
        error: port.error,
    }
}

fn convert_ingress_port_status_internal_to_v1(
    port: internal::ingress::IngressPortStatus,
) -> ingress::IngressPortStatus {
    ingress::IngressPortStatus {
        port: port.port,
        protocol: port.protocol,
        error: port.error,
    }
}

fn convert_ingress_class_spec_v1_to_internal(
    spec: ingress_class::IngressClassSpec,
) -> internal::ingress_class::IngressClassSpec {
    internal::ingress_class::IngressClassSpec {
        controller: spec.controller,
        parameters: spec
            .parameters
            .map(convert_ingress_class_params_v1_to_internal),
    }
}

fn convert_ingress_class_spec_internal_to_v1(
    spec: internal::ingress_class::IngressClassSpec,
) -> ingress_class::IngressClassSpec {
    ingress_class::IngressClassSpec {
        controller: spec.controller,
        parameters: spec
            .parameters
            .map(convert_ingress_class_params_internal_to_v1),
    }
}

fn convert_ingress_class_params_v1_to_internal(
    params: ingress_class::IngressClassParametersReference,
) -> internal::ingress_class::IngressClassParametersReference {
    internal::ingress_class::IngressClassParametersReference {
        api_group: params.api_group.unwrap_or_default(),
        kind: params.kind,
        name: params.name,
        namespace: params.namespace.unwrap_or_default(),
        scope: params.scope.unwrap_or_default(),
    }
}

fn convert_ingress_class_params_internal_to_v1(
    params: internal::ingress_class::IngressClassParametersReference,
) -> ingress_class::IngressClassParametersReference {
    ingress_class::IngressClassParametersReference {
        api_group: if params.api_group.is_empty() {
            None
        } else {
            Some(params.api_group)
        },
        kind: params.kind,
        name: params.name,
        namespace: if params.namespace.is_empty() {
            None
        } else {
            Some(params.namespace)
        },
        scope: if params.scope.is_empty() {
            None
        } else {
            Some(params.scope)
        },
    }
}

fn convert_network_policy_spec_v1_to_internal(
    spec: network_policy::NetworkPolicySpec,
) -> internal::network_policy::NetworkPolicySpec {
    internal::network_policy::NetworkPolicySpec {
        pod_selector: spec.pod_selector,
        ingress: spec
            .ingress
            .into_iter()
            .map(convert_np_ingress_rule_v1_to_internal)
            .collect(),
        egress: spec
            .egress
            .into_iter()
            .map(convert_np_egress_rule_v1_to_internal)
            .collect(),
        policy_types: spec
            .policy_types
            .into_iter()
            .map(convert_policy_type_v1_to_internal)
            .collect(),
    }
}

fn convert_network_policy_spec_internal_to_v1(
    spec: internal::network_policy::NetworkPolicySpec,
) -> network_policy::NetworkPolicySpec {
    network_policy::NetworkPolicySpec {
        pod_selector: spec.pod_selector,
        ingress: spec
            .ingress
            .into_iter()
            .map(convert_np_ingress_rule_internal_to_v1)
            .collect(),
        egress: spec
            .egress
            .into_iter()
            .map(convert_np_egress_rule_internal_to_v1)
            .collect(),
        policy_types: spec
            .policy_types
            .into_iter()
            .map(convert_policy_type_internal_to_v1)
            .collect(),
    }
}

fn convert_np_ingress_rule_v1_to_internal(
    rule: network_policy::NetworkPolicyIngressRule,
) -> internal::network_policy::NetworkPolicyIngressRule {
    internal::network_policy::NetworkPolicyIngressRule {
        ports: rule
            .ports
            .into_iter()
            .map(convert_np_port_v1_to_internal)
            .collect(),
        from: rule
            .from
            .into_iter()
            .map(convert_np_peer_v1_to_internal)
            .collect(),
    }
}

fn convert_np_ingress_rule_internal_to_v1(
    rule: internal::network_policy::NetworkPolicyIngressRule,
) -> network_policy::NetworkPolicyIngressRule {
    network_policy::NetworkPolicyIngressRule {
        ports: rule
            .ports
            .into_iter()
            .map(convert_np_port_internal_to_v1)
            .collect(),
        from: rule
            .from
            .into_iter()
            .map(convert_np_peer_internal_to_v1)
            .collect(),
    }
}

fn convert_np_egress_rule_v1_to_internal(
    rule: network_policy::NetworkPolicyEgressRule,
) -> internal::network_policy::NetworkPolicyEgressRule {
    internal::network_policy::NetworkPolicyEgressRule {
        ports: rule
            .ports
            .into_iter()
            .map(convert_np_port_v1_to_internal)
            .collect(),
        to: rule
            .to
            .into_iter()
            .map(convert_np_peer_v1_to_internal)
            .collect(),
    }
}

fn convert_np_egress_rule_internal_to_v1(
    rule: internal::network_policy::NetworkPolicyEgressRule,
) -> network_policy::NetworkPolicyEgressRule {
    network_policy::NetworkPolicyEgressRule {
        ports: rule
            .ports
            .into_iter()
            .map(convert_np_port_internal_to_v1)
            .collect(),
        to: rule
            .to
            .into_iter()
            .map(convert_np_peer_internal_to_v1)
            .collect(),
    }
}

fn convert_np_port_v1_to_internal(
    port: network_policy::NetworkPolicyPort,
) -> internal::network_policy::NetworkPolicyPort {
    internal::network_policy::NetworkPolicyPort {
        protocol: port.protocol,
        port: port.port,
        end_port: port.end_port,
    }
}

fn convert_np_port_internal_to_v1(
    port: internal::network_policy::NetworkPolicyPort,
) -> network_policy::NetworkPolicyPort {
    network_policy::NetworkPolicyPort {
        protocol: port.protocol,
        port: port.port,
        end_port: port.end_port,
    }
}

fn convert_np_peer_v1_to_internal(
    peer: network_policy::NetworkPolicyPeer,
) -> internal::network_policy::NetworkPolicyPeer {
    internal::network_policy::NetworkPolicyPeer {
        pod_selector: peer.pod_selector,
        namespace_selector: peer.namespace_selector,
        ip_block: peer.ip_block.map(convert_ip_block_v1_to_internal),
    }
}

fn convert_np_peer_internal_to_v1(
    peer: internal::network_policy::NetworkPolicyPeer,
) -> network_policy::NetworkPolicyPeer {
    network_policy::NetworkPolicyPeer {
        pod_selector: peer.pod_selector,
        namespace_selector: peer.namespace_selector,
        ip_block: peer.ip_block.map(convert_ip_block_internal_to_v1),
    }
}

fn convert_ip_block_v1_to_internal(
    block: network_policy::IPBlock,
) -> internal::network_policy::IPBlock {
    internal::network_policy::IPBlock {
        cidr: block.cidr,
        except: block.except,
    }
}

fn convert_ip_block_internal_to_v1(
    block: internal::network_policy::IPBlock,
) -> network_policy::IPBlock {
    network_policy::IPBlock {
        cidr: block.cidr,
        except: block.except,
    }
}

fn convert_policy_type_v1_to_internal(
    policy_type: network_policy::PolicyType,
) -> internal::network_policy::PolicyType {
    match policy_type {
        network_policy::PolicyType::Ingress => internal::network_policy::PolicyType::Ingress,
        network_policy::PolicyType::Egress => internal::network_policy::PolicyType::Egress,
    }
}

fn convert_policy_type_internal_to_v1(
    policy_type: internal::network_policy::PolicyType,
) -> network_policy::PolicyType {
    match policy_type {
        internal::network_policy::PolicyType::Ingress => network_policy::PolicyType::Ingress,
        internal::network_policy::PolicyType::Egress => network_policy::PolicyType::Egress,
    }
}

// ============================================================================
// Ingress Conversions
// ============================================================================

use crate::networking::v1::ingress::Ingress;

impl ToInternal<internal::Ingress> for Ingress {
    fn to_internal(self) -> internal::Ingress {
        internal::Ingress {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(convert_ingress_spec_v1_to_internal),
            status: self.status.map(convert_ingress_status_v1_to_internal),
        }
    }
}

impl FromInternal<internal::Ingress> for Ingress {
    fn from_internal(value: internal::Ingress) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(convert_ingress_spec_internal_to_v1),
            status: value.status.map(convert_ingress_status_internal_to_v1),
        };

        result
    }
}

// ============================================================================
// IngressClass Conversions
// ============================================================================

use crate::networking::v1::ingress_class::IngressClass;

impl ToInternal<internal::IngressClass> for IngressClass {
    fn to_internal(self) -> internal::IngressClass {
        internal::IngressClass {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: convert_ingress_class_spec_v1_to_internal(self.spec),
        }
    }
}

impl FromInternal<internal::IngressClass> for IngressClass {
    fn from_internal(value: internal::IngressClass) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: convert_ingress_class_spec_internal_to_v1(value.spec),
        };

        result
    }
}

// ============================================================================
// NetworkPolicy Conversions
// ============================================================================

use crate::networking::v1::network_policy::NetworkPolicy;

impl ToInternal<internal::NetworkPolicy> for NetworkPolicy {
    fn to_internal(self) -> internal::NetworkPolicy {
        internal::NetworkPolicy {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(convert_network_policy_spec_v1_to_internal),
        }
    }
}

impl FromInternal<internal::NetworkPolicy> for NetworkPolicy {
    fn from_internal(value: internal::NetworkPolicy) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(convert_network_policy_spec_internal_to_v1),
        };

        result
    }
}

// ============================================================================
// List Conversions
// ============================================================================

use crate::networking::v1::ingress::IngressList;
use crate::networking::v1::ingress_class::IngressClassList;
use crate::networking::v1::network_policy::NetworkPolicyList;

impl ToInternal<internal::IngressList> for IngressList {
    fn to_internal(self) -> internal::IngressList {
        internal::IngressList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::IngressList> for IngressList {
    fn from_internal(value: internal::IngressList) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(Ingress::from_internal)
                .collect(),
        };

        result
    }
}

impl ToInternal<internal::IngressClassList> for IngressClassList {
    fn to_internal(self) -> internal::IngressClassList {
        internal::IngressClassList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::IngressClassList> for IngressClassList {
    fn from_internal(value: internal::IngressClassList) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(IngressClass::from_internal)
                .collect(),
        };

        result
    }
}

impl ToInternal<internal::NetworkPolicyList> for NetworkPolicyList {
    fn to_internal(self) -> internal::NetworkPolicyList {
        internal::NetworkPolicyList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::NetworkPolicyList> for NetworkPolicyList {
    fn from_internal(value: internal::NetworkPolicyList) -> Self {
        let result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(NetworkPolicy::from_internal)
                .collect(),
        };

        result
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
    fn test_ingress_round_trip() {
        let original = Ingress {
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

        let internal = original.clone().to_internal();
        let mut round_trip = Ingress::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.type_meta.api_version, "networking.k8s.io/v1");
        assert_eq!(round_trip.type_meta.kind, "Ingress");
    }

    #[test]
    fn test_ingress_with_backend_round_trip() {
        use crate::networking::v1::ingress::{
            HTTPIngressPath, HTTPIngressRuleValue, IngressBackend, IngressRule,
            IngressServiceBackend, IngressSpec, PathType, ServiceBackendPort,
        };

        let original = Ingress {
            type_meta: TypeMeta {
                api_version: "networking.k8s.io/v1".to_string(),
                kind: "Ingress".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-ingress".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(IngressSpec {
                ingress_class_name: Some("nginx".to_string()),
                default_backend: Some(IngressBackend {
                    service: Some(IngressServiceBackend {
                        name: "default-service".to_string(),
                        port: Some(ServiceBackendPort {
                            number: Some(80),
                            name: String::new(),
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
                                        name: "http".to_string(),
                                        number: None,
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

        let internal = original.clone().to_internal();
        let mut round_trip = Ingress::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(
            round_trip
                .spec
                .as_ref()
                .unwrap()
                .default_backend
                .as_ref()
                .unwrap()
                .service
                .as_ref()
                .unwrap()
                .name,
            "default-service"
        );
        assert_eq!(round_trip.type_meta.api_version, "networking.k8s.io/v1");
    }

    #[test]
    fn test_ingress_class_round_trip() {
        let original = IngressClass {
            type_meta: TypeMeta {
                api_version: "networking.k8s.io/v1".to_string(),
                kind: "IngressClass".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-class".to_string()),
                ..Default::default()
            }),
            spec: ingress_class::IngressClassSpec {
                controller: "example.com/controller".to_string(),
                parameters: None,
            },
        };

        let internal = original.clone().to_internal();
        let mut round_trip = IngressClass::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.spec.controller, original.spec.controller);
        assert_eq!(round_trip.type_meta.api_version, "networking.k8s.io/v1");
    }

    #[test]
    fn test_network_policy_round_trip() {
        let original = NetworkPolicy {
            type_meta: TypeMeta {
                api_version: "networking.k8s.io/v1".to_string(),
                kind: "NetworkPolicy".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-policy".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: None,
        };

        let internal = original.clone().to_internal();
        let mut round_trip = NetworkPolicy::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.type_meta.api_version, "networking.k8s.io/v1");
    }

    #[test]
    fn test_network_policy_with_selectors_and_ports_round_trip() {
        use crate::networking::v1::network_policy::{
            NetworkPolicyIngressRule, NetworkPolicyPeer, NetworkPolicyPort, NetworkPolicySpec,
        };

        let mut selector = LabelSelector::default();
        selector
            .match_labels
            .insert("app".to_string(), "web".to_string());

        let original = NetworkPolicy {
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
                pod_selector: selector.clone(),
                ingress: vec![NetworkPolicyIngressRule {
                    ports: vec![
                        NetworkPolicyPort {
                            protocol: Some("TCP".to_string()),
                            port: Some(IntOrString::Int(80)),
                            end_port: None,
                        },
                        NetworkPolicyPort {
                            protocol: Some("TCP".to_string()),
                            port: Some(IntOrString::String("http".to_string())),
                            end_port: None,
                        },
                    ],
                    from: vec![NetworkPolicyPeer {
                        pod_selector: Some(selector.clone()),
                        namespace_selector: None,
                        ip_block: None,
                    }],
                }],
                egress: vec![],
                policy_types: vec![],
            }),
        };

        let internal = original.clone().to_internal();
        let mut round_trip = NetworkPolicy::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(
            round_trip
                .spec
                .as_ref()
                .unwrap()
                .pod_selector
                .match_labels
                .get("app"),
            Some(&"web".to_string())
        );
        assert_eq!(round_trip.type_meta.api_version, "networking.k8s.io/v1");
    }
}
