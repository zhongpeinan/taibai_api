use super::*;
use crate::common::{ApplyDefault, FromInternal, HasObjectMeta, ToInternal, VersionedObject};
use crate::networking::internal;

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault>() {}

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

    check_conversion::<IPAddress, internal::IPAddress>();
    check_conversion::<ServiceCIDR, internal::ServiceCIDR>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<IPAddress>();
    check_prost::<IPAddressList>();
    check_prost::<ServiceCIDR>();
    check_prost::<ServiceCIDRList>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = IPAddress::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = IPAddress::default();
    resource.metadata_mut().name = Some("ipaddress".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("ipaddress"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = IPAddress::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "networking.k8s.io/v1beta1");
    assert_eq!(resource.type_meta.kind, "IPAddress");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::IPAddress>();
    check::<internal::ServiceCIDR>();
}
