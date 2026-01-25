//! Phase 1 conversions: Meta + Reference + Resource
//!
//! ObjectReference, LocalObjectReference, TypedLocalObjectReference, ResourceRequirements

use crate::common::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::{LocalObjectReference, ObjectReference, TypedLocalObjectReference, resource};

// ============================================================================
// ObjectReference
// ============================================================================

impl ToInternal<internal::ObjectReference> for ObjectReference {
    fn to_internal(self) -> internal::ObjectReference {
        internal::ObjectReference {
            kind: self.kind.unwrap_or_default(),
            namespace: self.namespace.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            uid: self.uid.unwrap_or_default(),
            api_version: self.api_version.unwrap_or_default(),
            resource_version: self.resource_version.unwrap_or_default(),
            field_path: self.field_path.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::ObjectReference> for ObjectReference {
    fn from_internal(value: internal::ObjectReference) -> Self {
        Self {
            kind: if value.kind.is_empty() {
                None
            } else {
                Some(value.kind)
            },
            namespace: if value.namespace.is_empty() {
                None
            } else {
                Some(value.namespace)
            },
            name: if value.name.is_empty() {
                None
            } else {
                Some(value.name)
            },
            uid: if value.uid.is_empty() {
                None
            } else {
                Some(value.uid)
            },
            api_version: if value.api_version.is_empty() {
                None
            } else {
                Some(value.api_version)
            },
            resource_version: if value.resource_version.is_empty() {
                None
            } else {
                Some(value.resource_version)
            },
            field_path: if value.field_path.is_empty() {
                None
            } else {
                Some(value.field_path)
            },
        }
    }
}

// ============================================================================
// LocalObjectReference
// ============================================================================

impl ToInternal<internal::LocalObjectReference> for LocalObjectReference {
    fn to_internal(self) -> internal::LocalObjectReference {
        internal::LocalObjectReference { name: self.name }
    }
}

impl FromInternal<internal::LocalObjectReference> for LocalObjectReference {
    fn from_internal(value: internal::LocalObjectReference) -> Self {
        Self { name: value.name }
    }
}

// ============================================================================
// TypedLocalObjectReference
// ============================================================================

impl ToInternal<internal::TypedLocalObjectReference> for TypedLocalObjectReference {
    fn to_internal(self) -> internal::TypedLocalObjectReference {
        internal::TypedLocalObjectReference {
            api_group: self.api_group,
            kind: self.kind.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            namespace: None, // v1 doesn't have namespace, internal does
        }
    }
}

impl FromInternal<internal::TypedLocalObjectReference> for TypedLocalObjectReference {
    fn from_internal(value: internal::TypedLocalObjectReference) -> Self {
        Self {
            api_group: value.api_group,
            kind: if value.kind.is_empty() {
                None
            } else {
                Some(value.kind)
            },
            name: if value.name.is_empty() {
                None
            } else {
                Some(value.name)
            },
            // v1 doesn't have namespace field, so it's dropped
        }
    }
}

// ============================================================================
// ResourceRequirements
// ============================================================================

// Note: v1 ResourceRequirements has a 'claims' field that internal doesn't have.
// We drop it during conversion to internal and add an empty vec when converting back.

impl ToInternal<internal::ResourceRequirements> for resource::ResourceRequirements {
    fn to_internal(self) -> internal::ResourceRequirements {
        internal::ResourceRequirements {
            limits: self.limits,
            requests: self.requests,
            // claims field is dropped - not present in internal
        }
    }
}

impl FromInternal<internal::ResourceRequirements> for resource::ResourceRequirements {
    fn from_internal(value: internal::ResourceRequirements) -> Self {
        Self {
            limits: value.limits,
            requests: value.requests,
            claims: Vec::new(), // v1 has claims, internal doesn't
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::util::Quantity;
    use std::collections::BTreeMap;

    #[test]
    fn test_object_reference_roundtrip() {
        let v1_ref = ObjectReference {
            kind: Some("Pod".to_string()),
            namespace: Some("default".to_string()),
            name: Some("my-pod".to_string()),
            uid: Some("abc-123".to_string()),
            api_version: Some("v1".to_string()),
            resource_version: Some("12345".to_string()),
            field_path: Some("spec.containers[0]".to_string()),
        };

        let internal_ref = v1_ref.clone().to_internal();
        let roundtrip = ObjectReference::from_internal(internal_ref);

        assert_eq!(v1_ref, roundtrip);
    }

    #[test]
    fn test_local_object_reference_roundtrip() {
        let v1_ref = LocalObjectReference {
            name: Some("my-secret".to_string()),
        };

        let internal_ref = v1_ref.clone().to_internal();
        let roundtrip = LocalObjectReference::from_internal(internal_ref);

        assert_eq!(v1_ref, roundtrip);
    }

    #[test]
    fn test_typed_local_object_reference_roundtrip() {
        let v1_ref = TypedLocalObjectReference {
            api_group: Some("apps".to_string()),
            kind: Some("Deployment".to_string()),
            name: Some("my-deployment".to_string()),
        };

        let internal_ref = v1_ref.clone().to_internal();
        let roundtrip = TypedLocalObjectReference::from_internal(internal_ref);

        assert_eq!(v1_ref, roundtrip);
    }

    #[test]
    fn test_resource_requirements_roundtrip() {
        let mut limits = BTreeMap::new();
        limits.insert("cpu".to_string(), Quantity("2".to_string()));
        limits.insert("memory".to_string(), Quantity("4Gi".to_string()));

        let mut requests = BTreeMap::new();
        requests.insert("cpu".to_string(), Quantity("1".to_string()));
        requests.insert("memory".to_string(), Quantity("2Gi".to_string()));

        let v1_req = resource::ResourceRequirements {
            limits,
            requests,
            claims: Vec::new(), // This field is dropped in internal
        };

        let internal_req = v1_req.clone().to_internal();
        let roundtrip = resource::ResourceRequirements::from_internal(internal_req);

        assert_eq!(v1_req, roundtrip);
    }
}
