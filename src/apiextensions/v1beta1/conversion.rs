//! Conversions between apiextensions v1beta1 and internal types.
//!
//! Source: k8s.io/apiextensions-apiserver/pkg/apis/apiextensions/v1beta1/zz_generated.conversion.go

use crate::apiextensions::internal;
use crate::common::{ApplyDefault, FromInternal, ListMeta, ObjectMeta, ToInternal, TypeMeta};

use super::{
    ConditionStatus, CustomResourceColumnDefinition, CustomResourceConversion,
    CustomResourceDefinition, CustomResourceDefinitionCondition, CustomResourceDefinitionList,
    CustomResourceDefinitionNames, CustomResourceDefinitionSpec, CustomResourceDefinitionStatus,
    CustomResourceDefinitionVersion, CustomResourceSubresourceScale,
    CustomResourceSubresourceStatus, CustomResourceSubresources, CustomResourceValidation,
    ResourceScope, SelectableField, ServiceReference, WebhookClientConfig,
};

// ============================================================================
// Helper functions for metadata conversions
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
    if meta.is_empty() { None } else { Some(meta) }
}

// ============================================================================
// Enum conversions
// ============================================================================

impl From<super::ConversionStrategyType> for internal::ConversionStrategyType {
    fn from(value: super::ConversionStrategyType) -> Self {
        match value {
            super::ConversionStrategyType::None => internal::ConversionStrategyType::None,
            super::ConversionStrategyType::Webhook => internal::ConversionStrategyType::Webhook,
        }
    }
}

impl From<internal::ConversionStrategyType> for super::ConversionStrategyType {
    fn from(value: internal::ConversionStrategyType) -> Self {
        match value {
            internal::ConversionStrategyType::None => super::ConversionStrategyType::None,
            internal::ConversionStrategyType::Webhook => super::ConversionStrategyType::Webhook,
        }
    }
}

impl From<ResourceScope> for internal::ResourceScope {
    fn from(value: ResourceScope) -> Self {
        match value {
            ResourceScope::Cluster => internal::ResourceScope::Cluster,
            ResourceScope::Namespaced => internal::ResourceScope::Namespaced,
        }
    }
}

impl From<internal::ResourceScope> for ResourceScope {
    fn from(value: internal::ResourceScope) -> Self {
        match value {
            internal::ResourceScope::Cluster => ResourceScope::Cluster,
            internal::ResourceScope::Namespaced => ResourceScope::Namespaced,
        }
    }
}

impl From<ConditionStatus> for internal::ConditionStatus {
    fn from(value: ConditionStatus) -> Self {
        match value {
            ConditionStatus::True => internal::ConditionStatus::True,
            ConditionStatus::False => internal::ConditionStatus::False,
            ConditionStatus::Unknown => internal::ConditionStatus::Unknown,
        }
    }
}

impl From<internal::ConditionStatus> for ConditionStatus {
    fn from(value: internal::ConditionStatus) -> Self {
        match value {
            internal::ConditionStatus::True => ConditionStatus::True,
            internal::ConditionStatus::False => ConditionStatus::False,
            internal::ConditionStatus::Unknown => ConditionStatus::Unknown,
        }
    }
}

impl From<super::CustomResourceDefinitionConditionType>
    for internal::CustomResourceDefinitionConditionType
{
    fn from(value: super::CustomResourceDefinitionConditionType) -> Self {
        match value {
            super::CustomResourceDefinitionConditionType::Established => {
                internal::CustomResourceDefinitionConditionType::Established
            }
            super::CustomResourceDefinitionConditionType::NamesAccepted => {
                internal::CustomResourceDefinitionConditionType::NamesAccepted
            }
            super::CustomResourceDefinitionConditionType::NonStructuralSchema => {
                internal::CustomResourceDefinitionConditionType::NonStructuralSchema
            }
            super::CustomResourceDefinitionConditionType::Terminating => {
                internal::CustomResourceDefinitionConditionType::Terminating
            }
            super::CustomResourceDefinitionConditionType::KubernetesAPIApprovalPolicyConformant => {
                internal::CustomResourceDefinitionConditionType::KubernetesAPIApprovalPolicyConformant
            }
        }
    }
}

impl From<internal::CustomResourceDefinitionConditionType>
    for super::CustomResourceDefinitionConditionType
{
    fn from(value: internal::CustomResourceDefinitionConditionType) -> Self {
        match value {
            internal::CustomResourceDefinitionConditionType::Established => {
                super::CustomResourceDefinitionConditionType::Established
            }
            internal::CustomResourceDefinitionConditionType::NamesAccepted => {
                super::CustomResourceDefinitionConditionType::NamesAccepted
            }
            internal::CustomResourceDefinitionConditionType::NonStructuralSchema => {
                super::CustomResourceDefinitionConditionType::NonStructuralSchema
            }
            internal::CustomResourceDefinitionConditionType::Terminating => {
                super::CustomResourceDefinitionConditionType::Terminating
            }
            internal::CustomResourceDefinitionConditionType::KubernetesAPIApprovalPolicyConformant => {
                super::CustomResourceDefinitionConditionType::KubernetesAPIApprovalPolicyConformant
            }
        }
    }
}

