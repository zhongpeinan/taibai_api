//! Apps/v1 trait tests

use crate::apps::v1::{
    ControllerRevision, ControllerRevisionList, DaemonSet, DaemonSetList, Deployment,
    DeploymentList, ReplicaSet, ReplicaSetList, StatefulSet, StatefulSetList,
};
use crate::{generate_internal_object_meta_tests, generate_trait_tests};
use crate::common::{ApplyDefault, ResourceSchema, VersionedObject};

mod internal {
    pub use crate::apps::internal::{
        ControllerRevision, ControllerRevisionList, DaemonSet, DaemonSetList, Deployment,
        DeploymentList, ReplicaSet, ReplicaSetList, StatefulSet, StatefulSetList,
    };
}

generate_trait_tests!(
    api_version: "apps/v1",
    resources: [StatefulSet, Deployment, DaemonSet, ReplicaSet, ControllerRevision],
    list_resources: [
        StatefulSetList,
        DeploymentList,
        DaemonSetList,
        ReplicaSetList,
        ControllerRevisionList
    ],
    internal_resources: [
        internal::StatefulSet,
        internal::Deployment,
        internal::DaemonSet,
        internal::ReplicaSet,
        internal::ControllerRevision
    ],
    internal_list_resources: [
        internal::StatefulSetList,
        internal::DeploymentList,
        internal::DaemonSetList,
        internal::ReplicaSetList,
        internal::ControllerRevisionList
    ]
);

generate_internal_object_meta_tests!(
    resources: [
        internal::StatefulSet,
        internal::Deployment,
        internal::DaemonSet,
        internal::ReplicaSet,
        internal::ControllerRevision
    ]
);

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
    let group = <Deployment as ResourceSchema>::group_static();
    let version = <Deployment as ResourceSchema>::version_static();
    let expected_api_version = if group.is_empty() {
        version.to_string()
    } else {
        format!("{}/{}", group, version)
    };
    assert_eq!(resource.type_meta.api_version, expected_api_version);
    assert_eq!(
        resource.type_meta.kind,
        <Deployment as ResourceSchema>::kind_static()
    );
}

