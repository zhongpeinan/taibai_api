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

pub fn os_name_to_string(name: String) -> Option<internal::OSName> {
    match name.to_lowercase().as_str() {
        "linux" => Some(internal::OSName::Linux),
        "windows" => Some(internal::OSName::Windows),
        "" => None,
        _ => None,
    }
}

pub fn string_to_os_name(name: Option<internal::OSName>) -> String {
    match name {
        Some(internal::OSName::Linux) => "linux".to_string(),
        Some(internal::OSName::Windows) => "windows".to_string(),
        None => String::new(),
    }
}

pub fn option_string_to_dns_policy(s: Option<String>) -> Option<internal::DNSPolicy> {
    match s.as_deref() {
        Some("ClusterFirstWithHostNet") => Some(internal::DNSPolicy::ClusterFirstWithHostNet),
        Some("ClusterFirst") => Some(internal::DNSPolicy::ClusterFirst),
        Some("Default") => Some(internal::DNSPolicy::Default),
        Some("None") => Some(internal::DNSPolicy::None),
        Some(_) => None, // Unknown value
        None => None,    // Preserve None
    }
}

pub fn dns_policy_to_option_string(policy: Option<internal::DNSPolicy>) -> Option<String> {
    policy.map(|p| {
        match p {
            internal::DNSPolicy::ClusterFirstWithHostNet => "ClusterFirstWithHostNet",
            internal::DNSPolicy::ClusterFirst => "ClusterFirst",
            internal::DNSPolicy::Default => "Default",
            internal::DNSPolicy::None => "None",
        }
        .to_string()
    })
}

pub fn option_string_to_restart_policy(s: Option<String>) -> Option<internal::RestartPolicy> {
    match s.as_deref() {
        Some("Always") => Some(internal::RestartPolicy::Always),
        Some("OnFailure") => Some(internal::RestartPolicy::OnFailure),
        Some("Never") => Some(internal::RestartPolicy::Never),
        Some(_) => None, // Unknown value
        None => None,    // Preserve None
    }
}

pub fn restart_policy_to_option_string(policy: Option<internal::RestartPolicy>) -> Option<String> {
    policy.map(|p| {
        match p {
            internal::RestartPolicy::Always => "Always",
            internal::RestartPolicy::OnFailure => "OnFailure",
            internal::RestartPolicy::Never => "Never",
        }
        .to_string()
    })
}

pub fn option_string_to_pod_phase(s: Option<String>) -> Option<internal::PodPhase> {
    match s.as_deref() {
        Some("Pending") => Some(internal::PodPhase::Pending),
        Some("Running") => Some(internal::PodPhase::Running),
        Some("Succeeded") => Some(internal::PodPhase::Succeeded),
        Some("Failed") => Some(internal::PodPhase::Failed),
        Some("Unknown") => Some(internal::PodPhase::Unknown),
        Some(_) => None, // Unknown value
        None => None,    // Preserve None
    }
}

pub fn pod_phase_to_option_string(phase: Option<internal::PodPhase>) -> Option<String> {
    phase.map(|p| {
        match p {
            internal::PodPhase::Pending => "Pending",
            internal::PodPhase::Running => "Running",
            internal::PodPhase::Succeeded => "Succeeded",
            internal::PodPhase::Failed => "Failed",
            internal::PodPhase::Unknown => "Unknown",
        }
        .to_string()
    })
}

pub fn option_string_to_protocol(s: Option<String>) -> Option<internal::Protocol> {
    match s.as_deref() {
        Some("TCP") => Some(internal::Protocol::Tcp),
        Some("UDP") => Some(internal::Protocol::Udp),
        Some("SCTP") => Some(internal::Protocol::Sctp),
        Some(_) => None, // Unknown value
        None => None,    // Preserve None
    }
}

pub fn protocol_to_option_string(protocol: Option<internal::Protocol>) -> Option<String> {
    protocol.map(|p| {
        match p {
            internal::Protocol::Tcp => "TCP",
            internal::Protocol::Udp => "UDP",
            internal::Protocol::Sctp => "SCTP",
        }
        .to_string()
    })
}

// ============================================================================
// Node-related enum conversion helpers
// ============================================================================

pub fn option_string_to_taint_effect(s: Option<String>) -> Option<internal::TaintEffect> {
    match s.as_deref() {
        Some("NoSchedule") => Some(internal::TaintEffect::NoSchedule),
        Some("PreferNoSchedule") => Some(internal::TaintEffect::PreferNoSchedule),
        Some("NoExecute") => Some(internal::TaintEffect::NoExecute),
        Some(_) => None, // Unknown value
        None => None,    // Preserve None
    }
}

