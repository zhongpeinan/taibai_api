//! RBAC types from the Kubernetes RBAC v1 API
//!
//! This module contains types for Role-Based Access Control (RBAC).

use crate::common::{LabelSelector, ListMeta, ObjectMeta};
use serde::{Deserialize, Serialize};

/// PolicyRule holds information that describes a policy rule.
///
/// Corresponds to [Kubernetes PolicyRule](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L47)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRule {
    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds contained in this rule.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Rules holds all the PolicyRules for this Role.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<PolicyRule>,
}

/// RoleList is a collection of Roles.
///
/// Corresponds to [Kubernetes RoleList](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L166)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoleList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of Roles.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Role>,
}

/// RoleBinding references a role, but does not contain it.
///
/// Corresponds to [Kubernetes RoleBinding](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L128)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoleBinding {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Subjects holds references to the objects the role applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,

    /// RoleRef can reference a Role in the current namespace or a ClusterRole.
    pub role_ref: RoleRef,
}

/// RoleBindingList is a collection of RoleBindings.
///
/// Corresponds to [Kubernetes RoleBindingList](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L152)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoleBindingList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of RoleBindings.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RoleBinding>,
}

/// ClusterRole is a cluster level, logical grouping of PolicyRules.
///
/// Corresponds to [Kubernetes ClusterRole](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L180)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRole {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Rules holds all the PolicyRules for this ClusterRole.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<PolicyRule>,

    /// AggregationRule is an optional field that describes how to build the Rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation_rule: Option<AggregationRule>,
}

/// ClusterRoleList is a collection of ClusterRoles.
///
/// Corresponds to [Kubernetes ClusterRoleList](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L250)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of ClusterRoles.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleBinding {
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Subjects holds references to the objects the role applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,

    /// RoleRef can only reference a ClusterRole in the global namespace.
    pub role_ref: RoleRef,
}

/// ClusterRoleBindingList is a collection of ClusterRoleBindings.
///
/// Corresponds to [Kubernetes ClusterRoleBindingList](https://github.com/kubernetes/api/blob/master/rbac/v1/types.go#L237)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleBindingList {
    /// Standard list metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListMeta>,

    /// Items is a list of ClusterRoleBindings.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
mod tests {
    use super::*;

    #[test]
    fn test_policy_rule_default() {
        let rule = PolicyRule::default();
        assert!(rule.verbs.is_empty());
        assert!(rule.api_groups.is_empty());
        assert!(rule.resources.is_empty());
        assert!(rule.resource_names.is_empty());
        assert!(rule.non_resource_urls.is_empty());
    }

    #[test]
    fn test_policy_rule_with_verbs() {
        let rule = PolicyRule {
            verbs: vec!["get".to_string(), "list".to_string(), "watch".to_string()],
            api_groups: vec![api_group::CORE.to_string()],
            resources: vec!["pods".to_string()],
            ..Default::default()
        };
        assert_eq!(rule.verbs.len(), 3);
        assert_eq!(rule.verbs[0], "get");
    }

