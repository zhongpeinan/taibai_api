//! Default values for apiextensions v1beta1 API types
//!
//! Ported from k8s.io/apiextensions-apiserver/pkg/apis/apiextensions/v1beta1/defaults.go

use super::{
    CustomResourceConversion, CustomResourceDefinition, CustomResourceDefinitionList,
    CustomResourceDefinitionSpec, CustomResourceDefinitionVersion, ServiceReference,
};
use crate::common::ApplyDefault;

fn set_defaults_custom_resource_definition_spec(obj: &mut CustomResourceDefinitionSpec) {
    if obj.names.singular.is_empty() {
        obj.names.singular = obj.names.kind.to_lowercase();
    }
    if obj.names.list_kind.is_empty() && !obj.names.kind.is_empty() {
        obj.names.list_kind = format!("{}List", obj.names.kind);
    }

    if obj.versions.is_empty() && !obj.version.is_empty() {
        obj.versions = vec![CustomResourceDefinitionVersion {
            name: obj.version.clone(),
            storage: true,
            served: true,
            ..Default::default()
        }];
    }

    if obj.version.is_empty() && !obj.versions.is_empty() {
        obj.version = obj.versions[0].name.clone();
    }

    if obj.conversion.is_none() {
        obj.conversion = Some(CustomResourceConversion {
            strategy: super::ConversionStrategyType::None,
            webhook_client_config: None,
            conversion_review_versions: Vec::new(),
        });
    }

    if let Some(conversion) = obj.conversion.as_mut()
        && conversion.strategy == super::ConversionStrategyType::Webhook
        && conversion.conversion_review_versions.is_empty()
    {
        conversion
            .conversion_review_versions
            .push("v1beta1".to_string());
    }

    if obj.preserve_unknown_fields.is_none() {
        obj.preserve_unknown_fields = Some(true);
    }
}

fn set_defaults_service_reference(obj: &mut ServiceReference) {
    if obj.port.is_none() {
        obj.port = Some(443);
    }
}

fn set_defaults_custom_resource_definition(obj: &mut CustomResourceDefinition) {
    if obj.status.stored_versions.is_empty() {
        for version in &obj.spec.versions {
            if version.storage {
                obj.status.stored_versions.push(version.name.clone());
                break;
            }
        }
    }
}

impl ApplyDefault for CustomResourceDefinition {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apiextensions.k8s.io/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CustomResourceDefinition".to_string();
        }

        set_defaults_custom_resource_definition_spec(&mut self.spec);
        set_defaults_custom_resource_definition(self);

        if let Some(conversion) = self.spec.conversion.as_mut() {
            if let Some(service) = conversion
                .webhook_client_config
                .as_mut()
                .and_then(|c| c.service.as_mut())
            {
                set_defaults_service_reference(service);
            }
        }
    }
}

impl ApplyDefault for CustomResourceDefinitionList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apiextensions.k8s.io/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CustomResourceDefinitionList".to_string();
        }

        for item in &mut self.items {
            item.apply_default();
        }
    }
}

impl ApplyDefault for super::ConversionReview {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apiextensions.k8s.io/v1beta1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "ConversionReview".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;

    #[test]
    fn test_custom_resource_definition_defaults() {
        let mut crd = CustomResourceDefinition {
            type_meta: TypeMeta::default(),
            metadata: None,
            spec: CustomResourceDefinitionSpec {
                names: super::super::CustomResourceDefinitionNames {
                    kind: "Widget".to_string(),
                    ..Default::default()
                },
                version: "v1".to_string(),
                ..Default::default()
            },
            status: Default::default(),
        };

        crd.apply_default();

        assert_eq!(crd.type_meta.api_version, "apiextensions.k8s.io/v1beta1");
        assert_eq!(crd.type_meta.kind, "CustomResourceDefinition");
        assert_eq!(crd.spec.names.singular, "widget");
        assert_eq!(crd.spec.names.list_kind, "WidgetList");
        assert!(!crd.spec.versions.is_empty());
        assert_eq!(crd.spec.version, "v1");
        assert_eq!(crd.status.stored_versions, vec!["v1".to_string()]);
        assert_eq!(crd.spec.preserve_unknown_fields, Some(true));
    }

    #[test]
    fn test_conversion_review_defaults() {
        let mut review = super::super::ConversionReview::default();
        review.apply_default();
        assert_eq!(review.type_meta.api_version, "apiextensions.k8s.io/v1beta1");
        assert_eq!(review.type_meta.kind, "ConversionReview");
    }
}
