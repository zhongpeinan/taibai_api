use super::{
    FailurePolicyType, MatchCondition, MatchResources, MutatingWebhook,
    MutatingWebhookConfiguration, MutatingWebhookConfigurationList, NamedRuleWithOperations,
    OperationType, ParamKind, ParamRef, ParameterNotFoundActionType, ReinvocationPolicyType,
    RuleWithOperations, ServiceReference, SideEffectClass, ValidatingAdmissionPolicy,
    ValidatingAdmissionPolicyBinding, ValidatingAdmissionPolicyBindingList,
    ValidatingAdmissionPolicyList, ValidatingAdmissionPolicySpec, ValidatingAdmissionPolicyStatus,
    ValidatingWebhook, ValidatingWebhookConfiguration, ValidatingWebhookConfigurationList,
    Validation, ValidationAction, WebhookClientConfig,
};
use crate::admissionregistration::internal;
use crate::common::test_utils::assert_conversion_roundtrip;
use crate::common::{LabelSelector, ListMeta, ObjectMeta, TypeMeta};
use crate::core::internal::ByteString;
use std::collections::BTreeMap;

fn validating_webhook_configuration_basic() -> ValidatingWebhookConfiguration {
    ValidatingWebhookConfiguration {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("validate.example.io".to_string()),
            ..Default::default()
        }),
        webhooks: vec![ValidatingWebhook {
            name: "validate.example.io".to_string(),
            client_config: WebhookClientConfig {
                url: None,
                service: Some(ServiceReference {
                    namespace: "default".to_string(),
                    name: "validate-svc".to_string(),
                    path: None,
                    port: None,
                }),
                ca_bundle: ByteString(vec![1, 2, 3]),
            },
            rules: vec![RuleWithOperations {
                operations: vec![OperationType::Create],
                api_groups: vec!["apps".to_string()],
                api_versions: vec!["v1".to_string()],
                resources: vec!["deployments".to_string()],
                scope: None,
            }],
            failure_policy: None,
            match_policy: None,
            namespace_selector: None,
            object_selector: None,
            side_effects: Some(SideEffectClass::None),
            timeout_seconds: None,
            admission_review_versions: vec!["v1".to_string()],
            match_conditions: vec![MatchCondition {
                name: "has-label".to_string(),
                expression: "'app' in object.metadata.labels".to_string(),
            }],
        }],
    }
}

fn validating_webhook_configuration_list_basic() -> ValidatingWebhookConfigurationList {
    ValidatingWebhookConfigurationList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("1".to_string()),
            ..Default::default()
        }),
        items: vec![validating_webhook_configuration_basic()],
    }
}

fn mutating_webhook_configuration_basic() -> MutatingWebhookConfiguration {
    MutatingWebhookConfiguration {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("mutate.example.io".to_string()),
            ..Default::default()
        }),
        webhooks: vec![MutatingWebhook {
            name: "mutate.example.io".to_string(),
            client_config: WebhookClientConfig {
                url: Some("https://example.com/mutate".to_string()),
                service: None,
                ca_bundle: ByteString(vec![4, 5, 6]),
            },
            rules: vec![RuleWithOperations {
                operations: vec![OperationType::Update],
                api_groups: vec!["".to_string()],
                api_versions: vec!["v1".to_string()],
                resources: vec!["pods".to_string()],
                scope: None,
            }],
            failure_policy: Some(FailurePolicyType::Fail),
            match_policy: None,
            namespace_selector: Some(LabelSelector {
                match_labels: BTreeMap::from([("env".to_string(), "prod".to_string())]),
                ..Default::default()
            }),
            object_selector: None,
            side_effects: Some(SideEffectClass::NoneOnDryRun),
            timeout_seconds: Some(15),
            admission_review_versions: vec!["v1".to_string()],
            reinvocation_policy: Some(ReinvocationPolicyType::IfNeeded),
            match_conditions: Vec::new(),
        }],
    }
}

fn mutating_webhook_configuration_list_basic() -> MutatingWebhookConfigurationList {
    MutatingWebhookConfigurationList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("2".to_string()),
            ..Default::default()
        }),
        items: vec![mutating_webhook_configuration_basic()],
    }
}

