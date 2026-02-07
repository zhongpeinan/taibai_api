use crate::common::traits::AsRefStr;
use crate::core::internal::*;
use crate::impl_as_str_ref;

impl AsRefStr for DNSPolicy {
    fn as_str(&self) -> &str {
        match self {
            DNSPolicy::ClusterFirstWithHostNet => dns_policy::CLUSTER_FIRST_WITH_HOST_NET,
            DNSPolicy::ClusterFirst => dns_policy::CLUSTER_FIRST,
            DNSPolicy::Default => dns_policy::DEFAULT,
            DNSPolicy::None => dns_policy::NONE,
            DNSPolicy::Unknown(value) => value.as_str(),
        }
    }
}

impl AsRef<str> for DNSPolicy {
    fn as_ref(&self) -> &str {
        <Self as AsRefStr>::as_str(self)
    }
}

impl_as_str_ref!(PodPhase, {
    Pending => pod_phase::PENDING,
    Running => pod_phase::RUNNING,
    Succeeded => pod_phase::SUCCEEDED,
    Failed => pod_phase::FAILED,
    Unknown => pod_phase::UNKNOWN,
});

impl_as_str_ref!(PodConditionType, {
    PodScheduled => pod_condition_type::POD_SCHEDULED,
    Ready => pod_condition_type::READY,
    Initialized => pod_condition_type::INITIALIZED,
    ContainersReady => pod_condition_type::CONTAINERS_READY,
    DisruptionTarget => pod_condition_type::DISRUPTION_TARGET,
    PodResizePending => pod_condition_type::POD_RESIZE_PENDING,
    PodResizeInProgress => pod_condition_type::POD_RESIZE_IN_PROGRESS,
});

impl_as_str_ref!(RestartPolicy, {
    Always => restart_policy::ALWAYS,
    OnFailure => restart_policy::ON_FAILURE,
    Never => restart_policy::NEVER,
});

impl_as_str_ref!(Protocol, {
    Tcp => protocol::TCP,
    Udp => protocol::UDP,
    Sctp => protocol::SCTP,
});

impl_as_str_ref!(PullPolicy, {
    Always => pull_policy::ALWAYS,
    Never => pull_policy::NEVER,
    IfNotPresent => pull_policy::IF_NOT_PRESENT,
});

impl_as_str_ref!(ConditionStatus, {
    True => condition_status::TRUE,
    False => condition_status::FALSE,
    Unknown => condition_status::UNKNOWN,
});

impl_as_str_ref!(NamespacePhase, {
    Active => namespace_phase::ACTIVE,
    Terminating => namespace_phase::TERMINATING,
});

impl_as_str_ref!(NamespaceConditionType, {
    NamespaceDeletionDiscoveryFailure => namespace_condition_type::NAMESPACE_DELETION_DISCOVERY_FAILURE,
    NamespaceDeletionContentFailure => namespace_condition_type::NAMESPACE_DELETION_CONTENT_FAILURE,
    NamespaceDeletionGroupVersionParsingFailure => namespace_condition_type::NAMESPACE_DELETION_GV_PARSING_FAILURE,
});

impl_as_str_ref!(ServiceType, {
    ClusterIp => service_type::CLUSTER_IP,
    NodePort => service_type::NODE_PORT,
    LoadBalancer => service_type::LOAD_BALANCER,
    ExternalName => service_type::EXTERNAL_NAME,
});

impl_as_str_ref!(NodePhase, {
    Pending => node_phase::PENDING,
    Running => node_phase::RUNNING,
    Terminated => node_phase::TERMINATED,
});

impl_as_str_ref!(NodeConditionType, {
    Ready => node_condition_type::READY,
    MemoryPressure => node_condition_type::MEMORY_PRESSURE,
    DiskPressure => node_condition_type::DISK_PRESSURE,
    NetworkUnavailable => node_condition_type::NETWORK_UNAVAILABLE,
});

impl_as_str_ref!(NodeAddressType, {
    Hostname => node_address_type::HOSTNAME,
    InternalIp => node_address_type::INTERNAL_IP,
    ExternalIp => node_address_type::EXTERNAL_IP,
    InternalDns => node_address_type::INTERNAL_DNS,
    ExternalDns => node_address_type::EXTERNAL_DNS,
});

impl_as_str_ref!(TaintEffect, {
    NoSchedule => taint_effect::NO_SCHEDULE,
    PreferNoSchedule => taint_effect::PREFER_NO_SCHEDULE,
    NoExecute => taint_effect::NO_EXECUTE,
});