// ============================================================================
// Struct conversions
// ============================================================================

fn to_internal_custom_resource_validation(
    value: CustomResourceValidation,
) -> internal::CustomResourceValidation {
    internal::CustomResourceValidation {
        open_api_v3_schema: value.open_api_v3_schema,
    }
}

fn from_internal_custom_resource_validation(
    value: internal::CustomResourceValidation,
) -> CustomResourceValidation {
    CustomResourceValidation {
        open_api_v3_schema: value.open_api_v3_schema,
    }
}

fn to_internal_custom_resource_subresource_status(
    _value: CustomResourceSubresourceStatus,
) -> internal::CustomResourceSubresourceStatus {
    internal::CustomResourceSubresourceStatus {}
}

fn from_internal_custom_resource_subresource_status(
    _value: internal::CustomResourceSubresourceStatus,
) -> CustomResourceSubresourceStatus {
    CustomResourceSubresourceStatus {}
}

fn to_internal_custom_resource_subresource_scale(
    value: CustomResourceSubresourceScale,
) -> internal::CustomResourceSubresourceScale {
    internal::CustomResourceSubresourceScale {
        spec_replicas_path: value.spec_replicas_path,
        status_replicas_path: value.status_replicas_path,
        label_selector_path: value.label_selector_path,
    }
}

fn from_internal_custom_resource_subresource_scale(
    value: internal::CustomResourceSubresourceScale,
) -> CustomResourceSubresourceScale {
    CustomResourceSubresourceScale {
        spec_replicas_path: value.spec_replicas_path,
        status_replicas_path: value.status_replicas_path,
        label_selector_path: value.label_selector_path,
    }
}

fn to_internal_custom_resource_subresources(
    value: CustomResourceSubresources,
) -> internal::CustomResourceSubresources {
    internal::CustomResourceSubresources {
        status: value
            .status
            .map(to_internal_custom_resource_subresource_status),
        scale: value
            .scale
            .map(to_internal_custom_resource_subresource_scale),
    }
}

fn from_internal_custom_resource_subresources(
    value: internal::CustomResourceSubresources,
) -> CustomResourceSubresources {
    CustomResourceSubresources {
        status: value
            .status
            .map(from_internal_custom_resource_subresource_status),
        scale: value
            .scale
            .map(from_internal_custom_resource_subresource_scale),
    }
}

fn to_internal_custom_resource_column_definition(
    value: CustomResourceColumnDefinition,
) -> internal::CustomResourceColumnDefinition {
    internal::CustomResourceColumnDefinition {
        name: value.name,
        r#type: value.r#type,
        format: value.format,
        description: value.description,
        priority: value.priority,
        json_path: value.json_path,
    }
}

fn from_internal_custom_resource_column_definition(
    value: internal::CustomResourceColumnDefinition,
) -> CustomResourceColumnDefinition {
    CustomResourceColumnDefinition {
        name: value.name,
        r#type: value.r#type,
        format: value.format,
        description: value.description,
        priority: value.priority,
        json_path: value.json_path,
    }
}

fn to_internal_selectable_field(value: SelectableField) -> internal::SelectableField {
    internal::SelectableField {
        json_path: value.json_path,
    }
}

fn from_internal_selectable_field(value: internal::SelectableField) -> SelectableField {
    SelectableField {
        json_path: value.json_path,
    }
}

fn to_internal_custom_resource_definition_names(
    value: CustomResourceDefinitionNames,
) -> internal::CustomResourceDefinitionNames {
    internal::CustomResourceDefinitionNames {
        plural: value.plural,
        singular: value.singular,
        short_names: value.short_names,
        kind: value.kind,
        list_kind: value.list_kind,
        categories: value.categories,
    }
}

fn from_internal_custom_resource_definition_names(
    value: internal::CustomResourceDefinitionNames,
) -> CustomResourceDefinitionNames {
    CustomResourceDefinitionNames {
        plural: value.plural,
        singular: value.singular,
        short_names: value.short_names,
        kind: value.kind,
        list_kind: value.list_kind,
        categories: value.categories,
    }
}

fn to_internal_custom_resource_definition_condition(
    value: CustomResourceDefinitionCondition,
) -> internal::CustomResourceDefinitionCondition {
    internal::CustomResourceDefinitionCondition {
        r#type: value.r#type.into(),
        status: value.status.into(),
        last_transition_time: value.last_transition_time,
        reason: value.reason,
        message: value.message,
    }
}

