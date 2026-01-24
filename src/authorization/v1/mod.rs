//! Kubernetes Authorization API v1 types
//!
//! This module contains the authorization v1 API types.
//!
//! Source: https://github.com/kubernetes/api/blob/master/authorization/v1/types.go

use crate::authorization::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasTypeMeta, ResourceSchema, ToInternal, TypeMeta, VersionedObject,
};
use crate::common::{FieldSelectorRequirement, LabelSelectorRequirement, ObjectMeta};
use crate::impl_unimplemented_prost_message;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================================
// SubjectAccessReview
// ============================================================================

/// SubjectAccessReview checks whether or not a user or group can perform an action.
///
/// Corresponds to [Kubernetes SubjectAccessReview](https://github.com/kubernetes/api/blob/master/authorization/v1/types.go#L31)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubjectAccessReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec holds information about the request being evaluated.
    pub spec: SubjectAccessReviewSpec,
    /// Status is filled in by the server and indicates whether the request is allowed or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SubjectAccessReviewStatus>,
}

impl SubjectAccessReview {
    /// Creates a new SubjectAccessReview with the given spec.
    pub fn new(spec: SubjectAccessReviewSpec) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec,
            status: None,
        }
    }
}

// ============================================================================
// SelfSubjectAccessReview
// ============================================================================

/// SelfSubjectAccessReview checks whether or the current user can perform an action.
///
/// Not filling in a spec.namespace means "in all namespaces". Self is a special case,
/// because users should always be able to check whether they can perform an action.
///
/// Corresponds to [Kubernetes SelfSubjectAccessReview](https://github.com/kubernetes/api/blob/master/authorization/v1/types.go#L53)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectAccessReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec holds information about the request being evaluated. user and groups must be empty.
    pub spec: SelfSubjectAccessReviewSpec,
    /// Status is filled in by the server and indicates whether the request is allowed or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SubjectAccessReviewStatus>,
}

impl SelfSubjectAccessReview {
    /// Creates a new SelfSubjectAccessReview with the given spec.
    pub fn new(spec: SelfSubjectAccessReviewSpec) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec,
            status: None,
        }
    }
}

// ============================================================================
// LocalSubjectAccessReview
// ============================================================================

/// LocalSubjectAccessReview checks whether or not a user or group can perform an action in a given namespace.
///
/// Having a namespace scoped resource makes it much easier to grant namespace scoped policy
/// that includes permissions checking.
///
/// Corresponds to [Kubernetes LocalSubjectAccessReview](https://github.com/kubernetes/api/blob/master/authorization/v1/types.go#L76)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocalSubjectAccessReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec holds information about the request being evaluated. spec.namespace must be equal to the namespace
    /// you made the request against. If empty, it is defaulted.
    pub spec: SubjectAccessReviewSpec,
    /// Status is filled in by the server and indicates whether the request is allowed or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SubjectAccessReviewStatus>,
}

impl LocalSubjectAccessReview {
    /// Creates a new LocalSubjectAccessReview with the given spec.
    pub fn new(spec: SubjectAccessReviewSpec) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec,
            status: None,
        }
    }
}

// ============================================================================
// ResourceAttributes
// ============================================================================

/// ResourceAttributes includes the authorization attributes available for resource requests
/// to the Authorizer interface.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceAttributes {
    /// Namespace is the namespace of the action being requested.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy. "*" means all.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub verb: String,
    /// Group is the API Group of the Resource. "*" means all.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,
    /// Version is the API Version of the Resource. "*" means all.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// Resource is one of the existing resource types. "*" means all.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource: String,
    /// Subresource is one of the existing resource types. "" means none.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subresource: String,
    /// Name is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// fieldSelector describes the limitation on access based on field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_selector: Option<FieldSelectorAttributes>,
    /// labelSelector describes the limitation on access based on labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<LabelSelectorAttributes>,
}

// ============================================================================
// LabelSelectorAttributes
// ============================================================================

/// LabelSelectorAttributes indicates a label limited access.
///
/// Webhook authors are encouraged to:
/// * ensure rawSelector and requirements are not both set
/// * consider the requirements field if set
/// * not try to parse or consider the rawSelector field if set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelectorAttributes {
    /// rawSelector is the serialization of a field selector that would be included in a query parameter.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub raw_selector: String,
    /// requirements is the parsed interpretation of a label selector.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirements: Vec<LabelSelectorRequirement>,
}

