//! Environment-related conversions for core v1 ↔ internal API.

use crate::common::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::env as v1_env;
use crate::core::v1::reference::LocalObjectReference;

impl ToInternal<internal::EnvVar> for v1_env::EnvVar {
    fn to_internal(self) -> internal::EnvVar {
        internal::EnvVar {
            name: self.name,
            value: self.value,
            value_from: self.value_from.map(ToInternal::to_internal),
        }
    }
}

impl FromInternal<internal::EnvVar> for v1_env::EnvVar {
    fn from_internal(value: internal::EnvVar) -> Self {
        Self {
            name: value.name,
            value: value.value,
            value_from: value.value_from.map(v1_env::EnvVarSource::from_internal),
        }
    }
}

impl ToInternal<internal::EnvVarSource> for v1_env::EnvVarSource {
    fn to_internal(self) -> internal::EnvVarSource {
        internal::EnvVarSource {
            field_ref: self.field_ref.map(ToInternal::to_internal),
            resource_field_ref: self.resource_field_ref.map(ToInternal::to_internal),
            config_map_key_ref: self.config_map_key_ref.map(ToInternal::to_internal),
            secret_key_ref: self.secret_key_ref.map(ToInternal::to_internal),
            file_key_ref: self.file_key_ref.map(ToInternal::to_internal),
        }
    }
}

impl FromInternal<internal::EnvVarSource> for v1_env::EnvVarSource {
    fn from_internal(value: internal::EnvVarSource) -> Self {
        Self {
            field_ref: value
                .field_ref
                .map(crate::core::v1::ObjectFieldSelector::from_internal),
            resource_field_ref: value
                .resource_field_ref
                .map(crate::core::v1::ResourceFieldSelector::from_internal),
            config_map_key_ref: value
                .config_map_key_ref
                .map(crate::core::v1::ConfigMapKeySelector::from_internal),
            secret_key_ref: value
                .secret_key_ref
                .map(crate::core::v1::SecretKeySelector::from_internal),
            file_key_ref: value
                .file_key_ref
                .map(crate::core::v1::FileKeySelector::from_internal),
        }
    }
}

impl ToInternal<internal::EnvFromSource> for v1_env::EnvFromSource {
    fn to_internal(self) -> internal::EnvFromSource {
        internal::EnvFromSource {
            prefix: self.prefix,
            config_map_ref: self.config_map_ref.map(ToInternal::to_internal),
            secret_ref: self.secret_ref.map(ToInternal::to_internal),
        }
    }
}

impl FromInternal<internal::EnvFromSource> for v1_env::EnvFromSource {
    fn from_internal(value: internal::EnvFromSource) -> Self {
        Self {
            prefix: value.prefix,
            config_map_ref: value
                .config_map_ref
                .map(v1_env::ConfigMapEnvSource::from_internal),
            secret_ref: value.secret_ref.map(v1_env::SecretEnvSource::from_internal),
        }
    }
}

impl ToInternal<internal::ConfigMapEnvSource> for v1_env::ConfigMapEnvSource {
    fn to_internal(self) -> internal::ConfigMapEnvSource {
        internal::ConfigMapEnvSource {
            local_object_reference: self.local_object_reference.to_internal(),
            optional: self.optional,
        }
    }
}

impl FromInternal<internal::ConfigMapEnvSource> for v1_env::ConfigMapEnvSource {
    fn from_internal(value: internal::ConfigMapEnvSource) -> Self {
        Self {
            local_object_reference: LocalObjectReference::from_internal(
                value.local_object_reference,
            ),
            optional: value.optional,
        }
    }
}

impl ToInternal<internal::SecretEnvSource> for v1_env::SecretEnvSource {
    fn to_internal(self) -> internal::SecretEnvSource {
        internal::SecretEnvSource {
            local_object_reference: self.local_object_reference.to_internal(),
            optional: self.optional,
        }
    }
}

impl FromInternal<internal::SecretEnvSource> for v1_env::SecretEnvSource {
    fn from_internal(value: internal::SecretEnvSource) -> Self {
        Self {
            local_object_reference: LocalObjectReference::from_internal(
                value.local_object_reference,
            ),
            optional: value.optional,
        }
    }
}
