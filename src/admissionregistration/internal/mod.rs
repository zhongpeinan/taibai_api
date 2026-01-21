//! AdmissionRegistration internal API types
//!
//! This module contains the internal API types for admissionregistration.

use crate::admissionregistration::v1::{
    FailurePolicyType, MatchCondition, MatchResources, ParamKind, ParamRef, ReinvocationPolicyType,
    Variable,
};
use crate::common::{ListMeta, ObjectMeta};
use serde::{Deserialize, Serialize};

// ============================================================================
// Patch Types (Union Type)
// ============================================================================

/// PatchType specifies the type of patch operation for a mutation.
///
/// Corresponds to [Kubernetes PatchType](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1308)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum PatchType {
    /// ApplyConfiguration indicates that the mutation is using apply configuration.
    #[default]
    ApplyConfiguration,
    /// JSONPatch indicates that the object is mutated through JSON Patch.
    JSONPatch,
}

/// PatchType constants
pub mod patch_type {
    pub const APPLY_CONFIGURATION: &str = "ApplyConfiguration";
    pub const JSON_PATCH: &str = "JSONPatch";
}

/// ApplyConfiguration defines the desired configuration values of an object.
///
/// Corresponds to [Kubernetes ApplyConfiguration](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1319)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApplyConfiguration {
    /// expression will be evaluated by CEL to create an apply configuration.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
}

/// JSONPatch defines a JSON Patch.
///
/// Corresponds to [Kubernetes JSONPatch](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1363)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct JSONPatch {
    /// expression will be evaluated by CEL to create a JSON patch.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
}

/// Mutation specifies the operation that performs a Mutation.
/// This is a union type where patchType determines which variant is active.
///
/// Corresponds to [Kubernetes Mutation](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1288)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "patchType", rename_all = "PascalCase")]
pub enum Mutation {
    /// ApplyConfiguration defines the desired configuration values of an object.
    ApplyConfiguration(ApplyConfiguration),
    /// JSONPatch defines a JSON Patch.
    JSONPatch(JSONPatch),
}

// ============================================================================
// MutatingAdmissionPolicy Types
// ============================================================================

/// MutatingAdmissionPolicySpec is the specification of the desired behavior of the admission policy.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicySpec](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1200)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicySpec {
    /// paramKind specifies the kind of resources used to parameterize this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_kind: Option<ParamKind>,

    /// matchConstraints specifies what resources this policy is designed to validate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_constraints: Option<MatchResources>,

    /// variables contain definitions of variables that can be used in composition of other expressions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<Variable>,

    /// mutations contain operations to perform on matching objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutations: Vec<Mutation>,

    /// failurePolicy defines how to handle failures for the admission policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<FailurePolicyType>,

    /// matchConditions is a list of conditions that must be met for a request to be validated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub match_conditions: Vec<MatchCondition>,

    /// reinvocationPolicy indicates whether mutations may be called multiple times.
    #[serde(default)]
    pub reinvocation_policy: ReinvocationPolicyType,
}

/// MutatingAdmissionPolicy describes an admission policy that may mutate an object.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicy](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1176)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicy {
    /// Standard object's metadata.
    pub metadata: ObjectMeta,

    /// Specification of the desired behavior of the MutatingAdmissionPolicy.
    #[serde(default)]
    pub spec: MutatingAdmissionPolicySpec,
}

/// MutatingAdmissionPolicyList is a list of MutatingAdmissionPolicy.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicyList](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1189)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of MutatingAdmissionPolicy.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MutatingAdmissionPolicy>,
}

// ============================================================================
// MutatingAdmissionPolicyBinding Types
// ============================================================================

/// MutatingAdmissionPolicyBindingSpec is the specification of the MutatingAdmissionPolicyBinding.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicyBindingSpec](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1472)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBindingSpec {
    /// policyName references a MutatingAdmissionPolicy name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub policy_name: String,

    /// paramRef specifies the parameter resource used to configure the admission control policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_ref: Option<ParamRef>,

    /// matchResources limits what resources match this binding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_resources: Option<MatchResources>,
}

/// MutatingAdmissionPolicyBinding binds the MutatingAdmissionPolicy with parametrized resources.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicyBinding](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1438)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBinding {
    /// Standard object's metadata.
    pub metadata: ObjectMeta,

    /// Specification of the desired behavior of the MutatingAdmissionPolicyBinding.
    #[serde(default)]
    pub spec: MutatingAdmissionPolicyBindingSpec,
}

