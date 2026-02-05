//! RangeAllocation conversion implementations
//!
//! Includes: helper::RangeAllocation ↔ internal::RangeAllocation

use super::helpers::*;
use crate::common::traits::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::helper;

// ============================================================================
// RangeAllocation
// ============================================================================

impl ToInternal<internal::RangeAllocation> for helper::RangeAllocation {
    fn to_internal(self) -> internal::RangeAllocation {
        internal::RangeAllocation {
            type_meta: Default::default(),
            metadata: option_object_meta_to_meta(self.metadata),
            range: self.range,
            data: if self.data.is_empty() {
                None
            } else {
                Some(internal::ByteString(self.data))
            },
        }
    }
}

impl FromInternal<internal::RangeAllocation> for helper::RangeAllocation {
    fn from_internal(value: internal::RangeAllocation) -> Self {
        Self {
            type_meta: Default::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            range: value.range,
            data: value.data.map(|b| b.0).unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_allocation_roundtrip() {
        let v1 = helper::RangeAllocation {
            type_meta: Default::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("test-range".to_string()),
                ..Default::default()
            }),
            range: "10.0.0.0/24".to_string(),
            data: vec![1, 2, 3, 4],
        };

        let internal = v1.clone().to_internal();
        assert_eq!(internal.metadata.name.as_deref(), Some("test-range"));
        assert_eq!(internal.range, "10.0.0.0/24");
        assert_eq!(internal.data.as_ref().unwrap().0, vec![1, 2, 3, 4]);

        let roundtrip = helper::RangeAllocation::from_internal(internal);
        assert_eq!(v1.metadata, roundtrip.metadata);
        assert_eq!(v1.range, roundtrip.range);
        assert_eq!(v1.data, roundtrip.data);
    }

    #[test]
    fn test_range_allocation_empty_data() {
        let v1 = helper::RangeAllocation {
            type_meta: Default::default(),
            metadata: None,
            range: String::new(),
            data: vec![],
        };

        let internal = v1.to_internal();
        assert_eq!(internal.data, None);

        let roundtrip = helper::RangeAllocation::from_internal(internal);
        assert!(roundtrip.data.is_empty());
    }
}
