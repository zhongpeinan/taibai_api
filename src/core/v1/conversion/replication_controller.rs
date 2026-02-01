//! ReplicationController conversion implementations
//!
//! Includes: ReplicationController, ReplicationControllerList, ReplicationControllerSpec,
//! ReplicationControllerStatus, ReplicationControllerCondition, and RC PodTemplateSpec

use super::helpers::*;
use crate::common::traits::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::{pod, replication_controller, template};

// ============================================================================
// ReplicationController
// ============================================================================

impl ToInternal<internal::ReplicationController> for replication_controller::ReplicationController {
    fn to_internal(self) -> internal::ReplicationController {
        internal::ReplicationController {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(|s| s.to_internal()),
            status: self.status.map(|s| s.to_internal()).unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::ReplicationController>
    for replication_controller::ReplicationController
{
    fn from_internal(value: internal::ReplicationController) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value
                .spec
                .map(replication_controller::ReplicationControllerSpec::from_internal),
            status: Some(
                replication_controller::ReplicationControllerStatus::from_internal(value.status),
            ),
        };

        result
    }
}

// ============================================================================
// ReplicationControllerList
// ============================================================================

impl ToInternal<internal::ReplicationControllerList>
    for replication_controller::ReplicationControllerList
{
    fn to_internal(self) -> internal::ReplicationControllerList {
        internal::ReplicationControllerList {
            type_meta: crate::common::TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::ReplicationControllerList>
    for replication_controller::ReplicationControllerList
{
    fn from_internal(value: internal::ReplicationControllerList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(replication_controller::ReplicationController::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// ReplicationControllerSpec
// ============================================================================

impl ToInternal<internal::ReplicationControllerSpec>
    for replication_controller::ReplicationControllerSpec
{
    fn to_internal(self) -> internal::ReplicationControllerSpec {
        internal::ReplicationControllerSpec {
            replicas: self.replicas.unwrap_or_default(),
            min_ready_seconds: self.min_ready_seconds,
            selector: self.selector,
            template: self.template.map(|t| t.to_internal()),
        }
    }
}

impl FromInternal<internal::ReplicationControllerSpec>
    for replication_controller::ReplicationControllerSpec
{
    fn from_internal(value: internal::ReplicationControllerSpec) -> Self {
        Self {
            replicas: Some(value.replicas),
            min_ready_seconds: value.min_ready_seconds,
            selector: value.selector,
            template: value.template.map(template::PodTemplateSpec::from_internal),
        }
    }
}

// ============================================================================
// ReplicationControllerStatus
// ============================================================================

impl ToInternal<internal::ReplicationControllerStatus>
    for replication_controller::ReplicationControllerStatus
{
    fn to_internal(self) -> internal::ReplicationControllerStatus {
        internal::ReplicationControllerStatus {
            replicas: self.replicas,
            fully_labeled_replicas: self.fully_labeled_replicas,
            ready_replicas: self.ready_replicas,
            available_replicas: self.available_replicas,
            observed_generation: self.observed_generation,
            conditions: self
                .conditions
                .into_iter()
                .map(|c| c.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::ReplicationControllerStatus>
    for replication_controller::ReplicationControllerStatus
{
    fn from_internal(value: internal::ReplicationControllerStatus) -> Self {
        Self {
            replicas: value.replicas,
            fully_labeled_replicas: value.fully_labeled_replicas,
            ready_replicas: value.ready_replicas,
            available_replicas: value.available_replicas,
            observed_generation: value.observed_generation,
            conditions: value
                .conditions
                .into_iter()
                .map(replication_controller::ReplicationControllerCondition::from_internal)
                .collect(),
        }
    }
}

// ============================================================================
// ReplicationControllerCondition
// ============================================================================

impl ToInternal<internal::ReplicationControllerCondition>
    for replication_controller::ReplicationControllerCondition
{
    fn to_internal(self) -> internal::ReplicationControllerCondition {
        internal::ReplicationControllerCondition {
            r#type: self.r#type,
            status: self.status,
            last_transition_time: self.last_transition_time,
            reason: self.reason.unwrap_or_default(),
            message: self.message.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::ReplicationControllerCondition>
    for replication_controller::ReplicationControllerCondition
{
    fn from_internal(value: internal::ReplicationControllerCondition) -> Self {
        Self {
            r#type: value.r#type,
            status: value.status,
            last_transition_time: value.last_transition_time,
            reason: if value.reason.is_empty() {
                None
            } else {
                Some(value.reason)
            },
            message: if value.message.is_empty() {
                None
            } else {
                Some(value.message)
            },
        }
    }
}

// ============================================================================
// ReplicationController PodTemplateSpec
// ============================================================================

impl ToInternal<internal::replication_controller::PodTemplateSpec> for template::PodTemplateSpec {
    fn to_internal(self) -> internal::replication_controller::PodTemplateSpec {
        internal::replication_controller::PodTemplateSpec {
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(|s| s.to_internal()),
        }
    }
}

impl FromInternal<internal::replication_controller::PodTemplateSpec> for template::PodTemplateSpec {
    fn from_internal(value: internal::replication_controller::PodTemplateSpec) -> Self {
        Self {
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(pod::PodSpec::from_internal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_replication_controller_roundtrip() {
        let rc = replication_controller::ReplicationController {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("demo".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            spec: Some(replication_controller::ReplicationControllerSpec {
                replicas: Some(3),
                min_ready_seconds: Some(5),
                selector: BTreeMap::from([("app".to_string(), "demo".to_string())]),
                template: Some(template::PodTemplateSpec {
                    metadata: Some(crate::common::ObjectMeta {
                        labels: BTreeMap::from([("app".to_string(), "demo".to_string())]),
                        ..Default::default()
                    }),
                    spec: None,
                }),
            }),
            status: Some(replication_controller::ReplicationControllerStatus {
                replicas: 2,
                fully_labeled_replicas: 2,
                ready_replicas: 1,
                available_replicas: 1,
                observed_generation: Some(7),
                conditions: vec![replication_controller::ReplicationControllerCondition {
                    r#type: "Available".to_string(),
                    status: "True".to_string(),
                    last_transition_time: None,
                    reason: Some("Ready".to_string()),
                    message: None,
                }],
            }),
        };

        let internal = rc.clone().to_internal();
        assert_eq!(internal.metadata.name.as_deref(), Some("demo"));
        assert_eq!(internal.spec.as_ref().unwrap().replicas, 3);
        assert_eq!(
            internal
                .spec
                .as_ref()
                .and_then(|s| s.template.as_ref())
                .map(|t| t.metadata.labels.get("app").cloned())
                .flatten()
                .as_deref(),
            Some("demo")
        );

        let mut roundtrip = replication_controller::ReplicationController::from_internal(internal);
        roundtrip.apply_default();
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name.as_deref(),
            Some("demo")
        );
        assert_eq!(roundtrip.spec.as_ref().unwrap().replicas, Some(3));
        assert_eq!(
            roundtrip
                .spec
                .as_ref()
                .unwrap()
                .template
                .as_ref()
                .and_then(|t| t.metadata.as_ref())
                .and_then(|m| m.labels.get("app"))
                .map(String::as_str),
            Some("demo")
        );
        assert_eq!(
            roundtrip
                .status
                .as_ref()
                .unwrap()
                .conditions
                .first()
                .and_then(|c| c.reason.as_deref()),
            Some("Ready")
        );
        assert_eq!(
            roundtrip
                .status
                .as_ref()
                .unwrap()
                .conditions
                .first()
                .and_then(|c| c.message.as_deref()),
            None
        );
    }
}
