//! Validation constants and limits for core v1 API types
//!
//! These constants define limits, ranges, and valid values for validation.
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use std::collections::HashSet;
use std::sync::LazyLock;

// ============================================================================
// Error Messages
// ============================================================================

pub const IS_NEGATIVE_ERROR_MSG: &str = "must be greater than or equal to 0";
pub const IS_NOT_INTEGER_ERROR_MSG: &str = "must be an integer";
pub const IS_NOT_POSITIVE_ERROR_MSG: &str = "must be greater than zero";
pub const FIELD_IMMUTABLE_ERROR_MSG: &str = "field is immutable";
pub const IS_INVALID_QUOTA_RESOURCE: &str = "must be a standard resource for quota";

// ============================================================================
// Port Validation
// ============================================================================

pub const MIN_PORT: i32 = 1;
pub const MAX_PORT: i32 = 65535;
pub const MIN_NODE_PORT: i32 = 30000;
pub const MAX_NODE_PORT: i32 = 32767;

// ============================================================================
// Container Limits
// ============================================================================

pub const MAX_CONTAINERS_PER_POD: usize = 128;
pub const MAX_INIT_CONTAINERS_PER_POD: usize = 128;
pub const MAX_EPHEMERAL_CONTAINERS_PER_POD: usize = 32;

// ============================================================================
// Volume Limits
// ============================================================================

pub const MAX_VOLUMES_PER_POD: usize = 64;
pub const MAX_VOLUME_MOUNTS_PER_CONTAINER: usize = 64;
pub const MAX_VOLUME_DEVICES_PER_CONTAINER: usize = 64;

// ============================================================================
// DNS and Name Limits
// ============================================================================

pub const MAX_DNS1123_LABEL_LEN: usize = 63;
pub const MAX_DNS1123_SUBDOMAIN_LEN: usize = 253;
pub const MAX_DNS1035_LABEL_LEN: usize = 63;
pub const MAX_PATH_LEN: usize = 4096;
pub const MAX_ENV_VAR_NAME_LEN: usize = 253;

// ============================================================================
// File Mode Validation
// ============================================================================

pub const MAX_FILE_MODE: i32 = 0o777;
pub const FILE_MODE_ERROR_MSG: &str = "must be a number between 0 and 0777 (octal), both inclusive";

// ============================================================================
// PD Partition Range
// ============================================================================

pub const MIN_PD_PARTITION: i32 = 1;
pub const MAX_PD_PARTITION: i32 = 255;

// ============================================================================
// Protocol Constants
// ============================================================================

pub const PROTOCOL_TCP: &str = "TCP";
pub const PROTOCOL_UDP: &str = "UDP";
pub const PROTOCOL_SCTP: &str = "SCTP";

pub static SUPPORTED_PORT_PROTOCOLS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    s.insert(PROTOCOL_TCP);
    s.insert(PROTOCOL_UDP);
    s.insert(PROTOCOL_SCTP);
    s
});

// ============================================================================
// Service Type Constants
// ============================================================================

pub const SERVICE_TYPE_CLUSTER_IP: &str = "ClusterIP";
pub const SERVICE_TYPE_NODE_PORT: &str = "NodePort";
pub const SERVICE_TYPE_LOAD_BALANCER: &str = "LoadBalancer";
pub const SERVICE_TYPE_EXTERNAL_NAME: &str = "ExternalName";

// ============================================================================
// Restart Policy Constants
// ============================================================================

pub const RESTART_POLICY_ALWAYS: &str = "Always";
pub const RESTART_POLICY_ON_FAILURE: &str = "OnFailure";
pub const RESTART_POLICY_NEVER: &str = "Never";

pub static SUPPORTED_RESTART_POLICIES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    s.insert(RESTART_POLICY_ALWAYS);
    s.insert(RESTART_POLICY_ON_FAILURE);
    s.insert(RESTART_POLICY_NEVER);
    s
});

// ============================================================================
// DNS Policy Constants
// ============================================================================

pub const DNS_POLICY_CLUSTER_FIRST_WITH_HOST_NET: &str = "ClusterFirstWithHostNet";
pub const DNS_POLICY_CLUSTER_FIRST: &str = "ClusterFirst";
pub const DNS_POLICY_DEFAULT: &str = "Default";
pub const DNS_POLICY_NONE: &str = "None";

// ============================================================================
// Pull Policy Constants
// ============================================================================

pub const PULL_POLICY_ALWAYS: &str = "Always";
pub const PULL_POLICY_NEVER: &str = "Never";
pub const PULL_POLICY_IF_NOT_PRESENT: &str = "IfNotPresent";

pub static SUPPORTED_PULL_POLICIES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    s.insert(PULL_POLICY_ALWAYS);
    s.insert(PULL_POLICY_NEVER);
    s.insert(PULL_POLICY_IF_NOT_PRESENT);
    s
});

// ============================================================================
// HTTP Scheme Constants
// ============================================================================

pub const URI_SCHEME_HTTP: &str = "HTTP";
pub const URI_SCHEME_HTTPS: &str = "HTTPS";

pub static SUPPORTED_HTTP_SCHEMES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    s.insert(URI_SCHEME_HTTP);
    s.insert(URI_SCHEME_HTTPS);
    s
});

// ============================================================================
// Access Mode Constants (for PV/PVC)
// ============================================================================

pub const ACCESS_MODE_READ_WRITE_ONCE: &str = "ReadWriteOnce";
pub const ACCESS_MODE_READ_ONLY_MANY: &str = "ReadOnlyMany";
pub const ACCESS_MODE_READ_WRITE_MANY: &str = "ReadWriteMany";
pub const ACCESS_MODE_READ_WRITE_ONCE_POD: &str = "ReadWriteOncePod";

