//! Declarative macros for generating test code.
//!
//! These macros reduce boilerplate in tests by generating repetitive
//! test patterns at compile time.

/// Generates trait implementation tests for API resources.
///
/// This macro creates tests that verify resources implement the required
/// traits (VersionedObject, ApplyDefault, ResourceSchema, ToInternal, FromInternal).
///
/// The `list_resources` parameter is used for list types that do not implement
/// `VersionedObject`. The `internal_*` parameters are optional and require a
/// local `internal` module to be in scope.
///
/// # Examples
///
/// With internal resources:
/// ```ignore
/// generate_trait_tests!(
///     api_version: "apps/v1",
///     resources: [Deployment],
///     list_resources: [DeploymentList],
///     internal_resources: [Deployment],
///     internal_list_resources: [DeploymentList]
/// );
/// ```
///
/// Without internal resources (e.g., core/v1):
/// ```ignore
/// generate_trait_tests!(
///     api_version: "v1",
///     resources: [Pod],
///     list_resources: [PodList]
/// );
/// ```
#[macro_export]
#[allow(unused_macros)]
macro_rules! generate_trait_tests {
    // Variant with internal resources + list resources
    (
        api_version: $api_version:literal,
        resources: [ $($resource:path),+ $(,)? ],
        list_resources: [ $($list_resource:path),+ $(,)? ],
        internal_resources: [ $($internal:path),+ $(,)? ],
        internal_list_resources: [ $($internal_list:path),+ $(,)? ]
    ) => {
        #[test]
        fn resources_implement_required_traits() {
            fn check_versioned<T: $crate::common::VersionedObject + $crate::common::ApplyDefault>() {}
            fn check_default<T: Default>() {}
            fn check_schema<T: $crate::common::ResourceSchema>() {}

            $(
                check_versioned::<$resource>();
                check_default::<$resource>();
                check_schema::<$resource>();
            )+

            $(
                check_default::<$list_resource>();
                check_schema::<$list_resource>();
            )+
        }

        #[test]
        fn resources_have_conversion_traits() {
            fn check_conversion<T, I>()
            where
                T: $crate::common::ToInternal<I> + $crate::common::FromInternal<I>,
            {}

            $(
                check_conversion::<$resource, $internal>();
            )+

            $(
                check_conversion::<$list_resource, $internal_list>();
            )+
        }

        #[test]
        fn resources_implement_prost_message() {
            fn check_prost<T: prost::Message>() {}

            $(
                check_prost::<$resource>();
            )+

            $(
                check_prost::<$list_resource>();
            )+
        }
    };

    // Variant without internal resources (single resource list)
    (
        api_version: $api_version:literal,
        resources: [ $($resource:path),+ $(,)? ]
    ) => {
        #[test]
        fn resources_implement_required_traits() {
            fn check_versioned<T: $crate::common::VersionedObject + $crate::common::ApplyDefault>() {}
            fn check_default<T: Default>() {}
            fn check_schema<T: $crate::common::ResourceSchema>() {}

            $(
                check_versioned::<$resource>();
                check_default::<$resource>();
                check_schema::<$resource>();
            )+
        }

        #[test]
        fn resources_implement_prost_message() {
            fn check_prost<T: prost::Message>() {}

            $(
                check_prost::<$resource>();
            )+
        }
    };

    // Variant with list resources (no internal resources)
    (
        api_version: $api_version:literal,
        resources: [ $($resource:path),+ $(,)? ],
        list_resources: [ $($list_resource:path),+ $(,)? ]
    ) => {
        #[test]
        fn resources_implement_required_traits() {
            fn check_versioned<T: $crate::common::VersionedObject + $crate::common::ApplyDefault>() {}
            fn check_default<T: Default>() {}
            fn check_schema<T: $crate::common::ResourceSchema>() {}

            $(
                check_versioned::<$resource>();
                check_default::<$resource>();
                check_schema::<$resource>();
            )+

            $(
                check_default::<$list_resource>();
                check_schema::<$list_resource>();
            )+
        }

        #[test]
        fn resources_implement_prost_message() {
            fn check_prost<T: prost::Message>() {}

            $(
                check_prost::<$resource>();
            )+

            $(
                check_prost::<$list_resource>();
            )+
        }
    };
}

/// Generates internal resource trait tests for `HasObjectMeta`.
///
/// # Example
/// ```ignore
/// generate_internal_object_meta_tests!(
///     resources: [Pod, Deployment]
/// );
/// ```
#[macro_export]
macro_rules! generate_internal_object_meta_tests {
    (
        resources: [ $($resource:path),+ $(,)? ]
    ) => {
        #[test]
        fn internal_resources_implement_required_traits() {
            fn check<T: $crate::common::HasObjectMeta>() {}

            $(
                check::<$resource>();
            )+
        }
    };
}

/// Generates serde roundtrip tests for API resources.
///
/// This macro creates test functions that verify resources can be
/// serialized to JSON and back without data loss.
///
/// # Example
///
/// ```ignore
/// generate_serde_roundtrip_tests!(
///     api_version: "apps/v1",
///     tests: [
///         deployment: Deployment {
///             metadata: Some(test_object_meta("test", "default")),
///             spec: Some(test_deployment_spec()),
///             ..Default::default()
///         },
///     ]
/// );
/// ```
#[macro_export]
macro_rules! generate_serde_roundtrip_tests {
    (
        api_version: $api_version:literal,
        tests: [
            $(
                $test_name:ident: $resource:ident {
                    $( $field:ident: $value:expr ),* $(,)?
                }
            ),+ $(,)?
        ]
    ) => {
        use $crate::common::test_utils::assert_serde_roundtrip;

        $(
            mod $test_name {
                use super::*;

                fn value() -> $resource {
                    use $crate::common::TypeMeta;

                    $resource {
                        type_meta: TypeMeta {
                            api_version: $api_version.to_string(),
                            kind: stringify!($resource).to_string(),
                        },
                        $( $field: $value ),*,
                        ..Default::default()
                    }
                }

                #[test]
                fn serde_roundtrip() {
                    assert_serde_roundtrip(&value());
                }
            }
        )+
    };
}

/// Generates conversion roundtrip tests for API resources.
///
/// This macro creates test functions that verify resources can be
/// converted to internal types and back without data loss.
///
/// # Example
///
/// ```ignore
/// generate_conversion_roundtrip_tests!(
///     tests: [
///         deployment: test_deployment(),
///         deployment_list: test_deployment_list(),
///     ]
/// );
/// ```
#[macro_export]
macro_rules! generate_conversion_roundtrip_tests {
    (
        tests: [
            $(
                $test_name:ident: $value:expr
            ),+ $(,)?
        ]
    ) => {
        use $crate::common::test_utils::assert_conversion_roundtrip;

        $(
            mod $test_name {
                use super::*;

                #[test]
                fn conversion_roundtrip() {
                    assert_conversion_roundtrip($value);
                }
            }
        )+
    };
}