impl_as_str_ref!(PodQOSClass, {
    Guaranteed => pod_qos_class::GUARANTEED,
    Burstable => pod_qos_class::BURSTABLE,
    BestEffort => pod_qos_class::BEST_EFFORT,
});

impl_as_str_ref!(ServiceAffinity, {
    ClientIp => service_affinity::CLIENT_IP,
    None => service_affinity::NONE,
});

impl_as_str_ref!(ServiceInternalTrafficPolicy, {
    Cluster => service_internal_traffic_policy::CLUSTER,
    Local => service_internal_traffic_policy::LOCAL,
});

impl_as_str_ref!(ServiceExternalTrafficPolicy, {
    Cluster => service_external_traffic_policy::CLUSTER,
    Local => service_external_traffic_policy::LOCAL,
});

impl_as_str_ref!(IPFamily, {
    Ipv4 => ip_family::IPV4,
    Ipv6 => ip_family::IPV6,
});

impl_as_str_ref!(IPFamilyPolicy, {
    SingleStack => ip_family_policy::SINGLE_STACK,
    PreferDualStack => ip_family_policy::PREFER_DUAL_STACK,
    RequireDualStack => ip_family_policy::REQUIRE_DUAL_STACK,
});

impl_as_str_ref!(LoadBalancerIPMode, {
    Vip => load_balancer_ip_mode::VIP,
    Proxy => load_balancer_ip_mode::PROXY,
});

impl_as_str_ref!(ContainerRestartPolicy, {
    Always => container_restart_policy::ALWAYS,
    Never => container_restart_policy::NEVER,
    OnFailure => container_restart_policy::ON_FAILURE,
});

impl_as_str_ref!(PreemptionPolicy, {
    PreemptLowerPriority => preemption_policy::PREEMPT_LOWER_PRIORITY,
    Never => preemption_policy::NEVER,
});

impl_as_str_ref!(TerminationMessagePolicy, {
    File => termination_message_policy::FILE,
    FallbackToLogsOnError => termination_message_policy::FALLBACK_TO_LOGS_ON_ERROR,
});

impl_as_str_ref!(TolerationOperator, {
    Exists => toleration_operator::EXISTS,
    Equal => toleration_operator::EQUAL,
});

impl_as_str_ref!(NodeSelectorOperator, {
    In => node_selector_operator::IN,
    NotIn => node_selector_operator::NOT_IN,
    Exists => node_selector_operator::EXISTS,
    DoesNotExist => node_selector_operator::DOES_NOT_EXIST,
    Gt => node_selector_operator::GT,
    Lt => node_selector_operator::LT,
});

impl_as_str_ref!(OSName, {
    Linux => os_name::LINUX,
    Windows => os_name::WINDOWS,
});

impl_as_str_ref!(PodFSGroupChangePolicy, {
    OnRootMismatch => pod_fs_group_change_policy::ON_ROOT_MISMATCH,
    Always => pod_fs_group_change_policy::ALWAYS,
});

impl_as_str_ref!(SupplementalGroupsPolicy, {
    Merge => supplemental_groups_policy::MERGE,
    Strict => supplemental_groups_policy::STRICT,
});

impl_as_str_ref!(PodSELinuxChangePolicy, {
    Recursive => pod_selinux_change_policy::RECURSIVE,
    MountOption => pod_selinux_change_policy::MOUNT_OPTION,
});

impl_as_str_ref!(SeccompProfileType, {
    Unconfined => seccomp_profile_type::UNCONFINED,
    RuntimeDefault => seccomp_profile_type::RUNTIME_DEFAULT,
    Localhost => seccomp_profile_type::LOCALHOST,
});

impl_as_str_ref!(AppArmorProfileType, {
    Unconfined => app_armor_profile_type::UNCONFINED,
    RuntimeDefault => app_armor_profile_type::RUNTIME_DEFAULT,
    Localhost => app_armor_profile_type::LOCALHOST,
});

impl_as_str_ref!(ProcMountType, {
    Default => proc_mount_type::DEFAULT,
    Unmasked => proc_mount_type::UNMASKED,
});

impl_as_str_ref!(PersistentVolumeAccessMode, {
    ReadWriteOnce => persistent_volume_access_mode::READ_WRITE_ONCE,
    ReadOnlyMany => persistent_volume_access_mode::READ_ONLY_MANY,
    ReadWriteMany => persistent_volume_access_mode::READ_WRITE_MANY,
    ReadWriteOncePod => persistent_volume_access_mode::READ_WRITE_ONCE_POD,
});

