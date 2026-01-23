//! Kubernetes common metadata types
//!
//! This module contains the fundamental metadata types used across Kubernetes API objects.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::time::Timestamp;
use crate::impl_unimplemented_prost_message;

/// TypeMeta describes an individual object in an API response or request
/// with Kind and Version fields.
///
/// Corresponds to [Kubernetes TypeMeta](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L42)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TypeMeta {
    /// Kind is a string value representing the REST resource this object represents.
    ///
    /// Servers may infer this from the endpoint the client submits requests to.
    /// Cannot be updated.
    /// In CamelCase.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,

    /// APIVersion defines the versioned schema of this representation of an object.
    /// Servers should convert recognized schemas to the latest internal value,
    /// and may reject unrecognized values.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_version: String,
}

impl_unimplemented_prost_message!(TypeMeta);

/// ListMeta describes metadata that synthetic resources must have, including lists and status objects.
///
/// Corresponds to [Kubernetes ListMeta](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L2375)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListMeta {
    /// continue may be set if the user set a limit on the number of items returned, and indicates
    /// that the server has more data available.
    #[serde(rename = "continue", default, skip_serializing_if = "Option::is_none")]
    pub continue_: Option<String>,

    /// remainingItemCount is the number of subsequent items in the list which are not included
    /// in this list response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_item_count: Option<i64>,

    /// resourceVersion sets a resource version constraint on what kind of objects are included in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,

    /// SelfLink is a URL representing this list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

/// ObjectMeta is metadata that all persisted resources must have, which includes all objects
/// users must create.
///
/// Corresponds to [Kubernetes ObjectMeta](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L110)
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMeta {
    /// Name must be unique within a namespace.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// GenerateName is an optional prefix, used by the server, to generate a unique
    /// name ONLY IF the Name field has not been provided.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#idempotency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_name: Option<String>,

    /// Namespace defines the space within which each name must be unique.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// UID is the unique in time and space value for this object.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// An opaque value that represents the internal version of this object.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,

    /// A sequence number representing a specific generation of the desired state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,

    /// SelfLink is a URL representing this object.
    /// Deprecated: selfLink is a legacy read-only field that is no longer populated by the system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Map of string keys and values that can be used to organize and categorize objects.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,

    /// Annotations is an unstructured key value map stored with a resource.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,

    /// List of objects depended by this object. If ALL objects in the list have
    /// been deleted, this object will be garbage collected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owner_references: Vec<OwnerReference>,

    /// Must be empty before the object is deleted from the registry. Each entry
    /// is an identifier for the responsible component that will remove the entry
    /// from the list. If the deletionTimestamp of the object is non-nil, entries
    /// in this list can only be removed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finalizers: Vec<String>,

    /// ManagedFields maps workflow-id and version to the set of fields that are managed by that workflow.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub managed_fields: Vec<ManagedFieldsEntry>,

    /// CreationTimestamp is a timestamp representing the server time when this object was created.
    /// It is represented in RFC3339 form and is UTC. For example: "2024-01-15T10:00:00Z"
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<Timestamp>,

    /// DeletionTimestamp is RFC3339 string representing the time when this object will be deleted.
    /// This field is set by the server when a graceful deletion is initiated. For example: "2024-01-15T10:00:00Z"
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<Timestamp>,

    /// Number of seconds allowed for this object to gracefully terminate before
    /// it will be removed from the system. Only set when deletionTimestamp is also set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_grace_period_seconds: Option<i64>,
}

impl_unimplemented_prost_message!(ObjectMeta);

impl ObjectMeta {
    /// Get name, returns empty string if not set (Go-style zero value).
    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }

    /// Get namespace, returns empty string if not set (Go-style zero value).
    pub fn namespace(&self) -> &str {
        self.namespace.as_deref().unwrap_or("")
    }

    /// Get generate_name, returns empty string if not set (Go-style zero value).
    pub fn generate_name(&self) -> &str {
        self.generate_name.as_deref().unwrap_or("")
    }

    /// Get uid, returns empty string if not set (Go-style zero value).
    pub fn uid(&self) -> &str {
        self.uid.as_deref().unwrap_or("")
    }

    /// Get resource_version, returns empty string if not set (Go-style zero value).
    pub fn resource_version(&self) -> &str {
        self.resource_version.as_deref().unwrap_or("")
    }

    /// Get self_link, returns empty string if not set (Go-style zero value).
    pub fn self_link(&self) -> &str {
        self.self_link.as_deref().unwrap_or("")
    }

    /// Get generation, returns 0 if not set (Go-style zero value).
    pub fn generation(&self) -> i64 {
        self.generation.unwrap_or(0)
    }

    /// Get deletion_grace_period_seconds, returns 0 if not set (Go-style zero value).
    pub fn deletion_grace_period_seconds(&self) -> i64 {
        self.deletion_grace_period_seconds.unwrap_or(0)
    }
}

/// ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource
/// that the fieldset applies to.
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ManagedFieldsEntry {
    /// Manager is an identifier of the workflow managing these fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,

    /// Operation is the type of operation which lead to this ManagedFieldsEntry being merged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,

    /// APIVersion defines the version of this resource that this field set applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Time is the timestamp of when the ManagedFieldsEntry was added.
    /// It is represented in RFC3339 form and is UTC. For example: "2024-01-15T10:00:00Z"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Timestamp>,

    /// FieldsType is the discriminator for the different fields format and version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields_type: Option<String>,

    /// FieldsV1 holds the first JSON version of the fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields_v1: Option<serde_json::Value>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subresource: Option<String>,
}