/// MutatingAdmissionPolicyBindingList is a list of MutatingAdmissionPolicyBinding.
///
/// Corresponds to [Kubernetes MutatingAdmissionPolicyBindingList](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/admissionregistration/types.go#L1461)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBindingList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// List of PolicyBinding.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MutatingAdmissionPolicyBinding>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_type_default() {
        let patch_type = PatchType::default();
        assert_eq!(patch_type, PatchType::ApplyConfiguration);
    }

    #[test]
    fn test_apply_configuration_default() {
        let config = ApplyConfiguration::default();
        assert!(config.expression.is_empty());
    }

    #[test]
    fn test_json_patch_default() {
        let patch = JSONPatch::default();
        assert!(patch.expression.is_empty());
    }

    #[test]
    fn test_mutation_serialize_apply_configuration() {
        let mutation = Mutation::ApplyConfiguration(ApplyConfiguration {
            expression: "Object.spec{replicas: 3}".to_string(),
        });
        let json = serde_json::to_string(&mutation).unwrap();
        assert!(json.contains("ApplyConfiguration"));
        assert!(json.contains("expression"));
    }

    #[test]
    fn test_mutation_serialize_json_patch() {
        let mutation = Mutation::JSONPatch(JSONPatch {
            expression: "[{op: 'replace', path: '/spec/replicas', value: 3}]".to_string(),
        });
        let json = serde_json::to_string(&mutation).unwrap();
        assert!(json.contains("JSONPatch"));
        assert!(json.contains("expression"));
    }

    #[test]
    fn test_mutation_deserialize_apply_configuration() {
        let json = r#"{"patchType":"ApplyConfiguration","expression":"Object.spec{replicas: 3}"}"#;
        let mutation: Mutation = serde_json::from_str(json).unwrap();
        match mutation {
            Mutation::ApplyConfiguration(config) => {
                assert_eq!(config.expression, "Object.spec{replicas: 3}");
            }
            _ => panic!("Expected ApplyConfiguration variant"),
        }
    }

    #[test]
    fn test_mutation_deserialize_json_patch() {
        let json = r#"{"patchType":"JSONPatch","expression":"[{op: 'replace', path: '/spec/replicas', value: 3}]"}"#;
        let mutation: Mutation = serde_json::from_str(json).unwrap();
        match mutation {
            Mutation::JSONPatch(patch) => {
                assert_eq!(
                    patch.expression,
                    "[{op: 'replace', path: '/spec/replicas', value: 3}]"
                );
            }
            _ => panic!("Expected JSONPatch variant"),
        }
    }

    #[test]
    fn test_mutation_round_trip_apply_configuration() {
        let original = Mutation::ApplyConfiguration(ApplyConfiguration {
            expression: "Object.spec{replicas: 3}".to_string(),
        });
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Mutation = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_mutation_round_trip_json_patch() {
        let original = Mutation::JSONPatch(JSONPatch {
            expression: "[{op: 'replace', path: '/spec/replicas', value: 3}]".to_string(),
        });
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Mutation = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_constants() {
        assert_eq!(patch_type::APPLY_CONFIGURATION, "ApplyConfiguration");
        assert_eq!(patch_type::JSON_PATCH, "JSONPatch");
    }

    #[test]
    fn test_mutating_admission_policy_default() {
        let policy = MutatingAdmissionPolicy {
            metadata: None,
            spec: Default::default(),
        };
        assert!(policy.metadata.is_none());
        assert!(policy.spec.mutations.is_empty());
    }

    #[test]
    fn test_mutating_admission_policy_binding_default() {
        let binding = MutatingAdmissionPolicyBinding {
            metadata: None,
            spec: Default::default(),
        };
        assert!(binding.metadata.is_none());
        assert!(binding.spec.policy_name.is_empty());
    }

    #[test]
    fn test_mutating_admission_policy_spec_default() {
        let spec = MutatingAdmissionPolicySpec::default();
        assert!(spec.param_kind.is_none());
        assert!(spec.mutations.is_empty());
        assert_eq!(spec.reinvocation_policy, ReinvocationPolicyType::Never);
    }

    #[test]
    fn test_mutating_admission_policy_binding_spec_default() {
        let spec = MutatingAdmissionPolicyBindingSpec::default();
        assert!(spec.param_ref.is_none());
        assert!(spec.match_resources.is_none());
        assert!(spec.policy_name.is_empty());
    }

    #[test]
    fn test_mutating_admission_policy_spec_with_mutations() {
        let spec = MutatingAdmissionPolicySpec {
            mutations: vec![Mutation::ApplyConfiguration(ApplyConfiguration {
                expression: "Object.spec{replicas: 3}".to_string(),
            })],
            ..Default::default()
        };
        assert_eq!(spec.mutations.len(), 1);
    }

    #[test]
    fn test_mutating_admission_policy_serialize() {
        let policy = MutatingAdmissionPolicy {
            metadata: None,
            spec: MutatingAdmissionPolicySpec {
                mutations: vec![Mutation::ApplyConfiguration(ApplyConfiguration {
                    expression: "Object.spec{replicas: 3}".to_string(),
                })],
                ..Default::default()
            },
        };
        let json = serde_json::to_string(&policy).unwrap();
        assert!(json.contains("spec"));
        assert!(json.contains("mutations"));
    }

    #[test]
    fn test_mutating_admission_policy_binding_serialize() {
        let binding = MutatingAdmissionPolicyBinding {
            metadata: None,
            spec: MutatingAdmissionPolicyBindingSpec {
                policy_name: "test-policy".to_string(),
                ..Default::default()
            },
        };
        let json = serde_json::to_string(&binding).unwrap();
        assert!(json.contains("policyName"));
        assert!(json.contains("test-policy"));
    }
}
