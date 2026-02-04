//! Kubernetes Apps API types
//!
//! This module contains types from the Kubernetes apps API group.

pub mod internal;
pub mod v1;

#[cfg(test)]
pub mod tests;

// Re-export v1 types for convenience
pub use v1::{
    DEFAULT_DAEMON_SET_UNIQUE_LABEL_KEY, DaemonSet, DaemonSetCondition, DaemonSetList,
    DaemonSetSpec, DaemonSetStatus, DaemonSetUpdateStrategy, DaemonSetUpdateStrategyType,
    RollingUpdateDaemonSet,
};

pub use v1::{
    DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY, Deployment, DeploymentCondition, DeploymentConditionType,
    DeploymentList, DeploymentSpec, DeploymentStatus, DeploymentStrategy, DeploymentStrategyType,
    RollingUpdateDeployment,
};

pub use v1::{
    PersistentVolumeClaimRetentionPolicyType, ReplicaSet, ReplicaSetCondition,
    ReplicaSetConditionType, ReplicaSetList, ReplicaSetSpec, ReplicaSetStatus,
};

pub use v1::{
    ControllerRevision, ControllerRevisionList, PodManagementPolicyType,
    RollingUpdateStatefulSetStrategy, StatefulSet, StatefulSetCondition, StatefulSetList,
    StatefulSetOrdinals, StatefulSetPersistentVolumeClaimRetentionPolicy, StatefulSetSpec,
    StatefulSetStatus, StatefulSetUpdateStrategy, StatefulSetUpdateStrategyType,
};

// Re-export constants modules from v1
pub mod constants {
    pub use super::v1::{
        daemon_set_update_strategy_type, deployment_condition_type, deployment_strategy_type,
        persistent_volume_claim_retention_policy_type, pod_management_policy_type,
        replica_set_condition_type, stateful_set_update_strategy_type,
    };
}

// Re-export internal types
pub use internal::{
    DAEMON_SET_TEMPLATE_GENERATION_KEY,
    DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY as INTERNAL_DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY,
    DaemonSet as InternalDaemonSet, DaemonSetCondition as InternalDaemonSetCondition,
    DaemonSetList as InternalDaemonSetList, DaemonSetSpec as InternalDaemonSetSpec,
    DaemonSetStatus as InternalDaemonSetStatus,
    DaemonSetUpdateStrategy as InternalDaemonSetUpdateStrategy,
    DaemonSetUpdateStrategyType as InternalDaemonSetUpdateStrategyType,
    RollingUpdateDaemonSet as InternalRollingUpdateDaemonSet,
    daemon_set_update_strategy_type as internal_daemon_set_update_strategy_type,
};

pub use internal::{
    ControllerRevision as InternalControllerRevision,
    ControllerRevisionList as InternalControllerRevisionList, Deployment as InternalDeployment,
    DeploymentCondition as InternalDeploymentCondition,
    DeploymentConditionType as InternalDeploymentConditionType,
    DeploymentList as InternalDeploymentList, DeploymentRollback,
    DeploymentSpec as InternalDeploymentSpec, DeploymentStatus as InternalDeploymentStatus,
    DeploymentStrategy as InternalDeploymentStrategy,
    DeploymentStrategyType as InternalDeploymentStrategyType, ReplicaSet as InternalReplicaSet,
    ReplicaSetCondition as InternalReplicaSetCondition,
    ReplicaSetConditionType as InternalReplicaSetConditionType,
    ReplicaSetList as InternalReplicaSetList, ReplicaSetSpec as InternalReplicaSetSpec,
    ReplicaSetStatus as InternalReplicaSetStatus, RollbackConfig,
    RollingUpdateDeployment as InternalRollingUpdateDeployment,
    deployment_condition_type as internal_deployment_condition_type,
    deployment_strategy_type as internal_deployment_strategy_type,
    replica_set_condition_type as internal_replica_set_condition_type,
};

pub use internal::{
    PersistentVolumeClaimRetentionPolicyType as InternalPersistentVolumeClaimRetentionPolicyType,
    RollingUpdateStatefulSetStrategy as InternalRollingUpdateStatefulSetStrategy,
    StatefulSet as InternalStatefulSet, StatefulSetCondition as InternalStatefulSetCondition,
    StatefulSetList as InternalStatefulSetList, StatefulSetOrdinals as InternalStatefulSetOrdinals,
    StatefulSetPersistentVolumeClaimRetentionPolicy as InternalStatefulSetPersistentVolumeClaimRetentionPolicy,
    StatefulSetSpec as InternalStatefulSetSpec, StatefulSetStatus as InternalStatefulSetStatus,
    StatefulSetUpdateStrategy as InternalStatefulSetUpdateStrategy,
    StatefulSetUpdateStrategyType as InternalStatefulSetUpdateStrategyType,
    persistent_volume_claim_retention_policy_type as internal_persistent_volume_claim_retention_policy_type,
    pod_management_policy_type as internal_pod_management_policy_type,
    stateful_set_update_strategy_type as internal_stateful_set_update_strategy_type,
};
