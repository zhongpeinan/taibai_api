//! Default values for admissionregistration v1 API types
//!
//! Ported from k8s/pkg/apis/admissionregistration/v1/zz_generated.defaults.go

use crate::common::ApplyDefault;

use super::{
    MatchPolicyType, MatchResources, MutatingWebhook, MutatingWebhookConfiguration,
    MutatingWebhookConfigurationList, NamedRuleWithOperations, RuleWithOperations, ScopeType,
    ServiceReference, ValidatingAdmissionPolicy, ValidatingAdmissionPolicyBinding,
    ValidatingAdmissionPolicyBindingList, ValidatingAdmissionPolicyList,
    ValidatingAdmissionPolicySpec, ValidatingWebhook, ValidatingWebhookConfiguration,
    ValidatingWebhookConfigurationList,
};
use crate::admissionregistration::v1::FailurePolicyType;
use crate::common::LabelSelector;

fn set_defaults_validating_webhook(obj: &mut ValidatingWebhook) {
    if obj.failure_policy.is_none() {
        obj.failure_policy = Some(FailurePolicyType::Fail);
    }
    if obj.match_policy.is_none() {
        obj.match_policy = Some(MatchPolicyType::Equivalent);
    }
    if obj.namespace_selector.is_none() {
        obj.namespace_selector = Some(LabelSelector::default());
    }
    if obj.object_selector.is_none() {
        obj.object_selector = Some(LabelSelector::default());
    }
    if obj.timeout_seconds.is_none() {
        obj.timeout_seconds = Some(10);
    }
}

fn set_defaults_mutating_webhook(obj: &mut MutatingWebhook) {
    if obj.failure_policy.is_none() {
        obj.failure_policy = Some(FailurePolicyType::Fail);
    }
    if obj.match_policy.is_none() {
        obj.match_policy = Some(MatchPolicyType::Equivalent);
    }
    if obj.namespace_selector.is_none() {
        obj.namespace_selector = Some(LabelSelector::default());
    }
    if obj.object_selector.is_none() {
        obj.object_selector = Some(LabelSelector::default());
    }
    if obj.timeout_seconds.is_none() {
        obj.timeout_seconds = Some(10);
    }
    if obj.reinvocation_policy.is_none() {
        obj.reinvocation_policy = Some(super::ReinvocationPolicyType::Never);
    }
}

fn set_defaults_rule_with_operations(obj: &mut RuleWithOperations) {
    if obj.scope.is_none() {
        obj.scope = Some(ScopeType::AllScopes);
    }
}

fn set_defaults_named_rule_with_operations(obj: &mut NamedRuleWithOperations) {
    if obj.scope.is_none() {
        obj.scope = Some(ScopeType::AllScopes);
    }
}

fn set_defaults_service_reference(obj: &mut ServiceReference) {
    if obj.port.is_none() {
        obj.port = Some(443);
    }
}

fn set_defaults_validating_admission_policy_spec(obj: &mut ValidatingAdmissionPolicySpec) {
    if obj.failure_policy.is_none() {
        obj.failure_policy = Some(FailurePolicyType::Fail);
    }
}

fn set_defaults_match_resources(obj: &mut MatchResources) {
    if obj.match_policy.is_none() {
        obj.match_policy = Some(MatchPolicyType::Equivalent);
    }
    if obj.namespace_selector.is_none() {
        obj.namespace_selector = Some(LabelSelector::default());
    }
    if obj.object_selector.is_none() {
        obj.object_selector = Some(LabelSelector::default());
    }
}

impl ApplyDefault for MutatingWebhookConfiguration {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "admissionregistration.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "MutatingWebhookConfiguration".to_string();
        }

        for webhook in &mut self.webhooks {
            set_defaults_mutating_webhook(webhook);
            if let Some(service) = webhook.client_config.service.as_mut() {
                set_defaults_service_reference(service);
            }
            for rule in &mut webhook.rules {
                set_defaults_rule_with_operations(rule);
            }
        }
    }
}

impl ApplyDefault for MutatingWebhookConfigurationList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "admissionregistration.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "MutatingWebhookConfigurationList".to_string();
        }

        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl ApplyDefault for ValidatingAdmissionPolicy {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "admissionregistration.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ValidatingAdmissionPolicy".to_string();
        }

        set_defaults_validating_admission_policy_spec(&mut self.spec);
        if let Some(match_constraints) = self.spec.match_constraints.as_mut() {
            set_defaults_match_resources(match_constraints);
            for rule in &mut match_constraints.resource_rules {
                set_defaults_named_rule_with_operations(rule);
            }
            for rule in &mut match_constraints.exclude_resource_rules {
                set_defaults_named_rule_with_operations(rule);
            }
        }
    }
}

