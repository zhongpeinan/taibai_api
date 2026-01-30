//! Validation helpers for apiextensions API types.
//!
//! This module provides a minimal subset of validation rules aligned with upstream behavior.

use crate::apiextensions::internal;
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, not_supported, required, validate_object_meta,
    validate_object_meta_update,
};
use std::collections::BTreeSet;

/// Maximum number of selectable fields.
pub const MAX_SELECTABLE_FIELDS: usize = 8;

// ============================================================================
// Public validation entry points
// ============================================================================

/// Validates a CustomResourceDefinition.
pub fn validate_custom_resource_definition(obj: &internal::CustomResourceDefinition) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &obj.metadata,
        false,
        crate::common::validation::name_is_dns_subdomain,
        &Path::new("metadata"),
    );

    // name must be <plural>.<group>
    if let Some(name) = obj.metadata.name.as_ref() {
        let required_name = format!("{}.{}", obj.spec.names.plural, obj.spec.group);
        if name != &required_name {
            all_errs.push(invalid(
                &Path::new("metadata").child("name"),
                BadValue::String(name.clone()),
                "must be spec.names.plural + '.' + spec.group",
            ));
        }
    }

    all_errs.extend(validate_custom_resource_definition_spec(
        &obj.spec,
        &Path::new("spec"),
    ));
    all_errs.extend(validate_custom_resource_definition_status(
        &obj.status,
        &Path::new("status"),
    ));
    all_errs.extend(validate_custom_resource_definition_stored_versions(
        &obj.status.stored_versions,
        &obj.spec.versions,
        &Path::new("status").child("storedVersions"),
    ));

    all_errs
}

/// Validates a CustomResourceDefinition update.
pub fn validate_custom_resource_definition_update(
    obj: &internal::CustomResourceDefinition,
    old: &internal::CustomResourceDefinition,
) -> ErrorList {
    let mut all_errs =
        validate_object_meta_update(&obj.metadata, &old.metadata, &Path::new("metadata"));
    all_errs.extend(validate_custom_resource_definition_spec(
        &obj.spec,
        &Path::new("spec"),
    ));
    all_errs.extend(validate_custom_resource_definition_status(
        &obj.status,
        &Path::new("status"),
    ));
    all_errs.extend(validate_custom_resource_definition_stored_versions(
        &obj.status.stored_versions,
        &obj.spec.versions,
        &Path::new("status").child("storedVersions"),
    ));
    all_errs
}

/// Validates a CustomResourceDefinitionList.
pub fn validate_custom_resource_definition_list(
    obj: &internal::CustomResourceDefinitionList,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for (i, item) in obj.items.iter().enumerate() {
        all_errs.extend(
            validate_custom_resource_definition(item).with_prefix(&Path::new("items").index(i)),
        );
    }
    all_errs
}

// ============================================================================
// Spec & status validation
// ============================================================================

/// Validates CustomResourceDefinitionSpec.
pub fn validate_custom_resource_definition_spec(
    spec: &internal::CustomResourceDefinitionSpec,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if spec.group.is_empty() {
        all_errs.push(required(&fld_path.child("group"), ""));
    } else {
        for msg in crate::common::validation::is_dns1123_subdomain(&spec.group) {
            all_errs.push(invalid(
                &fld_path.child("group"),
                BadValue::String(spec.group.clone()),
                &msg,
            ));
        }
        if spec.group.split('.').count() < 2 {
            all_errs.push(invalid(
                &fld_path.child("group"),
                BadValue::String(spec.group.clone()),
                "should be a domain with at least one dot",
            ));
        }
    }

    if spec.names.plural.is_empty() {
        all_errs.push(required(&fld_path.child("names").child("plural"), ""));
    }
    if spec.names.kind.is_empty() {
        all_errs.push(required(&fld_path.child("names").child("kind"), ""));
    }

    if spec.versions.is_empty() {
        all_errs.push(required(&fld_path.child("versions"), ""));
    }

    if !spec.versions.is_empty() && !spec.version.is_empty() {
        if spec.versions[0].name != spec.version {
            all_errs.push(invalid(
                &fld_path.child("version"),
                BadValue::String(spec.version.clone()),
                "must match the first version in spec.versions",
            ));
        }
    }

    let mut names = BTreeSet::new();
    let mut storage_count = 0;
    for (i, version) in spec.versions.iter().enumerate() {
        let version_path = fld_path.child("versions").index(i);
        if version.name.is_empty() {
            all_errs.push(required(&version_path.child("name"), ""));
        } else if !names.insert(version.name.clone()) {
            all_errs.push(invalid(
                &version_path.child("name"),
                BadValue::String(version.name.clone()),
                "must be unique",
            ));
        }

        if version.storage {
            storage_count += 1;
        }

        if version.selectable_fields.len() > MAX_SELECTABLE_FIELDS {
            all_errs.push(invalid(
                &version_path.child("selectableFields"),
                BadValue::Int(version.selectable_fields.len() as i64),
                "must not exceed 8 selectable fields",
            ));
        }
    }

    if storage_count != 1 {
        all_errs.push(invalid(
            &fld_path.child("versions"),
            BadValue::Int(storage_count),
            "must have exactly one storage version",
        ));
    }

    if spec.selectable_fields.len() > MAX_SELECTABLE_FIELDS {
        all_errs.push(invalid(
            &fld_path.child("selectableFields"),
            BadValue::Int(spec.selectable_fields.len() as i64),
            "must not exceed 8 selectable fields",
        ));
    }

    if let Some(conversion) = spec.conversion.as_ref() {
        all_errs.extend(validate_custom_resource_conversion(
            conversion,
            &fld_path.child("conversion"),
        ));
    }

    all_errs
}

