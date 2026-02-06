//! Kubernetes Core v1 API types
//!
//! This module contains types from the Kubernetes core/v1 API group.

pub mod affinity;
pub mod binding;
pub mod component_status;
pub mod config;
pub mod conversion;
pub mod env;
pub mod ephemeral;
pub mod event;
pub mod helper;
pub mod namespace;
pub mod node;
pub mod persistent_volume;
pub mod pod;
pub mod pod_resources;
pub mod pod_status_result;
pub mod probe;
pub mod reference;
pub mod replication_controller;
pub mod resource;
pub mod security;
pub mod selector;
pub mod service;
pub mod template;
pub mod toleration;
pub mod topology;
pub mod validation;
pub mod volume;

pub use pod::{
    Container, ContainerPort, ContainerState, ContainerStateRunning, ContainerStateTerminated,
    ContainerStateWaiting, ContainerStatus, ContainerExtendedResourceRequest, HostAlias, HostIP,
    Pod, PodCondition, PodDNSConfig, PodDNSConfigOption, PodExtendedResourceClaimStatus, PodIP,
    PodList, PodOS, PodReadinessGate, PodSchedulingGate, PodSpec, PodStatus, dns_policy, os_name,
    pod_phase, restart_policy,
};

pub use pod_resources::{
    ContainerResizePolicy, ContainerUser, LinuxContainerUser, PodResourceClaim,
    PodResourceClaimStatus,
};

pub use namespace::{
    Namespace, NamespaceCondition, NamespaceList, NamespaceSpec, NamespaceStatus, condition_status,
    finalizer_name, namespace_condition_type, namespace_phase,
};

pub use reference::{LocalObjectReference, ObjectReference, TypedLocalObjectReference};

pub use replication_controller::{
    ReplicationController, ReplicationControllerCondition, ReplicationControllerList,
    ReplicationControllerSpec, ReplicationControllerStatus,
};

pub use binding::{Binding, Preconditions};

pub use helper::{
    ByteString, PodAttachOptions, PodExecOptions, PodLogOptions, PodPortForwardOptions,
    PodProxyOptions, RangeAllocation, SerializedReference, ServiceProxyOptions,
};

pub use event::{Event, EventList, EventSeries, EventSource, event_type};

pub use service::{
    CLUSTER_IP_NONE, DEFAULT_CLIENT_IP_SERVICE_AFFINITY_SECONDS, ip_family, ip_family_policy,
    load_balancer_condition, load_balancer_condition_reason, load_balancer_ip_mode, protocol,
    service_affinity, service_external_traffic_policy, service_internal_traffic_policy,
    service_traffic_distribution, service_type,
};

pub use service::{
    ClientIPConfig, EndpointAddress, EndpointPort, EndpointSubset, Endpoints, EndpointsList,
    LoadBalancerIngress, LoadBalancerStatus, PortStatus, Service, ServiceList, ServicePort,
    ServiceSpec, ServiceStatus, SessionAffinityConfig,
};

pub use config::{
    ConfigMap, ConfigMapList, Secret, SecretList, ServiceAccount, ServiceAccountList, secret_type,
};

pub use resource::{
    LimitRange, LimitRangeItem, LimitRangeList, LimitRangeSpec, ResourceClaim, ResourceList,
    ResourceQuota, ResourceQuotaList, ResourceQuotaScope, ResourceQuotaSpec, ResourceQuotaStatus,
    ResourceRequirements, ScopeSelector, ScopedResourceSelectorRequirement, limit_type,
    resource_name, resource_quota_scope, scope_selector_operator,
};

pub use node::{
    AttachedVolume, AvoidPods, ConfigMapNodeConfigSource, ContainerImage, DaemonEndpoint, Node,
    NodeAddress, NodeCondition, NodeConfigSource, NodeConfigStatus, NodeDaemonEndpoints,
    NodeFeatures, NodeList, NodeProxyOptions, NodeRuntimeHandler, NodeRuntimeHandlerFeatures,
    NodeSpec, NodeStatus, NodeSwapStatus, NodeSystemInfo, PodSignature, PreferAvoidPodsEntry,
    Taint,
};

