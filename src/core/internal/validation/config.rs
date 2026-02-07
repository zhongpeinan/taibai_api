//! ConfigMap, Secret, and ServiceAccount validation for Kubernetes core internal API
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use super::helpers::is_config_map_key;
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, name_is_dns_subdomain, required,
    validate_object_meta, validate_object_meta_update,
};
use crate::core::internal::SecretType;
use crate::core::internal::config::{ConfigMap, Secret, ServiceAccount};

// ============================================================================
// Constants
// ============================================================================

/// Maximum total size for ConfigMap or Secret data (1MB)
const MAX_DATA_SIZE: usize = 1048576;

/// Secret data key constants
mod secret_data_key {
    /// Docker config key for dockercfg type
    pub const DOCKER_CONFIG: &str = ".dockercfg";

    /// Docker config JSON key for dockerconfigjson type
    pub const DOCKER_CONFIG_JSON: &str = ".dockerconfigjson";

    /// Basic auth username key
    pub const BASIC_AUTH_USERNAME: &str = "username";

    /// Basic auth password key
    pub const BASIC_AUTH_PASSWORD: &str = "password";

    /// SSH private key
    pub const SSH_PRIVATE_KEY: &str = "ssh-privatekey";

    /// TLS certificate key
    pub const TLS_CERT: &str = "tls.crt";

    /// TLS private key
    pub const TLS_KEY: &str = "tls.key";
}

/// Service account annotation key for service account token secrets
const SERVICE_ACCOUNT_NAME_KEY: &str = "kubernetes.io/service-account.name";

// ============================================================================
// ConfigMap Validation
// ============================================================================

/// Validates a ConfigMap.
///
/// Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go `ValidateConfigMap`
pub fn validate_config_map(config_map: &ConfigMap) -> ErrorList {
    validate_config_map_with_path(config_map, &Path::nil())
}

fn validate_config_map_with_path(config_map: &ConfigMap, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (ConfigMap is namespaced)
    all_errs.extend(validate_object_meta(
        &config_map.metadata,
        true,
        name_is_dns_subdomain,
        &path.child("metadata"),
    ));

    // Validate data keys and check for duplicates
    let mut total_size = 0;
    for (key, value) in &config_map.data {
        for err in is_config_map_key(key) {
            all_errs.push(invalid(
                &path.child("data").key(key),
                BadValue::String(key.clone()),
                &err,
            ));
        }

        // Check for duplicate in binaryData
        if config_map.binary_data.contains_key(key) {
            all_errs.push(invalid(
                &path.child("data").key(key),
                BadValue::String(key.clone()),
                "duplicate of key present in binaryData",
            ));
        }

        total_size += value.len();
    }

    // Validate binaryData keys
    for (key, value) in &config_map.binary_data {
        for err in is_config_map_key(key) {
            all_errs.push(invalid(
                &path.child("binaryData").key(key),
                BadValue::String(key.clone()),
                &err,
            ));
        }

        total_size += value.0.len();
    }

    // Check total size limit
    if total_size > MAX_DATA_SIZE {
        all_errs.push(crate::common::validation::too_long(path, MAX_DATA_SIZE));
    }

    all_errs
}

/// Validates a ConfigMap update.
pub fn validate_config_map_update(new: &ConfigMap, old: &ConfigMap) -> ErrorList {
    validate_config_map_update_with_path(new, old, &Path::nil())
}

fn validate_config_map_update_with_path(
    new: &ConfigMap,
    old: &ConfigMap,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata update
    all_errs.extend(validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &path.child("metadata"),
    ));

    // Check immutability
    if let Some(true) = old.immutable {
        if new.immutable != Some(true) {
            all_errs.push(forbidden(
                &path.child("immutable"),
                "field is immutable when `immutable` is set",
            ));
        }

        if new.data != old.data {
            all_errs.push(forbidden(
                &path.child("data"),
                "field is immutable when `immutable` is set",
            ));
        }

        if new.binary_data != old.binary_data {
            all_errs.push(forbidden(
                &path.child("binaryData"),
                "field is immutable when `immutable` is set",
            ));
        }
    }

    // Validate the new ConfigMap
    all_errs.extend(validate_config_map_with_path(new, path));

    all_errs
}

// ============================================================================
// Secret Validation
// ============================================================================

