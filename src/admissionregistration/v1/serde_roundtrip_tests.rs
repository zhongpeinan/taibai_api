use super::{
    AuditAnnotation, FailurePolicyType, MatchCondition, MatchPolicyType, MatchResources,
    MutatingWebhook, MutatingWebhookConfiguration, MutatingWebhookConfigurationList,
    NamedRuleWithOperations, OperationType, ParamKind, ParamRef, ParameterNotFoundActionType,
    ReinvocationPolicyType, RuleWithOperations, ScopeType, ServiceReference, SideEffectClass,
    TypeChecking, ValidatingAdmissionPolicy, ValidatingAdmissionPolicyBinding,
    ValidatingAdmissionPolicyBindingList, ValidatingAdmissionPolicyList,
    ValidatingAdmissionPolicySpec, ValidatingAdmissionPolicyStatus, ValidatingWebhook,
    ValidatingWebhookConfiguration, ValidatingWebhookConfigurationList, Validation,
    ValidationAction, Variable, WebhookClientConfig,
};
use crate::common::test_utils::assert_serde_roundtrip;
use crate::common::{LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::ByteString;
use std::collections::BTreeMap;

fn validating_webhook_configuration_basic() -> ValidatingWebhookConfiguration {
    ValidatingWebhookConfiguration {
        type_meta: TypeMeta {
            api_version: "admissionregistration.k8s.io/v1".to_string(),
            kind: "ValidatingWebhookConfiguration".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("validate.example.io".to_string()),
            ..Default::default()
        }),
        webhooks: vec![ValidatingWebhook {
            name: "validate.example.io".to_string(),
            client_config: WebhookClientConfig {
                url: Some("https://example.com/validate".to_string()),
                service: None,
                ca_bundle: ByteString(vec![1, 2, 3]),
            },
            rules: vec![RuleWithOperations {
                operations: vec![OperationType::Create, OperationType::Update],
                api_groups: vec!["apps".to_string()],
                api_versions: vec!["v1".to_string()],
                resources: vec!["deployments".to_string()],
                scope: Some(ScopeType::Namespaced),
            }],
            failure_policy: Some(FailurePolicyType::Fail),
            match_policy: Some(MatchPolicyType::Equivalent),
            namespace_selector: Some(LabelSelector {
                match_labels: BTreeMap::from([("env".to_string(), "prod".to_string())]),
                ..Default::default()
            }),
            object_selector: None,
            side_effects: Some(SideEffectClass::None),
            timeout_seconds: Some(5),
            admission_review_versions: vec!["v1".to_string()],
            match_conditions: vec![MatchCondition {
                name: "has-label".to_string(),
                expression: "'env' in object.metadata.labels".to_string(),
            }],
        }],
    }
}

fn validating_webhook_configuration_list_basic() -> ValidatingWebhookConfigurationList {
    ValidatingWebhookConfigurationList {
        type_meta: TypeMeta {
            api_version: "admissionregistration.k8s.io/v1".to_string(),
            kind: "ValidatingWebhookConfigurationList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![validating_webhook_configuration_basic()],
    }
}

fn mutating_webhook_configuration_basic() -> MutatingWebhookConfiguration {
    MutatingWebhookConfiguration {
        type_meta: TypeMeta {
            api_version: "admissionregistration.k8s.io/v1".to_string(),
            kind: "MutatingWebhookConfiguration".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("mutate.example.io".to_string()),
            ..Default::default()
        }),
        webhooks: vec![MutatingWebhook {
            name: "mutate.example.io".to_string(),
            client_config: WebhookClientConfig {
                url: None,
                service: Some(ServiceReference {
                    namespace: "default".to_string(),
                    name: "webhook-svc".to_string(),
                    path: Some("/mutate".to_string()),
                    port: Some(8443),
                }),
                ca_bundle: ByteString(vec![4, 5, 6]),
            },
            rules: vec![RuleWithOperations {
                operations: vec![OperationType::Create],
                api_groups: vec!["".to_string()],
                api_versions: vec!["v1".to_string()],
                resources: vec!["pods".to_string()],
                scope: Some(ScopeType::Namespaced),
            }],
            failure_policy: Some(FailurePolicyType::Fail),
            match_policy: Some(MatchPolicyType::Exact),
            namespace_selector: None,
            object_selector: None,
            side_effects: Some(SideEffectClass::NoneOnDryRun),
            timeout_seconds: Some(10),
            admission_review_versions: vec!["v1".to_string()],
            reinvocation_policy: Some(ReinvocationPolicyType::IfNeeded),
            match_conditions: vec![],
        }],
    }
}

