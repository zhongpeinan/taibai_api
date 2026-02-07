//! Apiextensions internal types.
//!
//! Source: k8s.io/apiextensions-apiserver/pkg/apis/apiextensions/types.go

use crate::apiextensions::JSONSchemaProps;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::ByteString;
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};

// ============================================================================
// Enums
// ============================================================================

/// ConversionStrategyType describes different conversion types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum ConversionStrategyType {
    /// NoneConverter sets only apiVersion.
    #[default]
    None,
    /// WebhookConverter calls an external webhook for conversion.
    Webhook,
}

/// ConversionStrategyType constants
pub mod conversion_strategy_type {
    pub const NONE: &str = "None";
    pub const WEBHOOK: &str = "Webhook";
}

/// ResourceScope defines the different scopes available to a custom resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum ResourceScope {
    /// Cluster scoped resource.
    Cluster,
    /// Namespaced scoped resource.
    #[default]
    Namespaced,
}

/// ResourceScope constants
pub mod resource_scope {
    pub const CLUSTER: &str = "Cluster";
    pub const NAMESPACED: &str = "Namespaced";
}

/// ConditionStatus is the status of a condition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum ConditionStatus {
    /// Condition is true.
    True,
    /// Condition is false.
    False,
    /// Condition status is unknown.
    #[default]
    Unknown,
}

/// ConditionStatus constants
pub mod condition_status {
    pub const TRUE: &str = "True";
    pub const FALSE: &str = "False";
    pub const UNKNOWN: &str = "Unknown";
}

/// CustomResourceDefinitionConditionType is a valid value for CustomResourceDefinitionCondition.type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum CustomResourceDefinitionConditionType {
    /// Resource has become active.
    #[default]
    Established,
    /// Names are accepted.
    NamesAccepted,
    /// Non-structural schema.
    NonStructuralSchema,
    /// CustomResourceDefinition is terminating.
    Terminating,
    /// Kubernetes API approval policy conformant.
    KubernetesAPIApprovalPolicyConformant,
}

/// CustomResourceDefinitionConditionType constants
pub mod custom_resource_definition_condition_type {
    pub const ESTABLISHED: &str = "Established";
    pub const NAMES_ACCEPTED: &str = "NamesAccepted";
    pub const NON_STRUCTURAL_SCHEMA: &str = "NonStructuralSchema";
    pub const TERMINATING: &str = "Terminating";
    pub const KUBERNETES_API_APPROVAL_POLICY_CONFORMANT: &str =
        "KubernetesAPIApprovalPolicyConformant";
}

// ============================================================================
// Constants
// ============================================================================

/// CustomResourceCleanupFinalizer is the name of the finalizer which will delete instances of a CRD.
pub const CUSTOM_RESOURCE_CLEANUP_FINALIZER: &str = "customresourcecleanup.apiextensions.k8s.io";

// ============================================================================
// Core Types
// ============================================================================

/// CustomResourceDefinitionSpec describes how a user wants their resource to appear.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionSpec {
    /// Group is the group this resource belongs in.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,

    /// Version is the version this resource belongs in.
    /// Deprecated: use versions.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,

    /// Names are the names used to describe this custom resource.
    #[serde(default)]
    pub names: CustomResourceDefinitionNames,

    /// Scope indicates whether this resource is cluster or namespace scoped.
    #[serde(default)]
    pub scope: ResourceScope,

    /// Validation describes the validation methods for CustomResources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<CustomResourceValidation>,

    /// Subresources describes the subresources for CustomResource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subresources: Option<CustomResourceSubresources>,

    /// Versions is the list of all supported versions for this resource.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<CustomResourceDefinitionVersion>,

    /// AdditionalPrinterColumns are additional columns shown in kubectl.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_printer_columns: Vec<CustomResourceColumnDefinition>,

    /// SelectableFields specifies paths to fields that may be used as field selectors.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectable_fields: Vec<SelectableField>,

    /// Conversion defines conversion settings for the CRD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversion: Option<CustomResourceConversion>,

    /// PreserveUnknownFields disables pruning of object fields not specified in schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_unknown_fields: Option<bool>,
}

/// CustomResourceConversion describes how to convert different versions of a CR.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceConversion {
    /// Strategy specifies the conversion strategy.
    #[serde(default)]
    pub strategy: ConversionStrategyType,

    /// WebhookClientConfig is the instructions for how to call the webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_client_config: Option<WebhookClientConfig>,

    /// ConversionReviewVersions are the preferred ConversionReview versions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conversion_review_versions: Vec<String>,
}

/// WebhookClientConfig contains the information to make a TLS connection with the webhook.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebhookClientConfig {
    /// URL gives the location of the webhook in standard URL form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Service is a reference to the service for this webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceReference>,

    /// CABundle is a PEM encoded CA bundle.
    #[serde(default)]
    pub ca_bundle: ByteString,
}

/// ServiceReference holds a reference to Service.legacy.k8s.io.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceReference {
    /// Namespace is the namespace of the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,

    /// Name is the name of the service.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Path is an optional URL path at which the webhook will be contacted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Port is an optional service port at which the webhook will be contacted.
    #[serde(default)]
    pub port: i32,
}

/// CustomResourceDefinitionVersion describes a version for CRD.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionVersion {
    /// Name is the version name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Served enables/disables this version.
    #[serde(default)]
    pub served: bool,

    /// Storage indicates the storage version.
    #[serde(default)]
    pub storage: bool,

    /// Deprecated indicates this version is deprecated.
    #[serde(default)]
    pub deprecated: bool,

    /// DeprecationWarning overrides the default warning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecation_warning: Option<String>,

    /// Schema describes the schema for CustomResource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<CustomResourceValidation>,

    /// Subresources describes the subresources for CustomResource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subresources: Option<CustomResourceSubresources>,

    /// AdditionalPrinterColumns are additional columns shown in kubectl.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_printer_columns: Vec<CustomResourceColumnDefinition>,

    /// SelectableFields specifies paths to fields that may be used as field selectors.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selectable_fields: Vec<SelectableField>,
}

