//! APIRegistration v1beta1 API types.
//!
//! Source: k8s.io/kube-aggregator/pkg/apis/apiregistration/v1beta1/types.go

pub mod conversion;
pub mod defaults;

use crate::common::{HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, Timestamp, TypeMeta};
use crate::core::internal::ByteString;
use crate::{impl_unimplemented_prost_message, impl_versioned_object};
use serde::{Deserialize, Serialize};

// ============================================================================
// Enums / Constants
// ============================================================================

/// ConditionStatus indicates the status of a condition (true, false, or unknown).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum ConditionStatus {
    /// Condition is true.
    True,
    /// Condition is false.
    False,
    /// Condition status is unknown.
    #[default]
    Unknown,
}

/// ConditionStatus constants
pub mod condition_status {
    pub const TRUE: &str = "True";
    pub const FALSE: &str = "False";
    pub const UNKNOWN: &str = "Unknown";
}

/// APIServiceConditionType is a valid value for APIServiceCondition.type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum APIServiceConditionType {
    /// Available indicates that the service exists and is reachable.
    #[default]
    Available,
}

/// APIServiceConditionType constants
pub mod api_service_condition_type {
    pub const AVAILABLE: &str = "Available";
}

// ============================================================================
// Core Types
// ============================================================================

/// APIServiceList is a list of APIService objects.
///
/// Corresponds to [Kubernetes APIServiceList](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/kube-aggregator/pkg/apis/apiregistration/v1beta1/types.go#L29)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIServiceList {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is the list of APIService.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<APIService>,
}

/// ServiceReference holds a reference to Service.legacy.k8s.io.
///
/// Corresponds to [Kubernetes ServiceReference](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/kube-aggregator/pkg/apis/apiregistration/v1beta1/types.go#L42)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceReference {
    /// Namespace is the namespace of the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,

    /// Name is the name of the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Port on the service that is hosting the API service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

/// APIServiceSpec contains information for locating and communicating with a server.
///
/// Corresponds to [Kubernetes APIServiceSpec](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/kube-aggregator/pkg/apis/apiregistration/v1beta1/types.go#L56)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIServiceSpec {
    /// Service is a reference to the service for this API server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    /// Group is the API group name this server hosts.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,

    /// Version is the API version this server hosts.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,

    /// InsecureSkipTLSVerify disables TLS certificate verification when communicating with this server.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub insecure_skip_tls_verify: bool,

    /// CABundle is a PEM encoded CA bundle used to validate the server's certificate.
    #[serde(default)]
    pub ca_bundle: ByteString,

    /// GroupPriorityMinimum is the minimum priority this group should have.
    #[serde(default)]
    pub group_priority_minimum: i32,

    /// VersionPriority controls the ordering of this API version inside of its group.
    #[serde(default)]
    pub version_priority: i32,
}

/// APIServiceCondition describes the state of an APIService at a particular point.
///
/// Corresponds to [Kubernetes APIServiceCondition](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/kube-aggregator/pkg/apis/apiregistration/v1beta1/types.go#L103)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIServiceCondition {
    /// Type is the type of the condition.
    #[serde(rename = "type")]
    #[serde(default)]
    pub type_: APIServiceConditionType,

    /// Status is the status of the condition.
    #[serde(default)]
    pub status: ConditionStatus,

    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,

    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// APIServiceStatus contains derived information about an API server.
///
/// Corresponds to [Kubernetes APIServiceStatus](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/kube-aggregator/pkg/apis/apiregistration/v1beta1/types.go#L118)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIServiceStatus {
    /// Current service state of apiService.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<APIServiceCondition>,
}

/// APIService represents a server for a particular GroupVersion.
///
/// Corresponds to [Kubernetes APIService](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/kube-aggregator/pkg/apis/apiregistration/v1beta1/types.go#L157)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct APIService {
    /// Standard type metadata.
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec contains information for locating and communicating with a server.
    #[serde(default)]
    pub spec: APIServiceSpec,

    /// Status contains derived information about an API server.
    #[serde(default)]
    pub status: APIServiceStatus,
}
impl_versioned_object!(APIService);

// ============================================================================
// Trait Implementations
// ============================================================================

impl ResourceSchema for APIService {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apiregistration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "APIService"
    }
    fn resource(_: &Self::Meta) -> &str {
        "apiservices"
    }

    fn group_static() -> &'static str {
        "apiregistration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "APIService"
    }
    fn resource_static() -> &'static str {
        "apiservices"
    }
}

impl ResourceSchema for APIServiceList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "apiregistration.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1beta1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "APIServiceList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "apiservices"
    }

    fn group_static() -> &'static str {
        "apiregistration.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1beta1"
    }
    fn kind_static() -> &'static str {
        "APIServiceList"
    }
    fn resource_static() -> &'static str {
        "apiservices"
    }
}

impl HasTypeMeta for APIService {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for APIServiceList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(APIService);
impl_unimplemented_prost_message!(APIServiceList);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}

#[cfg(test)]
mod trait_tests;

// AsRefStr / AsRef<str> implementations for enums
crate::impl_as_str_ref!(ConditionStatus, {
    True => "True",
    False => "False",
    Unknown => "Unknown",
});

crate::impl_as_str_ref!(APIServiceConditionType, {
    Available => "Available",
});
