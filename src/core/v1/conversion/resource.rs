//! ResourceQuota and LimitRange conversion implementations
//!
//! Includes: LimitRange, ResourceQuota

use super::helpers::*;
use crate::common::traits::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::resource;

// ============================================================================
// LimitRange
// ============================================================================

impl ToInternal<internal::LimitRange> for resource::LimitRange {
    fn to_internal(self) -> internal::LimitRange {
        internal::LimitRange {
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(|spec| spec.to_internal()),
            ..Default::default()
        }
    }
}

impl FromInternal<internal::LimitRange> for resource::LimitRange {
    fn from_internal(value: internal::LimitRange) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(resource::LimitRangeSpec::from_internal),
        };

        result
    }
}

impl ToInternal<internal::LimitRangeSpec> for resource::LimitRangeSpec {
    fn to_internal(self) -> internal::LimitRangeSpec {
        internal::LimitRangeSpec {
            limits: self
                .limits
                .into_iter()
                .map(resource::LimitRangeItem::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::LimitRangeSpec> for resource::LimitRangeSpec {
    fn from_internal(value: internal::LimitRangeSpec) -> Self {
        Self {
            limits: value
                .limits
                .into_iter()
                .map(resource::LimitRangeItem::from_internal)
                .collect(),
        }
    }
}

impl ToInternal<internal::LimitRangeItem> for resource::LimitRangeItem {
    fn to_internal(self) -> internal::LimitRangeItem {
        internal::LimitRangeItem {
            r#type: string_to_limit_type(self.type_),
            min: self.min,
            max: self.max,
            default: self.default,
            default_request: self.default_request,
            max_limit_request_ratio: self.max_limit_request_ratio,
        }
    }
}

impl FromInternal<internal::LimitRangeItem> for resource::LimitRangeItem {
    fn from_internal(value: internal::LimitRangeItem) -> Self {
        Self {
            type_: limit_type_to_string(value.r#type),
            min: value.min,
            max: value.max,
            default: value.default,
            default_request: value.default_request,
            max_limit_request_ratio: value.max_limit_request_ratio,
        }
    }
}

impl ToInternal<internal::LimitRangeList> for resource::LimitRangeList {
    fn to_internal(self) -> internal::LimitRangeList {
        internal::LimitRangeList {
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(|item| item.to_internal())
                .collect(),
            ..Default::default()
        }
    }
}

impl FromInternal<internal::LimitRangeList> for resource::LimitRangeList {
    fn from_internal(value: internal::LimitRangeList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(resource::LimitRange::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// ResourceQuota
// ============================================================================

impl ToInternal<internal::ResourceQuota> for resource::ResourceQuota {
    fn to_internal(self) -> internal::ResourceQuota {
        internal::ResourceQuota {
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(|spec| spec.to_internal()),
            status: self
                .status
                .map(resource::ResourceQuotaStatus::to_internal)
                .unwrap_or_default(),
            ..Default::default()
        }
    }
}

impl FromInternal<internal::ResourceQuota> for resource::ResourceQuota {
    fn from_internal(value: internal::ResourceQuota) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(resource::ResourceQuotaSpec::from_internal),
            status: Some(resource::ResourceQuotaStatus::from_internal(value.status)),
        };

        result
    }
}

impl ToInternal<internal::ResourceQuotaSpec> for resource::ResourceQuotaSpec {
    fn to_internal(self) -> internal::ResourceQuotaSpec {
        internal::ResourceQuotaSpec {
            hard: self.hard,
            scopes: self
                .scopes
                .into_iter()
                .map(string_to_resource_quota_scope)
                .collect(),
            scope_selector: self
                .scope_selector
                .map(resource::ScopeSelector::to_internal),
        }
    }
}

impl FromInternal<internal::ResourceQuotaSpec> for resource::ResourceQuotaSpec {
    fn from_internal(value: internal::ResourceQuotaSpec) -> Self {
        Self {
            hard: value.hard,
            scopes: value
                .scopes
                .into_iter()
                .map(resource_quota_scope_to_string)
                .collect(),
            scope_selector: value
                .scope_selector
                .map(resource::ScopeSelector::from_internal),
        }
    }
}

impl ToInternal<internal::ResourceQuotaStatus> for resource::ResourceQuotaStatus {
    fn to_internal(self) -> internal::ResourceQuotaStatus {
        internal::ResourceQuotaStatus {
            hard: self.hard,
            used: self.used,
        }
    }
}

impl FromInternal<internal::ResourceQuotaStatus> for resource::ResourceQuotaStatus {
    fn from_internal(value: internal::ResourceQuotaStatus) -> Self {
        Self {
            hard: value.hard,
            used: value.used,
        }
    }
}

impl ToInternal<internal::ScopeSelector> for resource::ScopeSelector {
    fn to_internal(self) -> internal::ScopeSelector {
        internal::ScopeSelector {
            match_expressions: self
                .match_expressions
                .into_iter()
                .map(resource::ScopedResourceSelectorRequirement::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::ScopeSelector> for resource::ScopeSelector {
    fn from_internal(value: internal::ScopeSelector) -> Self {
        Self {
            match_expressions: value
                .match_expressions
                .into_iter()
                .map(resource::ScopedResourceSelectorRequirement::from_internal)
                .collect(),
        }
    }
}

impl ToInternal<internal::ScopedResourceSelectorRequirement>
    for resource::ScopedResourceSelectorRequirement
{
    fn to_internal(self) -> internal::ScopedResourceSelectorRequirement {
        internal::ScopedResourceSelectorRequirement {
            scope_name: string_to_resource_quota_scope(self.scope_name),
            operator: string_to_scope_selector_operator(self.operator),
            values: self.values,
        }
    }
}

impl FromInternal<internal::ScopedResourceSelectorRequirement>
    for resource::ScopedResourceSelectorRequirement
{
    fn from_internal(value: internal::ScopedResourceSelectorRequirement) -> Self {
        Self {
            scope_name: resource_quota_scope_to_string(value.scope_name),
            operator: scope_selector_operator_to_string(value.operator),
            values: value.values,
        }
    }
}

impl ToInternal<internal::ResourceQuotaList> for resource::ResourceQuotaList {
    fn to_internal(self) -> internal::ResourceQuotaList {
        internal::ResourceQuotaList {
            metadata: option_list_meta_to_meta(self.metadata),
            items: self
                .items
                .into_iter()
                .map(resource::ResourceQuota::to_internal)
                .collect(),
            ..Default::default()
        }
    }
}

impl FromInternal<internal::ResourceQuotaList> for resource::ResourceQuotaList {
    fn from_internal(value: internal::ResourceQuotaList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_list_meta(value.metadata),
            items: value
                .items
                .into_iter()
                .map(resource::ResourceQuota::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// Helper conversion functions
// ============================================================================

fn string_to_limit_type(value: String) -> internal::LimitType {
    match value.as_str() {
        resource::limit_type::POD => internal::LimitType::Pod,
        resource::limit_type::CONTAINER => internal::LimitType::Container,
        resource::limit_type::PERSISTENT_VOLUME_CLAIM => internal::LimitType::PersistentVolumeClaim,
        _ => internal::LimitType::default(),
    }
}

fn limit_type_to_string(value: internal::LimitType) -> String {
    match value {
        internal::LimitType::Pod => resource::limit_type::POD.to_string(),
        internal::LimitType::Container => resource::limit_type::CONTAINER.to_string(),
        internal::LimitType::PersistentVolumeClaim => {
            resource::limit_type::PERSISTENT_VOLUME_CLAIM.to_string()
        }
    }
}

fn string_to_resource_quota_scope(value: String) -> internal::ResourceQuotaScope {
    match value.as_str() {
        resource::resource_quota_scope::TERMINATING => internal::ResourceQuotaScope::Terminating,
        resource::resource_quota_scope::NOT_TERMINATING => {
            internal::ResourceQuotaScope::NotTerminating
        }
        resource::resource_quota_scope::BEST_EFFORT => internal::ResourceQuotaScope::BestEffort,
        resource::resource_quota_scope::NOT_BEST_EFFORT => {
            internal::ResourceQuotaScope::NotBestEffort
        }
        resource::resource_quota_scope::PRIORITY_CLASS => {
            internal::ResourceQuotaScope::PriorityClass
        }
        resource::resource_quota_scope::CROSS_NAMESPACE_POD_AFFINITY => {
            internal::ResourceQuotaScope::CrossNamespacePodAffinity
        }
        resource::resource_quota_scope::VOLUME_ATTRIBUTES_CLASS => {
            internal::ResourceQuotaScope::VolumeAttributesClass
        }
        _ => internal::ResourceQuotaScope::default(),
    }
}

fn resource_quota_scope_to_string(value: internal::ResourceQuotaScope) -> String {
    match value {
        internal::ResourceQuotaScope::Terminating => {
            resource::resource_quota_scope::TERMINATING.to_string()
        }
        internal::ResourceQuotaScope::NotTerminating => {
            resource::resource_quota_scope::NOT_TERMINATING.to_string()
        }
        internal::ResourceQuotaScope::BestEffort => {
            resource::resource_quota_scope::BEST_EFFORT.to_string()
        }
        internal::ResourceQuotaScope::NotBestEffort => {
            resource::resource_quota_scope::NOT_BEST_EFFORT.to_string()
        }
        internal::ResourceQuotaScope::PriorityClass => {
            resource::resource_quota_scope::PRIORITY_CLASS.to_string()
        }
        internal::ResourceQuotaScope::CrossNamespacePodAffinity => {
            resource::resource_quota_scope::CROSS_NAMESPACE_POD_AFFINITY.to_string()
        }
        internal::ResourceQuotaScope::VolumeAttributesClass => {
            resource::resource_quota_scope::VOLUME_ATTRIBUTES_CLASS.to_string()
        }
    }
}

fn string_to_scope_selector_operator(value: String) -> Option<internal::ScopeSelectorOperator> {
    match value.as_str() {
        "" => None,
        resource::scope_selector_operator::IN => Some(internal::ScopeSelectorOperator::In),
        resource::scope_selector_operator::NOT_IN => Some(internal::ScopeSelectorOperator::NotIn),
        resource::scope_selector_operator::EXISTS => Some(internal::ScopeSelectorOperator::Exists),
        resource::scope_selector_operator::DOES_NOT_EXIST => {
            Some(internal::ScopeSelectorOperator::DoesNotExist)
        }
        _ => Some(internal::ScopeSelectorOperator::default()),
    }
}

fn scope_selector_operator_to_string(value: Option<internal::ScopeSelectorOperator>) -> String {
    match value {
        None => String::new(),
        Some(internal::ScopeSelectorOperator::In) => {
            resource::scope_selector_operator::IN.to_string()
        }
        Some(internal::ScopeSelectorOperator::NotIn) => {
            resource::scope_selector_operator::NOT_IN.to_string()
        }
        Some(internal::ScopeSelectorOperator::Exists) => {
            resource::scope_selector_operator::EXISTS.to_string()
        }
        Some(internal::ScopeSelectorOperator::DoesNotExist) => {
            resource::scope_selector_operator::DOES_NOT_EXIST.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_range_roundtrip() {
        let limit_range = resource::LimitRange {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("limits".to_string()),
                ..Default::default()
            }),
            spec: Some(resource::LimitRangeSpec {
                limits: vec![resource::LimitRangeItem {
                    type_: resource::limit_type::CONTAINER.to_string(),
                    max: std::collections::BTreeMap::from([(
                        "cpu".to_string(),
                        crate::common::Quantity::from_str("1"),
                    )]),
                    ..Default::default()
                }],
            }),
        };

        let internal = limit_range.clone().to_internal();
        assert_eq!(internal.metadata.name.as_deref(), Some("limits"));
        assert_eq!(internal.spec.as_ref().unwrap().limits.len(), 1);

        let mut roundtrip = resource::LimitRange::from_internal(internal);
        roundtrip.apply_default();
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name.as_deref(),
            Some("limits")
        );
        assert_eq!(roundtrip.spec.as_ref().unwrap().limits.len(), 1);
    }

    #[test]
    fn test_resource_quota_roundtrip() {
        let quota = resource::ResourceQuota {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("rq".to_string()),
                ..Default::default()
            }),
            spec: Some(resource::ResourceQuotaSpec {
                hard: std::collections::BTreeMap::from([(
                    "cpu".to_string(),
                    crate::common::Quantity::from_str("2"),
                )]),
                scopes: vec![resource::resource_quota_scope::BEST_EFFORT.to_string()],
                scope_selector: Some(resource::ScopeSelector {
                    match_expressions: vec![resource::ScopedResourceSelectorRequirement {
                        scope_name: resource::resource_quota_scope::BEST_EFFORT.to_string(),
                        operator: resource::scope_selector_operator::EXISTS.to_string(),
                        values: vec![],
                    }],
                }),
            }),
            ..Default::default()
        };

        let internal = quota.clone().to_internal();
        assert_eq!(internal.metadata.name.as_deref(), Some("rq"));
        assert_eq!(internal.spec.as_ref().unwrap().scopes.len(), 1);

        let mut roundtrip = resource::ResourceQuota::from_internal(internal);
        roundtrip.apply_default();
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name.as_deref(),
            Some("rq")
        );
        assert_eq!(roundtrip.spec.as_ref().unwrap().scopes.len(), 1);
    }
}
