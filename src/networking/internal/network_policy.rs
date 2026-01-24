//! NetworkPolicy internal types
//!
//! This module contains internal types for network policy resources.

use crate::common::{ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// PolicyType
// ============================================================================

/// PolicyType describes the NetworkPolicy type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum PolicyType {
    #[serde(rename = "Ingress")]
    #[default]
    Ingress,
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct NetworkPolicy {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// spec represents the specification of the desired behavior for this NetworkPolicy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<NetworkPolicySpec>,
}
impl_has_object_meta!(NetworkPolicy);

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

/// NetworkPolicyIngressRule describes a particular set of traffic that is allowed to the pods.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyIngressRule {
    /// ports is a list of ports which should be made accessible.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<NetworkPolicyPort>,
    /// from is a list of sources which should be able to access the pods.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub from: Vec<NetworkPolicyPeer>,
}

// ============================================================================
// NetworkPolicyEgressRule
// ============================================================================

/// NetworkPolicyEgressRule describes a particular set of traffic that is allowed out of pods.
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

/// IPBlock describes a particular CIDR that is allowed to the pods.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IPBlock {
    /// cidr is a string representing the IP Block.
    #[serde(default)]
    pub cidr: String,
    /// except is a slice of CIDRs that should not be included.
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

impl crate::common::traits::HasTypeMeta for NetworkPolicy {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}
