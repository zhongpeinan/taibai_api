//! Conversions between admissionregistration v1 and internal types

use crate::admissionregistration::internal;
use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};

use super::{
    MutatingWebhook, MutatingWebhookConfiguration, MutatingWebhookConfigurationList,
    ValidatingAdmissionPolicy, ValidatingAdmissionPolicyBinding,
    ValidatingAdmissionPolicyBindingList, ValidatingAdmissionPolicyList,
    ValidatingAdmissionPolicySpec, ValidatingAdmissionPolicyStatus, ValidatingWebhook,
    ValidatingWebhookConfiguration, ValidatingWebhookConfigurationList,
};

// ============================================================================
// Helper Functions
// ============================================================================

fn is_empty_object_meta(meta: &ObjectMeta) -> bool {
    meta.name.is_none()
        && meta.generate_name.is_none()
        && meta.namespace.is_none()
        && meta.uid.is_none()
        && meta.resource_version.is_none()
        && meta.generation.is_none()
        && meta.self_link.is_none()
        && meta.labels.is_empty()
        && meta.annotations.is_empty()
        && meta.owner_references.is_empty()
        && meta.finalizers.is_empty()
        && meta.managed_fields.is_empty()
        && meta.creation_timestamp.is_none()
        && meta.deletion_timestamp.is_none()
        && meta.deletion_grace_period_seconds.is_none()
}

fn is_empty_list_meta(meta: &ListMeta) -> bool {
    meta.continue_.is_none()
        && meta.remaining_item_count.is_none()
        && meta.resource_version.is_none()
        && meta.self_link.is_none()
}

