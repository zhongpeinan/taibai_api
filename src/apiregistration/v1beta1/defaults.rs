//! Default values for apiregistration v1beta1 API types
//!
//! Ported from k8s.io/kube-aggregator/pkg/apis/apiregistration/v1beta1/defaults.go

use super::{APIService, APIServiceList, ServiceReference};
use crate::common::ApplyDefault;

fn set_defaults_service_reference(obj: &mut ServiceReference) {
    if obj.port.is_none() {
        obj.port = Some(443);
    }
}

impl ApplyDefault for APIService {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apiregistration.k8s.io/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "APIService".to_string();
        }

        if let Some(service) = self.spec.service.as_mut() {
            set_defaults_service_reference(service);
        }
    }
}

impl ApplyDefault for APIServiceList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apiregistration.k8s.io/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "APIServiceList".to_string();
        }

        for item in &mut self.items {
            item.apply_default();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_service_defaults_port() {
        let mut svc = APIService::default();
        svc.spec.service = Some(ServiceReference::default());
        svc.apply_default();
        assert_eq!(svc.spec.service.unwrap().port, Some(443));
    }
}
