//! Kubernetes Authorization API Internal Types
//!
//! This module contains type definitions from k8s.io/kubernetes/pkg/apis/authorization/types.go
//! that are used internally by the Kubernetes API.
//!
//! Source: https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/authorization/types.go

use crate::common::{FieldSelectorRequirement, LabelSelectorRequirement, ObjectMeta, TypeMeta};
use crate::impl_has_object_meta;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================================
// SubjectAccessReview
// ============================================================================

/// SubjectAccessReview checks whether or not a user or group can perform an action.
///
/// Not filling in a spec.namespace means "in all namespaces".
///
/// Corresponds to [Kubernetes SubjectAccessReview](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/authorization/types.go#L25)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SubjectAccessReview {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Spec holds information about the request being evaluated.
    pub spec: SubjectAccessReviewSpec,
    /// Status is filled in by the server and indicates whether the request is allowed or not.
    pub status: SubjectAccessReviewStatus,
}
    impl_has_object_meta!(SubjectAccessReview);

// ============================================================================
// SelfSubjectAccessReview
// ============================================================================

/// SelfSubjectAccessReview checks whether or the current user can perform an action.
///
/// Not filling in a spec.namespace means "in all namespaces". Self is a special case,
/// because users should always be able to check whether they can perform an action.
///
/// Corresponds to [Kubernetes SelfSubjectAccessReview](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/authorization/types.go#L40)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SelfSubjectAccessReview {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Spec holds information about the request being evaluated.
    pub spec: SelfSubjectAccessReviewSpec,
    /// Status is filled in by the server and indicates whether the request is allowed or not.
    pub status: SubjectAccessReviewStatus,
}
    impl_has_object_meta!(SelfSubjectAccessReview);

// ============================================================================
// LocalSubjectAccessReview
// ============================================================================

/// LocalSubjectAccessReview checks whether or not a user or group can perform an action in a given namespace.
///
/// Having a namespace scoped resource makes it much easier to grant namespace scoped policy
/// that includes permissions checking.
///
/// Corresponds to [Kubernetes LocalSubjectAccessReview](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/authorization/types.go#L54)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LocalSubjectAccessReview {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Spec holds information about the request being evaluated. spec.namespace must be equal to the namespace
    /// you made the request against. If empty, it is defaulted.
    pub spec: SubjectAccessReviewSpec,
    /// Status is filled in by the server and indicates whether the request is allowed or not.
    pub status: SubjectAccessReviewStatus,
}
    impl_has_object_meta!(LocalSubjectAccessReview);

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
/// Corresponds to [Kubernetes SelfSubjectRulesReview](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/authorization/types.go#L210)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SelfSubjectRulesReview {
    /// TypeMeta describes the type of this object.
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    pub metadata: ObjectMeta,
    /// Spec holds information about the request being evaluated.
    pub spec: SelfSubjectRulesReviewSpec,
    /// Status is filled in by the server and indicates the set of actions a user can perform.
    pub status: SubjectRulesReviewStatus,
}
    impl_has_object_meta!(SelfSubjectRulesReview);

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

    #[test]
    fn test_resource_attributes_default() {
        let attr = ResourceAttributes::default();
        assert!(attr.namespace.is_empty());
        assert!(attr.verb.is_empty());
        assert!(attr.group.is_empty());
    }

    #[test]
    fn test_non_resource_attributes_default() {
        let attr = NonResourceAttributes::default();
        assert!(attr.path.is_empty());
        assert!(attr.verb.is_empty());
    }

    #[test]
    fn test_subject_access_review_spec_default() {
        let spec = SubjectAccessReviewSpec::default();
        assert!(spec.user.is_empty());
        assert!(spec.groups.is_empty());
        assert!(spec.extra.is_empty());
        assert!(spec.uid.is_empty());
    }

    #[test]
    fn test_subject_access_review_status_default() {
        let status = SubjectAccessReviewStatus::default();
        assert!(!status.allowed);
        assert!(!status.denied);
        assert!(status.reason.is_empty());
        assert!(status.evaluation_error.is_empty());
    }

    #[test]
    fn test_resource_rule_default() {
        let rule = ResourceRule::default();
        assert!(rule.verbs.is_empty());
        assert!(rule.api_groups.is_empty());
        assert!(rule.resources.is_empty());
        assert!(rule.resource_names.is_empty());
    }

    #[test]
    fn test_non_resource_rule_default() {
        let rule = NonResourceRule::default();
        assert!(rule.verbs.is_empty());
        assert!(rule.non_resource_urls.is_empty());
    }

    #[test]
    fn test_field_selector_attributes_default() {
        let attr = FieldSelectorAttributes::default();
        assert!(attr.raw_selector.is_empty());
        assert!(attr.requirements.is_empty());
    }

    #[test]
    fn test_label_selector_attributes_default() {
        let attr = LabelSelectorAttributes::default();
        assert!(attr.raw_selector.is_empty());
        assert!(attr.requirements.is_empty());
    }

    #[test]
    fn test_extra_value_type() {
        let extra_value: ExtraValue = vec!["value1".to_string(), "value2".to_string()];
        assert_eq!(extra_value.len(), 2);
        assert_eq!(extra_value[0], "value1");
    }

    #[test]
    fn test_subject_access_review_serialize() {
        let review = SubjectAccessReview {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta::default(),
            spec: SubjectAccessReviewSpec {
                user: "test-user".to_string(),
                ..Default::default()
            },
            status: SubjectAccessReviewStatus::default(),
        };
        let json = serde_json::to_string(&review).unwrap();
        assert!(json.contains("\"user\":\"test-user\""));
    }

    #[test]
    fn test_resource_attributes_with_fields() {
        let attr = ResourceAttributes {
            namespace: "default".to_string(),
            verb: "get".to_string(),
            group: "".to_string(),
            version: "v1".to_string(),
            resource: "pods".to_string(),
            subresource: "".to_string(),
            name: "my-pod".to_string(),
            field_selector: None,
            label_selector: None,
        };
        assert_eq!(attr.namespace, "default");
        assert_eq!(attr.verb, "get");
        assert_eq!(attr.resource, "pods");
        assert_eq!(attr.name, "my-pod");
    }

    #[test]
    fn test_label_selector_attributes_with_requirements() {
        let attr = LabelSelectorAttributes {
            raw_selector: "".to_string(),
            requirements: vec![LabelSelectorRequirement {
                key: "environment".to_string(),
                operator: "In".to_string(),
                values: vec!["prod".to_string()],
            }],
        };
        assert_eq!(attr.requirements.len(), 1);
        assert_eq!(attr.requirements[0].key, "environment");
    }

    #[test]
    fn test_field_selector_attributes_with_requirements() {
        let attr = FieldSelectorAttributes {
            raw_selector: "".to_string(),
            requirements: vec![FieldSelectorRequirement {
                key: "status.phase".to_string(),
                operator: "Equals".to_string(),
                values: vec!["Running".to_string()],
            }],
        };
        assert_eq!(attr.requirements.len(), 1);
        assert_eq!(attr.requirements[0].key, "status.phase");
    }
}