fn option_object_meta_to_meta(meta: Option<ObjectMeta>) -> ObjectMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_object_meta(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn option_list_meta_to_meta(meta: Option<ListMeta>) -> ListMeta {
    meta.unwrap_or_default()
}

fn meta_to_option_list_meta(meta: ListMeta) -> Option<ListMeta> {
    if is_empty_list_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

// ============================================================================
// ValidatingWebhookConfiguration Conversions
// ============================================================================

impl ToInternal<internal::ValidatingWebhookConfiguration> for ValidatingWebhookConfiguration {
    fn to_internal(self) -> internal::ValidatingWebhookConfiguration {
        internal::ValidatingWebhookConfiguration {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            webhooks: self.webhooks.into_iter().map(|w| w.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::ValidatingWebhookConfiguration> for ValidatingWebhookConfiguration {
    fn from_internal(value: internal::ValidatingWebhookConfiguration) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            webhooks: value
                .webhooks
                .into_iter()
                .map(ValidatingWebhook::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

impl ToInternal<internal::ValidatingWebhookConfigurationList>
    for ValidatingWebhookConfigurationList
{
    fn to_internal(self) -> internal::ValidatingWebhookConfigurationList {
        internal::ValidatingWebhookConfigurationList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(ValidatingWebhookConfiguration::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::ValidatingWebhookConfigurationList>
    for ValidatingWebhookConfigurationList
{
    fn from_internal(value: internal::ValidatingWebhookConfigurationList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(ValidatingWebhookConfiguration::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// MutatingWebhookConfiguration Conversions
// ============================================================================

impl ToInternal<internal::MutatingWebhookConfiguration> for MutatingWebhookConfiguration {
    fn to_internal(self) -> internal::MutatingWebhookConfiguration {
        internal::MutatingWebhookConfiguration {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            webhooks: self.webhooks.into_iter().map(|w| w.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal::MutatingWebhookConfiguration> for MutatingWebhookConfiguration {
    fn from_internal(value: internal::MutatingWebhookConfiguration) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            webhooks: value
                .webhooks
                .into_iter()
                .map(MutatingWebhook::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

impl ToInternal<internal::MutatingWebhookConfigurationList> for MutatingWebhookConfigurationList {
    fn to_internal(self) -> internal::MutatingWebhookConfigurationList {
        internal::MutatingWebhookConfigurationList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(MutatingWebhookConfiguration::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::MutatingWebhookConfigurationList> for MutatingWebhookConfigurationList {
    fn from_internal(value: internal::MutatingWebhookConfigurationList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(MutatingWebhookConfiguration::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// ValidatingAdmissionPolicy Conversions
// ============================================================================

impl ToInternal<internal::ValidatingAdmissionPolicy> for ValidatingAdmissionPolicy {
    fn to_internal(self) -> internal::ValidatingAdmissionPolicy {
        internal::ValidatingAdmissionPolicy {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.to_internal(),
            status: self.status.to_internal(),
        }
    }
}

impl FromInternal<internal::ValidatingAdmissionPolicy> for ValidatingAdmissionPolicy {
    fn from_internal(value: internal::ValidatingAdmissionPolicy) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: ValidatingAdmissionPolicySpec::from_internal(value.spec),
            status: ValidatingAdmissionPolicyStatus::from_internal(value.status),
        };
        result.apply_default();
        result
    }
}

impl ToInternal<internal::ValidatingAdmissionPolicyList> for ValidatingAdmissionPolicyList {
    fn to_internal(self) -> internal::ValidatingAdmissionPolicyList {
        internal::ValidatingAdmissionPolicyList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(ValidatingAdmissionPolicy::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::ValidatingAdmissionPolicyList> for ValidatingAdmissionPolicyList {
    fn from_internal(value: internal::ValidatingAdmissionPolicyList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(ValidatingAdmissionPolicy::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// ValidatingAdmissionPolicyBinding Conversions
// ============================================================================

impl ToInternal<internal::ValidatingAdmissionPolicyBinding> for ValidatingAdmissionPolicyBinding {
    fn to_internal(self) -> internal::ValidatingAdmissionPolicyBinding {
        internal::ValidatingAdmissionPolicyBinding {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.to_internal(),
        }
    }
}

impl FromInternal<internal::ValidatingAdmissionPolicyBinding> for ValidatingAdmissionPolicyBinding {
    fn from_internal(value: internal::ValidatingAdmissionPolicyBinding) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: super::ValidatingAdmissionPolicyBindingSpec::from_internal(value.spec),
        };
        result.apply_default();
        result
    }
}

impl ToInternal<internal::ValidatingAdmissionPolicyBindingList>
    for ValidatingAdmissionPolicyBindingList
{
    fn to_internal(self) -> internal::ValidatingAdmissionPolicyBindingList {
        internal::ValidatingAdmissionPolicyBindingList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(ValidatingAdmissionPolicyBinding::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::ValidatingAdmissionPolicyBindingList>
    for ValidatingAdmissionPolicyBindingList
{
    fn from_internal(value: internal::ValidatingAdmissionPolicyBindingList) -> Self {
        let mut result = Self {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(ValidatingAdmissionPolicyBinding::from_internal)
                .collect(),
        };
        result.apply_default();
        result
    }
}

// ============================================================================
// Supporting Type Conversions
// ============================================================================

impl ToInternal<internal::ValidatingAdmissionPolicySpec> for ValidatingAdmissionPolicySpec {
    fn to_internal(self) -> internal::ValidatingAdmissionPolicySpec {
        internal::ValidatingAdmissionPolicySpec {
            param_kind: self.param_kind,
            match_constraints: self.match_constraints,
            validations: self.validations,
            match_conditions: self.match_conditions,
            failure_policy: self.failure_policy,
            audit_annotations: self.audit_annotations,
            variables: self.variables,
        }
    }
}

impl FromInternal<internal::ValidatingAdmissionPolicySpec> for ValidatingAdmissionPolicySpec {
    fn from_internal(value: internal::ValidatingAdmissionPolicySpec) -> Self {
        Self {
            param_kind: value.param_kind,
            match_constraints: value.match_constraints,
            validations: value.validations,
            match_conditions: value.match_conditions,
            failure_policy: value.failure_policy,
            audit_annotations: value.audit_annotations,
            variables: value.variables,
        }
    }
}

impl ToInternal<internal::ValidatingAdmissionPolicyStatus> for ValidatingAdmissionPolicyStatus {
    fn to_internal(self) -> internal::ValidatingAdmissionPolicyStatus {
        internal::ValidatingAdmissionPolicyStatus {
            observed_generation: self.observed_generation,
            type_checking: self.type_checking,
            conditions: self.conditions,
        }
    }
}

impl FromInternal<internal::ValidatingAdmissionPolicyStatus> for ValidatingAdmissionPolicyStatus {
    fn from_internal(value: internal::ValidatingAdmissionPolicyStatus) -> Self {
        Self {
            observed_generation: value.observed_generation,
            type_checking: value.type_checking,
            conditions: value.conditions,
        }
    }
}

impl ToInternal<internal::ValidatingAdmissionPolicyBindingSpec>
    for super::ValidatingAdmissionPolicyBindingSpec
{
    fn to_internal(self) -> internal::ValidatingAdmissionPolicyBindingSpec {
        internal::ValidatingAdmissionPolicyBindingSpec {
            policy_name: self.policy_name,
            param_ref: self.param_ref,
            match_resources: self.match_resources,
            validation_actions: self.validation_actions,
        }
    }
}

impl FromInternal<internal::ValidatingAdmissionPolicyBindingSpec>
    for super::ValidatingAdmissionPolicyBindingSpec
{
    fn from_internal(value: internal::ValidatingAdmissionPolicyBindingSpec) -> Self {
        Self {
            policy_name: value.policy_name,
            param_ref: value.param_ref,
            match_resources: value.match_resources,
            validation_actions: value.validation_actions,
        }
    }
}

impl ToInternal<internal::ValidatingWebhook> for ValidatingWebhook {
    fn to_internal(self) -> internal::ValidatingWebhook {
        internal::ValidatingWebhook {
            name: self.name,
            client_config: self.client_config,
            rules: self.rules,
            failure_policy: self.failure_policy,
            match_policy: self.match_policy,
            namespace_selector: self.namespace_selector,
            object_selector: self.object_selector,
            side_effects: self.side_effects,
            timeout_seconds: self.timeout_seconds,
            admission_review_versions: self.admission_review_versions,
            match_conditions: self.match_conditions,
        }
    }
}

impl FromInternal<internal::ValidatingWebhook> for ValidatingWebhook {
    fn from_internal(value: internal::ValidatingWebhook) -> Self {
        Self {
            name: value.name,
            client_config: value.client_config,
            rules: value.rules,
            failure_policy: value.failure_policy,
            match_policy: value.match_policy,
            namespace_selector: value.namespace_selector,
            object_selector: value.object_selector,
            side_effects: value.side_effects,
            timeout_seconds: value.timeout_seconds,
            admission_review_versions: value.admission_review_versions,
            match_conditions: value.match_conditions,
        }
    }
}

impl ToInternal<internal::MutatingWebhook> for MutatingWebhook {
    fn to_internal(self) -> internal::MutatingWebhook {
        internal::MutatingWebhook {
            name: self.name,
            client_config: self.client_config,
            rules: self.rules,
            failure_policy: self.failure_policy,
            match_policy: self.match_policy,
            namespace_selector: self.namespace_selector,
            object_selector: self.object_selector,
            side_effects: self.side_effects,
            timeout_seconds: self.timeout_seconds,
            admission_review_versions: self.admission_review_versions,
            reinvocation_policy: self.reinvocation_policy,
            match_conditions: self.match_conditions,
        }
    }
}

impl FromInternal<internal::MutatingWebhook> for MutatingWebhook {
    fn from_internal(value: internal::MutatingWebhook) -> Self {
        Self {
            name: value.name,
            client_config: value.client_config,
            rules: value.rules,
            failure_policy: value.failure_policy,
            match_policy: value.match_policy,
            namespace_selector: value.namespace_selector,
            object_selector: value.object_selector,
            side_effects: value.side_effects,
            timeout_seconds: value.timeout_seconds,
            admission_review_versions: value.admission_review_versions,
            reinvocation_policy: value.reinvocation_policy,
            match_conditions: value.match_conditions,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;

    #[test]
    fn test_validating_webhook_configuration_round_trip() {
        let original = ValidatingWebhookConfiguration {
            type_meta: TypeMeta {
                api_version: "admissionregistration.k8s.io/v1".to_string(),
                kind: "ValidatingWebhookConfiguration".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("vhc".to_string()),
                ..Default::default()
            }),
            webhooks: vec![ValidatingWebhook {
                name: "hook".to_string(),
                admission_review_versions: vec!["v1".to_string()],
                ..Default::default()
            }],
        };

        let internal = original.clone().to_internal();
        let round_trip = ValidatingWebhookConfiguration::from_internal(internal);

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.webhooks[0].name, "hook");
        assert_eq!(
            round_trip.type_meta.api_version,
            "admissionregistration.k8s.io/v1"
        );
        assert_eq!(round_trip.type_meta.kind, "ValidatingWebhookConfiguration");
    }

    #[test]
    fn test_validating_admission_policy_round_trip() {
        let original = ValidatingAdmissionPolicy {
            type_meta: TypeMeta {
                api_version: "admissionregistration.k8s.io/v1".to_string(),
                kind: "ValidatingAdmissionPolicy".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("policy".to_string()),
                ..Default::default()
            }),
            spec: ValidatingAdmissionPolicySpec::default(),
            status: ValidatingAdmissionPolicyStatus::default(),
        };

        let internal = original.clone().to_internal();
        let round_trip = ValidatingAdmissionPolicy::from_internal(internal);

        assert_eq!(round_trip.metadata, original.metadata);
        assert_eq!(round_trip.type_meta.kind, "ValidatingAdmissionPolicy");
    }
}
