//! Internal reference types

use serde::{Deserialize, Serialize};

/// TypedLocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TypedLocalObjectReference {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    pub api_group: Option<String>,

    /// Kind is the type of resource being referenced.
    pub kind: Option<String>,

    /// Name is the name of resource being referenced.
    pub name: Option<String>,
}
