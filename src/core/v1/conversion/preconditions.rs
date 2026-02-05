//! Preconditions conversion implementations
//!
//! Includes: binding::Preconditions ↔ internal::Preconditions

use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::binding;

// ============================================================================
// Preconditions
// ============================================================================

impl ToInternal<internal::Preconditions> for binding::Preconditions {
    fn to_internal(self) -> internal::Preconditions {
        internal::Preconditions {
            resource_version: None, // v1 does not have this field
            uid: self.uid,
        }
    }
}

impl FromInternal<internal::Preconditions> for binding::Preconditions {
    fn from_internal(value: internal::Preconditions) -> Self {
        Self {
            uid: value.uid,
            // resource_version dropped (v1 does not have this field)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preconditions_roundtrip() {
        let v1 = binding::Preconditions {
            uid: Some("abc-123".to_string()),
        };

        let internal = v1.clone().to_internal();
        assert_eq!(internal.uid.as_deref(), Some("abc-123"));
        assert_eq!(internal.resource_version, None);

        let roundtrip = binding::Preconditions::from_internal(internal);
        assert_eq!(v1, roundtrip);
    }

    #[test]
    fn test_preconditions_from_internal_drops_resource_version() {
        let internal = internal::Preconditions {
            resource_version: Some("12345".to_string()),
            uid: Some("uid-1".to_string()),
        };

        let v1 = binding::Preconditions::from_internal(internal);
        assert_eq!(v1.uid.as_deref(), Some("uid-1"));
        // resource_version is dropped in v1
    }
}
