//! Helper functions for conversions between core v1 and internal types
//!
//! Based on k8s.io/kubernetes/pkg/apis/core/v1/conversion.go

use crate::common::{ListMeta, ObjectMeta};
use crate::core::internal;

// ============================================================================
// Metadata conversion helpers
// ============================================================================

pub fn is_empty_object_meta(meta: &ObjectMeta) -> bool {
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

pub fn is_empty_list_meta(meta: &ListMeta) -> bool {
    meta.continue_.is_none()
        && meta.remaining_item_count.is_none()
        && meta.resource_version.is_none()
        && meta.self_link.is_none()
}

pub fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

pub fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

pub fn option_list_meta_to_meta(meta: Option<ListMeta>) -> ListMeta {
    meta.unwrap_or_default()
}

pub fn meta_to_option_list_meta(meta: ListMeta) -> Option<ListMeta> {
    if is_empty_list_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

// ============================================================================
// Enum conversion helpers
// ============================================================================

pub fn option_toleration_operator_to_option_enum(
    s: String,
) -> Option<internal::TolerationOperator> {
    match s.as_str() {
        "" => None,
        "Exists" => Some(internal::TolerationOperator::Exists),
        "Equal" => Some(internal::TolerationOperator::Equal),
        _ => Some(internal::TolerationOperator::Equal), // Default
    }
}

pub fn option_enum_to_toleration_operator(op: Option<internal::TolerationOperator>) -> String {
    match op {
        Some(internal::TolerationOperator::Exists) => "Exists".to_string(),
        Some(internal::TolerationOperator::Equal) => "Equal".to_string(),
        None => String::new(),
    }
}

pub fn option_taint_effect_to_option_enum(s: String) -> Option<internal::TaintEffect> {
    match s.as_str() {
        "" => None,
        "NoSchedule" => Some(internal::TaintEffect::NoSchedule),
        "PreferNoSchedule" => Some(internal::TaintEffect::PreferNoSchedule),
        "NoExecute" => Some(internal::TaintEffect::NoExecute),
        _ => None,
    }
}

pub fn option_enum_to_taint_effect(effect: Option<internal::TaintEffect>) -> String {
    match effect {
        Some(internal::TaintEffect::NoSchedule) => "NoSchedule".to_string(),
        Some(internal::TaintEffect::PreferNoSchedule) => "PreferNoSchedule".to_string(),
        Some(internal::TaintEffect::NoExecute) => "NoExecute".to_string(),
        None => String::new(),
    }
}

pub fn os_name_to_string(name: String) -> internal::OSName {
    match name.to_lowercase().as_str() {
        "linux" => internal::OSName::Linux,
        "windows" => internal::OSName::Windows,
        _ => internal::OSName::Linux, // Default
    }
}

pub fn string_to_os_name(name: internal::OSName) -> String {
    match name {
        internal::OSName::Linux => "linux".to_string(),
        internal::OSName::Windows => "windows".to_string(),
    }
}

pub fn option_string_to_dns_policy(s: Option<String>) -> internal::DNSPolicy {
    match s.as_deref() {
        Some("ClusterFirstWithHostNet") => internal::DNSPolicy::ClusterFirstWithHostNet,
        Some("ClusterFirst") => internal::DNSPolicy::ClusterFirst,
        Some("Default") => internal::DNSPolicy::Default,
        Some("None") => internal::DNSPolicy::None,
        _ => internal::DNSPolicy::default(),
    }
}

pub fn dns_policy_to_option_string(policy: internal::DNSPolicy) -> Option<String> {
    let s = match policy {
        internal::DNSPolicy::ClusterFirstWithHostNet => "ClusterFirstWithHostNet",
        internal::DNSPolicy::ClusterFirst => "ClusterFirst",
        internal::DNSPolicy::Default => "Default",
        internal::DNSPolicy::None => "None",
    };
    Some(s.to_string())
}

pub fn option_string_to_restart_policy(s: Option<String>) -> internal::RestartPolicy {
    match s.as_deref() {
        Some("Always") => internal::RestartPolicy::Always,
        Some("OnFailure") => internal::RestartPolicy::OnFailure,
        Some("Never") => internal::RestartPolicy::Never,
        _ => internal::RestartPolicy::default(),
    }
}

pub fn restart_policy_to_option_string(policy: internal::RestartPolicy) -> Option<String> {
    let s = match policy {
        internal::RestartPolicy::Always => "Always",
        internal::RestartPolicy::OnFailure => "OnFailure",
        internal::RestartPolicy::Never => "Never",
    };
    Some(s.to_string())
}

pub fn option_string_to_pod_phase(s: Option<String>) -> internal::PodPhase {
    match s.as_deref() {
        Some("Pending") => internal::PodPhase::Pending,
        Some("Running") => internal::PodPhase::Running,
        Some("Succeeded") => internal::PodPhase::Succeeded,
        Some("Failed") => internal::PodPhase::Failed,
        Some("Unknown") => internal::PodPhase::Unknown,
        _ => internal::PodPhase::default(),
    }
}

pub fn pod_phase_to_option_string(phase: internal::PodPhase) -> Option<String> {
    let s = match phase {
        internal::PodPhase::Pending => "Pending",
        internal::PodPhase::Running => "Running",
        internal::PodPhase::Succeeded => "Succeeded",
        internal::PodPhase::Failed => "Failed",
        internal::PodPhase::Unknown => "Unknown",
    };
    Some(s.to_string())
}

pub fn option_string_to_protocol(s: Option<String>) -> internal::Protocol {
    match s.as_deref() {
        Some("TCP") => internal::Protocol::Tcp,
        Some("UDP") => internal::Protocol::Udp,
        Some("SCTP") => internal::Protocol::Sctp,
        _ => internal::Protocol::default(),
    }
}

pub fn protocol_to_option_string(protocol: internal::Protocol) -> Option<String> {
    let s = match protocol {
        internal::Protocol::Tcp => "TCP",
        internal::Protocol::Udp => "UDP",
        internal::Protocol::Sctp => "SCTP",
    };
    Some(s.to_string())
}

// ============================================================================
// Node-related enum conversion helpers
// ============================================================================

pub fn option_string_to_taint_effect(s: Option<String>) -> internal::TaintEffect {
    match s.as_deref() {
        Some("NoSchedule") => internal::TaintEffect::NoSchedule,
        Some("PreferNoSchedule") => internal::TaintEffect::PreferNoSchedule,
        Some("NoExecute") => internal::TaintEffect::NoExecute,
        _ => internal::TaintEffect::NoSchedule, // Default
    }
}

pub fn taint_effect_to_option_string(effect: internal::TaintEffect) -> Option<String> {
    let s = match effect {
        internal::TaintEffect::NoSchedule => "NoSchedule",
        internal::TaintEffect::PreferNoSchedule => "PreferNoSchedule",
        internal::TaintEffect::NoExecute => "NoExecute",
    };
    Some(s.to_string())
}

pub fn string_to_node_address_type(s: String) -> internal::NodeAddressType {
    match s.as_str() {
        "Hostname" => internal::NodeAddressType::Hostname,
        "InternalIP" => internal::NodeAddressType::InternalIp,
        "ExternalIP" => internal::NodeAddressType::ExternalIp,
        "InternalDNS" => internal::NodeAddressType::InternalDns,
        "ExternalDNS" => internal::NodeAddressType::ExternalDns,
        _ => internal::NodeAddressType::default(),
    }
}

pub fn node_address_type_to_string(t: internal::NodeAddressType) -> String {
    match t {
        internal::NodeAddressType::Hostname => "Hostname",
        internal::NodeAddressType::InternalIp => "InternalIP",
        internal::NodeAddressType::ExternalIp => "ExternalIP",
        internal::NodeAddressType::InternalDns => "InternalDNS",
        internal::NodeAddressType::ExternalDns => "ExternalDNS",
    }
    .to_string()
}

pub fn string_to_condition_status(s: String) -> internal::ConditionStatus {
    match s.as_str() {
        "True" => internal::ConditionStatus::True,
        "False" => internal::ConditionStatus::False,
        "Unknown" => internal::ConditionStatus::Unknown,
        _ => internal::ConditionStatus::default(),
    }
}

pub fn condition_status_to_string(status: internal::ConditionStatus) -> String {
    match status {
        internal::ConditionStatus::True => "True",
        internal::ConditionStatus::False => "False",
        internal::ConditionStatus::Unknown => "Unknown",
    }
    .to_string()
}

pub fn option_string_to_node_phase(s: Option<String>) -> internal::NodePhase {
    match s.as_deref() {
        Some("Pending") => internal::NodePhase::Pending,
        Some("Running") => internal::NodePhase::Running,
        Some("Terminated") => internal::NodePhase::Terminated,
        _ => internal::NodePhase::default(),
    }
}

pub fn node_phase_to_option_string(phase: internal::NodePhase) -> Option<String> {
    let s = match phase {
        internal::NodePhase::Pending => "Pending",
        internal::NodePhase::Running => "Running",
        internal::NodePhase::Terminated => "Terminated",
    };
    Some(s.to_string())
}

// ============================================================================
// Namespace-related enum conversion helpers
// ============================================================================

pub fn option_string_to_namespace_phase(s: Option<String>) -> Option<internal::NamespacePhase> {
    match s.as_deref() {
        Some("Active") => Some(internal::NamespacePhase::Active),
        Some("Terminating") => Some(internal::NamespacePhase::Terminating),
        _ => None,
    }
}

pub fn namespace_phase_to_option_string(phase: Option<internal::NamespacePhase>) -> Option<String> {
    phase.map(|p| match p {
        internal::NamespacePhase::Active => "Active".to_string(),
        internal::NamespacePhase::Terminating => "Terminating".to_string(),
    })
}

pub fn string_to_namespace_condition_type(s: String) -> internal::NamespaceConditionType {
    match s.as_str() {
        "NamespaceDeletionDiscoveryFailure" => {
            internal::NamespaceConditionType::NamespaceDeletionDiscoveryFailure
        }
        "NamespaceDeletionContentFailure" => {
            internal::NamespaceConditionType::NamespaceDeletionContentFailure
        }
        "NamespaceDeletionGroupVersionParsingFailure" => {
            internal::NamespaceConditionType::NamespaceDeletionGroupVersionParsingFailure
        }
        _ => internal::NamespaceConditionType::default(),
    }
}

pub fn namespace_condition_type_to_string(t: internal::NamespaceConditionType) -> String {
    match t {
        internal::NamespaceConditionType::NamespaceDeletionDiscoveryFailure => {
            "NamespaceDeletionDiscoveryFailure"
        }
        internal::NamespaceConditionType::NamespaceDeletionContentFailure => {
            "NamespaceDeletionContentFailure"
        }
        internal::NamespaceConditionType::NamespaceDeletionGroupVersionParsingFailure => {
            "NamespaceDeletionGroupVersionParsingFailure"
        }
    }
    .to_string()
}

// ============================================================================
// Secret-related enum conversion helpers
// ============================================================================

pub fn option_string_to_secret_type(s: Option<String>) -> internal::SecretType {
    match s.as_deref() {
        Some("Opaque") | None => internal::SecretType::Opaque,
        Some("kubernetes.io/service-account-token") => internal::SecretType::ServiceAccountToken,
        Some("kubernetes.io/dockercfg") => internal::SecretType::Dockercfg,
        Some("kubernetes.io/dockerconfigjson") => internal::SecretType::DockerConfigJson,
        Some("kubernetes.io/basic-auth") => internal::SecretType::BasicAuth,
        Some("kubernetes.io/ssh-auth") => internal::SecretType::SshAuth,
        Some("kubernetes.io/tls") => internal::SecretType::Tls,
        Some("bootstrap.kubernetes.io/token") => internal::SecretType::BootstrapToken,
        _ => internal::SecretType::Opaque, // Default
    }
}

pub fn secret_type_to_option_string(t: internal::SecretType) -> Option<String> {
    let s = match t {
        internal::SecretType::Opaque => "Opaque",
        internal::SecretType::ServiceAccountToken => "kubernetes.io/service-account-token",
        internal::SecretType::Dockercfg => "kubernetes.io/dockercfg",
        internal::SecretType::DockerConfigJson => "kubernetes.io/dockerconfigjson",
        internal::SecretType::BasicAuth => "kubernetes.io/basic-auth",
        internal::SecretType::SshAuth => "kubernetes.io/ssh-auth",
        internal::SecretType::Tls => "kubernetes.io/tls",
        internal::SecretType::BootstrapToken => "bootstrap.kubernetes.io/token",
    };
    Some(s.to_string())
}