pub static SUPPORTED_ACCESS_MODES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    s.insert(ACCESS_MODE_READ_WRITE_ONCE);
    s.insert(ACCESS_MODE_READ_ONLY_MANY);
    s.insert(ACCESS_MODE_READ_WRITE_MANY);
    s.insert(ACCESS_MODE_READ_WRITE_ONCE_POD);
    s
});

// ============================================================================
// Reclaim Policy Constants
// ============================================================================

pub const RECLAIM_POLICY_RETAIN: &str = "Retain";
pub const RECLAIM_POLICY_DELETE: &str = "Delete";
pub const RECLAIM_POLICY_RECYCLE: &str = "Recycle";

pub static SUPPORTED_RECLAIM_POLICIES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    s.insert(RECLAIM_POLICY_RETAIN);
    s.insert(RECLAIM_POLICY_DELETE);
    s.insert(RECLAIM_POLICY_RECYCLE);
    s
});

// ============================================================================
// Volume Mode Constants
// ============================================================================

pub const VOLUME_MODE_BLOCK: &str = "Block";
pub const VOLUME_MODE_FILESYSTEM: &str = "Filesystem";

pub static SUPPORTED_VOLUME_MODES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    s.insert(VOLUME_MODE_BLOCK);
    s.insert(VOLUME_MODE_FILESYSTEM);
    s
});

// ============================================================================
// Host Path Type Constants
// ============================================================================

pub const HOST_PATH_UNSET: &str = "";
pub const HOST_PATH_DIRECTORY_OR_CREATE: &str = "DirectoryOrCreate";
pub const HOST_PATH_DIRECTORY: &str = "Directory";
pub const HOST_PATH_FILE_OR_CREATE: &str = "FileOrCreate";
pub const HOST_PATH_FILE: &str = "File";
pub const HOST_PATH_SOCKET: &str = "Socket";
pub const HOST_PATH_CHAR_DEVICE: &str = "CharDevice";
pub const HOST_PATH_BLOCK_DEVICE: &str = "BlockDevice";

pub static SUPPORTED_HOST_PATH_TYPES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    s.insert(HOST_PATH_UNSET);
    s.insert(HOST_PATH_DIRECTORY_OR_CREATE);
    s.insert(HOST_PATH_DIRECTORY);
    s.insert(HOST_PATH_FILE_OR_CREATE);
    s.insert(HOST_PATH_FILE);
    s.insert(HOST_PATH_SOCKET);
    s.insert(HOST_PATH_CHAR_DEVICE);
    s.insert(HOST_PATH_BLOCK_DEVICE);
    s
});

// ============================================================================
// OS Constants
// ============================================================================

pub const OS_LINUX: &str = "linux";
pub const OS_WINDOWS: &str = "windows";

pub static VALID_OS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    s.insert(OS_LINUX);
    s.insert(OS_WINDOWS);
    s
});

// ============================================================================
// Namespace Phase Constants
// ============================================================================

pub const NAMESPACE_PHASE_ACTIVE: &str = "Active";
pub const NAMESPACE_PHASE_TERMINATING: &str = "Terminating";

// ============================================================================
// Taint Effect Constants
// ============================================================================

pub const TAINT_EFFECT_NO_SCHEDULE: &str = "NoSchedule";
pub const TAINT_EFFECT_PREFER_NO_SCHEDULE: &str = "PreferNoSchedule";
pub const TAINT_EFFECT_NO_EXECUTE: &str = "NoExecute";

pub static SUPPORTED_TAINT_EFFECTS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    s.insert(TAINT_EFFECT_NO_SCHEDULE);
    s.insert(TAINT_EFFECT_PREFER_NO_SCHEDULE);
    s.insert(TAINT_EFFECT_NO_EXECUTE);
    s
});

// ============================================================================
// Resize Resource Constants
// ============================================================================

pub const RESOURCE_CPU: &str = "cpu";
pub const RESOURCE_MEMORY: &str = "memory";

pub static SUPPORTED_RESIZE_RESOURCES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut s = HashSet::new();
    s.insert(RESOURCE_CPU);
    s.insert(RESOURCE_MEMORY);
    s
});

// ============================================================================
// Downward API Field Path Expressions
// ============================================================================

pub static VALID_ENV_DOWNWARD_API_FIELD_PATH_EXPRESSIONS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| {
        let mut s = HashSet::new();
        s.insert("metadata.name");
        s.insert("metadata.namespace");
        s.insert("metadata.uid");
        s.insert("spec.nodeName");
        s.insert("spec.serviceAccountName");
        s.insert("status.hostIP");
        s.insert("status.podIP");
        s.insert("status.podIPs");
        s
    });

pub static VALID_VOLUME_DOWNWARD_API_FIELD_PATH_EXPRESSIONS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| {
        let mut s = HashSet::new();
        s.insert("metadata.name");
        s.insert("metadata.namespace");
        s.insert("metadata.uid");
        s.insert("metadata.labels");
        s.insert("metadata.annotations");
        s
    });

// ============================================================================
// Container Resource Field Path Expressions
// ============================================================================

pub static VALID_CONTAINER_RESOURCE_FIELD_PATH_EXPRESSIONS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| {
        let mut s = HashSet::new();
        s.insert("limits.cpu");
        s.insert("limits.memory");
        s.insert("limits.ephemeral-storage");
        s.insert("requests.cpu");
        s.insert("requests.memory");
        s.insert("requests.ephemeral-storage");
        s
    });
