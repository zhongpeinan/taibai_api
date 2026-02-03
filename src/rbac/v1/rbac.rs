//! RBAC types from the Kubernetes RBAC v1 API
//!
//! This module contains types for Role-Based Access Control (RBAC).

use crate::common::{ApplyDefault, HasTypeMeta, ResourceSchema, TypeMeta};
use crate::common::{LabelSelector, ListMeta, ObjectMeta};
use crate::impl_unimplemented_prost_message;
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

/// PolicyRule holds information that describes a policy rule.
///
/// Corresponds to [Kubernetes PolicyRule](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L47)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRule {
    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds contained in this rule.
    #[serde(default)]
    pub verbs: Vec<String>,

    /// APIGroups is the name of the APIGroup that contains the resources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api_groups: Vec<String>,

    /// Resources is a list of resources this rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,

    /// ResourceNames is an optional white list of names that the rule applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_names: Vec<String>,

    /// NonResourceURLs is a set of partial urls that a user should have access to.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "nonResourceURLs"
    )]
    pub non_resource_urls: Vec<String>,
}

/// Subject contains a reference to the object or user identities a role binding applies to.
///
/// Corresponds to [Kubernetes Subject](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L76)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    /// Kind of object being referenced.
    pub kind: String,

    /// APIGroup holds the API group of the referenced subject.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub api_group: String,

    /// Name of the object being referenced.
    pub name: String,

    /// Namespace of the referenced object.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
}

/// RoleRef contains information that points to the role being used.
///
/// Corresponds to [Kubernetes RoleRef](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L98)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RoleRef {
    /// APIGroup is the group for the resource being referenced.
    pub api_group: String,

    /// Kind is the type of resource being referenced.
    pub kind: String,

    /// Name is the name of resource being referenced.
    pub name: String,
}

/// Role is a namespaced, logical grouping of PolicyRules.
///
/// Corresponds to [Kubernetes Role](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L111)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Rules holds all the PolicyRules for this Role.
    #[serde(default)]
    pub rules: Vec<PolicyRule>,
}
impl_versioned_object!(Role);

/// RoleList is a collection of Roles.
///
/// Corresponds to [Kubernetes RoleList](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L166)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RoleList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of Roles.
    #[serde(default)]
    pub items: Vec<Role>,
}

/// RoleBinding references a role, but does not contain it.
///
/// Corresponds to [Kubernetes RoleBinding](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L128)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RoleBinding {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Subjects holds references to the objects the role applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,

    /// RoleRef can reference a Role in the current namespace or a ClusterRole.
    pub role_ref: RoleRef,
}
impl_versioned_object!(RoleBinding);

/// RoleBindingList is a collection of RoleBindings.
///
/// Corresponds to [Kubernetes RoleBindingList](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L152)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RoleBindingList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of RoleBindings.
    #[serde(default)]
    pub items: Vec<RoleBinding>,
}

/// ClusterRole is a cluster level, logical grouping of PolicyRules.
///
/// Corresponds to [Kubernetes ClusterRole](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L180)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRole {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Rules holds all the PolicyRules for this ClusterRole.
    #[serde(default)]
    pub rules: Vec<PolicyRule>,

    /// AggregationRule is an optional field that describes how to build the Rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation_rule: Option<AggregationRule>,
}

impl_versioned_object!(ClusterRole);

/// ClusterRoleList is a collection of ClusterRoles.
///
/// Corresponds to [Kubernetes ClusterRoleList](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L250)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of ClusterRoles.
    #[serde(default)]
    pub items: Vec<ClusterRole>,
}

/// AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole.
///
/// Corresponds to [Kubernetes AggregationRule](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L204)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AggregationRule {
    /// ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_role_selectors: Vec<LabelSelector>,
}

/// ClusterRoleBinding references a ClusterRole, but not contain it.
///
/// Corresponds to [Kubernetes ClusterRoleBinding](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L213)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleBinding {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Subjects holds references to the objects the role applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,

    /// RoleRef can only reference a ClusterRole in the global namespace.
    pub role_ref: RoleRef,
}
impl_versioned_object!(ClusterRoleBinding);

/// ClusterRoleBindingList is a collection of ClusterRoleBindings.
///
/// Corresponds to [Kubernetes ClusterRoleBindingList](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L237)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleBindingList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of ClusterRoleBindings.
    #[serde(default)]
    pub items: Vec<ClusterRoleBinding>,
}

/// Subject kind constants
pub mod subject_kind {
    /// Group kind
    pub const GROUP: &str = "Group";
    /// ServiceAccount kind
    pub const SERVICE_ACCOUNT: &str = "ServiceAccount";
    /// User kind
    pub const USER: &str = "User";
}

/// Auto update annotation key
pub const AUTO_UPDATE_ANNOTATION_KEY: &str = "rbac.authorization.kubernetes.io/autoupdate";

/// API group constants
pub mod api_group {
    /// Core API group
    pub const CORE: &str = "";
    /// RBAC API group
    pub const RBAC: &str = "rbac.authorization.k8s.io";
}

#[cfg(test)]
mod tests {}

// ============================================================================
// Trait Implementations for RBAC Resources
// ============================================================================

// ----------------------------------------------------------------------------
// ResourceSchema Implementation
// ----------------------------------------------------------------------------

