//! NetworkPolicy types from the Kubernetes Networking API
//!
//! This module contains types for network policy resources.
//!
//! Source: k8s.io/api/networking/v1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// ============================================================================
// PolicyType
// ============================================================================

/// PolicyType describes the NetworkPolicy type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PolicyType {
    /// PolicyTypeIngress specifies a policy that applies to ingress traffic.
    #[serde(rename = "Ingress")]
    #[default]
    Ingress,
    /// PolicyTypeEgress specifies a policy that applies to egress traffic.
    #[serde(rename = "Egress")]
    Egress,
}

pub mod policy_type {
    pub const INGRESS: &str = "Ingress";
    pub const EGRESS: &str = "Egress";
}

// ============================================================================
// NetworkPolicy
// ============================================================================

/// NetworkPolicy describes what network traffic is allowed for a set of pods.
///
/// Corresponds to [Kubernetes NetworkPolicy](https://github.com/kubernetes/api/blob/master/networking/v1/types.go#L30)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct NetworkPolicy {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// spec represents the specification of the desired behavior for this NetworkPolicy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<NetworkPolicySpec>,
}
impl_versioned_object!(NetworkPolicy);

/// NetworkPolicyList is a list of NetworkPolicy objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    /// Items is a list of NetworkPolicy objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<NetworkPolicy>,
}

// ============================================================================
// NetworkPolicySpec
// ============================================================================

/// NetworkPolicySpec provides the specification of a NetworkPolicy.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicySpec {
    /// podSelector selects the pods to which this NetworkPolicy object applies.
    #[serde(default)]
    pub pod_selector: String,
    /// ingress is a list of ingress rules to be applied to the selected pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingress: Vec<NetworkPolicyIngressRule>,
    /// egress is a list of egress rules to be applied to the selected pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub egress: Vec<NetworkPolicyEgressRule>,
    /// policyTypes is a list of rule types that the NetworkPolicy relates to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_types: Vec<PolicyType>,
}

// ============================================================================
// NetworkPolicyIngressRule
// ============================================================================

/// NetworkPolicyIngressRule describes a particular set of traffic that is allowed to the pods
/// matched by a NetworkPolicySpec's podSelector.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyIngressRule {
    /// ports is a list of ports which should be made accessible on the pods selected for this rule.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<NetworkPolicyPort>,
    /// from is a list of sources which should be able to access the pods selected for this rule.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub from: Vec<NetworkPolicyPeer>,
}

// ============================================================================
// NetworkPolicyEgressRule
// ============================================================================

/// NetworkPolicyEgressRule describes a particular set of traffic that is allowed out of pods
/// matched by a NetworkPolicySpec's podSelector.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyEgressRule {
    /// ports is a list of destination ports for outgoing traffic.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<NetworkPolicyPort>,
    /// to is a list of destinations for outgoing traffic.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub to: Vec<NetworkPolicyPeer>,
}

// ============================================================================
// NetworkPolicyPort
// ============================================================================

/// NetworkPolicyPort describes a port to allow traffic on.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyPort {
    /// protocol is the protocol (TCP, UDP, or SCTP) which traffic must match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// port is the port on the given protocol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// endPort is the end port for a range.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_port: Option<i32>,
}

// ============================================================================
// NetworkPolicyPeer
// ============================================================================

/// NetworkPolicyPeer describes a peer to allow traffic from/to.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyPeer {
    /// podSelector is a label selector which selects pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod_selector: Option<String>,
    /// namespaceSelector is a label selector for namespaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<String>,
    /// ipBlock is a CIDR range with optional exceptions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_block: Option<IPBlock>,
}

// ============================================================================
// IPBlock
// ============================================================================

/// IPBlock describes a particular CIDR (Ex. "192.168.1.0/24","2001:db8::/64") that is allowed
/// to the pods matched by a NetworkPolicySpec's podSelector.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IPBlock {
    /// cidr is a string representing the IP Block.
    #[serde(default)]
    pub cidr: String,
    /// except is a slice of CIDRs that should not be included within an IP Block.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub except: Vec<String>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl crate::common::traits::ResourceSchema for NetworkPolicy {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "NetworkPolicy"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "networkpolicies"
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
        "NetworkPolicy"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "networkpolicies"
    }
}

impl crate::common::traits::ResourceSchema for NetworkPolicyList {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "NetworkPolicyList"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "networkpolicies"
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
        "NetworkPolicyList"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "networkpolicies"
    }
}

impl crate::common::traits::HasTypeMeta for NetworkPolicy {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl crate::common::traits::ApplyDefault for NetworkPolicy {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "NetworkPolicy".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for NetworkPolicyList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "NetworkPolicyList".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for NetworkPolicy {}
impl crate::common::traits::UnimplementedConversion for NetworkPolicyList {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