/// Validates a Secret.
///
/// Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go `ValidateSecret`
pub fn validate_secret(secret: &Secret) -> ErrorList {
    validate_secret_with_path(secret, &Path::nil())
}

fn validate_secret_with_path(secret: &Secret, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (Secret is namespaced)
    all_errs.extend(validate_object_meta(
        &secret.metadata,
        true,
        name_is_dns_subdomain,
        &path.child("metadata"),
    ));

    // Validate data keys and check total size
    let data_path = path.child("data");
    let mut total_size = 0;
    for (key, value) in &secret.data {
        for err in is_config_map_key(key) {
            all_errs.push(invalid(
                &data_path.key(key),
                BadValue::String(key.clone()),
                &err,
            ));
        }

        total_size += value.0.len();
    }

    if total_size > MAX_DATA_SIZE {
        all_errs.push(crate::common::validation::too_long(
            &data_path,
            MAX_DATA_SIZE,
        ));
    }

    // Type-specific validation
    match secret.r#type {
        SecretType::Opaque => {
            // No specific validation for opaque secrets
        }
        SecretType::ServiceAccountToken => {
            if secret
                .metadata
                .annotations
                .get(SERVICE_ACCOUNT_NAME_KEY)
                .map_or(true, |v| v.is_empty())
            {
                all_errs.push(required(
                    &path
                        .child("metadata")
                        .child("annotations")
                        .key(SERVICE_ACCOUNT_NAME_KEY),
                    "",
                ));
            }
        }
        SecretType::Dockercfg => {
            if !secret.data.contains_key(secret_data_key::DOCKER_CONFIG) {
                all_errs.push(required(&data_path.key(secret_data_key::DOCKER_CONFIG), ""));
            } else if let Some(docker_cfg_bytes) = secret.data.get(secret_data_key::DOCKER_CONFIG) {
                if let Err(e) =
                    serde_json::from_slice::<serde_json::Value>(docker_cfg_bytes.as_ref())
                {
                    all_errs.push(invalid(
                        &data_path.key(secret_data_key::DOCKER_CONFIG),
                        BadValue::String("<secret contents redacted>".to_string()),
                        &e.to_string(),
                    ));
                }
            }
        }
        SecretType::DockerConfigJson => {
            if !secret
                .data
                .contains_key(secret_data_key::DOCKER_CONFIG_JSON)
            {
                all_errs.push(required(
                    &data_path.key(secret_data_key::DOCKER_CONFIG_JSON),
                    "",
                ));
            } else if let Some(docker_cfg_json_bytes) =
                secret.data.get(secret_data_key::DOCKER_CONFIG_JSON)
            {
                if let Err(e) =
                    serde_json::from_slice::<serde_json::Value>(docker_cfg_json_bytes.as_ref())
                {
                    all_errs.push(invalid(
                        &data_path.key(secret_data_key::DOCKER_CONFIG_JSON),
                        BadValue::String("<secret contents redacted>".to_string()),
                        &e.to_string(),
                    ));
                }
            }
        }
        SecretType::BasicAuth => {
            let has_username = secret
                .data
                .contains_key(secret_data_key::BASIC_AUTH_USERNAME);
            let has_password = secret
                .data
                .contains_key(secret_data_key::BASIC_AUTH_PASSWORD);

            if !has_username && !has_password {
                all_errs.push(required(
                    &data_path.key(secret_data_key::BASIC_AUTH_USERNAME),
                    "",
                ));
                all_errs.push(required(
                    &data_path.key(secret_data_key::BASIC_AUTH_PASSWORD),
                    "",
                ));
            }
        }
        SecretType::SshAuth => {
            if let Some(key_data) = secret.data.get(secret_data_key::SSH_PRIVATE_KEY) {
                if key_data.0.is_empty() {
                    all_errs.push(required(
                        &data_path.key(secret_data_key::SSH_PRIVATE_KEY),
                        "",
                    ));
                }
            } else {
                all_errs.push(required(
                    &data_path.key(secret_data_key::SSH_PRIVATE_KEY),
                    "",
                ));
            }
        }
        SecretType::Tls => {
            if !secret.data.contains_key(secret_data_key::TLS_CERT) {
                all_errs.push(required(&data_path.key(secret_data_key::TLS_CERT), ""));
            }
            if !secret.data.contains_key(secret_data_key::TLS_KEY) {
                all_errs.push(required(&data_path.key(secret_data_key::TLS_KEY), ""));
            }
        }
        SecretType::BootstrapToken => {
            // No specific validation for bootstrap token secrets beyond type
        }
    }

    all_errs
}

