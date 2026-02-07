use super::*;

crate::impl_as_str_ref!(PodManagementPolicyType, {
    OrderedReady => pod_management_policy_type::ORDERED_READY,
    Parallel => pod_management_policy_type::PARALLEL,
});

crate::impl_as_str_ref!(StatefulSetUpdateStrategyType, {
    RollingUpdate => stateful_set_update_strategy_type::ROLLING_UPDATE,
    OnDelete => stateful_set_update_strategy_type::ON_DELETE,
});

crate::impl_as_str_ref!(PersistentVolumeClaimRetentionPolicyType, {
    Retain => persistent_volume_claim_retention_policy_type::RETAIN,
    Delete => persistent_volume_claim_retention_policy_type::DELETE,
});

crate::impl_as_str_ref!(StatefulSetConditionType, {
    Unknown => "",
});

crate::impl_as_str_ref!(DeploymentStrategyType, {
    Recreate => deployment_strategy_type::RECREATE,
    RollingUpdate => deployment_strategy_type::ROLLING_UPDATE,
});

crate::impl_as_str_ref!(DeploymentConditionType, {
    Available => deployment_condition_type::AVAILABLE,
    Progressing => deployment_condition_type::PROGRESSING,
    ReplicaFailure => deployment_condition_type::REPLICA_FAILURE,
});

crate::impl_as_str_ref!(DaemonSetUpdateStrategyType, {
    RollingUpdate => daemon_set_update_strategy_type::ROLLING_UPDATE,
    OnDelete => daemon_set_update_strategy_type::ON_DELETE,
});

crate::impl_as_str_ref!(DaemonSetConditionType, {
    Unknown => "",
});

crate::impl_as_str_ref!(ReplicaSetConditionType, {
    ReplicaFailure => replica_set_condition_type::REPLICA_FAILURE,
});