pub fn taint_effect_to_option_string(effect: Option<internal::TaintEffect>) -> Option<String> {
    effect.map(|e| {
        match e {
            internal::TaintEffect::NoSchedule => "NoSchedule",
            internal::TaintEffect::PreferNoSchedule => "PreferNoSchedule",
            internal::TaintEffect::NoExecute => "NoExecute",
        }
        .to_string()
    })
}

pub fn string_to_node_address_type(s: String) -> Option<internal::NodeAddressType> {
    match s.as_str() {
        "Hostname" => Some(internal::NodeAddressType::Hostname),
        "InternalIP" => Some(internal::NodeAddressType::InternalIp),
        "ExternalIP" => Some(internal::NodeAddressType::ExternalIp),
        "InternalDNS" => Some(internal::NodeAddressType::InternalDns),
        "ExternalDNS" => Some(internal::NodeAddressType::ExternalDns),
        "" => None,
        _ => None,
    }
}

pub fn node_address_type_to_string(t: Option<internal::NodeAddressType>) -> String {
    match t {
        Some(internal::NodeAddressType::Hostname) => "Hostname",
        Some(internal::NodeAddressType::InternalIp) => "InternalIP",
        Some(internal::NodeAddressType::ExternalIp) => "ExternalIP",
        Some(internal::NodeAddressType::InternalDns) => "InternalDNS",
        Some(internal::NodeAddressType::ExternalDns) => "ExternalDNS",
        None => "",
    }
    .to_string()
}

pub fn string_to_condition_status(s: String) -> Option<internal::ConditionStatus> {
    match s.as_str() {
        "True" => Some(internal::ConditionStatus::True),
        "False" => Some(internal::ConditionStatus::False),
        "Unknown" => Some(internal::ConditionStatus::Unknown),
        "" => None,
        _ => None,
    }
}

pub fn condition_status_to_string(status: Option<internal::ConditionStatus>) -> String {
    match status {
        Some(internal::ConditionStatus::True) => "True",
        Some(internal::ConditionStatus::False) => "False",
        Some(internal::ConditionStatus::Unknown) => "Unknown",
        None => "",
    }
    .to_string()
}

pub fn option_string_to_node_phase(s: Option<String>) -> Option<internal::NodePhase> {
    match s.as_deref() {
        Some("Pending") => Some(internal::NodePhase::Pending),
        Some("Running") => Some(internal::NodePhase::Running),
        Some("Terminated") => Some(internal::NodePhase::Terminated),
        Some(_) => None, // Unknown value
        None => None,    // Preserve None
    }
}

pub fn node_phase_to_option_string(phase: Option<internal::NodePhase>) -> Option<String> {
    phase.map(|p| {
        match p {
            internal::NodePhase::Pending => "Pending",
            internal::NodePhase::Running => "Running",
            internal::NodePhase::Terminated => "Terminated",
        }
        .to_string()
    })
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

pub fn option_string_to_secret_type(s: Option<String>) -> Option<internal::SecretType> {
    match s.as_deref() {
        Some("Opaque") => Some(internal::SecretType::Opaque),
        Some("kubernetes.io/service-account-token") => {
            Some(internal::SecretType::ServiceAccountToken)
        }
        Some("kubernetes.io/dockercfg") => Some(internal::SecretType::Dockercfg),
        Some("kubernetes.io/dockerconfigjson") => Some(internal::SecretType::DockerConfigJson),
        Some("kubernetes.io/basic-auth") => Some(internal::SecretType::BasicAuth),
        Some("kubernetes.io/ssh-auth") => Some(internal::SecretType::SshAuth),
        Some("kubernetes.io/tls") => Some(internal::SecretType::Tls),
        Some("bootstrap.kubernetes.io/token") => Some(internal::SecretType::BootstrapToken),
        Some(_) => None, // Unknown value
        None => None,    // Preserve None
    }
}

pub fn secret_type_to_option_string(t: Option<internal::SecretType>) -> Option<String> {
    t.map(|st| {
        match st {
            internal::SecretType::Opaque => "Opaque",
            internal::SecretType::ServiceAccountToken => "kubernetes.io/service-account-token",
            internal::SecretType::Dockercfg => "kubernetes.io/dockercfg",
            internal::SecretType::DockerConfigJson => "kubernetes.io/dockerconfigjson",
            internal::SecretType::BasicAuth => "kubernetes.io/basic-auth",
            internal::SecretType::SshAuth => "kubernetes.io/ssh-auth",
            internal::SecretType::Tls => "kubernetes.io/tls",
            internal::SecretType::BootstrapToken => "bootstrap.kubernetes.io/token",
        }
        .to_string()
    })
}
