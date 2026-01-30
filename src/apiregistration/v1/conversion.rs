//! Conversions between apiregistration v1 and internal types.
//!
//! Source: k8s.io/kube-aggregator/pkg/apis/apiregistration/v1/zz_generated.conversion.go

use crate::apiregistration::internal;
use crate::common::{
    ApplyDefault, FromInternal, ListMeta, ObjectMeta, Timestamp, ToInternal, TypeMeta,
};

use super::{
    APIService, APIServiceCondition, APIServiceConditionType, APIServiceList, APIServiceSpec,
    APIServiceStatus, ConditionStatus, ServiceReference,
};

// ============================================================================
// Helper Functions
// ============================================================================

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if meta.is_empty() { None } else { Some(meta) }
}

fn option_list_meta_to_meta(meta: Option<ListMeta>) -> ListMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_list_meta(meta: ListMeta) -> Option<ListMeta> {
    if meta.is_empty() { None } else { Some(meta) }
}

fn option_timestamp_to_timestamp(value: Option<Timestamp>) -> Timestamp {
    value.unwrap_or_else(Timestamp::zero)
}

fn timestamp_to_option_timestamp(value: Timestamp) -> Option<Timestamp> {
    if value.is_zero() { None } else { Some(value) }
}

// ============================================================================
// Enum Conversions
// ============================================================================

impl From<ConditionStatus> for internal::ConditionStatus {
    fn from(value: ConditionStatus) -> Self {
        match value {
            ConditionStatus::True => internal::ConditionStatus::True,
            ConditionStatus::False => internal::ConditionStatus::False,
            ConditionStatus::Unknown => internal::ConditionStatus::Unknown,
        }
    }
}

impl From<internal::ConditionStatus> for ConditionStatus {
    fn from(value: internal::ConditionStatus) -> Self {
        match value {
            internal::ConditionStatus::True => ConditionStatus::True,
            internal::ConditionStatus::False => ConditionStatus::False,
            internal::ConditionStatus::Unknown => ConditionStatus::Unknown,
        }
    }
}

impl From<APIServiceConditionType> for internal::APIServiceConditionType {
    fn from(value: APIServiceConditionType) -> Self {
        match value {
            APIServiceConditionType::Available => internal::APIServiceConditionType::Available,
        }
    }
}

impl From<internal::APIServiceConditionType> for APIServiceConditionType {
    fn from(value: internal::APIServiceConditionType) -> Self {
        match value {
            internal::APIServiceConditionType::Available => APIServiceConditionType::Available,
        }
    }
}

// ============================================================================
// Supporting Type Conversions
// ============================================================================

impl ToInternal<internal::ServiceReference> for ServiceReference {
    fn to_internal(self) -> internal::ServiceReference {
        internal::ServiceReference {
            namespace: self.namespace,
            name: self.name,
            port: self.port.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::ServiceReference> for ServiceReference {
    fn from_internal(value: internal::ServiceReference) -> Self {
        let port = if value.port == 0 {
            None
        } else {
            Some(value.port)
        };
        Self {
            namespace: value.namespace,
            name: value.name,
            port,
        }
    }
}

impl ToInternal<internal::APIServiceSpec> for APIServiceSpec {
    fn to_internal(self) -> internal::APIServiceSpec {
        internal::APIServiceSpec {
            service: self.service.map(ServiceReference::to_internal),
            group: self.group,
            version: self.version,
            insecure_skip_tls_verify: self.insecure_skip_tls_verify,
            ca_bundle: self.ca_bundle,
            group_priority_minimum: self.group_priority_minimum,
            version_priority: self.version_priority,
        }
    }
}

impl FromInternal<internal::APIServiceSpec> for APIServiceSpec {
    fn from_internal(value: internal::APIServiceSpec) -> Self {
        Self {
            service: value.service.map(ServiceReference::from_internal),
            group: value.group,
            version: value.version,
            insecure_skip_tls_verify: value.insecure_skip_tls_verify,
            ca_bundle: value.ca_bundle,
            group_priority_minimum: value.group_priority_minimum,
            version_priority: value.version_priority,
        }
    }
}

impl ToInternal<internal::APIServiceCondition> for APIServiceCondition {
    fn to_internal(self) -> internal::APIServiceCondition {
        internal::APIServiceCondition {
            type_: self.type_.into(),
            status: self.status.into(),
            last_transition_time: option_timestamp_to_timestamp(self.last_transition_time),
            reason: self.reason,
            message: self.message,
        }
    }
}

impl FromInternal<internal::APIServiceCondition> for APIServiceCondition {
    fn from_internal(value: internal::APIServiceCondition) -> Self {
        Self {
            type_: value.type_.into(),
            status: value.status.into(),
            last_transition_time: timestamp_to_option_timestamp(value.last_transition_time),
            reason: value.reason,
            message: value.message,
        }
    }
}

impl ToInternal<internal::APIServiceStatus> for APIServiceStatus {
    fn to_internal(self) -> internal::APIServiceStatus {
        internal::APIServiceStatus {
            conditions: self
                .conditions
                .into_iter()
                .map(APIServiceCondition::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::APIServiceStatus> for APIServiceStatus {
    fn from_internal(value: internal::APIServiceStatus) -> Self {
        Self {
            conditions: value
                .conditions
                .into_iter()
                .map(APIServiceCondition::from_internal)
                .collect(),
        }
    }
}

// ============================================================================
// Top-level Conversions
// ============================================================================

impl ToInternal<internal::APIService> for APIService {
    fn to_internal(self) -> internal::APIService {
        internal::APIService {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.to_internal(),
            status: self.status.to_internal(),
        }
    }
}

impl FromInternal<internal::APIService> for APIService {
    fn from_internal(value: internal::APIService) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: APIServiceSpec::from_internal(value.spec),
            status: APIServiceStatus::from_internal(value.status),
        };
        result.apply_default();
        result
    }
}

impl ToInternal<internal::APIServiceList> for APIServiceList {
    fn to_internal(self) -> internal::APIServiceList {
        internal::APIServiceList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(APIService::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::APIServiceList> for APIServiceList {
    fn from_internal(value: internal::APIServiceList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(APIService::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;

    #[test]
    fn api_service_round_trip() {
        let original = APIService {
            type_meta: TypeMeta {
                api_version: "apiregistration.k8s.io/v1".to_string(),
                kind: "APIService".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("v1.apps".to_string()),
                ..Default::default()
            }),
            spec: APIServiceSpec {
                group: "apps".to_string(),
                version: "v1".to_string(),
                group_priority_minimum: 1000,
                version_priority: 20,
                ..Default::default()
            },
            status: Default::default(),
        };

        let internal = original.clone().to_internal();
        let round_trip = APIService::from_internal(internal);

        assert_eq!(round_trip.spec.group, original.spec.group);
        assert_eq!(round_trip.spec.version, original.spec.version);
        assert_eq!(round_trip.type_meta.kind, "APIService");
    }
}
