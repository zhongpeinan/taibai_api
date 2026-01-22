//! ServiceCIDR types from the Kubernetes Networking API
//!
//! This module contains types for service CIDR resources.
//!
//! Source: k8s.io/api/networking/v1alpha1/types.go

use crate::common::{ListMeta, ObjectMeta, TypeMeta};
use crate::impl_versioned_object;
use serde::{Deserialize, Serialize};

// ============================================================================
// ServiceCIDR
// ============================================================================

/// ServiceCIDR defines a range of IP addresses.
///
/// Corresponds to [Kubernetes ServiceCIDR](https://github.com/kubernetes/api/blob/master/networking/v1alpha1/types.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDR {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,
    /// spec is the desired state of the ServiceCIDR.
    #[serde(default)]
    pub spec: ServiceCIDRSpec,
}
impl_versioned_object!(ServiceCIDR);

impl Default for ServiceCIDR {
    fn default() -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: ServiceCIDRSpec::default(),
        }
    }
}

/// ServiceCIDRList is a list of ServiceCIDR objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRList {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Standard list metadata.
    #[serde(default)]
    pub metadata: Option<ListMeta>,
    /// Items is the list of ServiceCIDR objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceCIDR>,
}

// ============================================================================
// ServiceCIDRSpec
// ============================================================================

/// ServiceCIDRSpec defines the desired state of ServiceCIDR.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRSpec {
    /// CIDRs defines the IP addresses in CIDR notation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cidrs: Vec<String>,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl crate::common::traits::ResourceSchema for ServiceCIDR {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1alpha1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "ServiceCIDR"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "servicecidrs"
    }

    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "networking.k8s.io"
    }

    fn version_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "v1alpha1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "ServiceCIDR"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "servicecidrs"
    }
}

impl crate::common::traits::ResourceSchema for ServiceCIDRList {
    type Meta = ();

    fn group(_meta: &Self::Meta) -> &str {
        "networking.k8s.io"
    }

    fn version(_meta: &Self::Meta) -> &str {
        "v1alpha1"
    }

    fn kind(_meta: &Self::Meta) -> &str {
        "ServiceCIDRList"
    }

    fn resource(_meta: &Self::Meta) -> &str {
        "servicecidrs"
    }

    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "networking.k8s.io"
    }

    fn version_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "v1alpha1"
    }

    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "ServiceCIDRList"
    }

    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        "servicecidrs"
    }
}

impl crate::common::traits::HasTypeMeta for ServiceCIDR {
    fn type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    fn type_meta_mut(&mut self) -> &mut TypeMeta {
        &mut self.type_meta
    }
}

impl crate::common::traits::ApplyDefault for ServiceCIDR {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ServiceCIDR".to_string();
        }
    }
}

impl crate::common::traits::ApplyDefault for ServiceCIDRList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "networking.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ServiceCIDRList".to_string();
        }
    }
}

impl crate::common::traits::UnimplementedConversion for ServiceCIDR {}
impl crate::common::traits::UnimplementedConversion for ServiceCIDRList {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_cidr_default() {
        let sc = ServiceCIDR::default();
        assert!(sc.metadata.is_none());
        assert!(sc.spec.cidrs.is_empty());
    }

    #[test]
    fn test_service_cidr_serialize() {
        let sc = ServiceCIDR {
            type_meta: TypeMeta {
                kind: "ServiceCIDR".to_string(),
                api_version: "networking.k8s.io/v1alpha1".to_string(),
            },
            metadata: Some(ObjectMeta {
                name: Some("test-cidr".to_string()),
                ..Default::default()
            }),
            spec: ServiceCIDRSpec {
                cidrs: vec!["192.168.0.0/16".to_string()],
            },
        };
        let json = serde_json::to_string(&sc).unwrap();
        assert!(json.contains(r#""kind":"ServiceCIDR""#));
        assert!(json.contains("192.168.0.0/16"));
    }

    #[test]
    fn test_service_cidr_apply_default() {
        let mut sc = ServiceCIDR {
            type_meta: TypeMeta::default(),
            ..Default::default()
        };
        sc.apply_default();
        assert_eq!(sc.type_meta.api_version, "networking.k8s.io/v1alpha1");
        assert_eq!(sc.type_meta.kind, "ServiceCIDR");
    }

    #[test]
    fn test_service_cidr_list_default() {
        let list = ServiceCIDRList::default();
        assert!(list.items.is_empty());
    }
}
