//! SerializedReference conversion implementations
//!
//! Includes: helper::SerializedReference ↔ internal::SerializedReference

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::helper;
use crate::core::v1::reference::ObjectReference;

// ============================================================================
// SerializedReference
// ============================================================================

impl ToInternal<internal::SerializedReference> for helper::SerializedReference {
    fn to_internal(self) -> internal::SerializedReference {
        internal::SerializedReference {
            reference: self.reference.map(|r| r.to_internal()),
        }
    }
}

impl FromInternal<internal::SerializedReference> for helper::SerializedReference {
    fn from_internal(value: internal::SerializedReference) -> Self {
        Self {
            type_meta: Default::default(),
            reference: value.reference.map(ObjectReference::from_internal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialized_reference_roundtrip() {
        let v1 = helper::SerializedReference {
            type_meta: Default::default(),
            reference: Some(ObjectReference {
                kind: Some("Pod".to_string()),
                namespace: Some("default".to_string()),
                name: Some("my-pod".to_string()),
                uid: Some("abc-123".to_string()),
                api_version: Some("v1".to_string()),
                resource_version: Some("12345".to_string()),
                field_path: None,
            }),
        };

        let internal = v1.clone().to_internal();
        assert_eq!(internal.reference.as_ref().unwrap().kind, "Pod");
        assert_eq!(internal.reference.as_ref().unwrap().namespace, "default");

        let roundtrip = helper::SerializedReference::from_internal(internal);
        assert_eq!(v1.reference, roundtrip.reference);
    }

    #[test]
    fn test_serialized_reference_none() {
        let v1 = helper::SerializedReference {
            type_meta: Default::default(),
            reference: None,
        };

        let internal = v1.to_internal();
        assert_eq!(internal.reference, None);

        let roundtrip = helper::SerializedReference::from_internal(internal);
        assert_eq!(roundtrip.reference, None);
    }
}
