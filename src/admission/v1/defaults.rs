//! Default values for admission v1 API types
//!
//! Ported from k8s/pkg/apis/admission/v1/zz_generated.defaults.go

use crate::common::ApplyDefault;

use super::AdmissionReview;

impl ApplyDefault for AdmissionReview {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "admission.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "AdmissionReview".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admission_review_apply_default() {
        let mut review = AdmissionReview::default();
        review.apply_default();

        assert_eq!(review.type_meta.api_version, "admission.k8s.io/v1");
        assert_eq!(review.type_meta.kind, "AdmissionReview");
    }

    #[test]
    fn test_admission_review_preserves_existing_values() {
        use crate::common::TypeMeta;
        let mut review = AdmissionReview {
            type_meta: TypeMeta {
                api_version: "custom.api/v1".to_string(),
                kind: "CustomKind".to_string(),
            },
            ..Default::default()
        };
        review.apply_default();

        // Existing values should not be overwritten
        assert_eq!(review.type_meta.api_version, "custom.api/v1");
        assert_eq!(review.type_meta.kind, "CustomKind");
    }
}
