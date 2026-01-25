use super::*;
use crate::common::{ApplyDefault, FromInternal, HasObjectMeta, ToInternal, VersionedObject};
use crate::networking::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}

    check_versioned::<Ingress>();
    check_versioned::<IngressClass>();
    check_versioned::<NetworkPolicy>();
    check_versioned::<IPAddress>();
    check_versioned::<ServiceCIDR>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<Ingress, internal::Ingress>();
    check_conversion::<IngressList, internal::IngressList>();
    check_conversion::<IngressClass, internal::IngressClass>();
    check_conversion::<IngressClassList, internal::IngressClassList>();
    check_conversion::<NetworkPolicy, internal::NetworkPolicy>();
    check_conversion::<NetworkPolicyList, internal::NetworkPolicyList>();
    check_conversion::<IPAddress, internal::IPAddress>();
    check_conversion::<ServiceCIDR, internal::ServiceCIDR>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<Ingress>();
    check_prost::<IngressList>();
    check_prost::<IngressClass>();
    check_prost::<IngressClassList>();
    check_prost::<NetworkPolicy>();
    check_prost::<NetworkPolicyList>();
    check_prost::<IPAddress>();
    check_prost::<IPAddressList>();
    check_prost::<ServiceCIDR>();
    check_prost::<ServiceCIDRList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = Ingress::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = Ingress::default();
    resource.metadata_mut().name = Some("ingress".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("ingress"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = Ingress::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "networking.k8s.io/v1");
    assert_eq!(resource.type_meta.kind, "Ingress");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::Ingress>();
    check::<internal::IngressClass>();
    check::<internal::NetworkPolicy>();
    check::<internal::IPAddress>();
    check::<internal::ServiceCIDR>();
}
