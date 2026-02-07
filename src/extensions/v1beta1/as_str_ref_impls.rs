use super::*;

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

crate::impl_as_str_ref!(ReplicaSetConditionType, {
    ReplicaFailure => "ReplicaFailure",
});

crate::impl_as_str_ref!(PathType, {
    Exact => "Exact",
    Prefix => "Prefix",
    ImplementationSpecific => "ImplementationSpecific",
});

crate::impl_as_str_ref!(PolicyType, {
    Ingress => "Ingress",
    Egress => "Egress",
});
