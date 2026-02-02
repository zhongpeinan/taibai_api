//! RoleBinding and RoleBindingList conversions

use crate::common::{FromInternal, ToInternal, TypeMeta};
use crate::rbac::internal;
use crate::rbac::v1::rbac::{RoleBinding, RoleBindingList};

impl ToInternal<internal::RoleBinding> for RoleBinding {
    fn to_internal(mut self) -> internal::RoleBinding {
        self.type_meta = TypeMeta::default();
        self
    }
}

impl FromInternal<internal::RoleBinding> for RoleBinding {
    fn from_internal(mut value: internal::RoleBinding) -> Self {
        value.type_meta = TypeMeta::default();
        value
    }
}

impl ToInternal<internal::RoleBindingList> for RoleBindingList {
    fn to_internal(mut self) -> internal::RoleBindingList {
        self.type_meta = TypeMeta::default();
        self.items = self
            .items
            .into_iter()
            .map(|item| item.to_internal())
            .collect();
        self
    }
}

impl FromInternal<internal::RoleBindingList> for RoleBindingList {
    fn from_internal(mut value: internal::RoleBindingList) -> Self {
        value.type_meta = TypeMeta::default();
        value.items = value
            .items
            .into_iter()
            .map(RoleBinding::from_internal)
            .collect();
        value
    }
}