/// SelectableField specifies the JSON path of a field that may be used with field selectors.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectableField {
    /// jsonPath is a simple JSON path to a field.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub json_path: String,
}

/// CustomResourceColumnDefinition specifies a column for server side printing.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceColumnDefinition {
    /// Name is a human readable name for the column.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    /// Type is an OpenAPI type definition for this column.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    /// Format is an optional OpenAPI type definition for this column.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub format: String,

    /// Description is a human readable description of this column.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,

    /// Priority is an integer defining the relative importance of this column.
    #[serde(default)]
    pub priority: i32,

    /// JSONPath is a simple JSON path.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub json_path: String,
}

/// CustomResourceDefinitionNames indicates the names to serve this CustomResourceDefinition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionNames {
    /// Plural is the plural name of the resource to serve.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub plural: String,

    /// Singular is the singular name of the resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub singular: String,

    /// ShortNames are short names for the resource.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub short_names: Vec<String>,

    /// Kind is the serialized kind of the resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,

    /// ListKind is the serialized kind of the list for this resource.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub list_kind: String,

    /// Categories is a list of grouped resources this custom resource belongs to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
}

/// CustomResourceDefinitionCondition contains details for the current condition of this CRD.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionCondition {
    /// Type is the type of the condition.
    #[serde(default)]
    pub r#type: CustomResourceDefinitionConditionType,

    /// Status is the status of the condition.
    #[serde(default)]
    pub status: ConditionStatus,

    /// LastTransitionTime is last time the condition transitioned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<crate::common::Timestamp>,

    /// Reason is a unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,

    /// Message is a human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionStatus {
    /// Conditions indicate state for particular aspects of a CRD.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<CustomResourceDefinitionCondition>,

    /// AcceptedNames are the names that are actually being used to serve discovery.
    #[serde(default)]
    pub accepted_names: CustomResourceDefinitionNames,

    /// StoredVersions are all versions of CustomResources that were ever persisted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stored_versions: Vec<String>,
}

/// CustomResourceDefinition represents a resource that should be exposed on the API server.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinition {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,

    /// Standard object's metadata.
    pub metadata: ObjectMeta,

    /// Spec describes how the user wants the resources to appear.
    #[serde(default)]
    pub spec: CustomResourceDefinitionSpec,

    /// Status indicates the actual state of the CustomResourceDefinition.
    #[serde(default)]
    pub status: CustomResourceDefinitionStatus,
}
impl_has_object_meta!(CustomResourceDefinition);

/// CustomResourceDefinitionList is a list of CustomResourceDefinition objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionList {
    /// Standard type metadata (not serialized in internal version).
    #[serde(skip)]
    pub type_meta: TypeMeta,

    /// Standard list metadata.
    #[serde(default)]
    pub metadata: ListMeta,

    /// Items list individual CustomResourceDefinition objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CustomResourceDefinition>,
}

/// CustomResourceValidation is a list of validation methods for CustomResources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceValidation {
    /// OpenAPIV3Schema is the OpenAPI v3 schema to be validated against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_api_v3_schema: Option<JSONSchemaProps>,
}

/// CustomResourceSubresources defines the status and scale subresources for CustomResources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceSubresources {
    /// Status denotes the status subresource for CustomResources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CustomResourceSubresourceStatus>,

    /// Scale denotes the scale subresource for CustomResources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<CustomResourceSubresourceScale>,
}

/// CustomResourceSubresourceStatus defines how to serve the status subresource for CustomResources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceSubresourceStatus {}

/// CustomResourceSubresourceScale defines how to serve the scale subresource for CustomResources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceSubresourceScale {
    /// SpecReplicasPath defines the JSON path for Scale.spec.replicas.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub spec_replicas_path: String,

    /// StatusReplicasPath defines the JSON path for Scale.status.replicas.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status_replicas_path: String,

    /// LabelSelectorPath defines the JSON path for Scale.status.selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector_path: Option<String>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {}

// AsRefStr / AsRef<str> implementations for enums
crate::impl_as_str_ref!(ConversionStrategyType, {
    None => conversion_strategy_type::NONE,
    Webhook => conversion_strategy_type::WEBHOOK,
});

crate::impl_as_str_ref!(CustomResourceDefinitionConditionType, {
    Established => custom_resource_definition_condition_type::ESTABLISHED,
    NamesAccepted => custom_resource_definition_condition_type::NAMES_ACCEPTED,
    NonStructuralSchema => custom_resource_definition_condition_type::NON_STRUCTURAL_SCHEMA,
    Terminating => custom_resource_definition_condition_type::TERMINATING,
    KubernetesAPIApprovalPolicyConformant => custom_resource_definition_condition_type::KUBERNETES_API_APPROVAL_POLICY_CONFORMANT,
});

// AsRefStr / AsRef<str> implementations for enums
crate::impl_as_str_ref!(ResourceScope, {
    Cluster => resource_scope::CLUSTER,
    Namespaced => resource_scope::NAMESPACED,
});

crate::impl_as_str_ref!(ConditionStatus, {
    True => condition_status::TRUE,
    False => condition_status::FALSE,
    Unknown => condition_status::UNKNOWN,
});
