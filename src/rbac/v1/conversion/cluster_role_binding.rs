//! ClusterRoleBinding and ClusterRoleBindingList conversions

use crate::common::{FromInternal, ToInternal, TypeMeta};
use crate::rbac::internal;
use crate::rbac::v1::rbac::{ClusterRoleBinding, ClusterRoleBindingList};

impl ToInternal<internal::ClusterRoleBinding> for ClusterRoleBinding {
    fn to_internal(mut self) -> internal::ClusterRoleBinding {
        self.type_meta = TypeMeta::default();
        self
    }
}

impl FromInternal<internal::ClusterRoleBinding> for ClusterRoleBinding {
    fn from_internal(mut value: internal::ClusterRoleBinding) -> Self {
        value.type_meta = TypeMeta::default();
        value
    }
}

impl ToInternal<internal::ClusterRoleBindingList> for ClusterRoleBindingList {
    fn to_internal(mut self) -> internal::ClusterRoleBindingList {
        self.type_meta = TypeMeta::default();
        self.items = self
            .items
            .into_iter()
            .map(|item| item.to_internal())
            .collect();
        self
    }
}

impl FromInternal<internal::ClusterRoleBindingList> for ClusterRoleBindingList {
    fn from_internal(mut value: internal::ClusterRoleBindingList) -> Self {
        value.type_meta = TypeMeta::default();
        value.items = value
            .items
            .into_iter()
            .map(ClusterRoleBinding::from_internal)
            .collect();
        value
    }
}