pub use node::{node_address_type, node_condition_type, node_phase, taint_effect};

pub use persistent_volume::{
    PersistentVolume, PersistentVolumeClaim, PersistentVolumeClaimCondition,
    PersistentVolumeClaimList, PersistentVolumeClaimSpec, PersistentVolumeClaimStatus,
    PersistentVolumeClaimVolumeSource, PersistentVolumeList, PersistentVolumeSource,
    PersistentVolumeSpec, PersistentVolumeStatus, TypedObjectReference, VolumeNodeAffinity,
    VolumeResourceRequirements,
};

pub use persistent_volume::{
    persistent_volume_access_mode, persistent_volume_claim_condition_type,
    persistent_volume_claim_phase, persistent_volume_mode, persistent_volume_phase,
    persistent_volume_reclaim_policy,
};

pub use volume::{
    CSIVolumeSource, ClusterTrustBundleProjection, ConfigMapProjection, ConfigMapVolumeSource,
    DownwardAPIProjection, DownwardAPIVolumeFile, DownwardAPIVolumeSource, EphemeralVolumeSource,
    GlusterfsVolumeSource, HostPathVolumeSource, ISCSIVolumeSource, ImageVolumeSource, KeyToPath,
    LocalVolumeSource, NFSVolumeSource, PersistentVolumeClaimTemplate, PodCertificateProjection,
    ProjectedVolumeSource, SecretProjection, SecretVolumeSource, ServiceAccountTokenProjection,
    Volume, VolumeDevice, VolumeMount, VolumeMountStatus, VolumeProjection, VolumeSource,
};

pub use volume::{
    host_path_type, mount_propagation_mode, pull_policy, recursive_read_only_mode, storage_medium,
};

pub use probe::{
    ExecAction, GRPCAction, HTTPGetAction, HTTPHeader, Lifecycle, LifecycleHandler, Probe,
    ProbeHandler, SleepAction, TCPSocketAction,
};

pub use probe::uri_scheme;

pub use env::{ConfigMapEnvSource, EnvFromSource, EnvVar, EnvVarSource, SecretEnvSource};

pub use selector::{
    ConfigMapKeySelector, FileKeySelector, ObjectFieldSelector, ResourceFieldSelector,
    SecretKeySelector,
};

pub use selector::{
    object_field_path, object_field_selector_api_version, resource_field_selector_resource,
};

pub use security::{
    AppArmorProfile, Capabilities, PodSecurityContext, SELinuxOptions, SeccompProfile,
    SecurityContext, Sysctl, WindowsSecurityContextOptions,
};

pub use security::{
    app_armor_profile_type, capability, proc_mount_type, seccomp_profile_type,
    supplemental_groups_policy,
};

pub use affinity::{
    Affinity, NodeAffinity, NodeSelector, NodeSelectorOperator, NodeSelectorRequirement,
    NodeSelectorSimple, NodeSelectorTerm, PodAffinity, PodAffinityTerm, PodAntiAffinity,
    PreferredSchedulingTerm, WeightedPodAffinityTerm,
};

pub use affinity::node_selector_operator;

pub use toleration::{Toleration, TolerationOperator};

pub use toleration::{toleration_effect, toleration_operator};

pub use template::{
    PodTemplate, PodTemplateList, PodTemplateSpec, apply_pod_template_spec_defaults,
};

pub use ephemeral::{EphemeralContainer, EphemeralContainerCommon};

pub use ephemeral::image_pull_policy;

pub use topology::TopologySpreadConstraint;

pub use topology::{node_affinity_policy, when_unsatisfiable};

pub use component_status::{
    ComponentCondition, ComponentConditionType, ComponentStatus, ComponentStatusList,
};

pub use pod_status_result::PodStatusResult;

// Representative tests for the core/v1 API group
#[cfg(test)]
mod test_core_group;

// Tests for ApplyDefault implementations
#[cfg(test)]
mod defaults_test;

// Roundtrip tests for core/v1 resources
#[cfg(test)]
mod conversion_roundtrip;
#[cfg(test)]
mod serde_roundtrip;

// Tests moved to core/tests/ directory