// ============================================================================
// FieldSelectorAttributes
// ============================================================================

/// FieldSelectorAttributes indicates a field limited access.
///
/// Webhook authors are encouraged to:
/// * ensure rawSelector and requirements are not both set
/// * consider the requirements field if set
/// * not try to parse or consider the rawSelector field if set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FieldSelectorAttributes {
    /// rawSelector is the serialization of a field selector that would be included in a query parameter.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub raw_selector: String,
    /// requirements is the parsed interpretation of a field selector.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirements: Vec<FieldSelectorRequirement>,
}

// ============================================================================
// NonResourceAttributes
// ============================================================================

/// NonResourceAttributes includes the authorization attributes available for non-resource
/// requests to the Authorizer interface.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NonResourceAttributes {
    /// Path is the URL path of the request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// Verb is the standard HTTP verb.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub verb: String,
}

// ============================================================================
// SubjectAccessReviewSpec
// ============================================================================

/// SubjectAccessReviewSpec is a description of the access request.
///
/// Exactly one of ResourceAttributes and NonResourceAttributes must be set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubjectAccessReviewSpec {
    /// ResourceAttributes describes information for a resource access request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_attributes: Option<ResourceAttributes>,
    /// NonResourceAttributes describes information for a non-resource access request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_resource_attributes: Option<NonResourceAttributes>,
    /// User is the user you're testing for.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user: String,
    /// Groups is the groups you're testing for.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    /// Extra corresponds to the user.Info.GetExtra() method from the authenticator.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, ExtraValue>,
    /// UID information about the requesting user.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub uid: String,
}

/// ExtraValue masks the value so protobuf can generate.
///
/// This is a type alias for Vec<String> for compatibility with Kubernetes API.
pub type ExtraValue = Vec<String>;

// ============================================================================
// SelfSubjectAccessReviewSpec
// ============================================================================

/// SelfSubjectAccessReviewSpec is a description of the access request.
///
/// Exactly one of ResourceAttributes and NonResourceAttributes must be set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectAccessReviewSpec {
    /// ResourceAttributes describes information for a resource access request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_attributes: Option<ResourceAttributes>,
    /// NonResourceAttributes describes information for a non-resource access request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_resource_attributes: Option<NonResourceAttributes>,
}

// ============================================================================
// SubjectAccessReviewStatus
// ============================================================================

/// SubjectAccessReviewStatus represents the current state of a SubjectAccessReview.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubjectAccessReviewStatus {
    /// Allowed is required. True if the action would be allowed, false otherwise.
    #[serde(default)]
    pub allowed: bool,
    /// Denied is optional. True if the action would be denied, otherwise false.
    #[serde(default)]
    pub denied: bool,
    /// Reason is optional. It indicates why a request was allowed or denied.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    /// EvaluationError is an indication that some error occurred during the authorization check.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub evaluation_error: String,
}

// ============================================================================
// SelfSubjectRulesReview
// ============================================================================

/// SelfSubjectRulesReview enumerates the set of actions the current user can perform within a namespace.
///
/// The returned list of actions may be incomplete depending on the server's authorization mode,
/// and any errors experienced during the evaluation.
///
/// Corresponds to [Kubernetes SelfSubjectRulesReview](https://github.com/kubernetes/api/blob/master/authorization/v1/types.go#L266)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectRulesReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// Spec holds information about the request being evaluated.
    pub spec: SelfSubjectRulesReviewSpec,
    /// Status is filled in by the server and indicates the set of actions a user can perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SubjectRulesReviewStatus>,
}

impl SelfSubjectRulesReview {
    /// Creates a new SelfSubjectRulesReview with the given spec.
    pub fn new(spec: SelfSubjectRulesReviewSpec) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec,
            status: None,
        }
    }
}

// ============================================================================
// SelfSubjectRulesReviewSpec
// ============================================================================

/// SelfSubjectRulesReviewSpec defines the specification for SelfSubjectRulesReview.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectRulesReviewSpec {
    /// Namespace to evaluate rules for. Required.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}

// ============================================================================
// SubjectRulesReviewStatus
// ============================================================================

