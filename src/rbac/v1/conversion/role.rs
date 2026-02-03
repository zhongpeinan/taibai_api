//! Role and RoleList conversions

use crate::common::{FromInternal, ToInternal, TypeMeta};
use crate::rbac::internal;
use crate::rbac::v1::rbac::{Role, RoleList};

impl ToInternal<internal::Role> for Role {
    fn to_internal(mut self) -> internal::Role {
        self.type_meta = TypeMeta::default();
        self
    }
}

impl FromInternal<internal::Role> for Role {
    fn from_internal(mut value: internal::Role) -> Self {
        value.type_meta = TypeMeta::default();
        value
    }
}

impl ToInternal<internal::RoleList> for RoleList {
    fn to_internal(mut self) -> internal::RoleList {
        self.type_meta = TypeMeta::default();
        self.items = self
            .items
            .into_iter()
            .map(|item| item.to_internal())
            .collect();
        self
    }
}

impl FromInternal<internal::RoleList> for RoleList {
    fn from_internal(mut value: internal::RoleList) -> Self {
        value.type_meta = TypeMeta::default();
        value.items = value.items.into_iter().map(Role::from_internal).collect();
        value
    }
}