/// Validates CustomResourceDefinitionStatus.
pub fn validate_custom_resource_definition_status(
    _status: &internal::CustomResourceDefinitionStatus,
    _fld_path: &Path,
) -> ErrorList {
    ErrorList::new()
}

/// Validates storedVersions against spec.versions.
pub fn validate_custom_resource_definition_stored_versions(
    stored_versions: &[String],
    versions: &[internal::CustomResourceDefinitionVersion],
    fld_path: &Path,
) -> ErrorList {
    if stored_versions.is_empty() {
        let mut errs = ErrorList::new();
        errs.push(invalid(
            fld_path,
            BadValue::String("".to_string()),
            "must have at least one stored version",
        ));
        return errs;
    }

    let mut stored_map = std::collections::BTreeMap::new();
    for (i, v) in stored_versions.iter().enumerate() {
        stored_map.insert(v.clone(), i);
    }

    let mut all_errs = ErrorList::new();
    for version in versions {
        if version.storage && !stored_map.contains_key(&version.name) {
            all_errs.push(invalid(
                fld_path,
                BadValue::String(version.name.clone()),
                "must contain the storage version",
            ));
        }
        stored_map.remove(&version.name);
    }

    for (version, i) in stored_map {
        all_errs.push(invalid(
            &fld_path.index(i),
            BadValue::String(version),
            "missing from spec.versions",
        ));
    }

    all_errs
}

// ============================================================================
// Conversion validation helpers
// ============================================================================

fn validate_custom_resource_conversion(
    conversion: &internal::CustomResourceConversion,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    match conversion.strategy {
        internal::ConversionStrategyType::None => {}
        internal::ConversionStrategyType::Webhook => {
            if conversion.webhook_client_config.is_none() {
                all_errs.push(required(&fld_path.child("webhookClientConfig"), ""));
            }
            if conversion.conversion_review_versions.is_empty() {
                all_errs.push(required(&fld_path.child("conversionReviewVersions"), ""));
            }
        }
    }

    if let Some(client) = conversion.webhook_client_config.as_ref() {
        all_errs.extend(validate_webhook_client_config(
            client,
            &fld_path.child("webhookClientConfig"),
        ));
    }

    all_errs
}

fn validate_webhook_client_config(
    client: &internal::WebhookClientConfig,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let has_url = client.url.as_ref().is_some_and(|v| !v.is_empty());
    let has_service = client.service.is_some();

    if has_url == has_service {
        all_errs.push(invalid(
            fld_path,
            BadValue::String("url/service".to_string()),
            "exactly one of url or service must be set",
        ));
    }

    if let Some(service) = client.service.as_ref() {
        if service.namespace.is_empty() {
            all_errs.push(required(&fld_path.child("service").child("namespace"), ""));
        }
        if service.name.is_empty() {
            all_errs.push(required(&fld_path.child("service").child("name"), ""));
        }
        if service.port != 0 && (service.port < 1 || service.port > 65535) {
            all_errs.push(not_supported(
                &fld_path.child("service").child("port"),
                BadValue::Int(service.port as i64),
                &["1-65535"],
            ));
        }
    }

    all_errs
}

// ============================================================================
// ErrorList helper
// ============================================================================

trait ErrorListExt {
    fn with_prefix(self, prefix: &Path) -> ErrorList;
}

impl ErrorListExt for ErrorList {
    fn with_prefix(mut self, prefix: &Path) -> ErrorList {
        for err in &mut self.errors {
            if !err.field.is_empty() {
                err.field = format!("{}.{}", prefix, err.field);
            } else {
                err.field = prefix.to_string();
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ObjectMeta;

    #[test]
    fn test_validate_custom_resource_definition_basic() {
        let crd = internal::CustomResourceDefinition {
            type_meta: Default::default(),
            metadata: ObjectMeta {
                name: Some("widgets.example.com".to_string()),
                ..Default::default()
            },
            spec: internal::CustomResourceDefinitionSpec {
                group: "example.com".to_string(),
                names: internal::CustomResourceDefinitionNames {
                    plural: "widgets".to_string(),
                    kind: "Widget".to_string(),
                    ..Default::default()
                },
                versions: vec![internal::CustomResourceDefinitionVersion {
                    name: "v1".to_string(),
                    served: true,
                    storage: true,
                    ..Default::default()
                }],
                ..Default::default()
            },
            status: internal::CustomResourceDefinitionStatus {
                stored_versions: vec!["v1".to_string()],
                ..Default::default()
            },
        };

        let errs = validate_custom_resource_definition(&crd);
        assert!(
            errs.is_empty(),
            "expected no errors, got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_custom_resource_definition_missing_group() {
        let crd = internal::CustomResourceDefinition {
            type_meta: Default::default(),
            metadata: ObjectMeta {
                name: Some("widgets.example.com".to_string()),
                ..Default::default()
            },
            spec: internal::CustomResourceDefinitionSpec {
                group: "".to_string(),
                names: internal::CustomResourceDefinitionNames {
                    plural: "widgets".to_string(),
                    kind: "Widget".to_string(),
                    ..Default::default()
                },
                versions: vec![internal::CustomResourceDefinitionVersion {
                    name: "v1".to_string(),
                    served: true,
                    storage: true,
                    ..Default::default()
                }],
                ..Default::default()
            },
            status: internal::CustomResourceDefinitionStatus {
                stored_versions: vec!["v1".to_string()],
                ..Default::default()
            },
        };

        let errs = validate_custom_resource_definition(&crd);
        assert!(!errs.is_empty());
    }
}
