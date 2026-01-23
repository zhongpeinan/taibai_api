//! Object reference types from the Kubernetes Core v1 API
//!
//! This module contains types for referencing Kubernetes objects.

use serde::{Deserialize, Serialize};

/// ObjectReference contains enough information to let you inspect or modify the referred object.
///
/// Corresponds to [Kubernetes ObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7408)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectReference {
    /// Kind of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Namespace of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Name of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// UID of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Specific resourceVersion to which this reference is made, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,

    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
}

/// LocalObjectReference is a reference to another object within the same namespace.
///
/// Corresponds to [Kubernetes LocalObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7459)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocalObjectReference {
    /// Name of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// TypedLocalObjectReference is a reference to another object within the same namespace
/// with a specific kind and optionally an API group.
///
/// Corresponds to [Kubernetes TypedLocalObjectReference](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L7489)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TypedLocalObjectReference {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,

    /// Kind is the type of resource being referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Name is the name of resource being referenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {}
