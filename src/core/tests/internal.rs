//! Core internal API trait tests

use crate::generate_internal_object_meta_tests;
use crate::core::internal;

generate_internal_object_meta_tests!(
    resources: [
        internal::Namespace,
        internal::Node,
        internal::ComponentStatus,
        internal::ConfigMap,
        internal::Secret,
        internal::ServiceAccount,
        internal::Binding,
        internal::Event,
        internal::LimitRange,
        internal::ResourceQuota,
        internal::Service,
        internal::Endpoints,
        internal::PersistentVolume,
        internal::PersistentVolumeClaim,
        internal::Pod,
        internal::ReplicationController,
        internal::PodTemplate,
        internal::PodStatusResult
    ]
);