/// OwnerReference contains enough information to let you identify an owning object.
/// An owning object must be in the same namespace as the dependent, or be cluster-scoped,
/// so there is no namespace field.
///
/// Corresponds to [Kubernetes OwnerReference](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L267)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub struct OwnerReference {
    /// API version of the referent.
    pub api_version: String,

    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,

    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#names
    pub name: String,

    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids
    pub uid: String,

    /// If true, this reference points to the managing controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<bool>,

    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then
    /// the owner cannot be deleted from the key-value store until this
    /// reference is removed.
    /// See https://kubernetes.io/docs/concepts/architecture/garbage-collection/#foreground-deletion
    /// for how the garbage collector interacts with this field and enforces the foreground deletion.
    /// Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_owner_deletion: Option<bool>,
}

impl_unimplemented_prost_message!(OwnerReference);

/// Condition defines an observation of a resource's state.
///
/// Corresponds to [Kubernetes Condition](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1339)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    /// Type of condition in CamelCase or in foo.example.com/CamelCase.
    #[serde(rename = "type")]
    pub type_: String,

    /// Status of the condition, one of True, False, Unknown.
    pub status: String,

    /// ObservedGeneration represents the .metadata.generation that the condition was set based upon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// LastTransitionTime is the last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<Timestamp>,

    /// Reason contains a programmatic identifier indicating the reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Message is a human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// LabelSelector is a label query over a set of resources.
///
/// Corresponds to [Kubernetes LabelSelector](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1210)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelector {
    /// matchLabels is a map of {key,value} pairs.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub match_labels: BTreeMap<String, String>,

    /// matchExpressions is a list of label selector requirements.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_expressions: Vec<LabelSelectorRequirement>,
}

/// LabelSelectorRequirement is a selector that contains values, a key, and an operator.
///
/// Corresponds to [Kubernetes LabelSelectorRequirement](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1246)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelectorRequirement {
    /// key is the label key that the selector applies to.
    pub key: String,

    /// operator represents a key's relationship to a set of values.
    pub operator: String,

    /// values is an array of string values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// Label selector operator constants
pub mod label_selector_operator {
    /// In means the label must match one of the values
    pub const IN: &str = "In";
    /// NotIn means the label must not match any of the values
    pub const NOT_IN: &str = "NotIn";
    /// Exists means the label must exist (values must be empty)
    pub const EXISTS: &str = "Exists";
    /// DoesNotExist means the label must not exist
    pub const DOES_NOT_EXIST: &str = "DoesNotExist";
}

/// FieldSelectorRequirement is a selector that contains values, a key, and an operator.
///
/// Corresponds to [Kubernetes FieldSelectorRequirement](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1283)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FieldSelectorRequirement {
    /// key is the field key that the selector applies to.
    pub key: String,

    /// operator represents a key's relationship to a set of values.
    pub operator: String,

    /// values is an array of string values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

/// Field selector operator constants
pub mod field_selector_operator {
    /// In means the field must match one of the values
    pub const IN: &str = "In";
    /// NotIn means the field must not match any of the values
    pub const NOT_IN: &str = "NotIn";
    /// Exists means the field must exist (values must be empty)
    pub const EXISTS: &str = "Exists";
    /// DoesNotExist means the field must not exist
    pub const DOES_NOT_EXIST: &str = "DoesNotExist";
}

/// GroupVersionKind unambiguously identifies a kind.
///
/// Corresponds to [Kubernetes GroupVersionKind](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L76)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupVersionKind {
    /// Group is the API group.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Version is the API version.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// Kind is the resource kind.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
}

/// GroupVersionResource unambiguously identifies a resource.
///
/// Corresponds to [Kubernetes GroupVersionResource](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L86)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupVersionResource {
    /// Group is the API group.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Version is the API version.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// Resource is the resource name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
}

/// GroupResource identifies a resource by group and resource name.
///
/// Corresponds to [Kubernetes GroupResource](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L1198)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupResource {
    /// Group is the API group.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Resource is the resource name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
}

/// StatusCause is a brief explanation of the reason for a condition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusCause {
    /// A machine-readable description of the cause of the error.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// A human-readable description of the cause of the error.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// The field of the resource that has caused this error.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub field: String,
}

/// StatusDetails is a set of additional properties that MAY be set by the
/// server to provide additional information about a response.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusDetails {
    /// The name attribute of the resource associated with the status StatusReason.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The group attribute of the resource associated with the status StatusReason.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// The kind attribute of the resource associated with the status StatusReason.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    /// The UID attribute of the resource associated with the status StatusReason.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
    /// The Causes array includes more details associated with the StatusReason failure.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub causes: Vec<StatusCause>,
    /// If specified, the time in seconds before the operation should be retried.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: Option<i32>,
}

/// Status is a return value for calls that don't return other objects.
///
/// Corresponds to [Kubernetes Status](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go#L2356)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,
    /// Status of the operation (one of "Success" or "Failure").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// A human-readable description of the status of this operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// A machine-readable description of why this operation is in the "Failure" status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Extended data associated with the reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<StatusDetails>,
    /// Suggested HTTP return code for this status, 0 if not set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
}

/// Status constants
pub mod status {
    /// StatusSuccess indicates that the operation succeeded
    pub const SUCCESS: &str = "Success";
    /// StatusFailure indicates that the operation failed
    pub const FAILURE: &str = "Failure";
}

#[cfg(test)]
mod tests {}
