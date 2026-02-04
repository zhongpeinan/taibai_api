//! Test fixtures for metadata types.

use crate::common::{ListMeta, ObjectMeta};

/// Creates a basic ObjectMeta for testing.
///
/// # Example
///
/// ```ignore
/// let meta = test_object_meta("my-deployment", "default");
/// ```
pub fn test_object_meta(name: &str, namespace: &str) -> ObjectMeta {
    ObjectMeta {
        name: Some(name.to_string()),
        namespace: Some(namespace.to_string()),
        ..Default::default()
    }
}

/// Creates a basic ListMeta for testing.
///
/// # Example
///
/// ```ignore
/// let meta = test_list_meta("123");
/// ```
pub fn test_list_meta(resource_version: &str) -> ListMeta {
    ListMeta {
        resource_version: Some(resource_version.to_string()),
        ..Default::default()
    }
}

/// Creates an ObjectMeta with labels for testing.
///
/// # Example
///
/// ```ignore
/// let meta = test_object_meta_with_labels(
///     "my-pod",
///     "default",
///     vec![("app", "web"), ("env", "test")]
/// );
/// ```
pub fn test_object_meta_with_labels<I, K, V>(
    name: &str,
    namespace: &str,
    labels: I,
) -> ObjectMeta
where
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: Into<String>,
{
    use std::collections::BTreeMap;

    let mut meta = test_object_meta(name, namespace);
    meta.labels = labels
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect::<BTreeMap<_, _>>();
    meta
}
