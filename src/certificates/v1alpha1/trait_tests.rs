use super::*;
use crate::certificates::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, HasTypeMeta, ResourceSchema, ToInternal,
    VersionedObject,
};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}
    fn check_has_type_meta<T: HasTypeMeta>() {}

    check_versioned::<ClusterTrustBundle>();
    check_versioned::<PodCertificateRequest>();

    check_default::<ClusterTrustBundle>();
    check_default::<ClusterTrustBundleList>();
    check_default::<PodCertificateRequest>();
    check_default::<PodCertificateRequestList>();

    check_schema::<ClusterTrustBundle>();
    check_schema::<ClusterTrustBundleList>();
    check_schema::<PodCertificateRequest>();
    check_schema::<PodCertificateRequestList>();

    check_has_type_meta::<ClusterTrustBundle>();
    check_has_type_meta::<ClusterTrustBundleList>();
    check_has_type_meta::<PodCertificateRequest>();
    check_has_type_meta::<PodCertificateRequestList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<ClusterTrustBundle, internal::ClusterTrustBundle>();
    check_conversion::<ClusterTrustBundleList, internal::ClusterTrustBundleList>();
    check_conversion::<PodCertificateRequest, internal::PodCertificateRequest>();
    check_conversion::<PodCertificateRequestList, internal::PodCertificateRequestList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<ClusterTrustBundle>();
    check_prost::<ClusterTrustBundleList>();
    check_prost::<PodCertificateRequest>();
    check_prost::<PodCertificateRequestList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = ClusterTrustBundle::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = ClusterTrustBundle::default();
    resource.metadata_mut().name = Some("bundle".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("bundle"));

    let resource = PodCertificateRequest::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = PodCertificateRequest::default();
    resource.metadata_mut().name = Some("pcr".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("pcr"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = ClusterTrustBundle::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "certificates.k8s.io/v1alpha1"
    );
    assert_eq!(resource.type_meta.kind, "ClusterTrustBundle");

    let mut resource = PodCertificateRequest::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "certificates.k8s.io/v1alpha1"
    );
    assert_eq!(resource.type_meta.kind, "PodCertificateRequest");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}
    fn check_schema<T: ResourceSchema>() {}

    check::<internal::CertificateSigningRequest>();
    check::<internal::ClusterTrustBundle>();
    check::<internal::PodCertificateRequest>();

    check_schema::<internal::CertificateSigningRequest>();
    check_schema::<internal::CertificateSigningRequestList>();
    check_schema::<internal::ClusterTrustBundle>();
    check_schema::<internal::ClusterTrustBundleList>();
    check_schema::<internal::PodCertificateRequest>();
    check_schema::<internal::PodCertificateRequestList>();
}
