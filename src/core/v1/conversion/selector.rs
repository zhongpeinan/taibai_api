//! Selector conversions between v1 and internal core types.

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal::selector as internal_selector;
use crate::core::v1::selector as v1_selector;

// ObjectFieldSelector
impl ToInternal<internal_selector::ObjectFieldSelector> for v1_selector::ObjectFieldSelector {
    fn to_internal(self) -> internal_selector::ObjectFieldSelector {
        internal_selector::ObjectFieldSelector {
            api_version: self.api_version,
            field_path: self.field_path,
        }
    }
}

impl FromInternal<internal_selector::ObjectFieldSelector> for v1_selector::ObjectFieldSelector {
    fn from_internal(value: internal_selector::ObjectFieldSelector) -> Self {
        Self {
            api_version: value.api_version,
            field_path: value.field_path,
        }
    }
}

// ResourceFieldSelector
impl ToInternal<internal_selector::ResourceFieldSelector> for v1_selector::ResourceFieldSelector {
    fn to_internal(self) -> internal_selector::ResourceFieldSelector {
        internal_selector::ResourceFieldSelector {
            container_name: self.container_name,
            resource: self.resource,
            divisor: self.divisor,
        }
    }
}

impl FromInternal<internal_selector::ResourceFieldSelector> for v1_selector::ResourceFieldSelector {
    fn from_internal(value: internal_selector::ResourceFieldSelector) -> Self {
        Self {
            container_name: value.container_name,
            resource: value.resource,
            divisor: value.divisor,
        }
    }
}

// ConfigMapKeySelector
impl ToInternal<internal_selector::ConfigMapKeySelector> for v1_selector::ConfigMapKeySelector {
    fn to_internal(self) -> internal_selector::ConfigMapKeySelector {
        internal_selector::ConfigMapKeySelector {
            name: self.name,
            key: self.key,
            optional: self.optional,
        }
    }
}

impl FromInternal<internal_selector::ConfigMapKeySelector> for v1_selector::ConfigMapKeySelector {
    fn from_internal(value: internal_selector::ConfigMapKeySelector) -> Self {
        Self {
            name: value.name,
            key: value.key,
            optional: value.optional,
        }
    }
}

// SecretKeySelector
impl ToInternal<internal_selector::SecretKeySelector> for v1_selector::SecretKeySelector {
    fn to_internal(self) -> internal_selector::SecretKeySelector {
        internal_selector::SecretKeySelector {
            name: self.name,
            key: self.key,
            optional: self.optional,
        }
    }
}

impl FromInternal<internal_selector::SecretKeySelector> for v1_selector::SecretKeySelector {
    fn from_internal(value: internal_selector::SecretKeySelector) -> Self {
        Self {
            name: value.name,
            key: value.key,
            optional: value.optional,
        }
    }
}

// FileKeySelector
impl ToInternal<internal_selector::FileKeySelector> for v1_selector::FileKeySelector {
    fn to_internal(self) -> internal_selector::FileKeySelector {
        internal_selector::FileKeySelector {
            volume_name: self.volume_name,
            path: self.path,
            key: self.key,
            optional: None,
        }
    }
}

impl FromInternal<internal_selector::FileKeySelector> for v1_selector::FileKeySelector {
    fn from_internal(value: internal_selector::FileKeySelector) -> Self {
        Self {
            volume_name: value.volume_name,
            path: value.path,
            key: value.key,
        }
    }
}
