//! Default values for certificates v1alpha1 API types

use crate::common::ApplyDefault;

use super::{
    ClusterTrustBundle, ClusterTrustBundleList, PodCertificateRequest, PodCertificateRequestList,
};

impl ApplyDefault for ClusterTrustBundle {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "certificates.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ClusterTrustBundle".to_string();
        }
    }
}

impl ApplyDefault for ClusterTrustBundleList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "certificates.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ClusterTrustBundleList".to_string();
        }
    }
}

impl ApplyDefault for PodCertificateRequest {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "certificates.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PodCertificateRequest".to_string();
        }
    }
}

impl ApplyDefault for PodCertificateRequestList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "certificates.k8s.io/v1alpha1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "PodCertificateRequestList".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;

    #[test]
    fn test_bundle_apply_default_sets_type_meta() {
        let mut bundle = ClusterTrustBundle::default();
        bundle.apply_default();

        assert_eq!(bundle.type_meta.api_version, "certificates.k8s.io/v1alpha1");
        assert_eq!(bundle.type_meta.kind, "ClusterTrustBundle");
    }

    #[test]
    fn test_pod_request_apply_default_preserves_type_meta() {
        let mut req = PodCertificateRequest {
            type_meta: TypeMeta {
                api_version: "custom.io/v1".to_string(),
                kind: "Custom".to_string(),
            },
            ..Default::default()
        };
        req.apply_default();

        assert_eq!(req.type_meta.api_version, "custom.io/v1");
        assert_eq!(req.type_meta.kind, "Custom");
    }
}