    #[test]
    fn test_policy_rule_serialize() {
        let rule = PolicyRule {
            verbs: vec!["get".to_string()],
            api_groups: vec![],
            resources: vec!["pods".to_string()],
            resource_names: vec![],
            non_resource_urls: vec![],
        };
        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("\"verbs\":[\"get\"]"));
        assert!(json.contains("\"resources\":[\"pods\"]"));
    }

    #[test]
    fn test_policy_rule_deserialize() {
        let json = r#"{"verbs":["get"],"resources":["pods"]}"#;
        let rule: PolicyRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.verbs.len(), 1);
        assert_eq!(rule.verbs[0], "get");
        assert_eq!(rule.resources[0], "pods");
    }

    #[test]
    fn test_subject_default() {
        let subject = Subject {
            kind: subject_kind::USER.to_string(),
            api_group: api_group::RBAC.to_string(),
            name: "jane".to_string(),
            namespace: String::new(),
        };
        assert_eq!(subject.kind, "User");
        assert_eq!(subject.api_group, "rbac.authorization.k8s.io");
        assert_eq!(subject.name, "jane");
        assert!(subject.namespace.is_empty());
    }

    #[test]
    fn test_subject_serialize() {
        let subject = Subject {
            kind: subject_kind::SERVICE_ACCOUNT.to_string(),
            api_group: String::new(),
            name: "my-sa".to_string(),
            namespace: "default".to_string(),
        };
        let json = serde_json::to_string(&subject).unwrap();
        assert!(json.contains("\"kind\":\"ServiceAccount\""));
        assert!(json.contains("\"name\":\"my-sa\""));
        assert!(json.contains("\"namespace\":\"default\""));
    }

    #[test]
    fn test_subject_deserialize() {
        let json = r#"{"kind":"User","name":"john","apiGroup":"rbac.authorization.k8s.io"}"#;
        let subject: Subject = serde_json::from_str(json).unwrap();
        assert_eq!(subject.kind, "User");
        assert_eq!(subject.name, "john");
        assert_eq!(subject.api_group, "rbac.authorization.k8s.io");
    }

    #[test]
    fn test_role_ref() {
        let role_ref = RoleRef {
            api_group: api_group::RBAC.to_string(),
            kind: "Role".to_string(),
            name: "my-role".to_string(),
        };
        assert_eq!(role_ref.api_group, "rbac.authorization.k8s.io");
        assert_eq!(role_ref.kind, "Role");
        assert_eq!(role_ref.name, "my-role");
    }

    #[test]
    fn test_role_ref_serialize() {
        let role_ref = RoleRef {
            api_group: "rbac.authorization.k8s.io".to_string(),
            kind: "ClusterRole".to_string(),
            name: "admin".to_string(),
        };
        let json = serde_json::to_string(&role_ref).unwrap();
        assert!(json.contains("\"apiGroup\":\"rbac.authorization.k8s.io\""));
        assert!(json.contains("\"kind\":\"ClusterRole\""));
        assert!(json.contains("\"name\":\"admin\""));
    }

    #[test]
    fn test_role_ref_deserialize() {
        let json = r#"{"apiGroup":"rbac.authorization.k8s.io","kind":"Role","name":"reader"}"#;
        let role_ref: RoleRef = serde_json::from_str(json).unwrap();
        assert_eq!(role_ref.kind, "Role");
        assert_eq!(role_ref.name, "reader");
    }

    #[test]
    fn test_role_default() {
        let role = Role {
            metadata: None,
            rules: vec![],
        };
        assert!(role.metadata.is_none());
        assert!(role.rules.is_empty());
    }

    #[test]
    fn test_role_with_rules() {
        let role = Role {
            metadata: Some(ObjectMeta {
                name: Some("pod-reader".to_string()),
                ..Default::default()
            }),
            rules: vec![PolicyRule {
                verbs: vec!["get".to_string(), "list".to_string()],
                api_groups: vec![api_group::CORE.to_string()],
                resources: vec!["pods".to_string()],
                ..Default::default()
            }],
        };
        assert_eq!(role.rules.len(), 1);
        assert_eq!(role.rules[0].verbs.len(), 2);
    }

    #[test]
    fn test_role_serialize() {
        let role = Role {
            metadata: Some(ObjectMeta {
                name: Some("my-role".to_string()),
                ..Default::default()
            }),
            rules: vec![],
        };
        let json = serde_json::to_string(&role).unwrap();
        assert!(json.contains("\"name\":\"my-role\""));
    }

    #[test]
    fn test_role_deserialize() {
        let json = r#"{"metadata":{"name":"test-role"},"rules":[]}"#;
        let role: Role = serde_json::from_str(json).unwrap();
        assert_eq!(
            role.metadata.as_ref().unwrap().name,
            Some("test-role".to_string())
        );
    }

    #[test]
    fn test_role_list() {
        let list = RoleList {
            metadata: Some(ListMeta {
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            items: vec![Role {
                metadata: Some(ObjectMeta {
                    name: Some("role1".to_string()),
                    ..Default::default()
                }),
                rules: vec![],
            }],
        };
        assert_eq!(list.items.len(), 1);
        assert_eq!(
            list.metadata.as_ref().unwrap().resource_version,
            Some("12345".to_string())
        );
    }

    #[test]
    fn test_role_binding_with_subjects() {
        let binding = RoleBinding {
            metadata: Some(ObjectMeta {
                name: Some("read-pods".to_string()),
                ..Default::default()
            }),
            subjects: vec![Subject {
                kind: subject_kind::USER.to_string(),
                api_group: api_group::RBAC.to_string(),
                name: "jane".to_string(),
                namespace: String::new(),
            }],
            role_ref: RoleRef {
                api_group: api_group::RBAC.to_string(),
                kind: "Role".to_string(),
                name: "pod-reader".to_string(),
            },
        };
        assert_eq!(binding.subjects.len(), 1);
        assert_eq!(binding.role_ref.name, "pod-reader");
    }

    #[test]
    fn test_role_binding_serialize() {
        let binding = RoleBinding {
            metadata: Some(ObjectMeta {
                name: Some("my-binding".to_string()),
                ..Default::default()
            }),
            subjects: vec![],
            role_ref: RoleRef {
                api_group: "rbac.authorization.k8s.io".to_string(),
                kind: "Role".to_string(),
                name: "my-role".to_string(),
            },
        };
        let json = serde_json::to_string(&binding).unwrap();
        assert!(json.contains("\"name\":\"my-binding\""));
        assert!(json.contains("\"roleRef\""));
    }

    #[test]
    fn test_role_binding_deserialize() {
        let json = r#"{
            "metadata": {"name": "test-binding"},
            "subjects": [],
            "roleRef": {"apiGroup": "rbac.authorization.k8s.io", "kind": "Role", "name": "test"}
        }"#;
        let binding: RoleBinding = serde_json::from_str(json).unwrap();
        assert_eq!(binding.role_ref.name, "test");
    }

    #[test]
    fn test_cluster_role_with_aggregation() {
        let cluster_role = ClusterRole {
            metadata: Some(ObjectMeta {
                name: Some("aggregate-role".to_string()),
                ..Default::default()
            }),
            rules: vec![],
            aggregation_rule: Some(AggregationRule {
                cluster_role_selectors: vec![LabelSelector {
                    match_labels: {
                        let mut map = std::collections::HashMap::new();
                        map.insert(
                            "rbac.example.com/aggregate-to-admin".to_string(),
                            "true".to_string(),
                        );
                        map
                    },
                    match_expressions: vec![],
                }],
            }),
        };
        assert!(cluster_role.aggregation_rule.is_some());
        assert_eq!(
            cluster_role
                .aggregation_rule
                .as_ref()
                .unwrap()
                .cluster_role_selectors
                .len(),
            1
        );
    }

    #[test]
    fn test_cluster_role_serialize() {
        let cluster_role = ClusterRole {
            metadata: Some(ObjectMeta {
                name: Some("cluster-admin".to_string()),
                ..Default::default()
            }),
            rules: vec![PolicyRule {
                verbs: vec!["*".to_string()],
                api_groups: vec!["*".to_string()],
                resources: vec!["*".to_string()],
                ..Default::default()
            }],
            aggregation_rule: None,
        };
        let json = serde_json::to_string(&cluster_role).unwrap();
        assert!(json.contains("\"name\":\"cluster-admin\""));
        assert!(json.contains("\"verbs\":[\"*\"]"));
    }

    #[test]
    fn test_cluster_role_deserialize() {
        let json =
            r#"{"metadata":{"name":"admin"},"rules":[{"verbs":["get"],"resources":["pods"]}]}"#;
        let cluster_role: ClusterRole = serde_json::from_str(json).unwrap();
        assert_eq!(cluster_role.rules.len(), 1);
    }

    #[test]
    fn test_aggregation_rule_default() {
        let rule = AggregationRule::default();
        assert!(rule.cluster_role_selectors.is_empty());
    }

    #[test]
    fn test_aggregation_rule_with_selectors() {
        let rule = AggregationRule {
            cluster_role_selectors: vec![LabelSelector {
                match_labels: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("key".to_string(), "value".to_string());
                    map
                },
                match_expressions: vec![],
            }],
        };
        assert_eq!(rule.cluster_role_selectors.len(), 1);
    }

    #[test]
    fn test_cluster_role_binding() {
        let binding = ClusterRoleBinding {
            metadata: Some(ObjectMeta {
                name: Some("cluster-admin-binding".to_string()),
                ..Default::default()
            }),
            subjects: vec![Subject {
                kind: subject_kind::USER.to_string(),
                api_group: api_group::RBAC.to_string(),
                name: "admin".to_string(),
                namespace: String::new(),
            }],
            role_ref: RoleRef {
                api_group: api_group::RBAC.to_string(),
                kind: "ClusterRole".to_string(),
                name: "cluster-admin".to_string(),
            },
        };
        assert_eq!(binding.subjects.len(), 1);
        assert_eq!(binding.role_ref.kind, "ClusterRole");
    }

    #[test]
    fn test_subject_kind_constants() {
        assert_eq!(subject_kind::GROUP, "Group");
        assert_eq!(subject_kind::SERVICE_ACCOUNT, "ServiceAccount");
        assert_eq!(subject_kind::USER, "User");
    }

    #[test]
    fn test_api_group_constants() {
        assert_eq!(api_group::CORE, "");
        assert_eq!(api_group::RBAC, "rbac.authorization.k8s.io");
    }

    #[test]
    fn test_auto_update_annotation_key() {
        assert_eq!(
            AUTO_UPDATE_ANNOTATION_KEY,
            "rbac.authorization.kubernetes.io/autoupdate"
        );
    }

    #[test]
    fn test_policy_rule_round_trip() {
        let original = PolicyRule {
            verbs: vec!["get".to_string(), "list".to_string()],
            api_groups: vec![api_group::CORE.to_string()],
            resources: vec!["pods".to_string(), "services".to_string()],
            resource_names: vec![],
            non_resource_urls: vec![],
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PolicyRule = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_subject_round_trip() {
        let original = Subject {
            kind: subject_kind::SERVICE_ACCOUNT.to_string(),
            api_group: String::new(),
            name: "my-sa".to_string(),
            namespace: "default".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Subject = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_role_round_trip() {
        let original = Role {
            metadata: Some(ObjectMeta {
                name: Some("test-role".to_string()),
                ..Default::default()
            }),
            rules: vec![PolicyRule {
                verbs: vec!["*".to_string()],
                api_groups: vec!["*".to_string()],
                resources: vec!["*".to_string()],
                ..Default::default()
            }],
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Role = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_cluster_role_round_trip() {
        let original = ClusterRole {
            metadata: Some(ObjectMeta {
                name: Some("test-cluster-role".to_string()),
                ..Default::default()
            }),
            rules: vec![],
            aggregation_rule: Some(AggregationRule {
                cluster_role_selectors: vec![],
            }),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ClusterRole = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_role_binding_round_trip() {
        let original = RoleBinding {
            metadata: Some(ObjectMeta {
                name: Some("test-binding".to_string()),
                ..Default::default()
            }),
            subjects: vec![Subject {
                kind: subject_kind::USER.to_string(),
                api_group: api_group::RBAC.to_string(),
                name: "test-user".to_string(),
                namespace: String::new(),
            }],
            role_ref: RoleRef {
                api_group: api_group::RBAC.to_string(),
                kind: "Role".to_string(),
                name: "test-role".to_string(),
            },
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: RoleBinding = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_cluster_role_binding_round_trip() {
        let original = ClusterRoleBinding {
            metadata: Some(ObjectMeta {
                name: Some("test-cluster-binding".to_string()),
                ..Default::default()
            }),
            subjects: vec![],
            role_ref: RoleRef {
                api_group: api_group::RBAC.to_string(),
                kind: "ClusterRole".to_string(),
                name: "admin".to_string(),
            },
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ClusterRoleBinding = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
