//! Eviction conversions

use crate::common::{FromInternal, ToInternal, TypeMeta};
use crate::policy::internal;
use crate::policy::v1::Eviction;

impl ToInternal<internal::Eviction> for Eviction {
    fn to_internal(self) -> internal::Eviction {
        internal::Eviction {
            type_meta: TypeMeta::default(),
            metadata: self.metadata.unwrap_or_default(),
            // Note: Internal Eviction uses Option<ObjectMeta> for delete_options
            // while v1 uses Option<DeleteOptions>. This is a type mismatch in the
            // internal types. For now, we pass None as they cannot be converted.
            delete_options: None,
        }
    }
}

impl FromInternal<internal::Eviction> for Eviction {
    fn from_internal(value: internal::Eviction) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: Some(value.metadata),
            // Note: Internal Eviction uses Option<ObjectMeta> for delete_options
            // while v1 uses Option<DeleteOptions>. This is a type mismatch in the
            // internal types. For now, we pass None as they cannot be converted.
            delete_options: None,
        }
    }
}
