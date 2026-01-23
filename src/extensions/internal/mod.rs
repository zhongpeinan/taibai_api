//! Extensions internal API types
//!
//! This module contains the internal types for the Kubernetes Extensions API.
//!
//! The extensions API group is deprecated. Most types have been moved to:
//! - `apps/v1beta2` and `apps/v1` for Deployment, DaemonSet, ReplicaSet
//! - `networking.k8s.io/v1beta1` and `networking.k8s.io/v1` for Ingress, NetworkPolicy
//!
//! Source: k8s-pkg/apis/extensions/types.go

// Re-export all v1beta1 types (internal and external types are essentially the same)
pub use crate::extensions::v1beta1::{
    DAEMON_SET_TEMPLATE_GENERATION_KEY, DEFAULT_DAEMON_SET_UNIQUE_LABEL_KEY,
    DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY, DaemonSet, DaemonSetCondition, DaemonSetList,
    DaemonSetSpec, DaemonSetStatus, DaemonSetUpdateStrategy, DaemonSetUpdateStrategyType,
    Deployment, DeploymentCondition, DeploymentConditionType, DeploymentList, DeploymentRollback,
    DeploymentSpec, DeploymentStatus, DeploymentStrategy, DeploymentStrategyType, HTTPIngressPath,
    HTTPIngressRuleValue, IPBlock, Ingress, IngressBackend, IngressList,
    IngressLoadBalancerIngress, IngressLoadBalancerStatus, IngressPortStatus, IngressRule,
    IngressSpec, IngressStatus, IngressTLS, NetworkPolicy, NetworkPolicyEgressRule,
    NetworkPolicyIngressRule, NetworkPolicyList, NetworkPolicyPeer, NetworkPolicyPort,
    NetworkPolicySpec, PathType, PolicyType, ReplicaSet, ReplicaSetCondition,
    ReplicaSetConditionType, ReplicaSetList, ReplicaSetSpec, ReplicaSetStatus, RollbackConfig,
    RollingUpdateDaemonSet, RollingUpdateDeployment, Scale, ScaleSpec, ScaleStatus,
};

// ============================================================================
// Additional Internal Constants
// ============================================================================

/// ServiceExternalLoadBalancer is the name of the annotation that
/// marks a service as using an external load balancer.
pub const SERVICE_EXTERNAL_LOAD_BALANCER: &str =
    "service.beta.kubernetes.io/external-load-balancer";

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
}
