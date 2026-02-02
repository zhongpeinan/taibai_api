//! ReplicationController types from Kubernetes Core v1 API
//!
//! This module contains types for the ReplicationController resource,
//! which ensures a specified number of pod replicas are running.

use crate::common::{
    ApplyDefault, HasTypeMeta, ListMeta, ObjectMeta, ResourceSchema, TypeMeta, VersionedObject,
};
use crate::core::v1::template::PodTemplateSpec;
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};

// ============================================================================
// ReplicationController
// ============================================================================

/// ReplicationController ensures that a specified number of pod replicas are running at any given time.
///
/// Corresponds to [Kubernetes ReplicationController](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3367)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationController {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Spec defines the specification of the desired behavior of the replication controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ReplicationControllerSpec>,

    /// Status represents the current status of the replication controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReplicationControllerStatus>,
}

/// ReplicationControllerSpec defines the desired behavior of a replication controller.
///
/// Corresponds to [Kubernetes ReplicationControllerSpec](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3380)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerSpec {
    /// Replicas is the number of desired replicas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,

    /// Minimum number of seconds for which a newly created pod should be ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,

    /// Selector is a label query over pods that should match the replicas count.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub selector: std::collections::BTreeMap<String, String>,

    /// Template is the object that describes the pod that will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<PodTemplateSpec>,
}

/// ReplicationControllerStatus represents the current status of a replication controller.
///
/// Corresponds to [Kubernetes ReplicationControllerStatus](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3395)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerStatus {
    /// Replicas is the number of actual replicas.
    #[serde(default)]
    pub replicas: i32,

    /// FullyLabeledReplicas is the number of pods that have labels.
    #[serde(default)]
    pub fully_labeled_replicas: i32,

    /// ReadyReplicas is the number of ready replicas.
    #[serde(default)]
    pub ready_replicas: i32,

    /// AvailableReplicas is the number of available replicas.
    #[serde(default)]
    pub available_replicas: i32,

    /// ObservedGeneration is the most recent generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// Conditions represent the latest available observations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ReplicationControllerCondition>,
}

/// ReplicationControllerCondition describes the state of a replication controller at a certain point.
///
/// Corresponds to [Kubernetes ReplicationControllerCondition](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3414)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerCondition {
    /// Type of replication controller condition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    /// Status of the condition, one of True, False, Unknown.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String,

    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<crate::common::Timestamp>,

    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// A human-readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// ReplicationControllerList is a collection of replication controllers.
///
/// Corresponds to [Kubernetes ReplicationControllerList](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L3374)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerList {
    /// TypeMeta describes the type of this object
    #[serde(flatten)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of replication controllers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ReplicationController>,
}

// ============================================================================
// Trait Implementations for ReplicationController Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for ReplicationController {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ReplicationController"
    }
    fn resource(_: &Self::Meta) -> &str {
        "replicationcontrollers"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ReplicationController"
    }
    fn resource_static() -> &'static str {
        "replicationcontrollers"
    }
}

impl ResourceSchema for ReplicationControllerList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        ""
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ReplicationControllerList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "replicationcontrollers"
    }

    fn group_static() -> &'static str {
        ""
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ReplicationControllerList"
    }
    fn resource_static() -> &'static str {
        "replicationcontrollers"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for ReplicationController {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ReplicationControllerList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

impl VersionedObject for ReplicationController {
    fn metadata(&self) -> &ObjectMeta {
        use std::sync::OnceLock;
        self.metadata.as_ref().unwrap_or_else(|| {
            static DEFAULT: OnceLock<ObjectMeta> = OnceLock::new();
            DEFAULT.get_or_init(ObjectMeta::default)
        })
    }

    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        self.metadata.get_or_insert_with(ObjectMeta::default)
    }
}

// Note: ReplicationControllerList does not implement VersionedObject because its metadata is ListMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for ReplicationController {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ReplicationController".to_string();
        }

        if let Some(ref mut spec) = self.spec {
            let template_labels = spec
                .template
                .as_ref()
                .and_then(|template| template.metadata.as_ref())
                .map(|meta| &meta.labels);

            if let Some(labels) = template_labels
                && !labels.is_empty()
            {
                if spec.selector.is_empty() {
                    spec.selector = labels.clone();
                }

                let meta = self.metadata.get_or_insert_with(ObjectMeta::default);
                if meta.labels.is_empty() {
                    meta.labels = labels.clone();
                }
            }
        }
    }
}

impl ApplyDefault for ReplicationControllerList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ReplicationControllerList".to_string();
        }
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(ReplicationController);
impl_unimplemented_prost_message!(ReplicationControllerList);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ApplyDefault;
    use std::collections::BTreeMap;

    #[test]
    fn test_replication_controller_defaults_selector_and_labels() {
        let mut rc = ReplicationController {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: Some(ReplicationControllerSpec {
                replicas: Some(1),
                min_ready_seconds: None,
                selector: BTreeMap::new(),
                template: Some(PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        labels: {
                            let mut labels = BTreeMap::new();
                            labels.insert("app".to_string(), "demo".to_string());
                            labels
                        },
                        ..Default::default()
                    }),
                    spec: None,
                }),
            }),
            status: None,
        };

        rc.apply_default();

        let spec = rc.spec.as_ref().unwrap();
        assert_eq!(spec.selector.get("app").map(String::as_str), Some("demo"));
        assert_eq!(
            rc.metadata
                .as_ref()
                .and_then(|meta| meta.labels.get("app"))
                .map(String::as_str),
            Some("demo")
        );
    }
}
