//! Binding conversion implementations
//!
//! Includes: Binding

use super::helpers::*;
use crate::common::traits::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::binding;

// ============================================================================
// Binding
// ============================================================================

impl ToInternal<internal::Binding> for binding::Binding {
    fn to_internal(self) -> internal::Binding {
        internal::Binding {
            metadata: option_object_meta_to_meta(self.metadata),
            target: self.target,
        }
    }
}

impl FromInternal<internal::Binding> for binding::Binding {
    fn from_internal(value: internal::Binding) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            target: value.target,
        };
        result.apply_default();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_roundtrip() {
        let binding = binding::Binding {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("bind-me".to_string()),
                namespace: Some("default".to_string()),
                ..Default::default()
            }),
            target: crate::core::v1::reference::ObjectReference {
                api_version: Some("v1".to_string()),
                kind: Some("Node".to_string()),
                name: Some("node-1".to_string()),
                ..Default::default()
            },
        };

        let internal = binding.clone().to_internal();
        assert_eq!(internal.metadata.name.as_deref(), Some("bind-me"));
        assert_eq!(internal.target.name.as_deref(), Some("node-1"));

        let roundtrip = binding::Binding::from_internal(internal);
        assert_eq!(
            roundtrip.metadata.as_ref().unwrap().name.as_deref(),
            Some("bind-me")
        );
        assert_eq!(roundtrip.target.name.as_deref(), Some("node-1"));
    }
}
