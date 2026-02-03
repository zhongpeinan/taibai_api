//! Environment validation for internal core API types.

use crate::common::validation::{BadValue, ErrorList, Path, invalid, required};
use crate::core::internal::validation::selector::{
    is_valid_env_var_name, validate_config_map_key_selector,
    validate_container_resource_field_selector, validate_file_key_selector,
    validate_object_field_selector, validate_secret_key_selector,
};
use crate::core::internal::{ConfigMapEnvSource, EnvFromSource, EnvVar, SecretEnvSource};

// ============================================================================
// Environment Variable Validation
// ============================================================================

/// Validates a list of environment variables.
pub fn validate_env(vars: &[EnvVar], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, ev) in vars.iter().enumerate() {
        let idx_path = path.index(i);

        if ev.name.is_empty() {
            all_errs.push(required(&idx_path.child("name"), "name is required"));
        } else if !is_valid_env_var_name(&ev.name) {
            all_errs.push(invalid(
                &idx_path.child("name"),
                BadValue::String(ev.name.clone()),
                "invalid environment variable name",
            ));
        }

        all_errs.extend(validate_env_var_value_from(
            ev,
            &idx_path.child("valueFrom"),
        ));
    }

    all_errs
}

/// Validates an EnvVarSource.
fn validate_env_var_value_from(ev: &EnvVar, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let Some(ref value_from) = ev.value_from else {
        return all_errs;
    };

    let mut num_sources = 0;

    if let Some(ref field_ref) = value_from.field_ref {
        num_sources += 1;
        all_errs.extend(validate_object_field_selector(
            field_ref,
            &path.child("fieldRef"),
        ));
    }

    if let Some(ref resource_field_ref) = value_from.resource_field_ref {
        num_sources += 1;
        all_errs.extend(validate_container_resource_field_selector(
            resource_field_ref,
            &path.child("resourceFieldRef"),
            false,
        ));
    }

    if let Some(ref config_map_key_ref) = value_from.config_map_key_ref {
        num_sources += 1;
        all_errs.extend(validate_config_map_key_selector(
            config_map_key_ref,
            &path.child("configMapKeyRef"),
        ));
    }

    if let Some(ref secret_key_ref) = value_from.secret_key_ref {
        num_sources += 1;
        all_errs.extend(validate_secret_key_selector(
            secret_key_ref,
            &path.child("secretKeyRef"),
        ));
    }

    if let Some(ref file_key_ref) = value_from.file_key_ref {
        num_sources += 1;
        all_errs.extend(validate_file_key_selector(
            file_key_ref,
            &path.child("fileKeyRef"),
        ));
    }

    if num_sources == 0 {
        all_errs.push(invalid(
            path,
            BadValue::String(String::new()),
            "must specify one of: fieldRef, resourceFieldRef, configMapKeyRef, secretKeyRef, or fileKeyRef",
        ));
    } else if !ev.value.is_empty() && num_sources > 0 {
        all_errs.push(invalid(
            path,
            BadValue::String(String::new()),
            "may not be specified when value is not empty",
        ));
    } else if num_sources > 1 {
        all_errs.push(invalid(
            path,
            BadValue::String(String::new()),
            "may not have more than one field specified at a time",
        ));
    }

    all_errs
}

// ============================================================================
// EnvFrom Validation
// ============================================================================

/// Validates a list of EnvFromSource entries.
pub fn validate_env_from(vars: &[EnvFromSource], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, ev) in vars.iter().enumerate() {
        let idx_path = path.index(i);

        if !ev.prefix.is_empty() && !is_valid_env_var_name(&ev.prefix) {
            all_errs.push(invalid(
                &idx_path.child("prefix"),
                BadValue::String(ev.prefix.clone()),
                "invalid environment variable prefix",
            ));
        }

        let mut num_sources = 0;

        if let Some(ref config_map_ref) = ev.config_map_ref {
            num_sources += 1;
            all_errs.extend(validate_config_map_env_source(
                config_map_ref,
                &idx_path.child("configMapRef"),
            ));
        }

        if let Some(ref secret_ref) = ev.secret_ref {
            num_sources += 1;
            all_errs.extend(validate_secret_env_source(
                secret_ref,
                &idx_path.child("secretRef"),
            ));
        }

        if num_sources == 0 {
            all_errs.push(invalid(
                &idx_path,
                BadValue::String(String::new()),
                "must specify one of: configMapRef or secretRef",
            ));
        } else if num_sources > 1 {
            all_errs.push(invalid(
                &idx_path,
                BadValue::String(String::new()),
                "may not have more than one field specified at a time",
            ));
        }
    }

    all_errs
}

// ============================================================================
// Source Validation Functions
// ============================================================================

fn validate_config_map_env_source(source: &ConfigMapEnvSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref name) = source.local_object_reference.name {
        if name.is_empty() {
            all_errs.push(required(&path.child("name"), "name is required"));
        } else if !crate::common::validation::is_dns1123_subdomain(name).is_empty() {
            all_errs.push(invalid(
                &path.child("name"),
                BadValue::String(name.clone()),
                "must be a valid DNS subdomain",
            ));
        }
    } else {
        all_errs.push(required(&path.child("name"), "name is required"));
    }

    all_errs
}

fn validate_secret_env_source(source: &SecretEnvSource, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(ref name) = source.local_object_reference.name {
        if name.is_empty() {
            all_errs.push(required(&path.child("name"), "name is required"));
        } else if !crate::common::validation::is_dns1123_subdomain(name).is_empty() {
            all_errs.push(invalid(
                &path.child("name"),
                BadValue::String(name.clone()),
                "must be a valid DNS subdomain",
            ));
        }
    } else {
        all_errs.push(required(&path.child("name"), "name is required"));
    }

    all_errs
}