/// Validates a Secret update.
pub fn validate_secret_update(new: &Secret, old: &Secret) -> ErrorList {
    validate_secret_update_with_path(new, old, &Path::nil())
}

fn validate_secret_update_with_path(new: &Secret, old: &Secret, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata update
    all_errs.extend(validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &path.child("metadata"),
    ));

    // Type is immutable
    if new.r#type != old.r#type {
        all_errs.push(forbidden(&path.child("type"), "field is immutable"));
    }

    // Check immutability
    if let Some(true) = old.immutable {
        if new.immutable != Some(true) {
            all_errs.push(forbidden(
                &path.child("immutable"),
                "field is immutable when `immutable` is set",
            ));
        }

        if new.data != old.data {
            all_errs.push(forbidden(
                &path.child("data"),
                "field is immutable when `immutable` is set",
            ));
        }
    }

    // Validate the new Secret
    all_errs.extend(validate_secret_with_path(new, path));

    all_errs
}

// ============================================================================
// ServiceAccount Validation
// ============================================================================

/// Validates a ServiceAccount.
///
/// Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go `ValidateServiceAccount`
pub fn validate_service_account(service_account: &ServiceAccount) -> ErrorList {
    validate_service_account_with_path(service_account, &Path::nil())
}

fn validate_service_account_with_path(service_account: &ServiceAccount, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata (ServiceAccount is namespaced)
    all_errs.extend(validate_object_meta(
        &service_account.metadata,
        true,
        name_is_dns_subdomain,
        &path.child("metadata"),
    ));

    all_errs
}

/// Validates a ServiceAccount update.
pub fn validate_service_account_update(new: &ServiceAccount, old: &ServiceAccount) -> ErrorList {
    validate_service_account_update_with_path(new, old, &Path::nil())
}