fn validating_admission_policy_basic() -> ValidatingAdmissionPolicy {
    ValidatingAdmissionPolicy {
        type_meta: TypeMeta::default(),
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
                namespace_selector: None,
                object_selector: None,
                resource_rules: vec![NamedRuleWithOperations {
                    resource_names: Vec::new(),
                    operations: vec![OperationType::Create],
                    api_groups: vec!["".to_string()],
                    api_versions: vec!["v1".to_string()],
                    resources: vec!["configmaps".to_string()],
                    scope: None,
                }],
                exclude_resource_rules: Vec::new(),
                match_policy: None,
            }),
            validations: vec![Validation {
                expression: "true".to_string(),
                message: String::new(),
                message_expression: String::new(),
            }],
            failure_policy: None,
            audit_annotations: Vec::new(),
            match_conditions: Vec::new(),
            variables: Vec::new(),
        },
        status: ValidatingAdmissionPolicyStatus {
            observed_generation: 2,
            type_checking: None,
            conditions: Vec::new(),
        },
    }
}

fn validating_admission_policy_list_basic() -> ValidatingAdmissionPolicyList {
    ValidatingAdmissionPolicyList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("3".to_string()),
            ..Default::default()
        }),
        items: vec![validating_admission_policy_basic()],
    }
}

fn validating_admission_policy_binding_basic() -> ValidatingAdmissionPolicyBinding {
    ValidatingAdmissionPolicyBinding {
        type_meta: TypeMeta::default(),
        metadata: Some(ObjectMeta {
            name: Some("binding-a".to_string()),
            ..Default::default()
        }),
        spec: super::ValidatingAdmissionPolicyBindingSpec {
            policy_name: "policy-a".to_string(),
            param_ref: Some(ParamRef {
                name: "params".to_string(),
                namespace: "default".to_string(),
                selector: None,
                parameter_not_found_action: Some(ParameterNotFoundActionType::Allow),
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
                    scope: None,
                }],
                exclude_resource_rules: Vec::new(),
                match_policy: None,
            }),
            validation_actions: vec![ValidationAction::Deny],
        },
    }
}

fn validating_admission_policy_binding_list_basic() -> ValidatingAdmissionPolicyBindingList {
    ValidatingAdmissionPolicyBindingList {
        type_meta: TypeMeta::default(),
        metadata: Some(ListMeta {
            resource_version: Some("4".to_string()),
            ..Default::default()
        }),
        items: vec![validating_admission_policy_binding_basic()],
    }
}

#[test]
fn conversion_roundtrip_validating_webhook_configuration() {
    assert_conversion_roundtrip::<
        ValidatingWebhookConfiguration,
        internal::ValidatingWebhookConfiguration,
    >(validating_webhook_configuration_basic());
}

#[test]
fn conversion_roundtrip_validating_webhook_configuration_list() {
    assert_conversion_roundtrip::<
        ValidatingWebhookConfigurationList,
        internal::ValidatingWebhookConfigurationList,
    >(validating_webhook_configuration_list_basic());
}

#[test]
fn conversion_roundtrip_mutating_webhook_configuration() {
    assert_conversion_roundtrip::<
        MutatingWebhookConfiguration,
        internal::MutatingWebhookConfiguration,
    >(mutating_webhook_configuration_basic());
}

#[test]
fn conversion_roundtrip_mutating_webhook_configuration_list() {
    assert_conversion_roundtrip::<
        MutatingWebhookConfigurationList,
        internal::MutatingWebhookConfigurationList,
    >(mutating_webhook_configuration_list_basic());
}

#[test]
fn conversion_roundtrip_validating_admission_policy() {
    assert_conversion_roundtrip::<ValidatingAdmissionPolicy, internal::ValidatingAdmissionPolicy>(
        validating_admission_policy_basic(),
    );
}

#[test]
fn conversion_roundtrip_validating_admission_policy_list() {
    assert_conversion_roundtrip::<
        ValidatingAdmissionPolicyList,
        internal::ValidatingAdmissionPolicyList,
    >(validating_admission_policy_list_basic());
}

#[test]
fn conversion_roundtrip_validating_admission_policy_binding() {
    assert_conversion_roundtrip::<
        ValidatingAdmissionPolicyBinding,
        internal::ValidatingAdmissionPolicyBinding,
    >(validating_admission_policy_binding_basic());
}

#[test]
fn conversion_roundtrip_validating_admission_policy_binding_list() {
    assert_conversion_roundtrip::<
        ValidatingAdmissionPolicyBindingList,
        internal::ValidatingAdmissionPolicyBindingList,
    >(validating_admission_policy_binding_list_basic());
}
