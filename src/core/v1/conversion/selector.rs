//! Selector-related conversions for core v1 ↔ internal API.

use crate::common::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::selector as v1_selector;

impl ToInternal<internal::ObjectFieldSelector> for v1_selector::ObjectFieldSelector {
    fn to_internal(self) -> internal::ObjectFieldSelector {
        internal::ObjectFieldSelector {
            api_version: self.api_version,
            field_path: self.field_path,
        }
    }
}

impl FromInternal<internal::ObjectFieldSelector> for v1_selector::ObjectFieldSelector {
    fn from_internal(value: internal::ObjectFieldSelector) -> Self {
        Self {
            api_version: value.api_version,
            field_path: value.field_path,
        }
    }
}

impl ToInternal<internal::ResourceFieldSelector> for v1_selector::ResourceFieldSelector {
    fn to_internal(self) -> internal::ResourceFieldSelector {
        internal::ResourceFieldSelector {
            container_name: self.container_name,
            resource: self.resource,
            divisor: self.divisor,
        }
    }
}

impl FromInternal<internal::ResourceFieldSelector> for v1_selector::ResourceFieldSelector {
    fn from_internal(value: internal::ResourceFieldSelector) -> Self {
        Self {
            container_name: value.container_name,
            resource: value.resource,
            divisor: value.divisor,
        }
    }
}

impl ToInternal<internal::ConfigMapKeySelector> for v1_selector::ConfigMapKeySelector {
    fn to_internal(self) -> internal::ConfigMapKeySelector {
        internal::ConfigMapKeySelector {
            name: self.name,
            key: self.key,
            optional: self.optional,
        }
    }
}

impl FromInternal<internal::ConfigMapKeySelector> for v1_selector::ConfigMapKeySelector {
    fn from_internal(value: internal::ConfigMapKeySelector) -> Self {
        Self {
            name: value.name,
            key: value.key,
            optional: value.optional,
        }
    }
}

impl ToInternal<internal::SecretKeySelector> for v1_selector::SecretKeySelector {
    fn to_internal(self) -> internal::SecretKeySelector {
        internal::SecretKeySelector {
            name: self.name,
            key: self.key,
            optional: self.optional,
        }
    }
}

impl FromInternal<internal::SecretKeySelector> for v1_selector::SecretKeySelector {
    fn from_internal(value: internal::SecretKeySelector) -> Self {
        Self {
            name: value.name,
            key: value.key,
            optional: value.optional,
        }
    }
}

impl ToInternal<internal::FileKeySelector> for v1_selector::FileKeySelector {
    fn to_internal(self) -> internal::FileKeySelector {
        internal::FileKeySelector {
            volume_name: self.volume_name,
            path: self.path,
            key: self.key,
            optional: None,
        }
    }
}

impl FromInternal<internal::FileKeySelector> for v1_selector::FileKeySelector {
    fn from_internal(value: internal::FileKeySelector) -> Self {
        Self {
            volume_name: value.volume_name,
            path: value.path,
            key: value.key,
        }
    }
}
