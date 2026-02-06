//! Workaround: HasObjectMeta for Options types
//!
//! **Issue**: <https://github.com/xxx/taibai/issues/147>
//!
//! # Problem
//!
//! Kubernetes Options types (e.g., `NodeProxyOptions`, `PodExecOptions`) have
//! `TypeMeta` but NO `ObjectMeta`. However, downstream code has constraints like:
//!
//! ```ignore
//! fn process<T: HasObjectMeta>(obj: &T) { ... }
//! ```
//!
//! We cannot modify downstream code, so these types must satisfy `HasObjectMeta`.
//!
//! # Solution
//!
//! Provide a macro [`impl_options_object!`] that implements both `HasTypeMeta`
//! and `HasObjectMeta` for Options types. The `HasObjectMeta` impl returns a
//! shared empty `ObjectMeta`.
//!
//! # Limitations
//!
//! - `meta()` returns a shared static empty `ObjectMeta`
//! - `meta_mut()` **panics** — Options types have no real metadata to mutate
//!
//! # Upstream Reference
//!
//! These types in Kubernetes have only `TypeMeta`, no `ObjectMeta`:
//!
//! | Type | Source |
//! |------|--------|
//! | `PodExecOptions` | `k8s.io/api/core/v1/types.go` |
//! | `PodAttachOptions` | `k8s.io/api/core/v1/types.go` |
//! | `PodLogOptions` | `k8s.io/api/core/v1/types.go` |
//! | `PodPortForwardOptions` | `k8s.io/api/core/v1/types.go` |
//! | `PodProxyOptions` | `k8s.io/api/core/v1/types.go` |
//! | `ServiceProxyOptions` | `k8s.io/api/core/v1/types.go` |
//!
//! # Example
//!
//! ```ignore
//! use taibai_api::core::v1::PodExecOptions;
//! use taibai_api::common::HasObjectMeta;
//!
//! let opts = PodExecOptions::default();
//!
//! // This works (returns empty metadata)
//! let meta = opts.meta();
//! assert!(meta.name.is_none());
//!
//! // This panics!
//! // opts.meta_mut().name = Some("test".to_string());
//! ```

use std::sync::LazyLock;

use crate::common::ObjectMeta;

// ============================================================================
// Shared empty metadata
// ============================================================================

/// Shared empty [`ObjectMeta`] for Options types.
///
/// This static is returned by [`HasObjectMeta::meta()`] for all Options types.
pub static EMPTY_OBJECT_META: LazyLock<ObjectMeta> = LazyLock::new(ObjectMeta::default);

// ============================================================================
// Marker trait
// ============================================================================

/// Marker trait for Kubernetes Options types.
///
/// Options types have `TypeMeta` but no `ObjectMeta` in upstream Kubernetes.
/// Types implementing this marker have a phantom `HasObjectMeta` implementation
/// that returns empty metadata.
///
/// Use [`impl_options_object!`] macro to implement this trait along with
/// the required trait implementations.
pub trait OptionsObject: Send + Sync {}

// ============================================================================
// Macro for implementing Options types
// ============================================================================

/// Implements `HasTypeMeta`, `HasObjectMeta`, and `OptionsObject` for an Options type.
///
/// Options types have `type_meta: TypeMeta` field but no `ObjectMeta`.
/// This macro provides:
/// - `HasTypeMeta` - delegates to `type_meta` field
/// - `HasObjectMeta` - returns shared empty `ObjectMeta`, panics on `meta_mut()`
/// - `OptionsObject` - marker trait
///
/// # Usage
///
/// ```ignore
/// impl_options_object!(PodExecOptions);
/// impl_options_object!(PodLogOptions);
/// ```
///
/// # Requirements
///
/// The type must have a `pub type_meta: TypeMeta` field.
#[macro_export]
macro_rules! impl_options_object {
    ($type:ty) => {
        impl $crate::common::HasTypeMeta for $type {
            fn type_meta(&self) -> &$crate::common::TypeMeta {
                &self.type_meta
            }
            fn type_meta_mut(&mut self) -> &mut $crate::common::TypeMeta {
                &mut self.type_meta
            }
        }

        impl $crate::common::HasObjectMeta for $type {
            fn meta(&self) -> &$crate::common::ObjectMeta {
                &$crate::common::compat::options_object::EMPTY_OBJECT_META
            }

            fn meta_mut(&mut self) -> &mut $crate::common::ObjectMeta {
                panic!(
                    "[taibai compat] Cannot get mutable ObjectMeta for Options type `{}`.\n\
                     \n\
                     Options types in Kubernetes API don't have ObjectMeta - they only have TypeMeta.\n\
                     This HasObjectMeta implementation exists solely for downstream trait bound compatibility.\n\
                     \n\
                     If you need mutable metadata, use a Resource type (Pod, Deployment, etc.) instead.\n\
                     \n\
                     See: https://github.com/xxx/taibai/issues/147",
                    stringify!($type)
                )
            }
        }

        impl $crate::common::compat::options_object::OptionsObject for $type {}
    };
}

// ============================================================================
// Options type registrations
// ============================================================================
//
// Add new Options types here using impl_options_object!()
//
// Requirements:
// - Type must have `pub type_meta: TypeMeta` field
// - Type must NOT have `ObjectMeta` (those are Resources, not Options)

impl_options_object!(crate::core::v1::PodLogOptions);
impl_options_object!(crate::core::v1::PodAttachOptions);
impl_options_object!(crate::core::v1::PodExecOptions);
impl_options_object!(crate::core::v1::PodPortForwardOptions);
impl_options_object!(crate::core::v1::PodProxyOptions);
impl_options_object!(crate::core::v1::ServiceProxyOptions);
impl_options_object!(crate::core::v1::NodeProxyOptions);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::common::HasObjectMeta;
    use crate::core::v1::PodExecOptions;
    use crate::{core::internal, generate_options_trait_tests};

    generate_options_trait_tests!(
        options: [
            (crate::core::v1::PodLogOptions, internal::PodLogOptions),
            (crate::core::v1::PodAttachOptions, internal::PodAttachOptions),
            (crate::core::v1::PodExecOptions, internal::PodExecOptions),
            (crate::core::v1::PodPortForwardOptions, internal::PodPortForwardOptions),
            (crate::core::v1::PodProxyOptions, internal::PodProxyOptions),
            (crate::core::v1::ServiceProxyOptions, internal::ServiceProxyOptions),
            (crate::core::v1::NodeProxyOptions, internal::NodeProxyOptions),
        ]
    );

    #[test]
    fn meta_returns_empty_object_meta() {
        let opts = PodExecOptions::default();
        let meta = opts.meta();

        assert!(meta.name.is_none());
        assert!(meta.namespace.is_none());
        assert!(meta.uid.is_none());
    }

    #[test]
    #[should_panic(expected = "Cannot get mutable ObjectMeta")]
    fn meta_mut_panics() {
        let mut opts = PodExecOptions::default();
        let _ = opts.meta_mut();
    }
}