fn validate_service_account_update_with_path(
    new: &ServiceAccount,
    old: &ServiceAccount,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate metadata update
    all_errs.extend(validate_object_meta_update(
        &new.metadata,
        &old.metadata,
        &path.child("metadata"),
    ));

    // Validate the new ServiceAccount
    all_errs.extend(validate_service_account_with_path(new, path));

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, TypeMeta};
    use crate::core::internal::helper::ByteString;
    use std::collections::BTreeMap;

    fn create_test_config_map(name: &str) -> ConfigMap {
        ConfigMap {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some(name.to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            immutable: None,
            data: BTreeMap::new(),
            binary_data: BTreeMap::new(),
        }
    }

    fn create_test_secret(name: &str) -> Secret {
        Secret {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some(name.to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            immutable: None,
            data: BTreeMap::new(),
            string_data: BTreeMap::new(),
            r#type: SecretType::Opaque,
        }
    }

    fn create_test_service_account(name: &str) -> ServiceAccount {
        ServiceAccount {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some(name.to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            secrets: vec![],
            image_pull_secrets: vec![],
            automount_service_account_token: None,
        }
    }

    // ConfigMap tests
    #[test]
    fn test_validate_config_map_valid() {
        let mut config_map = create_test_config_map("test-config");
        config_map
            .data
            .insert("key1".to_string(), "value1".to_string());

        let errs = validate_config_map(&config_map);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_config_map_invalid_key() {
        let mut config_map = create_test_config_map("test-config");
        config_map
            .data
            .insert("INVALID KEY!".to_string(), "value1".to_string());

        let errs = validate_config_map(&config_map);
        assert!(!errs.is_empty(), "Expected errors for invalid key");
    }

    #[test]
    fn test_validate_config_map_duplicate_keys() {
        let mut config_map = create_test_config_map("test-config");
        config_map
            .data
            .insert("key1".to_string(), "value1".to_string());
        config_map
            .binary_data
            .insert("key1".to_string(), ByteString(vec![1, 2, 3]));

        let errs = validate_config_map(&config_map);
        assert!(
            !errs.is_empty(),
            "Expected errors for duplicate keys between data and binaryData"
        );
    }

    #[test]
    fn test_validate_config_map_immutable_cannot_change_data() {
        let mut old_config = create_test_config_map("test-config");
        old_config.immutable = Some(true);
        old_config
            .data
            .insert("key1".to_string(), "value1".to_string());
        old_config.metadata.resource_version = Some("123".to_string());

        let mut new_config = create_test_config_map("test-config");
        new_config.immutable = Some(true);
        new_config
            .data
            .insert("key1".to_string(), "value2".to_string());
        new_config.metadata.resource_version = Some("123".to_string());

        let errs = validate_config_map_update(&new_config, &old_config);
        let data_errs: Vec<_> = errs
            .errors
            .iter()
            .filter(|e| e.field.contains("data"))
            .collect();
        assert!(
            !data_errs.is_empty(),
            "Expected errors for changing immutable data"
        );
    }

    // Secret tests
    #[test]
    fn test_validate_secret_valid_opaque() {
        let mut secret = create_test_secret("test-secret");
        secret
            .data
            .insert("password".to_string(), ByteString(vec![1, 2, 3]));

        let errs = validate_secret(&secret);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_secret_tls_requires_keys() {
        let mut secret = create_test_secret("test-tls-secret");
        secret.r#type = SecretType::Tls;

        let errs = validate_secret(&secret);
        assert!(!errs.is_empty(), "Expected errors for missing TLS keys");
    }

    #[test]
    fn test_validate_secret_tls_valid() {
        let mut secret = create_test_secret("test-tls-secret");
        secret.r#type = SecretType::Tls;
        secret.data.insert(
            secret_data_key::TLS_CERT.to_string(),
            ByteString(vec![1, 2, 3]),
        );
        secret.data.insert(
            secret_data_key::TLS_KEY.to_string(),
            ByteString(vec![4, 5, 6]),
        );

        let errs = validate_secret(&secret);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_secret_basic_auth_requires_username_or_password() {
        let mut secret = create_test_secret("test-basic-auth-secret");
        secret.r#type = SecretType::BasicAuth;

        let errs = validate_secret(&secret);
        assert!(
            !errs.is_empty(),
            "Expected errors for missing username and password"
        );
    }

    #[test]
    fn test_validate_secret_basic_auth_valid_with_username() {
        let mut secret = create_test_secret("test-basic-auth-secret");
        secret.r#type = SecretType::BasicAuth;
        secret.data.insert(
            secret_data_key::BASIC_AUTH_USERNAME.to_string(),
            ByteString(vec![1, 2]),
        );

        let errs = validate_secret(&secret);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_secret_ssh_auth_requires_key() {
        let mut secret = create_test_secret("test-ssh-secret");
        secret.r#type = SecretType::SshAuth;

        let errs = validate_secret(&secret);
        assert!(!errs.is_empty(), "Expected errors for missing SSH key");
    }

    #[test]
    fn test_validate_secret_type_immutable() {
        let mut old_secret = create_test_secret("test-secret");
        old_secret.r#type = SecretType::Opaque;
        old_secret.metadata.resource_version = Some("123".to_string());

        let mut new_secret = create_test_secret("test-secret");
        new_secret.r#type = SecretType::Tls;
        new_secret.metadata.resource_version = Some("123".to_string());

        let errs = validate_secret_update(&new_secret, &old_secret);
        let type_errs: Vec<_> = errs
            .errors
            .iter()
            .filter(|e| e.field.contains("type"))
            .collect();
        assert!(
            !type_errs.is_empty(),
            "Expected errors for changing secret type"
        );
    }

    #[test]
    fn test_validate_secret_service_account_token_requires_annotation() {
        let mut secret = create_test_secret("test-sa-token");
        secret.r#type = SecretType::ServiceAccountToken;

        let errs = validate_secret(&secret);
        assert!(
            !errs.is_empty(),
            "Expected errors for missing service account name annotation"
        );
    }

    #[test]
    fn test_validate_secret_dockercfg_requires_data() {
        let mut secret = create_test_secret("test-dockercfg");
        secret.r#type = SecretType::Dockercfg;

        let errs = validate_secret(&secret);
        assert!(
            !errs.is_empty(),
            "Expected errors for missing .dockercfg data"
        );
    }

    // ServiceAccount tests
    #[test]
    fn test_validate_service_account_valid() {
        let sa = create_test_service_account("test-sa");

        let errs = validate_service_account(&sa);
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_service_account_invalid_name() {
        let sa = create_test_service_account("INVALID_NAME");

        let errs = validate_service_account(&sa);
        assert!(!errs.is_empty(), "Expected errors for invalid name");
    }
}
