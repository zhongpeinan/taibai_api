//! Validation for Kubernetes API Discovery API types
//!
//! Note: Kubernetes upstream does not validate apidiscovery types.
//! This module provides minimal validation for required fields (non-empty checks).

use crate::apidiscovery::v2::{
    APIGroupDiscovery, APIGroupDiscoveryList, APIResourceDiscovery, APISubresourceDiscovery,
    APIVersionDiscovery,
};
use crate::common::validation::{ErrorList, Path, required};

// ============================================================================
// APIGroupDiscovery Validation
// ============================================================================

pub fn validate_api_group_discovery(obj: &APIGroupDiscovery) -> ErrorList {
    validate_api_group_discovery_with_path(obj, &Path::nil())
}

fn validate_api_group_discovery_with_path(obj: &APIGroupDiscovery, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate versions is non-empty
    if obj.versions.is_empty() {
        all_errs.push(required(&base_path.child("versions"), ""));
    } else {
        for (i, version) in obj.versions.iter().enumerate() {
            all_errs.extend(validate_api_version_discovery(
                version,
                &base_path.child("versions").index(i),
            ));
        }
    }

    all_errs
}

pub fn validate_api_group_discovery_list(obj: &APIGroupDiscoveryList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in obj.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_api_group_discovery_with_path(item, &item_path));
    }

    all_errs
}

// ============================================================================
// APIVersionDiscovery Validation
// ============================================================================

fn validate_api_version_discovery(obj: &APIVersionDiscovery, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Version must be non-empty
    if obj.version.is_empty() {
        all_errs.push(required(&fld_path.child("version"), ""));
    }

    // Resources must be non-empty
    if obj.resources.is_empty() {
        all_errs.push(required(&fld_path.child("resources"), ""));
    } else {
        for (i, resource) in obj.resources.iter().enumerate() {
            all_errs.extend(validate_api_resource_discovery(
                resource,
                &fld_path.child("resources").index(i),
            ));
        }
    }

    all_errs
}

// ============================================================================
// APIResourceDiscovery Validation
// ============================================================================

fn validate_api_resource_discovery(obj: &APIResourceDiscovery, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Resource must be non-empty
    if obj.resource.is_empty() {
        all_errs.push(required(&fld_path.child("resource"), ""));
    }

    // Verbs must be non-empty
    if obj.verbs.is_empty() {
        all_errs.push(required(&fld_path.child("verbs"), ""));
    }

    // Validate subresources if present
    for (i, subresource) in obj.subresources.iter().enumerate() {
        all_errs.extend(validate_api_subresource_discovery(
            subresource,
            &fld_path.child("subresources").index(i),
        ));
    }

    all_errs
}

// ============================================================================
// APISubresourceDiscovery Validation
// ============================================================================

fn validate_api_subresource_discovery(obj: &APISubresourceDiscovery, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Subresource must be non-empty
    if obj.subresource.is_empty() {
        all_errs.push(required(&fld_path.child("subresource"), ""));
    }

    // Verbs must be non-empty
    if obj.verbs.is_empty() {
        all_errs.push(required(&fld_path.child("verbs"), ""));
    }

    all_errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apidiscovery::internal::{DiscoveryFreshness, ResourceScope};
    use crate::common::{GroupVersionKind, TypeMeta};

    #[test]
    fn test_validate_api_group_discovery_valid() {
        let obj = APIGroupDiscovery {
            type_meta: TypeMeta {
                api_version: "apidiscovery.k8s.io/v2".to_string(),
                kind: "APIGroupDiscovery".to_string(),
            },
            metadata: None,
            versions: vec![APIVersionDiscovery {
                version: "v1".to_string(),
                resources: vec![APIResourceDiscovery {
                    resource: "pods".to_string(),
                    response_kind: Some(GroupVersionKind {
                        group: "".to_string(),
                        version: "v1".to_string(),
                        kind: "Pod".to_string(),
                    }),
                    scope: ResourceScope::Namespaced,
                    singular_resource: "pod".to_string(),
                    verbs: vec!["get".to_string(), "list".to_string()],
                    short_names: vec!["po".to_string()],
                    categories: vec![],
                    subresources: vec![],
                }],
                freshness: Some(DiscoveryFreshness::Current),
            }],
        };

        let errs = validate_api_group_discovery(&obj);
        assert!(
            errs.is_empty(),
            "expected no errors, got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_api_group_discovery_empty_versions() {
        let obj = APIGroupDiscovery {
            type_meta: TypeMeta {
                api_version: "apidiscovery.k8s.io/v2".to_string(),
                kind: "APIGroupDiscovery".to_string(),
            },
            metadata: None,
            versions: vec![],
        };

        let errs = validate_api_group_discovery(&obj);
        assert!(!errs.is_empty());
        // The error field should contain "versions"
        assert!(
            errs.errors.iter().any(|e| e.field.contains("versions")),
            "Expected 'versions', got: {:?}",
            errs.errors
        );
    }

    #[test]
    fn test_validate_api_group_discovery_list_item_index() {
        // Test that errors from list items include the item index in the path
        let obj = APIGroupDiscoveryList {
            type_meta: TypeMeta {
                api_version: "apidiscovery.k8s.io/v2".to_string(),
                kind: "APIGroupDiscoveryList".to_string(),
            },
            metadata: None,
            items: vec![APIGroupDiscovery {
                type_meta: TypeMeta {
                    api_version: "apidiscovery.k8s.io/v2".to_string(),
                    kind: "APIGroupDiscovery".to_string(),
                },
                metadata: None,
                versions: vec![],
            }],
        };

        let errs = validate_api_group_discovery_list(&obj);
        assert!(!errs.is_empty());
        // The error should reference items[0].versions, not just versions
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("items[0].versions"))
        );
    }

    #[test]
    fn test_validate_api_resource_discovery_empty_verbs() {
        let obj = APIResourceDiscovery {
            resource: "pods".to_string(),
            response_kind: None,
            scope: ResourceScope::Namespaced,
            singular_resource: "pod".to_string(),
            verbs: vec![],
            short_names: vec![],
            categories: vec![],
            subresources: vec![],
        };

        let errs = validate_api_resource_discovery(&obj, &Path::new("resources").index(0));
        assert!(!errs.is_empty());
        assert!(errs.errors.iter().any(|e| e.field.contains("verbs")));
    }

    #[test]
    fn test_validate_api_resource_discovery_empty_resource() {
        let obj = APIResourceDiscovery {
            resource: "".to_string(),
            response_kind: None,
            scope: ResourceScope::Namespaced,
            singular_resource: "pod".to_string(),
            verbs: vec!["get".to_string()],
            short_names: vec![],
            categories: vec![],
            subresources: vec![],
        };

        let errs = validate_api_resource_discovery(&obj, &Path::new("resources").index(0));
        assert!(!errs.is_empty());
        assert!(errs.errors.iter().any(|e| e.field.contains("resource")));
    }
}