impl ResourceSchema for Role {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "rbac.authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "Role"
    }
    fn resource(_: &Self::Meta) -> &str {
        "roles"
    }

    fn group_static() -> &'static str {
        "rbac.authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "Role"
    }
    fn resource_static() -> &'static str {
        "roles"
    }
}

impl ResourceSchema for RoleList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "rbac.authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "RoleList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "roles"
    }

    fn group_static() -> &'static str {
        "rbac.authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "RoleList"
    }
    fn resource_static() -> &'static str {
        "roles"
    }
}

impl ResourceSchema for ClusterRole {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "rbac.authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ClusterRole"
    }
    fn resource(_: &Self::Meta) -> &str {
        "clusterroles"
    }

    fn group_static() -> &'static str {
        "rbac.authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ClusterRole"
    }
    fn resource_static() -> &'static str {
        "clusterroles"
    }
}

impl ResourceSchema for ClusterRoleList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "rbac.authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ClusterRoleList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "clusterroles"
    }

    fn group_static() -> &'static str {
        "rbac.authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ClusterRoleList"
    }
    fn resource_static() -> &'static str {
        "clusterroles"
    }
}

impl ResourceSchema for RoleBinding {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "rbac.authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "RoleBinding"
    }
    fn resource(_: &Self::Meta) -> &str {
        "rolebindings"
    }

    fn group_static() -> &'static str {
        "rbac.authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "RoleBinding"
    }
    fn resource_static() -> &'static str {
        "rolebindings"
    }
}

impl ResourceSchema for RoleBindingList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "rbac.authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "RoleBindingList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "rolebindings"
    }

    fn group_static() -> &'static str {
        "rbac.authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "RoleBindingList"
    }
    fn resource_static() -> &'static str {
        "rolebindings"
    }
}

impl ResourceSchema for ClusterRoleBinding {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "rbac.authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ClusterRoleBinding"
    }
    fn resource(_: &Self::Meta) -> &str {
        "clusterrolebindings"
    }

    fn group_static() -> &'static str {
        "rbac.authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ClusterRoleBinding"
    }
    fn resource_static() -> &'static str {
        "clusterrolebindings"
    }
}

impl ResourceSchema for ClusterRoleBindingList {
    type Meta = ();

    fn group(_: &Self::Meta) -> &str {
        "rbac.authorization.k8s.io"
    }
    fn version(_: &Self::Meta) -> &str {
        "v1"
    }
    fn kind(_: &Self::Meta) -> &str {
        "ClusterRoleBindingList"
    }
    fn resource(_: &Self::Meta) -> &str {
        "clusterrolebindings"
    }

    fn group_static() -> &'static str {
        "rbac.authorization.k8s.io"
    }
    fn version_static() -> &'static str {
        "v1"
    }
    fn kind_static() -> &'static str {
        "ClusterRoleBindingList"
    }
    fn resource_static() -> &'static str {
        "clusterrolebindings"
    }
}

// ----------------------------------------------------------------------------
// HasTypeMeta Implementation
// ----------------------------------------------------------------------------

impl HasTypeMeta for Role {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for RoleList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ClusterRole {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ClusterRoleList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for RoleBinding {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for RoleBindingList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ClusterRoleBinding {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl HasTypeMeta for ClusterRoleBindingList {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }
    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

// ----------------------------------------------------------------------------
// VersionedObject Implementation
// ----------------------------------------------------------------------------

// Note: List types do not implement VersionedObject because they have ListMeta, not ObjectMeta

// ----------------------------------------------------------------------------
// ApplyDefaults Implementation
// ----------------------------------------------------------------------------

impl ApplyDefault for Role {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "rbac.authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "Role".to_string();
        }
    }
}

impl ApplyDefault for RoleList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "rbac.authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "RoleList".to_string();
        }
    }
}

impl ApplyDefault for ClusterRole {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "rbac.authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ClusterRole".to_string();
        }
    }
}

impl ApplyDefault for ClusterRoleList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "rbac.authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ClusterRoleList".to_string();
        }
    }
}

impl ApplyDefault for RoleBinding {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "rbac.authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "RoleBinding".to_string();
        }
        if self.role_ref.api_group.is_empty() {
            self.role_ref.api_group = "rbac.authorization.k8s.io".to_string();
        }
    }
}

impl ApplyDefault for RoleBindingList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "rbac.authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "RoleBindingList".to_string();
        }
    }
}

impl ApplyDefault for ClusterRoleBinding {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "rbac.authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ClusterRoleBinding".to_string();
        }
        if self.role_ref.api_group.is_empty() {
            self.role_ref.api_group = "rbac.authorization.k8s.io".to_string();
        }
    }
}

impl ApplyDefault for ClusterRoleBindingList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "rbac.authorization.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ClusterRoleBindingList".to_string();
        }
    }
}

// ----------------------------------------------------------------------------
// Protobuf Placeholder
// ----------------------------------------------------------------------------

impl_unimplemented_prost_message!(Role);
impl_unimplemented_prost_message!(RoleList);
impl_unimplemented_prost_message!(ClusterRole);
impl_unimplemented_prost_message!(ClusterRoleList);
impl_unimplemented_prost_message!(RoleBinding);
impl_unimplemented_prost_message!(RoleBindingList);
impl_unimplemented_prost_message!(ClusterRoleBinding);
impl_unimplemented_prost_message!(ClusterRoleBindingList);
