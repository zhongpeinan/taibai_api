use super::*;
use crate::certificates::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, ResourceSchema, ToInternal, VersionedObject,
};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}

    check_versioned::<CertificateSigningRequest>();
    check_versioned::<ClusterTrustBundle>();

    check_default::<CertificateSigningRequest>();
    check_default::<CertificateSigningRequestList>();
    check_default::<ClusterTrustBundle>();
    check_default::<ClusterTrustBundleList>();

    check_schema::<CertificateSigningRequest>();
    check_schema::<CertificateSigningRequestList>();
    check_schema::<ClusterTrustBundle>();
    check_schema::<ClusterTrustBundleList>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<CertificateSigningRequest, internal::CertificateSigningRequest>();
    check_conversion::<CertificateSigningRequestList, internal::CertificateSigningRequestList>();
    check_conversion::<ClusterTrustBundle, internal::ClusterTrustBundle>();
    check_conversion::<ClusterTrustBundleList, internal::ClusterTrustBundleList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<CertificateSigningRequest>();
    check_prost::<CertificateSigningRequestList>();
    check_prost::<ClusterTrustBundle>();
    check_prost::<ClusterTrustBundleList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = CertificateSigningRequest::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = CertificateSigningRequest::default();
    resource.metadata_mut().name = Some("csr".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("csr"));

    let resource = ClusterTrustBundle::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = ClusterTrustBundle::default();
    resource.metadata_mut().name = Some("bundle".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("bundle"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = CertificateSigningRequest::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "certificates.k8s.io/v1beta1"
    );
    assert_eq!(resource.type_meta.kind, "CertificateSigningRequest");

    let mut resource = ClusterTrustBundle::default();
    resource.apply_default();
    assert_eq!(
        resource.type_meta.api_version,
        "certificates.k8s.io/v1beta1"
    );
    assert_eq!(resource.type_meta.kind, "ClusterTrustBundle");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::CertificateSigningRequest>();
}
