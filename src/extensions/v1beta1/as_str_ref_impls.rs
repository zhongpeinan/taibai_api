use super::*;

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

crate::impl_as_str_ref!(ReplicaSetConditionType, {
    ReplicaFailure => replica_set_condition_type::REPLICA_FAILURE,
});

crate::impl_as_str_ref!(PathType, {
    Exact => path_type::EXACT,
    Prefix => path_type::PREFIX,
    ImplementationSpecific => path_type::IMPLEMENTATION_SPECIFIC,
});

crate::impl_as_str_ref!(PolicyType, {
    Ingress => policy_type::INGRESS,
    Egress => policy_type::EGRESS,
});
