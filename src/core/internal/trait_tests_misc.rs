use super::*;
use crate::common::HasObjectMeta;

#[test]
fn internal_resources_implement_required_traits() {
    fn check<T: HasObjectMeta>() {}

    check::<Binding>();
    check::<Event>();
}
