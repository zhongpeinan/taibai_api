//! Default values for apiextensions v1 API types
//!
//! Ported from k8s.io/apiextensions-apiserver/pkg/apis/apiextensions/v1/defaults.go

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
    if obj.conversion.is_none() {
        obj.conversion = Some(CustomResourceConversion {
            strategy: super::ConversionStrategyType::None,
            webhook: None,
        });
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

fn set_defaults_custom_resource_definition_version(_obj: &mut CustomResourceDefinitionVersion) {}

impl ApplyDefault for CustomResourceDefinition {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apiextensions.k8s.io/v1".to_string();
        }
        if self.type_meta.kind.is_empty() {
            self.type_meta.kind = "CustomResourceDefinition".to_string();
        }

        set_defaults_custom_resource_definition_spec(&mut self.spec);
        set_defaults_custom_resource_definition(self);

        if let Some(conversion) = self.spec.conversion.as_mut() {
            if let Some(webhook) = conversion.webhook.as_mut() {
                if let Some(service) = webhook
                    .client_config
                    .as_mut()
                    .and_then(|c| c.service.as_mut())
                {
                    set_defaults_service_reference(service);
                }
            }
        }

        for version in &mut self.spec.versions {
            set_defaults_custom_resource_definition_version(version);
        }
    }
}

impl ApplyDefault for CustomResourceDefinitionList {
    fn apply_default(&mut self) {
        if self.type_meta.api_version.is_empty() {
            self.type_meta.api_version = "apiextensions.k8s.io/v1".to_string();
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
            self.type_meta.api_version = "apiextensions.k8s.io/v1".to_string();
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
                versions: vec![CustomResourceDefinitionVersion {
                    name: "v1".to_string(),
                    storage: true,
                    served: true,
                    ..Default::default()
                }],
                ..Default::default()
            },
            status: Default::default(),
        };

        crd.apply_default();

        assert_eq!(crd.type_meta.api_version, "apiextensions.k8s.io/v1");
        assert_eq!(crd.type_meta.kind, "CustomResourceDefinition");
        assert_eq!(crd.spec.names.singular, "widget");
        assert_eq!(crd.spec.names.list_kind, "WidgetList");
        assert!(crd.spec.conversion.is_some());
        assert_eq!(crd.status.stored_versions, vec!["v1".to_string()]);
    }

    #[test]
    fn test_conversion_review_defaults() {
        let mut review = super::super::ConversionReview::default();
        review.apply_default();
        assert_eq!(review.type_meta.api_version, "apiextensions.k8s.io/v1");
        assert_eq!(review.type_meta.kind, "ConversionReview");
    }
}