fn mutating_webhook_configuration_list_basic() -> MutatingWebhookConfigurationList {
    MutatingWebhookConfigurationList {
        type_meta: TypeMeta {
            api_version: "admissionregistration.k8s.io/v1".to_string(),
            kind: "MutatingWebhookConfigurationList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![mutating_webhook_configuration_basic()],
    }
}

fn validating_admission_policy_basic() -> ValidatingAdmissionPolicy {
    ValidatingAdmissionPolicy {
        type_meta: TypeMeta {
            api_version: "admissionregistration.k8s.io/v1".to_string(),
            kind: "ValidatingAdmissionPolicy".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("policy-a".to_string()),
            ..Default::default()
        }),
        spec: ValidatingAdmissionPolicySpec {
            param_kind: Some(ParamKind {
                api_version: "v1".to_string(),
                kind: "ConfigMap".to_string(),
            }),
            match_constraints: Some(MatchResources {
                namespace_selector: Some(LabelSelector {
                    match_labels: BTreeMap::from([("team".to_string(), "infra".to_string())]),
                    ..Default::default()
                }),
                object_selector: None,
                resource_rules: vec![NamedRuleWithOperations {
                    resource_names: vec!["config-a".to_string()],
                    operations: vec![OperationType::Create],
                    api_groups: vec!["".to_string()],
                    api_versions: vec!["v1".to_string()],
                    resources: vec!["configmaps".to_string()],
                    scope: Some(ScopeType::Namespaced),
                }],
                exclude_resource_rules: Vec::new(),
                match_policy: Some(MatchPolicyType::Equivalent),
            }),
            validations: vec![Validation {
                expression: "object.metadata.name.startsWith('a')".to_string(),
                message: "name must start with a".to_string(),
                message_expression: "'invalid name'".to_string(),
            }],
            failure_policy: Some(FailurePolicyType::Fail),
            audit_annotations: vec![AuditAnnotation {
                key: "policy".to_string(),
                value_expression: "'true'".to_string(),
            }],
            match_conditions: vec![MatchCondition {
                name: "has-label".to_string(),
                expression: "'env' in object.metadata.labels".to_string(),
            }],
            variables: vec![Variable {
                name: "env".to_string(),
                expression: "object.metadata.labels['env']".to_string(),
            }],
        },
        status: ValidatingAdmissionPolicyStatus {
            observed_generation: 1,
            type_checking: Some(TypeChecking {
                expression_warnings: vec![super::ExpressionWarning {
                    field_ref: "spec.validations[0]".to_string(),
                    warning: "type mismatch".to_string(),
                }],
            }),
            conditions: Vec::new(),
        },
    }
}

fn validating_admission_policy_list_basic() -> ValidatingAdmissionPolicyList {
    ValidatingAdmissionPolicyList {
        type_meta: TypeMeta {
            api_version: "admissionregistration.k8s.io/v1".to_string(),
            kind: "ValidatingAdmissionPolicyList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("3".to_string()),
            ..Default::default()
        }),
        items: vec![validating_admission_policy_basic()],
    }
}

fn validating_admission_policy_binding_basic() -> ValidatingAdmissionPolicyBinding {
    ValidatingAdmissionPolicyBinding {
        type_meta: TypeMeta {
            api_version: "admissionregistration.k8s.io/v1".to_string(),
            kind: "ValidatingAdmissionPolicyBinding".to_string(),
        },
        metadata: Some(ObjectMeta {
            name: Some("binding-a".to_string()),
            ..Default::default()
        }),
        spec: super::ValidatingAdmissionPolicyBindingSpec {
            policy_name: "policy-a".to_string(),
            param_ref: Some(ParamRef {
                name: "params".to_string(),
                namespace: "default".to_string(),
                selector: Some(LabelSelector {
                    match_labels: BTreeMap::from([("tier".to_string(), "platform".to_string())]),
                    ..Default::default()
                }),
                parameter_not_found_action: Some(ParameterNotFoundActionType::Deny),
            }),
            match_resources: Some(MatchResources {
                namespace_selector: None,
                object_selector: None,
                resource_rules: vec![NamedRuleWithOperations {
                    resource_names: Vec::new(),
                    operations: vec![OperationType::Update],
                    api_groups: vec!["apps".to_string()],
                    api_versions: vec!["v1".to_string()],
                    resources: vec!["deployments".to_string()],
                    scope: Some(ScopeType::Namespaced),
                }],
                exclude_resource_rules: Vec::new(),
                match_policy: Some(MatchPolicyType::Equivalent),
            }),
            validation_actions: vec![ValidationAction::Deny, ValidationAction::Audit],
        },
    }
}

fn validating_admission_policy_binding_list_basic() -> ValidatingAdmissionPolicyBindingList {
    ValidatingAdmissionPolicyBindingList {
        type_meta: TypeMeta {
            api_version: "admissionregistration.k8s.io/v1".to_string(),
            kind: "ValidatingAdmissionPolicyBindingList".to_string(),
        },
        metadata: Some(ListMeta {
            resource_version: Some("4".to_string()),
            ..Default::default()
        }),
        items: vec![validating_admission_policy_binding_basic()],
    }
}

#[test]
fn serde_roundtrip_validating_webhook_configuration() {
    assert_serde_roundtrip(&validating_webhook_configuration_basic());
}

#[test]
fn serde_roundtrip_validating_webhook_configuration_list() {
    assert_serde_roundtrip(&validating_webhook_configuration_list_basic());
}

#[test]
fn serde_roundtrip_mutating_webhook_configuration() {
    assert_serde_roundtrip(&mutating_webhook_configuration_basic());
}

#[test]
fn serde_roundtrip_mutating_webhook_configuration_list() {
    assert_serde_roundtrip(&mutating_webhook_configuration_list_basic());
}

#[test]
fn serde_roundtrip_validating_admission_policy() {
    assert_serde_roundtrip(&validating_admission_policy_basic());
}

#[test]
fn serde_roundtrip_validating_admission_policy_list() {
    assert_serde_roundtrip(&validating_admission_policy_list_basic());
}

#[test]
fn serde_roundtrip_validating_admission_policy_binding() {
    assert_serde_roundtrip(&validating_admission_policy_binding_basic());
}

#[test]
fn serde_roundtrip_validating_admission_policy_binding_list() {
    assert_serde_roundtrip(&validating_admission_policy_binding_list_basic());
}
