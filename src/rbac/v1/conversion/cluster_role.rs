//! ClusterRole and ClusterRoleList conversions

use crate::common::{FromInternal, ToInternal, TypeMeta};
use crate::rbac::internal;
use crate::rbac::v1::rbac::{ClusterRole, ClusterRoleList};

impl ToInternal<internal::ClusterRole> for ClusterRole {
    fn to_internal(mut self) -> internal::ClusterRole {
        self.type_meta = TypeMeta::default();
        self
    }
}

impl FromInternal<internal::ClusterRole> for ClusterRole {
    fn from_internal(mut value: internal::ClusterRole) -> Self {
        value.type_meta = TypeMeta::default();
        value
    }
}

impl ToInternal<internal::ClusterRoleList> for ClusterRoleList {
    fn to_internal(mut self) -> internal::ClusterRoleList {
        self.type_meta = TypeMeta::default();
        self.items = self
            .items
            .into_iter()
            .map(|item| item.to_internal())
            .collect();
        self
    }
}

impl FromInternal<internal::ClusterRoleList> for ClusterRoleList {
    fn from_internal(mut value: internal::ClusterRoleList) -> Self {
        value.type_meta = TypeMeta::default();
        value.items = value
            .items
            .into_iter()
            .map(ClusterRole::from_internal)
            .collect();
        value
    }
}