impl ApplyDefault for ValidatingAdmissionPolicyBinding {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "admissionregistration.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ValidatingAdmissionPolicyBinding".to_string();
        }

        if let Some(match_resources) = self.spec.match_resources.as_mut() {
            set_defaults_match_resources(match_resources);
            for rule in &mut match_resources.resource_rules {
                set_defaults_named_rule_with_operations(rule);
            }
            for rule in &mut match_resources.exclude_resource_rules {
                set_defaults_named_rule_with_operations(rule);
            }
        }
    }
}

impl ApplyDefault for ValidatingAdmissionPolicyBindingList {
    fn apply_default(&mut self) {
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl ApplyDefault for ValidatingAdmissionPolicyList {
    fn apply_default(&mut self) {
        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl ApplyDefault for ValidatingWebhookConfiguration {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "admissionregistration.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ValidatingWebhookConfiguration".to_string();
        }

        for webhook in &mut self.webhooks {
            set_defaults_validating_webhook(webhook);
            if let Some(service) = webhook.client_config.service.as_mut() {
                set_defaults_service_reference(service);
            }
            for rule in &mut webhook.rules {
                set_defaults_rule_with_operations(rule);
            }
        }
    }
}

impl ApplyDefault for ValidatingWebhookConfigurationList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "admissionregistration.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ValidatingWebhookConfigurationList".to_string();
        }

        for item in &mut self.items {
            item.apply_default();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{LabelSelector, TypeMeta};

    #[test]
    fn test_mutating_webhook_configuration_defaults() {
        let mut config = MutatingWebhookConfiguration {
            type_meta: TypeMeta::default(),
            metadata: None,
            webhooks: vec![MutatingWebhook {
                name: "example".to_string(),
                client_config: super::super::WebhookClientConfig {
                    service: Some(ServiceReference {
                        namespace: "default".to_string(),
                        name: "svc".to_string(),
                        path: None,
                        port: None,
                    }),
                    ..Default::default()
                },
                rules: vec![RuleWithOperations::default()],
                ..Default::default()
            }],
        };

        config.apply_default();

        assert_eq!(
            config.type_meta.api_version,
            "admissionregistration.k8s.io/v1"
        );
        assert_eq!(config.type_meta.kind, "MutatingWebhookConfiguration");
        let webhook = &config.webhooks[0];
        assert_eq!(webhook.failure_policy, Some(FailurePolicyType::Fail));
        assert_eq!(webhook.match_policy, Some(MatchPolicyType::Equivalent));
        assert!(webhook.namespace_selector.is_some());
        assert!(webhook.object_selector.is_some());
        assert_eq!(webhook.timeout_seconds, Some(10));
        assert_eq!(
            webhook.reinvocation_policy,
            Some(super::super::ReinvocationPolicyType::Never)
        );
        assert_eq!(
            webhook.client_config.service.as_ref().unwrap().port,
            Some(443)
        );
        assert_eq!(webhook.rules[0].scope, Some(ScopeType::AllScopes));
    }

    #[test]
    fn test_validating_admission_policy_defaults_match_resources() {
        let mut policy = ValidatingAdmissionPolicy {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: ValidatingAdmissionPolicySpec {
                match_constraints: Some(MatchResources {
                    match_policy: None,
                    namespace_selector: None,
                    object_selector: None,
                    resource_rules: vec![NamedRuleWithOperations::default()],
                    exclude_resource_rules: Vec::new(),
                }),
                ..Default::default()
            },
            status: Default::default(),
        };

        policy.apply_default();

        assert_eq!(policy.spec.failure_policy, Some(FailurePolicyType::Fail));
        let match_constraints = policy.spec.match_constraints.as_ref().unwrap();
        assert_eq!(
            match_constraints.match_policy,
            Some(MatchPolicyType::Equivalent)
        );
        assert_eq!(
            match_constraints.resource_rules[0].scope,
            Some(ScopeType::AllScopes)
        );
        assert_eq!(
            match_constraints.namespace_selector,
            Some(LabelSelector::default())
        );
        assert_eq!(
            match_constraints.object_selector,
            Some(LabelSelector::default())
        );
    }
}