/// SubjectRulesReviewStatus contains the result of a rules check.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubjectRulesReviewStatus {
    /// ResourceRules is the list of actions the subject is allowed to perform on resources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_rules: Vec<ResourceRule>,
    /// NonResourceRules is the list of actions the subject is allowed to perform on non-resources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub non_resource_rules: Vec<NonResourceRule>,
    /// Incomplete is true when the rules returned by this call are incomplete.
    #[serde(default)]
    pub incomplete: bool,
    /// EvaluationError can appear in combination with Rules.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub evaluation_error: String,
}

// ============================================================================
// ResourceRule
// ============================================================================

/// ResourceRule is the list of actions the subject is allowed to perform on resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRule {
    /// Verb is a list of kubernetes resource API verbs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
    /// APIGroups is the name of the APIGroup that contains the resources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_groups: Vec<String>,
    /// Resources is a list of resources this rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    /// ResourceNames is an optional white list of names that the rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_names: Vec<String>,
}

// ============================================================================
// NonResourceRule
// ============================================================================

/// NonResourceRule holds information that describes a rule for the non-resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NonResourceRule {
    /// Verb is a list of kubernetes non-resource API verbs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verbs: Vec<String>,
    /// NonResourceURLs is a set of partial urls that a user should have access to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub non_resource_urls: Vec<String>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, TypeMeta};
    use std::collections::BTreeMap;

    #[test]
    fn test_subject_access_review_round_trip() {
        let mut extra = BTreeMap::new();
        extra.insert("scopes".to_string(), vec!["a".to_string(), "b".to_string()]);

        let internal_obj = internal::SubjectAccessReview {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("review".to_string()),
                namespace: Some("ns".to_string()),
                ..ObjectMeta::default()
            },
            spec: internal::SubjectAccessReviewSpec {
                resource_attributes: Some(internal::ResourceAttributes {
                    namespace: "ns".to_string(),
                    verb: "get".to_string(),
                    group: "apps".to_string(),
                    version: "v1".to_string(),
                    resource: "deployments".to_string(),
                    subresource: String::new(),
                    name: "demo".to_string(),
                    field_selector: None,
                    label_selector: None,
                }),
                non_resource_attributes: None,
                user: "alice".to_string(),
                groups: vec!["devs".to_string()],
                extra,
                uid: "uid-1".to_string(),
            },
            status: internal::SubjectAccessReviewStatus {
                allowed: true,
                denied: false,
                reason: "ok".to_string(),
                evaluation_error: String::new(),
            },
        };

        let v1_obj = SubjectAccessReview::from_internal(internal_obj.clone());
        assert_eq!(v1_obj.type_meta.api_version, "authorization.k8s.io/v1");
        assert_eq!(v1_obj.type_meta.kind, "SubjectAccessReview");

        let round_trip = v1_obj.to_internal();
        assert_eq!(round_trip, internal_obj);
    }

    #[test]
    fn test_to_internal_drops_type_meta() {
        let v1_obj = SubjectAccessReview {
            type_meta: TypeMeta {
                api_version: "authorization.k8s.io/v1".to_string(),
                kind: "SubjectAccessReview".to_string(),
            },
            metadata: None,
            spec: SubjectAccessReviewSpec::default(),
            status: None,
        };

        let internal_obj = v1_obj.to_internal();
        assert_eq!(internal_obj.type_meta, TypeMeta::default());
        assert_eq!(internal_obj.metadata, ObjectMeta::default());
        assert_eq!(
            internal_obj.status,
            internal::SubjectAccessReviewStatus::default()
        );
    }
}

// ============================================================================
// Trait Implementations for Authorization Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for SubjectAccessReview {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "SubjectAccessReview"
    }
    fn resource(_: &Self::Meta) -> &str {
        "subjectaccessreviews"
    }

    fn group_static() -> &'static str {
        "authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "SubjectAccessReview"
    }
    fn resource_static() -> &'static str {
        "subjectaccessreviews"
    }
}

impl ResourceSchema for SelfSubjectAccessReview {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "SelfSubjectAccessReview"
    }
    fn resource(_: &Self::Meta) -> &str {
        "selfsubjectaccessreviews"
    }

    fn group_static() -> &'static str {
        "authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "SelfSubjectAccessReview"
    }
    fn resource_static() -> &'static str {
        "selfsubjectaccessreviews"
    }
}

impl ResourceSchema for LocalSubjectAccessReview {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "LocalSubjectAccessReview"
    }
    fn resource(_: &Self::Meta) -> &str {
        "localsubjectaccessreviews"
    }

    fn group_static() -> &'static str {
        "authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "LocalSubjectAccessReview"
    }
    fn resource_static() -> &'static str {
        "localsubjectaccessreviews"
    }
}

