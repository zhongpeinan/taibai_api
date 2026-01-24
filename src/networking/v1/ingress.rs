//! Ingress types from the Kubernetes Networking API
//!
//! This module contains types for ingress resources.
//!
//! Source: k8s.io/api/networking/v1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================================
// Ingress
// ============================================================================

/// Ingress represents a name that can be used to access services in the cluster.
///
/// Corresponds to [Kubernetes Ingress](https://github.com/kubernetes/api/blob/master/networking/v1/types.go#L242)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct Ingress {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// spec is the desired state of the Ingress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<IngressSpec>,
    /// status is the current state of the Ingress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IngressStatus>,
}
impl_versioned_object!(Ingress);


/// IngressList is a collection of Ingress objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    /// Items is the list of Ingress objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Ingress>,
}

// ============================================================================
// IngressSpec
// ============================================================================

/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressSpec {
    /// ingressClassName is the name of an IngressClass cluster resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress_class_name: Option<String>,
    /// defaultBackend is the backend that should handle requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_backend: Option<IngressBackend>,
    /// rules is a list of host rules used to configure the Ingress.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<IngressRule>,
    /// tls is the TLS configuration.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tls: Vec<IngressTLS>,
}

// ============================================================================
// IngressStatus
// ============================================================================

/// IngressStatus describes the current state of the Ingress.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressStatus {
    /// loadBalancer contains the current status of the load-balancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<IngressLoadBalancerStatus>,
}

// ============================================================================
// IngressRule
// ============================================================================

/// IngressRule represents the rules mapping the paths under a specified host to the related backend services.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressRule {
    /// host is the fully qualified domain name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
    /// http is a list of HTTP selectors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPIngressRuleValue>,
}

// ============================================================================
// HTTPIngressRuleValue
// ============================================================================

/// HTTPIngressRuleValue is a list of HTTP selectors.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressRuleValue {
    /// paths is a collection of paths.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<HTTPIngressPath>,
}

// ============================================================================
// HTTPIngressPath
// ============================================================================

/// HTTPIngressPath associates a path with a backend.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HTTPIngressPath {
    /// path is matched against the path of an incoming request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// pathType determines the interpretation of the path.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path_type: String,
    /// backend defines the referenced service endpoint.
    #[serde(default)]
    pub backend: IngressBackend,
}

// ============================================================================
// IngressBackend
// ============================================================================

/// IngressBackend describes all endpoints for a given service and port.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressBackend {
    /// serviceName specifies the name of the referenced service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// servicePort specifies the port of the referenced service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_port: Option<i32>,
    /// resource is an ObjectRef to another Kubernetes resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<BTreeMap<String, String>>,
}

// ============================================================================
// IngressTLS
// ============================================================================

/// IngressTLS describes the transport layer security associated with an ingress.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressTLS {
    /// hosts is a list of hosts included in the TLS certificate.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hosts: Vec<String>,
    /// secretName is the name of the secret used to terminate TLS traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}

// ============================================================================
// IngressLoadBalancerStatus
// ============================================================================

/// IngressLoadBalancerStatus represents the status of a load-balancer.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerStatus {
    /// ingress is a list containing ingress points.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<IngressLoadBalancerIngress>,
}

// ============================================================================
// IngressLoadBalancerIngress
// ============================================================================

/// IngressLoadBalancerIngress represents the status of a load-balancer ingress point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerIngress {
    /// ip is the IP address.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ip: String,
    /// hostname is the hostname.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hostname: String,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl crate::common::traits::ResourceSchema for Ingress {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "Ingress"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "ingresses"
    }

    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "networking.k8s.io"
    }

    fn version_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "v1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "Ingress"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "ingresses"
    }
}

impl crate::common::traits::ResourceSchema for IngressList {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "IngressList"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "ingresses"
    }

    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "networking.k8s.io"
    }

    fn version_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "v1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "IngressList"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "ingresses"
    }
}

impl crate::common::traits::HasTypeMeta for Ingress {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl crate::common::traits::ApplyDefault for Ingress {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Ingress".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for IngressList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "IngressList".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for Ingress {}
impl crate::common::traits::UnimplementedConversion for IngressList {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