fn from_internal_custom_resource_definition_condition(
    value: internal::CustomResourceDefinitionCondition,
) -> CustomResourceDefinitionCondition {
    CustomResourceDefinitionCondition {
        r#type: value.r#type.into(),
        status: value.status.into(),
        last_transition_time: value.last_transition_time,
        reason: value.reason,
        message: value.message,
    }
}

fn to_internal_custom_resource_definition_status(
    value: CustomResourceDefinitionStatus,
) -> internal::CustomResourceDefinitionStatus {
    internal::CustomResourceDefinitionStatus {
        conditions: value
            .conditions
            .into_iter()
            .map(to_internal_custom_resource_definition_condition)
            .collect(),
        accepted_names: to_internal_custom_resource_definition_names(value.accepted_names),
        stored_versions: value.stored_versions,
    }
}

fn from_internal_custom_resource_definition_status(
    value: internal::CustomResourceDefinitionStatus,
) -> CustomResourceDefinitionStatus {
    CustomResourceDefinitionStatus {
        conditions: value
            .conditions
            .into_iter()
            .map(from_internal_custom_resource_definition_condition)
            .collect(),
        accepted_names: from_internal_custom_resource_definition_names(value.accepted_names),
        stored_versions: value.stored_versions,
    }
}

fn to_internal_service_reference(value: ServiceReference) -> internal::ServiceReference {
    internal::ServiceReference {
        namespace: value.namespace,
        name: value.name,
        path: value.path,
        port: value.port.unwrap_or_default(),
    }
}

fn from_internal_service_reference(value: internal::ServiceReference) -> ServiceReference {
    ServiceReference {
        namespace: value.namespace,
        name: value.name,
        path: value.path,
        port: Some(value.port),
    }
}

fn to_internal_webhook_client_config(value: WebhookClientConfig) -> internal::WebhookClientConfig {
    internal::WebhookClientConfig {
        url: value.url,
        service: value.service.map(to_internal_service_reference),
        ca_bundle: value.ca_bundle,
    }
}

fn from_internal_webhook_client_config(
    value: internal::WebhookClientConfig,
) -> WebhookClientConfig {
    WebhookClientConfig {
        url: value.url,
        service: value.service.map(from_internal_service_reference),
        ca_bundle: value.ca_bundle,
    }
}

fn to_internal_custom_resource_conversion(
    value: CustomResourceConversion,
) -> internal::CustomResourceConversion {
    internal::CustomResourceConversion {
        strategy: value.strategy.into(),
        webhook_client_config: value
            .webhook_client_config
            .map(to_internal_webhook_client_config),
        conversion_review_versions: value.conversion_review_versions,
    }
}

fn from_internal_custom_resource_conversion(
    value: internal::CustomResourceConversion,
) -> CustomResourceConversion {
    CustomResourceConversion {
        strategy: value.strategy.into(),
        webhook_client_config: value
            .webhook_client_config
            .map(from_internal_webhook_client_config),
        conversion_review_versions: value.conversion_review_versions,
    }
}

fn to_internal_custom_resource_definition_version(
    value: CustomResourceDefinitionVersion,
) -> internal::CustomResourceDefinitionVersion {
    internal::CustomResourceDefinitionVersion {
        name: value.name,
        served: value.served,
        storage: value.storage,
        deprecated: value.deprecated,
        deprecation_warning: value.deprecation_warning,
        schema: value.schema.map(to_internal_custom_resource_validation),
        subresources: value
            .subresources
            .map(to_internal_custom_resource_subresources),
        additional_printer_columns: value
            .additional_printer_columns
            .into_iter()
            .map(to_internal_custom_resource_column_definition)
            .collect(),
        selectable_fields: value
            .selectable_fields
            .into_iter()
            .map(to_internal_selectable_field)
            .collect(),
    }
}

fn from_internal_custom_resource_definition_version(
    value: internal::CustomResourceDefinitionVersion,
) -> CustomResourceDefinitionVersion {
    CustomResourceDefinitionVersion {
        name: value.name,
        served: value.served,
        storage: value.storage,
        deprecated: value.deprecated,
        deprecation_warning: value.deprecation_warning,
        schema: value.schema.map(from_internal_custom_resource_validation),
        subresources: value
            .subresources
            .map(from_internal_custom_resource_subresources),
        additional_printer_columns: value
            .additional_printer_columns
            .into_iter()
            .map(from_internal_custom_resource_column_definition)
            .collect(),
        selectable_fields: value
            .selectable_fields
            .into_iter()
            .map(from_internal_selectable_field)
            .collect(),
    }
}

