use super::*;
use crate::apps::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<StatefulSet>();
    check_versioned::<Deployment>();
    check_versioned::<DaemonSet>();
    check_versioned::<ReplicaSet>();
    check_versioned::<ControllerRevision>();

    check_default::<StatefulSet>();
    check_default::<StatefulSetList>();
    check_default::<Deployment>();
    check_default::<DeploymentList>();
    check_default::<DaemonSet>();
    check_default::<DaemonSetList>();
    check_default::<ReplicaSet>();
    check_default::<ReplicaSetList>();
    check_default::<ControllerRevision>();
    check_default::<ControllerRevisionList>();

    check_schema::<StatefulSet>();
    check_schema::<StatefulSetList>();
    check_schema::<Deployment>();
    check_schema::<DeploymentList>();
    check_schema::<DaemonSet>();
    check_schema::<DaemonSetList>();
    check_schema::<ReplicaSet>();
    check_schema::<ReplicaSetList>();
    check_schema::<ControllerRevision>();
    check_schema::<ControllerRevisionList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<StatefulSet, internal::StatefulSet>();
    check_conversion::<StatefulSetList, internal::StatefulSetList>();
    check_conversion::<Deployment, internal::Deployment>();
    check_conversion::<DeploymentList, internal::DeploymentList>();
    check_conversion::<DaemonSet, internal::DaemonSet>();
    check_conversion::<DaemonSetList, internal::DaemonSetList>();
    check_conversion::<ReplicaSet, internal::ReplicaSet>();
    check_conversion::<ReplicaSetList, internal::ReplicaSetList>();
    check_conversion::<ControllerRevision, internal::ControllerRevision>();
    check_conversion::<ControllerRevisionList, internal::ControllerRevisionList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<StatefulSet>();
    check_prost::<StatefulSetList>();
    check_prost::<Deployment>();
    check_prost::<DeploymentList>();
    check_prost::<DaemonSet>();
    check_prost::<DaemonSetList>();
    check_prost::<ReplicaSet>();
    check_prost::<ReplicaSetList>();
    check_prost::<ControllerRevision>();
    check_prost::<ControllerRevisionList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = Deployment::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = Deployment::default();
    resource.metadata_mut().name = Some("deployment".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("deployment"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = Deployment::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "apps/v1");
    assert_eq!(resource.type_meta.kind, "Deployment");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::StatefulSet>();
    check::<internal::Deployment>();
    check::<internal::DaemonSet>();
    check::<internal::ReplicaSet>();
    check::<internal::ControllerRevision>();
}
