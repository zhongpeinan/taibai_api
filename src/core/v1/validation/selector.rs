//! Selector validation for Kubernetes core/v1 API.
//!
//! Delegates to internal validation for consistency.

use crate::common::validation::{ErrorList, Path};
use crate::core::internal::selector as internal_selector;
use crate::core::internal::validation::selector as internal_selector_validation;
use crate::core::v1::selector::{
    ConfigMapKeySelector, FileKeySelector, ObjectFieldSelector, ResourceFieldSelector,
    SecretKeySelector,
};

#[allow(dead_code)]
pub(crate) fn validate_object_field_selector(
    selector: &ObjectFieldSelector,
    path: &Path,
) -> ErrorList {
    let internal_selector = internal_selector::ObjectFieldSelector {
        api_version: selector.api_version.clone(),
        field_path: selector.field_path.clone(),
    };
    internal_selector_validation::validate_object_field_selector(&internal_selector, path)
}

#[allow(dead_code)]
pub(crate) fn validate_container_resource_field_selector(
    selector: &ResourceFieldSelector,
    path: &Path,
    volume: bool,
) -> ErrorList {
    let internal_selector = internal_selector::ResourceFieldSelector {
        container_name: selector.container_name.clone(),
        resource: selector.resource.clone(),
        divisor: selector.divisor.clone(),
    };
    internal_selector_validation::validate_container_resource_field_selector(
        &internal_selector,
        path,
        volume,
    )
}

#[allow(dead_code)]
pub(crate) fn validate_config_map_key_selector(
    selector: &ConfigMapKeySelector,
    path: &Path,
) -> ErrorList {
    let internal_selector = internal_selector::ConfigMapKeySelector {
        name: selector.name.clone(),
        key: selector.key.clone(),
        optional: selector.optional,
    };
    internal_selector_validation::validate_config_map_key_selector(&internal_selector, path)
}

#[allow(dead_code)]
pub(crate) fn validate_secret_key_selector(selector: &SecretKeySelector, path: &Path) -> ErrorList {
    let internal_selector = internal_selector::SecretKeySelector {
        name: selector.name.clone(),
        key: selector.key.clone(),
        optional: selector.optional,
    };
    internal_selector_validation::validate_secret_key_selector(&internal_selector, path)
}

#[allow(dead_code)]
pub(crate) fn validate_file_key_selector(selector: &FileKeySelector, path: &Path) -> ErrorList {
    let internal_selector = internal_selector::FileKeySelector {
        volume_name: selector.volume_name.clone(),
        path: selector.path.clone(),
        key: selector.key.clone(),
        optional: None,
    };
    internal_selector_validation::validate_file_key_selector(&internal_selector, path)
}

#[allow(dead_code)]
pub(crate) fn is_valid_env_var_name(name: &str) -> bool {
    internal_selector_validation::is_valid_env_var_name(name)
}

#[cfg(test)]
pub(crate) fn is_valid_config_map_key(key: &str) -> bool {
    internal_selector_validation::is_valid_config_map_key(key)
}