fn to_internal_custom_resource_definition_spec(
    value: CustomResourceDefinitionSpec,
) -> internal::CustomResourceDefinitionSpec {
    internal::CustomResourceDefinitionSpec {
        group: value.group,
        version: value.version,
        names: to_internal_custom_resource_definition_names(value.names),
        scope: value.scope.into(),
        validation: value.validation.map(to_internal_custom_resource_validation),
        subresources: value
            .subresources
            .map(to_internal_custom_resource_subresources),
        versions: value
            .versions
            .into_iter()
            .map(to_internal_custom_resource_definition_version)
            .collect(),
        additional_printer_columns: value
            .additional_printer_columns
            .into_iter()
            .map(to_internal_custom_resource_column_definition)
            .collect(),
        selectable_fields: value
            .selectable_fields
            .into_iter()
            .map(to_internal_selectable_field)
            .collect(),
        conversion: value.conversion.map(to_internal_custom_resource_conversion),
        preserve_unknown_fields: value.preserve_unknown_fields,
    }
}

fn from_internal_custom_resource_definition_spec(
    value: internal::CustomResourceDefinitionSpec,
) -> CustomResourceDefinitionSpec {
    let internal::CustomResourceDefinitionSpec {
        group,
        version,
        names,
        scope,
        validation,
        subresources,
        versions,
        additional_printer_columns,
        selectable_fields,
        conversion,
        preserve_unknown_fields,
    } = value;

    CustomResourceDefinitionSpec {
        group,
        version,
        names: from_internal_custom_resource_definition_names(names),
        scope: scope.into(),
        validation: validation.map(from_internal_custom_resource_validation),
        subresources: subresources.map(from_internal_custom_resource_subresources),
        versions: versions
            .into_iter()
            .map(from_internal_custom_resource_definition_version)
            .collect(),
        additional_printer_columns: additional_printer_columns
            .into_iter()
            .map(from_internal_custom_resource_column_definition)
            .collect(),
        selectable_fields: selectable_fields
            .into_iter()
            .map(from_internal_selectable_field)
            .collect(),
        conversion: conversion.map(from_internal_custom_resource_conversion),
        preserve_unknown_fields,
    }
}

// ============================================================================
// Top-level conversions
// ============================================================================

impl ToInternal<internal::CustomResourceDefinition> for CustomResourceDefinition {
    fn to_internal(self) -> internal::CustomResourceDefinition {
        internal::CustomResourceDefinition {
            type_meta: TypeMeta::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            spec: to_internal_custom_resource_definition_spec(self.spec),
            status: to_internal_custom_resource_definition_status(self.status),
        }
    }
}

impl FromInternal<internal::CustomResourceDefinition> for CustomResourceDefinition {
    fn from_internal(internal: internal::CustomResourceDefinition) -> Self {
        let mut result = CustomResourceDefinition {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_object_meta(internal.metadata),
            spec: from_internal_custom_resource_definition_spec(internal.spec),
            status: from_internal_custom_resource_definition_status(internal.status),
        };

        result
    }
}

impl ToInternal<internal::CustomResourceDefinitionList> for CustomResourceDefinitionList {
    fn to_internal(self) -> internal::CustomResourceDefinitionList {
        internal::CustomResourceDefinitionList {
            type_meta: TypeMeta::default(),
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
        }
    }
}

impl FromInternal<internal::CustomResourceDefinitionList> for CustomResourceDefinitionList {
    fn from_internal(internal: internal::CustomResourceDefinitionList) -> Self {
        let mut result = CustomResourceDefinitionList {
            type_meta: TypeMeta::default(),
            metadata: meta_to_option_list_meta(internal.metadata),
            items: internal
                .items
                .into_iter()
                .map(CustomResourceDefinition::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// Placeholder conversions for ConversionReview (no internal peer type)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_resource_definition_roundtrip() {
        let v1 = CustomResourceDefinition {
            type_meta: TypeMeta {
                api_version: "apiextensions.k8s.io/v1beta1".to_string(),
                kind: "CustomResourceDefinition".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("widgets.example.com".to_string()),
                ..Default::default()
            }),
            spec: CustomResourceDefinitionSpec {
                group: "example.com".to_string(),
                version: "v1".to_string(),
                names: CustomResourceDefinitionNames {
                    plural: "widgets".to_string(),
                    kind: "Widget".to_string(),
                    ..Default::default()
                },
                scope: ResourceScope::Namespaced,
                versions: vec![CustomResourceDefinitionVersion {
                    name: "v1".to_string(),
                    served: true,
                    storage: true,
                    ..Default::default()
                }],
                ..Default::default()
            },
            status: Default::default(),
        };

        let internal = v1.clone().to_internal();
        let mut round_trip = CustomResourceDefinition::from_internal(internal);
        round_trip.apply_default();

        assert_eq!(round_trip.spec.group, v1.spec.group);
        assert_eq!(round_trip.spec.names.kind, v1.spec.names.kind);
        assert_eq!(round_trip.spec.version, "v1");
    }
}
