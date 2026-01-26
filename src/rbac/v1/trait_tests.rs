use super::*;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};
use crate::rbac::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<Role>();
    check_versioned::<RoleBinding>();
    check_versioned::<ClusterRole>();
    check_versioned::<ClusterRoleBinding>();

    check_default::<Role>();
    check_default::<RoleList>();
    check_default::<RoleBinding>();
    check_default::<RoleBindingList>();
    check_default::<ClusterRole>();
    check_default::<ClusterRoleList>();
    check_default::<ClusterRoleBinding>();
    check_default::<ClusterRoleBindingList>();

    check_schema::<Role>();
    check_schema::<RoleList>();
    check_schema::<RoleBinding>();
    check_schema::<RoleBindingList>();
    check_schema::<ClusterRole>();
    check_schema::<ClusterRoleList>();
    check_schema::<ClusterRoleBinding>();
    check_schema::<ClusterRoleBindingList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<Role, internal::Role>();
    check_conversion::<RoleList, internal::RoleList>();
    check_conversion::<RoleBinding, internal::RoleBinding>();
    check_conversion::<RoleBindingList, internal::RoleBindingList>();
    check_conversion::<ClusterRole, internal::ClusterRole>();
    check_conversion::<ClusterRoleList, internal::ClusterRoleList>();
    check_conversion::<ClusterRoleBinding, internal::ClusterRoleBinding>();
    check_conversion::<ClusterRoleBindingList, internal::ClusterRoleBindingList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<Role>();
    check_prost::<RoleList>();
    check_prost::<RoleBinding>();
    check_prost::<RoleBindingList>();
    check_prost::<ClusterRole>();
    check_prost::<ClusterRoleList>();
    check_prost::<ClusterRoleBinding>();
    check_prost::<ClusterRoleBindingList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = Role::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = Role::default();
    resource.metadata_mut().name = Some("role".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("role"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = Role::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "rbac.authorization.k8s.io/v1"
    );
    assert_eq!(resource.type_meta.kind, "Role");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::Role>();
    check::<internal::RoleBinding>();
    check::<internal::ClusterRole>();
    check::<internal::ClusterRoleBinding>();
}
