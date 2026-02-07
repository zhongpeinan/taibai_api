use super::*;

crate::impl_as_str_ref!(PodManagementPolicyType, {
    OrderedReady => "OrderedReady",
    Parallel => "Parallel",
});

crate::impl_as_str_ref!(StatefulSetUpdateStrategyType, {
    RollingUpdate => "RollingUpdate",
    OnDelete => "OnDelete",
});

crate::impl_as_str_ref!(PersistentVolumeClaimRetentionPolicyType, {
    Retain => "Retain",
    Delete => "Delete",
});

crate::impl_as_str_ref!(StatefulSetConditionType, {
    Unknown => "Unknown",
});

crate::impl_as_str_ref!(DeploymentStrategyType, {
    Recreate => "Recreate",
    RollingUpdate => "RollingUpdate",
});

crate::impl_as_str_ref!(DeploymentConditionType, {
    Available => "Available",
    Progressing => "Progressing",
    ReplicaFailure => "ReplicaFailure",
});

crate::impl_as_str_ref!(DaemonSetUpdateStrategyType, {
    RollingUpdate => "RollingUpdate",
    OnDelete => "OnDelete",
});

crate::impl_as_str_ref!(DaemonSetConditionType, {
    Unknown => "Unknown",
});

crate::impl_as_str_ref!(ReplicaSetConditionType, {
    ReplicaFailure => "ReplicaFailure",
});
