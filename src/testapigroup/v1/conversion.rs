//! Conversion functions between testapigroup v1 and internal types.

use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};
use crate::testapigroup::internal;

use super::{
    Carp, CarpCondition, CarpConditionType, CarpInfo, CarpList, CarpPhase, CarpSpec, CarpStatus,
    ConditionStatus, RestartPolicy,
};

// ============================================================================
// Helper functions for metadata conversions
// ============================================================================

fn is_empty_object_meta(meta: &ObjectMeta) -> bool {
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

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_list_meta_to_meta(meta: Option<ListMeta>) -> ListMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_list_meta(meta: ListMeta) -> Option<ListMeta> {
    if meta.is_empty() { None } else { Some(meta) }
}

// ============================================================================
// Newtype conversions
// ============================================================================

impl From<ConditionStatus> for internal::ConditionStatus {
    fn from(value: ConditionStatus) -> Self {
        internal::ConditionStatus(value.0)
    }
}

impl From<internal::ConditionStatus> for ConditionStatus {
    fn from(value: internal::ConditionStatus) -> Self {
        ConditionStatus(value.0)
    }
}

impl From<CarpConditionType> for internal::CarpConditionType {
    fn from(value: CarpConditionType) -> Self {
        internal::CarpConditionType(value.0)
    }
}

impl From<internal::CarpConditionType> for CarpConditionType {
    fn from(value: internal::CarpConditionType) -> Self {
        CarpConditionType(value.0)
    }
}

impl From<CarpPhase> for internal::CarpPhase {
    fn from(value: CarpPhase) -> Self {
        internal::CarpPhase(value.0)
    }
}

impl From<internal::CarpPhase> for CarpPhase {
    fn from(value: internal::CarpPhase) -> Self {
        CarpPhase(value.0)
    }
}

impl From<RestartPolicy> for internal::RestartPolicy {
    fn from(value: RestartPolicy) -> Self {
        internal::RestartPolicy(value.0)
    }
}

impl From<internal::RestartPolicy> for RestartPolicy {
    fn from(value: internal::RestartPolicy) -> Self {
        RestartPolicy(value.0)
    }
}

// ============================================================================
// CarpCondition conversions
// ============================================================================

impl From<CarpCondition> for internal::CarpCondition {
    fn from(value: CarpCondition) -> Self {
        internal::CarpCondition {
            type_: value.type_.into(),
            status: value.status.into(),
            last_probe_time: value.last_probe_time,
            last_transition_time: value.last_transition_time,
            reason: value.reason,
            message: value.message,
        }
    }
}

impl From<internal::CarpCondition> for CarpCondition {
    fn from(value: internal::CarpCondition) -> Self {
        CarpCondition {
            type_: value.type_.into(),
            status: value.status.into(),
            last_probe_time: value.last_probe_time,
            last_transition_time: value.last_transition_time,
            reason: value.reason,
            message: value.message,
        }
    }
}

// ============================================================================
// CarpInfo conversions
// ============================================================================

impl From<CarpInfo> for internal::CarpInfo {
    fn from(value: CarpInfo) -> Self {
        internal::CarpInfo {
            a: value.a,
            b: value.b,
            c: value.c,
            data: value.data,
        }
    }
}

impl From<internal::CarpInfo> for CarpInfo {
    fn from(value: internal::CarpInfo) -> Self {
        CarpInfo {
            a: value.a,
            b: value.b,
            c: value.c,
            data: value.data,
        }
    }
}

// ============================================================================
// CarpSpec conversions
// ============================================================================

impl ToInternal<internal::CarpSpec> for CarpSpec {
    fn to_internal(self) -> internal::CarpSpec {
        internal::CarpSpec {
            restart_policy: self.restart_policy.into(),
            termination_grace_period_seconds: self.termination_grace_period_seconds,
            active_deadline_seconds: self.active_deadline_seconds,
            node_selector: self.node_selector,
            service_account_name: self.service_account_name,
            node_name: self.node_name,
            hostname: self.hostname,
            subdomain: self.subdomain,
            scheduler_name: self.scheduler_name,
        }
    }
}

impl FromInternal<internal::CarpSpec> for CarpSpec {
    fn from_internal(value: internal::CarpSpec) -> Self {
        CarpSpec {
            restart_policy: value.restart_policy.into(),
            termination_grace_period_seconds: value.termination_grace_period_seconds,
            active_deadline_seconds: value.active_deadline_seconds,
            node_selector: value.node_selector,
            service_account_name: value.service_account_name,
            deprecated_service_account: String::new(),
            node_name: value.node_name,
            host_network: false,
            host_pid: false,
            host_ipc: false,
            hostname: value.hostname,
            subdomain: value.subdomain,
            scheduler_name: value.scheduler_name,
        }
    }
}

// ============================================================================
// CarpStatus conversions
// ============================================================================

impl ToInternal<internal::CarpStatus> for CarpStatus {
    fn to_internal(self) -> internal::CarpStatus {
        internal::CarpStatus {
            phase: self.phase.into(),
            conditions: self.conditions.into_iter().map(Into::into).collect(),
            message: self.message,
            reason: self.reason,
            host_ip: self.host_ip,
            carp_ip: self.carp_ip,
            start_time: self.start_time,
            infos: self.infos.into_iter().map(Into::into).collect(),
        }
    }
}

impl FromInternal<internal::CarpStatus> for CarpStatus {
    fn from_internal(value: internal::CarpStatus) -> Self {
        CarpStatus {
            phase: value.phase.into(),
            conditions: value.conditions.into_iter().map(Into::into).collect(),
            message: value.message,
            reason: value.reason,
            host_ip: value.host_ip,
            carp_ip: value.carp_ip,
            start_time: value.start_time,
            infos: value.infos.into_iter().map(Into::into).collect(),
        }
    }
}

// ============================================================================
// Carp conversions
// ============================================================================

impl ToInternal<internal::Carp> for Carp {
    fn to_internal(self) -> internal::Carp {
        internal::Carp {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.unwrap_or_default().to_internal(),
            status: self.status.unwrap_or_default().to_internal(),
        }
    }
}

impl FromInternal<internal::Carp> for Carp {
    fn from_internal(value: internal::Carp) -> Self {
        let mut result = Carp {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: Some(CarpSpec::from_internal(value.spec)),
            status: Some(CarpStatus::from_internal(value.status)),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// CarpList conversions
// ============================================================================

impl ToInternal<internal::CarpList> for CarpList {
    fn to_internal(self) -> internal::CarpList {
        internal::CarpList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::CarpList> for CarpList {
    fn from_internal(value: internal::CarpList) -> Self {
        let mut result = CarpList {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value.items.into_iter().map(Carp::from_internal).collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carp_round_trip_drops_v1_only_fields() {
        let v1 = Carp {
            type_meta: TypeMeta {
                api_version: "testapigroup.apimachinery.k8s.io/v1".to_string(),
                kind: "Carp".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("carp-1".to_string()),
                ..Default::default()
            }),
            spec: Some(CarpSpec {
                restart_policy: RestartPolicy::from("Always"),
                termination_grace_period_seconds: Some(10),
                active_deadline_seconds: None,
                node_selector: Default::default(),
                service_account_name: "default".to_string(),
                deprecated_service_account: "deprecated".to_string(),
                node_name: "node-a".to_string(),
                host_network: true,
                host_pid: true,
                host_ipc: true,
                hostname: "carp".to_string(),
                subdomain: "sub".to_string(),
                scheduler_name: "custom".to_string(),
            }),
            status: Some(CarpStatus {
                phase: CarpPhase::from("Running"),
                ..Default::default()
            }),
        };

        let internal = v1.clone().to_internal();
        let round_trip = Carp::from_internal(internal);

        let spec = round_trip.spec.expect("spec should be present");
        assert_eq!(spec.deprecated_service_account, "");
        assert!(!spec.host_network);
        assert!(!spec.host_pid);
        assert!(!spec.host_ipc);

        assert_eq!(
            round_trip.type_meta.api_version,
            "testapigroup.apimachinery.k8s.io/v1"
        );
        assert_eq!(round_trip.type_meta.kind, "Carp");
    }

    #[test]
    fn carp_list_round_trip_applies_defaults() {
        let internal = internal::CarpList {
            type_meta: TypeMeta::default(),
            metadata: ListMeta::default(),
            items: vec![],
        };

        let v1 = CarpList::from_internal(internal);
        assert_eq!(
            v1.type_meta.api_version,
            "testapigroup.apimachinery.k8s.io/v1"
        );
        assert_eq!(v1.type_meta.kind, "CarpList");
    }
}