impl_as_str_ref!(PersistentVolumeReclaimPolicy, {
    Recycle => persistent_volume_reclaim_policy::RECYCLE,
    Delete => persistent_volume_reclaim_policy::DELETE,
    Retain => persistent_volume_reclaim_policy::RETAIN,
});

impl_as_str_ref!(PersistentVolumeMode, {
    Block => persistent_volume_mode::BLOCK,
    Filesystem => persistent_volume_mode::FILESYSTEM,
});

impl_as_str_ref!(PersistentVolumePhase, {
    Pending => persistent_volume_phase::PENDING,
    Available => persistent_volume_phase::AVAILABLE,
    Bound => persistent_volume_phase::BOUND,
    Released => persistent_volume_phase::RELEASED,
    Failed => persistent_volume_phase::FAILED,
});

impl_as_str_ref!(PersistentVolumeClaimPhase, {
    Pending => persistent_volume_claim_phase::PENDING,
    Bound => persistent_volume_claim_phase::BOUND,
    Lost => persistent_volume_claim_phase::LOST,
});

impl_as_str_ref!(HostPathType, {
    Unset => host_path_type::UNSET,
    DirectoryOrCreate => host_path_type::DIRECTORY_OR_CREATE,
    Directory => host_path_type::DIRECTORY,
    FileOrCreate => host_path_type::FILE_OR_CREATE,
    File => host_path_type::FILE,
    Socket => host_path_type::SOCKET,
    CharDevice => host_path_type::CHAR_DEVICE,
    BlockDevice => host_path_type::BLOCK_DEVICE,
});

impl_as_str_ref!(StorageMedium, {
    Default => storage_medium::DEFAULT,
    Memory => storage_medium::MEMORY,
    HugePages => storage_medium::HUGE_PAGES,
    HugePagesPrefix => storage_medium::HUGE_PAGES_PREFIX,
});

impl_as_str_ref!(MountPropagationMode, {
    None => mount_propagation_mode::NONE,
    HostToContainer => mount_propagation_mode::HOST_TO_CONTAINER,
    Bidirectional => mount_propagation_mode::BIDIRECTIONAL,
});

impl_as_str_ref!(RecursiveReadOnlyMode, {
    Disabled => recursive_read_only_mode::DISABLED,
    IfPossible => recursive_read_only_mode::IF_POSSIBLE,
    Enabled => recursive_read_only_mode::ENABLED,
});

impl_as_str_ref!(ResourceResizeRestartPolicy, {
    NotRequired => resource_resize_restart_policy::NOT_REQUIRED,
    RestartContainer => resource_resize_restart_policy::RESTART_CONTAINER,
});

impl_as_str_ref!(PodResizeStatus, {
    InProgress => pod_resize_status::IN_PROGRESS,
    Deferred => pod_resize_status::DEFERRED,
    Infeasible => pod_resize_status::INFEASIBLE,
});

impl_as_str_ref!(AzureDataDiskCachingMode, {
    None => azure_data_disk_caching_mode::NONE,
    ReadOnly => azure_data_disk_caching_mode::READ_ONLY,
    ReadWrite => azure_data_disk_caching_mode::READ_WRITE,
});

impl_as_str_ref!(AzureDataDiskKind, {
    Shared => azure_data_disk_kind::SHARED,
    Dedicated => azure_data_disk_kind::DEDICATED,
    Managed => azure_data_disk_kind::MANAGED,
});

impl_as_str_ref!(LimitType, {
    Pod => limit_type::POD,
    Container => limit_type::CONTAINER,
    PersistentVolumeClaim => limit_type::PERSISTENT_VOLUME_CLAIM,
});

impl_as_str_ref!(ResourceQuotaScope, {
    Terminating => resource_quota_scope::TERMINATING,
    NotTerminating => resource_quota_scope::NOT_TERMINATING,
    BestEffort => resource_quota_scope::BEST_EFFORT,
    NotBestEffort => resource_quota_scope::NOT_BEST_EFFORT,
    PriorityClass => resource_quota_scope::PRIORITY_CLASS,
    CrossNamespacePodAffinity => resource_quota_scope::CROSS_NAMESPACE_POD_AFFINITY,
    VolumeAttributesClass => resource_quota_scope::VOLUME_ATTRIBUTES_CLASS,
});

impl_as_str_ref!(SecretType, {
    Opaque => secret_type::OPAQUE,
    ServiceAccountToken => secret_type::SERVICE_ACCOUNT_TOKEN,
    Dockercfg => secret_type::DOCKERCFG,
    DockerConfigJson => secret_type::DOCKER_CONFIG_JSON,
    BasicAuth => secret_type::BASIC_AUTH,
    SshAuth => secret_type::SSH_AUTH,
    Tls => secret_type::TLS,
    BootstrapToken => secret_type::BOOTSTRAP_TOKEN,
});