impl ResourceSchema for SelfSubjectRulesReview {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "SelfSubjectRulesReview"
    }
    fn resource(_: &Self::Meta) -> &str {
        "selfsubjectrulesreviews"
    }

    fn group_static() -> &'static str {
        "authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "SelfSubjectRulesReview"
    }
    fn resource_static() -> &'static str {
        "selfsubjectrulesreviews"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for SubjectAccessReview {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for SelfSubjectAccessReview {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for LocalSubjectAccessReview {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for SelfSubjectRulesReview {
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

impl VersionedObject for SubjectAccessReview {
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

impl VersionedObject for SelfSubjectAccessReview {
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

impl VersionedObject for LocalSubjectAccessReview {
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

impl VersionedObject for SelfSubjectRulesReview {
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

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for SubjectAccessReview {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "SubjectAccessReview".to_string();
        }
    }
}

impl ApplyDefault for SelfSubjectAccessReview {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "SelfSubjectAccessReview".to_string();
        }
    }
}

impl ApplyDefault for LocalSubjectAccessReview {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "LocalSubjectAccessReview".to_string();
        }
    }
}

impl ApplyDefault for SelfSubjectRulesReview {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "SelfSubjectRulesReview".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Version Conversion Implementations
// ----------------------------------------------------------------------------

fn to_internal_resource_attributes(
    resource_attributes: ResourceAttributes,
) -> internal::ResourceAttributes {
    internal::ResourceAttributes {
        namespace: resource_attributes.namespace,
        verb: resource_attributes.verb,
        group: resource_attributes.group,
        version: resource_attributes.version,
        resource: resource_attributes.resource,
        subresource: resource_attributes.subresource,
        name: resource_attributes.name,
        field_selector: resource_attributes
            .field_selector
            .map(to_internal_field_selector_attributes),
        label_selector: resource_attributes
            .label_selector
            .map(to_internal_label_selector_attributes),
    }
}

fn from_internal_resource_attributes(
    resource_attributes: internal::ResourceAttributes,
) -> ResourceAttributes {
    ResourceAttributes {
        namespace: resource_attributes.namespace,
        verb: resource_attributes.verb,
        group: resource_attributes.group,
        version: resource_attributes.version,
        resource: resource_attributes.resource,
        subresource: resource_attributes.subresource,
        name: resource_attributes.name,
        field_selector: resource_attributes
            .field_selector
            .map(from_internal_field_selector_attributes),
        label_selector: resource_attributes
            .label_selector
            .map(from_internal_label_selector_attributes),
    }
}

fn to_internal_label_selector_attributes(
    selector: LabelSelectorAttributes,
) -> internal::LabelSelectorAttributes {
    internal::LabelSelectorAttributes {
        raw_selector: selector.raw_selector,
        requirements: selector.requirements,
    }
}

fn from_internal_label_selector_attributes(
    selector: internal::LabelSelectorAttributes,
) -> LabelSelectorAttributes {
    LabelSelectorAttributes {
        raw_selector: selector.raw_selector,
        requirements: selector.requirements,
    }
}

fn to_internal_field_selector_attributes(
    selector: FieldSelectorAttributes,
) -> internal::FieldSelectorAttributes {
    internal::FieldSelectorAttributes {
        raw_selector: selector.raw_selector,
        requirements: selector.requirements,
    }
}

fn from_internal_field_selector_attributes(
    selector: internal::FieldSelectorAttributes,
) -> FieldSelectorAttributes {
    FieldSelectorAttributes {
        raw_selector: selector.raw_selector,
        requirements: selector.requirements,
    }
}

fn to_internal_non_resource_attributes(
    attributes: NonResourceAttributes,
) -> internal::NonResourceAttributes {
    internal::NonResourceAttributes {
        path: attributes.path,
        verb: attributes.verb,
    }
}

fn from_internal_non_resource_attributes(
    attributes: internal::NonResourceAttributes,
) -> NonResourceAttributes {
    NonResourceAttributes {
        path: attributes.path,
        verb: attributes.verb,
    }
}

fn to_internal_subject_access_review_spec(
    spec: SubjectAccessReviewSpec,
) -> internal::SubjectAccessReviewSpec {
    internal::SubjectAccessReviewSpec {
        resource_attributes: spec
            .resource_attributes
            .map(to_internal_resource_attributes),
        non_resource_attributes: spec
            .non_resource_attributes
            .map(to_internal_non_resource_attributes),
        user: spec.user,
        groups: spec.groups,
        extra: spec.extra,
        uid: spec.uid,
    }
}

fn from_internal_subject_access_review_spec(
    spec: internal::SubjectAccessReviewSpec,
) -> SubjectAccessReviewSpec {
    SubjectAccessReviewSpec {
        resource_attributes: spec
            .resource_attributes
            .map(from_internal_resource_attributes),
        non_resource_attributes: spec
            .non_resource_attributes
            .map(from_internal_non_resource_attributes),
        user: spec.user,
        groups: spec.groups,
        extra: spec.extra,
        uid: spec.uid,
    }
}

fn to_internal_self_subject_access_review_spec(
    spec: SelfSubjectAccessReviewSpec,
) -> internal::SelfSubjectAccessReviewSpec {
    internal::SelfSubjectAccessReviewSpec {
        resource_attributes: spec
            .resource_attributes
            .map(to_internal_resource_attributes),
        non_resource_attributes: spec
            .non_resource_attributes
            .map(to_internal_non_resource_attributes),
    }
}

fn from_internal_self_subject_access_review_spec(
    spec: internal::SelfSubjectAccessReviewSpec,
) -> SelfSubjectAccessReviewSpec {
    SelfSubjectAccessReviewSpec {
        resource_attributes: spec
            .resource_attributes
            .map(from_internal_resource_attributes),
        non_resource_attributes: spec
            .non_resource_attributes
            .map(from_internal_non_resource_attributes),
    }
}

fn to_internal_subject_access_review_status(
    status: SubjectAccessReviewStatus,
) -> internal::SubjectAccessReviewStatus {
    internal::SubjectAccessReviewStatus {
        allowed: status.allowed,
        denied: status.denied,
        reason: status.reason,
        evaluation_error: status.evaluation_error,
    }
}

fn from_internal_subject_access_review_status(
    status: internal::SubjectAccessReviewStatus,
) -> SubjectAccessReviewStatus {
    SubjectAccessReviewStatus {
        allowed: status.allowed,
        denied: status.denied,
        reason: status.reason,
        evaluation_error: status.evaluation_error,
    }
}

fn to_internal_self_subject_rules_review_spec(
    spec: SelfSubjectRulesReviewSpec,
) -> internal::SelfSubjectRulesReviewSpec {
    internal::SelfSubjectRulesReviewSpec {
        namespace: spec.namespace,
    }
}

fn from_internal_self_subject_rules_review_spec(
    spec: internal::SelfSubjectRulesReviewSpec,
) -> SelfSubjectRulesReviewSpec {
    SelfSubjectRulesReviewSpec {
        namespace: spec.namespace,
    }
}

fn to_internal_resource_rule(rule: ResourceRule) -> internal::ResourceRule {
    internal::ResourceRule {
        verbs: rule.verbs,
        api_groups: rule.api_groups,
        resources: rule.resources,
        resource_names: rule.resource_names,
    }
}

fn from_internal_resource_rule(rule: internal::ResourceRule) -> ResourceRule {
    ResourceRule {
        verbs: rule.verbs,
        api_groups: rule.api_groups,
        resources: rule.resources,
        resource_names: rule.resource_names,
    }
}

fn to_internal_non_resource_rule(rule: NonResourceRule) -> internal::NonResourceRule {
    internal::NonResourceRule {
        verbs: rule.verbs,
        non_resource_urls: rule.non_resource_urls,
    }
}

fn from_internal_non_resource_rule(rule: internal::NonResourceRule) -> NonResourceRule {
    NonResourceRule {
        verbs: rule.verbs,
        non_resource_urls: rule.non_resource_urls,
    }
}

fn to_internal_subject_rules_review_status(
    status: SubjectRulesReviewStatus,
) -> internal::SubjectRulesReviewStatus {
    internal::SubjectRulesReviewStatus {
        resource_rules: status
            .resource_rules
            .into_iter()
            .map(to_internal_resource_rule)
            .collect(),
        non_resource_rules: status
            .non_resource_rules
            .into_iter()
            .map(to_internal_non_resource_rule)
            .collect(),
        incomplete: status.incomplete,
        evaluation_error: status.evaluation_error,
    }
}

fn from_internal_subject_rules_review_status(
    status: internal::SubjectRulesReviewStatus,
) -> SubjectRulesReviewStatus {
    SubjectRulesReviewStatus {
        resource_rules: status
            .resource_rules
            .into_iter()
            .map(from_internal_resource_rule)
            .collect(),
        non_resource_rules: status
            .non_resource_rules
            .into_iter()
            .map(from_internal_non_resource_rule)
            .collect(),
        incomplete: status.incomplete,
        evaluation_error: status.evaluation_error,
    }
}

impl ToInternal<internal::SubjectAccessReview> for SubjectAccessReview {
    fn to_internal(self) -> internal::SubjectAccessReview {
        internal::SubjectAccessReview {
            type_meta: TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            spec: to_internal_subject_access_review_spec(self.spec),
            status: to_internal_subject_access_review_status(self.status.unwrap_or_default()),
        }
    }
}

impl FromInternal<internal::SubjectAccessReview> for SubjectAccessReview {
    fn from_internal(internal: internal::SubjectAccessReview) -> Self {
        let mut out = SubjectAccessReview {
            type_meta: TypeMeta::default(),
            metadata: Some(internal.metadata),
            spec: from_internal_subject_access_review_spec(internal.spec),
            status: Some(from_internal_subject_access_review_status(internal.status)),
        };
        out.apply_default();
        out
    }
}

impl ToInternal<internal::SelfSubjectAccessReview> for SelfSubjectAccessReview {
    fn to_internal(self) -> internal::SelfSubjectAccessReview {
        internal::SelfSubjectAccessReview {
            type_meta: TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            spec: to_internal_self_subject_access_review_spec(self.spec),
            status: to_internal_subject_access_review_status(self.status.unwrap_or_default()),
        }
    }
}

impl FromInternal<internal::SelfSubjectAccessReview> for SelfSubjectAccessReview {
    fn from_internal(internal: internal::SelfSubjectAccessReview) -> Self {
        let mut out = SelfSubjectAccessReview {
            type_meta: TypeMeta::default(),
            metadata: Some(internal.metadata),
            spec: from_internal_self_subject_access_review_spec(internal.spec),
            status: Some(from_internal_subject_access_review_status(internal.status)),
        };
        out.apply_default();
        out
    }
}

impl ToInternal<internal::LocalSubjectAccessReview> for LocalSubjectAccessReview {
    fn to_internal(self) -> internal::LocalSubjectAccessReview {
        internal::LocalSubjectAccessReview {
            type_meta: TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            spec: to_internal_subject_access_review_spec(self.spec),
            status: to_internal_subject_access_review_status(self.status.unwrap_or_default()),
        }
    }
}

impl FromInternal<internal::LocalSubjectAccessReview> for LocalSubjectAccessReview {
    fn from_internal(internal: internal::LocalSubjectAccessReview) -> Self {
        let mut out = LocalSubjectAccessReview {
            type_meta: TypeMeta::default(),
            metadata: Some(internal.metadata),
            spec: from_internal_subject_access_review_spec(internal.spec),
            status: Some(from_internal_subject_access_review_status(internal.status)),
        };
        out.apply_default();
        out
    }
}

impl ToInternal<internal::SelfSubjectRulesReview> for SelfSubjectRulesReview {
    fn to_internal(self) -> internal::SelfSubjectRulesReview {
        internal::SelfSubjectRulesReview {
            type_meta: TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            spec: to_internal_self_subject_rules_review_spec(self.spec),
            status: to_internal_subject_rules_review_status(self.status.unwrap_or_default()),
        }
    }
}

impl FromInternal<internal::SelfSubjectRulesReview> for SelfSubjectRulesReview {
    fn from_internal(internal: internal::SelfSubjectRulesReview) -> Self {
        let mut out = SelfSubjectRulesReview {
            type_meta: TypeMeta::default(),
            metadata: Some(internal.metadata),
            spec: from_internal_self_subject_rules_review_spec(internal.spec),
            status: Some(from_internal_subject_rules_review_status(internal.status)),
        };
        out.apply_default();
        out
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(SubjectAccessReview);
impl_unimplemented_prost_message!(SelfSubjectAccessReview);
impl_unimplemented_prost_message!(LocalSubjectAccessReview);
impl_unimplemented_prost_message!(SelfSubjectRulesReview);
