use super::*;
use crate::apiextensions::internal;
use crate::common::{
    ApplyDefault, FromInternal, HasObjectMeta, HasTypeMeta, ResourceSchema, ToInternal,
    VersionedObject,
};

#[test]
fn top_level_resources_implement_required_traits() {
    fn check_versioned<T: VersionedObject + ApplyDefault + Default>() {}
    fn check_non_versioned<T: ApplyDefault + Default + HasTypeMeta + ResourceSchema>() {}
    fn check_default<T: Default>() {}
    fn check_schema<T: ResourceSchema>() {}
    fn check_has_type_meta<T: HasTypeMeta>() {}

    check_versioned::<CustomResourceDefinition>();
    check_default::<CustomResourceDefinition>();
    check_schema::<CustomResourceDefinition>();
    check_has_type_meta::<CustomResourceDefinition>();

    check_non_versioned::<CustomResourceDefinitionList>();
    check_default::<CustomResourceDefinitionList>();
    check_schema::<CustomResourceDefinitionList>();
    check_has_type_meta::<CustomResourceDefinitionList>();

    check_non_versioned::<ConversionReview>();
    check_default::<ConversionReview>();
    check_schema::<ConversionReview>();
    check_has_type_meta::<ConversionReview>();
}

#[test]
fn top_level_resources_have_conversion_traits() {
    fn check_conversion<T, I>()
    where
        T: ToInternal<I> + FromInternal<I>,
    {
    }

    check_conversion::<CustomResourceDefinition, internal::CustomResourceDefinition>();
    check_conversion::<CustomResourceDefinitionList, internal::CustomResourceDefinitionList>();
}

#[test]
fn top_level_resources_implement_prost_message() {
    fn check_prost<T: prost::Message>() {}

    check_prost::<CustomResourceDefinition>();
    check_prost::<CustomResourceDefinitionList>();
    check_prost::<ConversionReview>();
    check_prost::<ConversionRequest>();
    check_prost::<ConversionResponse>();
}

#[test]
fn versioned_object_metadata_access() {
    let resource = CustomResourceDefinition::default();
    let meta = resource.metadata();
    assert!(meta.name.is_none());

    let mut resource = CustomResourceDefinition::default();
    resource.metadata_mut().name = Some("crd".to_string());
    assert_eq!(resource.metadata().name.as_deref(), Some("crd"));
}

#[test]
fn apply_default_sets_type_meta() {
    let mut resource = CustomResourceDefinition::default();
    resource.apply_default();
    assert_eq!(resource.type_meta.api_version, "apiextensions.k8s.io/v1");
    assert_eq!(resource.type_meta.kind, "CustomResourceDefinition");

    let mut review = ConversionReview::default();
    review.apply_default();
    assert_eq!(review.type_meta.api_version, "apiextensions.k8s.io/v1");
    assert_eq!(review.type_meta.kind, "ConversionReview");
}

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<internal::CustomResourceDefinition>();
}