impl_as_str_ref!(ReplicationControllerConditionType, {
    ReplicaFailure => replication_controller_condition_type::REPLICA_FAILURE,
});

impl_as_str_ref!(ComponentConditionType, {
    Healthy => component_condition_type::HEALTHY,
});

impl_as_str_ref!(Signal, {
    Sigabrt => signal::SIGABRT,
    Sigalrm => signal::SIGALRM,
    Sigbus => signal::SIGBUS,
    Sigchld => signal::SIGCHLD,
    Sigcld => signal::SIGCLD,
    Sigcont => signal::SIGCONT,
    Sigfpe => signal::SIGFPE,
    Sighup => signal::SIGHUP,
    Sigill => signal::SIGILL,
    Sigint => signal::SIGINT,
    Sigio => signal::SIGIO,
    Sigiot => signal::SIGIOT,
    Sigkill => signal::SIGKILL,
    Sigpipe => signal::SIGPIPE,
    Sigpoll => signal::SIGPOLL,
    Sigprof => signal::SIGPROF,
    Sigpwr => signal::SIGPWR,
    Sigquit => signal::SIGQUIT,
    Sigsegv => signal::SIGSEGV,
    Sigstkflt => signal::SIGSTKFLT,
    Sigstop => signal::SIGSTOP,
    Sigsys => signal::SIGSYS,
    Sigterm => signal::SIGTERM,
    Sigtrap => signal::SIGTRAP,
    Sigtstp => signal::SIGTSTP,
    Sigttin => signal::SIGTTIN,
    Sigttou => signal::SIGTTOU,
    Sigurg => signal::SIGURG,
    Sigusr1 => signal::SIGUSR1,
    Sigusr2 => signal::SIGUSR2,
    Sigvtalrm => signal::SIGVTALRM,
    Sigwinch => signal::SIGWINCH,
    Sigxcpu => signal::SIGXCPU,
    Sigxfsz => signal::SIGXFSZ,
    Sigrtmin => signal::SIGRTMIN,
    SigrtminPlus1 => signal::SIGRTMIN_PLUS_1,
    SigrtminPlus2 => signal::SIGRTMIN_PLUS_2,
    SigrtminPlus3 => signal::SIGRTMIN_PLUS_3,
    SigrtminPlus4 => signal::SIGRTMIN_PLUS_4,
    SigrtminPlus5 => signal::SIGRTMIN_PLUS_5,
    SigrtminPlus6 => signal::SIGRTMIN_PLUS_6,
    SigrtminPlus7 => signal::SIGRTMIN_PLUS_7,
    SigrtminPlus8 => signal::SIGRTMIN_PLUS_8,
    SigrtminPlus9 => signal::SIGRTMIN_PLUS_9,
    SigrtminPlus10 => signal::SIGRTMIN_PLUS_10,
    SigrtminPlus11 => signal::SIGRTMIN_PLUS_11,
    SigrtminPlus12 => signal::SIGRTMIN_PLUS_12,
    SigrtminPlus13 => signal::SIGRTMIN_PLUS_13,
    SigrtminPlus14 => signal::SIGRTMIN_PLUS_14,
    SigrtminPlus15 => signal::SIGRTMIN_PLUS_15,
    SigrtmaxMinus14 => signal::SIGRTMAX_MINUS_14,
    SigrtmaxMinus13 => signal::SIGRTMAX_MINUS_13,
    SigrtmaxMinus12 => signal::SIGRTMAX_MINUS_12,
    SigrtmaxMinus11 => signal::SIGRTMAX_MINUS_11,
    SigrtmaxMinus10 => signal::SIGRTMAX_MINUS_10,
    SigrtmaxMinus9 => signal::SIGRTMAX_MINUS_9,
    SigrtmaxMinus8 => signal::SIGRTMAX_MINUS_8,
    SigrtmaxMinus7 => signal::SIGRTMAX_MINUS_7,
    SigrtmaxMinus6 => signal::SIGRTMAX_MINUS_6,
    SigrtmaxMinus5 => signal::SIGRTMAX_MINUS_5,
    SigrtmaxMinus4 => signal::SIGRTMAX_MINUS_4,
    SigrtmaxMinus3 => signal::SIGRTMAX_MINUS_3,
    SigrtmaxMinus2 => signal::SIGRTMAX_MINUS_2,
    SigrtmaxMinus1 => signal::SIGRTMAX_MINUS_1,
    Sigrtmax => signal::SIGRTMAX,
});
