//! Validation for Kubernetes Authorization API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/authorization/validation.

use crate::authorization::v1::{
    FieldSelectorAttributes, LabelSelectorAttributes, LocalSubjectAccessReview, ResourceAttributes,
    SelfSubjectAccessReview, SelfSubjectAccessReviewSpec, SubjectAccessReview,
    SubjectAccessReviewSpec,
};
use crate::common::ObjectMeta;
use crate::common::meta::{field_selector_operator, label_selector_operator};
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, required, validate_qualified_name,
};

// ============================================================================
// SubjectAccessReview Validation
// ============================================================================

pub fn validate_subject_access_review(obj: &SubjectAccessReview) -> ErrorList {
    let mut all_errs = validate_subject_access_review_spec(&obj.spec, &Path::new("spec"));
    all_errs.extend(validate_metadata_empty(
        obj.metadata.as_ref(),
        false,
        &Path::new("metadata"),
    ));
    all_errs
}

pub fn validate_self_subject_access_review(obj: &SelfSubjectAccessReview) -> ErrorList {
    let mut all_errs = validate_self_subject_access_review_spec(&obj.spec, &Path::new("spec"));
    all_errs.extend(validate_metadata_empty(
        obj.metadata.as_ref(),
        false,
        &Path::new("metadata"),
    ));
    all_errs
}

pub fn validate_local_subject_access_review(obj: &LocalSubjectAccessReview) -> ErrorList {
    let mut all_errs = validate_subject_access_review_spec(&obj.spec, &Path::new("spec"));
    all_errs.extend(validate_metadata_empty(
        obj.metadata.as_ref(),
        true,
        &Path::new("metadata"),
    ));

    if let Some(resource_attributes) = obj.spec.resource_attributes.as_ref() {
        let namespace = obj
            .metadata
            .as_ref()
            .and_then(|meta| meta.namespace.clone())
            .unwrap_or_default();
        if resource_attributes.namespace != namespace {
            all_errs.push(invalid(
                &Path::new("spec")
                    .child("resourceAttributes")
                    .child("namespace"),
                BadValue::String(resource_attributes.namespace.clone()),
                "must match metadata.namespace",
            ));
        }
    }

    if let Some(non_resource_attributes) = obj.spec.non_resource_attributes.as_ref() {
        all_errs.push(invalid(
            &Path::new("spec").child("nonResourceAttributes"),
            BadValue::String(format!("{:?}", non_resource_attributes)),
            "disallowed on this kind of request",
        ));
    }

    all_errs
}

fn validate_subject_access_review_spec(
    spec: &SubjectAccessReviewSpec,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let resource_path = fld_path.child("resourceAttributes");
    let non_resource_path = fld_path.child("nonResourceAttributes");

    if spec.resource_attributes.is_some() && spec.non_resource_attributes.is_some() {
        all_errs.push(invalid(
            &non_resource_path,
            BadValue::String("set".to_string()),
            "cannot be specified in combination with resourceAttributes",
        ));
    }
    if spec.resource_attributes.is_none() && spec.non_resource_attributes.is_none() {
        all_errs.push(invalid(
            &resource_path,
            BadValue::String("null".to_string()),
            "exactly one of nonResourceAttributes or resourceAttributes must be specified",
        ));
    }
    if spec.user.is_empty() && spec.groups.is_empty() {
        all_errs.push(invalid(
            &fld_path.child("user"),
            BadValue::String(spec.user.clone()),
            "at least one of user or group must be specified",
        ));
    }

    all_errs.extend(validate_resource_attributes(
        spec.resource_attributes.as_ref(),
        &resource_path,
    ));

    all_errs
}

fn validate_self_subject_access_review_spec(
    spec: &SelfSubjectAccessReviewSpec,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let resource_path = fld_path.child("resourceAttributes");
    let non_resource_path = fld_path.child("nonResourceAttributes");

    if spec.resource_attributes.is_some() && spec.non_resource_attributes.is_some() {
        all_errs.push(invalid(
            &non_resource_path,
            BadValue::String("set".to_string()),
            "cannot be specified in combination with resourceAttributes",
        ));
    }
    if spec.resource_attributes.is_none() && spec.non_resource_attributes.is_none() {
        all_errs.push(invalid(
            &resource_path,
            BadValue::String("null".to_string()),
            "exactly one of nonResourceAttributes or resourceAttributes must be specified",
        ));
    }

    all_errs.extend(validate_resource_attributes(
        spec.resource_attributes.as_ref(),
        &resource_path,
    ));

    all_errs
}

fn validate_metadata_empty(
    metadata: Option<&ObjectMeta>,
    allow_namespace: bool,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut meta_copy = metadata.cloned().unwrap_or_default();
    meta_copy.managed_fields.clear();
    if allow_namespace {
        meta_copy.namespace = None;
    }

    if meta_copy != ObjectMeta::default() {
        let detail = if allow_namespace {
            "must be empty except for namespace"
        } else {
            "must be empty"
        };
        all_errs.push(invalid(
            fld_path,
            BadValue::String("metadata".to_string()),
            detail,
        ));
    }

    all_errs
}

fn validate_resource_attributes(
    resource_attributes: Option<&ResourceAttributes>,
    fld_path: &Path,
) -> ErrorList {
    let Some(resource_attributes) = resource_attributes else {
        return ErrorList::new();
    };
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_field_selector_attributes(
        resource_attributes.field_selector.as_ref(),
        &fld_path.child("fieldSelector"),
    ));
    all_errs.extend(validate_label_selector_attributes(
        resource_attributes.label_selector.as_ref(),
        &fld_path.child("labelSelector"),
    ));

    all_errs
}

fn validate_field_selector_attributes(
    selector: Option<&FieldSelectorAttributes>,
    fld_path: &Path,
) -> ErrorList {
    let Some(selector) = selector else {
        return ErrorList::new();
    };
    let mut all_errs = ErrorList::new();

    if !selector.raw_selector.is_empty() && !selector.requirements.is_empty() {
        all_errs.push(invalid(
            &fld_path.child("rawSelector"),
            BadValue::String(selector.raw_selector.clone()),
            "may not specified at the same time as requirements",
        ));
    }
    if selector.raw_selector.is_empty() && selector.requirements.is_empty() {
        all_errs.push(required(
            &fld_path.child("requirements"),
            &format!(
                "when {} is specified, requirements or rawSelector is required",
                fld_path
            ),
        ));
    }

    for (i, requirement) in selector.requirements.iter().enumerate() {
        let req_path = fld_path.child("requirements").index(i);
        all_errs.extend(validate_selector_requirement(
            requirement.key.as_str(),
            requirement.operator.as_str(),
            &requirement.values,
            &req_path,
            true,
            &[
                field_selector_operator::IN,
                field_selector_operator::NOT_IN,
                field_selector_operator::EXISTS,
                field_selector_operator::DOES_NOT_EXIST,
            ],
        ));
    }

    all_errs
}

fn validate_label_selector_attributes(
    selector: Option<&LabelSelectorAttributes>,
    fld_path: &Path,
) -> ErrorList {
    let Some(selector) = selector else {
        return ErrorList::new();
    };
    let mut all_errs = ErrorList::new();

    if !selector.raw_selector.is_empty() && !selector.requirements.is_empty() {
        all_errs.push(invalid(
            &fld_path.child("rawSelector"),
            BadValue::String(selector.raw_selector.clone()),
            "may not specified at the same time as requirements",
        ));
    }
    if selector.raw_selector.is_empty() && selector.requirements.is_empty() {
        all_errs.push(required(
            &fld_path.child("requirements"),
            &format!(
                "when {} is specified, requirements or rawSelector is required",
                fld_path
            ),
        ));
    }

    for (i, requirement) in selector.requirements.iter().enumerate() {
        let req_path = fld_path.child("requirements").index(i);
        all_errs.extend(validate_selector_requirement(
            requirement.key.as_str(),
            requirement.operator.as_str(),
            &requirement.values,
            &req_path,
            true,
            &[
                label_selector_operator::IN,
                label_selector_operator::NOT_IN,
                label_selector_operator::EXISTS,
                label_selector_operator::DOES_NOT_EXIST,
            ],
        ));
    }

    all_errs
}

fn validate_selector_requirement(
    key: &str,
    operator: &str,
    values: &[String],
    fld_path: &Path,
    allow_unknown_operator: bool,
    known_operators: &[&str],
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if key.is_empty() {
        all_errs.push(required(&fld_path.child("key"), "must be non-empty"));
    } else {
        all_errs.extend(validate_qualified_name(key, &fld_path.child("key")));
    }

    if operator.is_empty() {
        all_errs.push(required(&fld_path.child("operator"), "must be non-empty"));
        return all_errs;
    }

    let is_known = known_operators.iter().any(|known| *known == operator);
    if !is_known && !allow_unknown_operator {
        all_errs.push(invalid(
            &fld_path.child("operator"),
            BadValue::String(operator.to_string()),
            "unsupported operator",
        ));
        return all_errs;
    }

    if is_known {
        let is_in =
            operator == label_selector_operator::IN || operator == field_selector_operator::IN;
        let is_not_in = operator == label_selector_operator::NOT_IN
            || operator == field_selector_operator::NOT_IN;
        let is_exists = operator == label_selector_operator::EXISTS
            || operator == field_selector_operator::EXISTS;
        let is_not_exists = operator == label_selector_operator::DOES_NOT_EXIST
            || operator == field_selector_operator::DOES_NOT_EXIST;

        if is_in || is_not_in {
            if values.is_empty() {
                all_errs.push(invalid(
                    &fld_path.child("values"),
                    BadValue::String(String::new()),
                    "must provide at least one value",
                ));
            }
        } else if is_exists || is_not_exists {
            if !values.is_empty() {
                all_errs.push(invalid(
                    &fld_path.child("values"),
                    BadValue::String(format!("{:?}", values)),
                    "must be empty for this operator",
                ));
            }
        }
    }

    all_errs
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authorization::v1::{
        FieldSelectorAttributes, LabelSelectorAttributes, NonResourceAttributes,
        ResourceAttributes, SubjectAccessReviewSpec,
    };

    #[test]
    fn test_validate_subject_access_review_spec_requires_attributes() {
        let spec = SubjectAccessReviewSpec {
            resource_attributes: None,
            non_resource_attributes: None,
            user: "alice".to_string(),
            groups: vec![],
            extra: Default::default(),
            uid: String::new(),
        };

        let errs = validate_subject_access_review_spec(&spec, &Path::new("spec"));
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_validate_subject_access_review_requires_user_or_group() {
        let spec = SubjectAccessReviewSpec {
            resource_attributes: Some(ResourceAttributes::default()),
            non_resource_attributes: None,
            user: String::new(),
            groups: vec![],
            extra: Default::default(),
            uid: String::new(),
        };

        let errs = validate_subject_access_review_spec(&spec, &Path::new("spec"));
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_validate_self_subject_access_review_metadata_must_be_empty() {
        let obj = SelfSubjectAccessReview {
            type_meta: Default::default(),
            metadata: Some(ObjectMeta {
                name: Some("name".to_string()),
                ..ObjectMeta::default()
            }),
            spec: SelfSubjectAccessReviewSpec {
                resource_attributes: Some(ResourceAttributes::default()),
                non_resource_attributes: None,
            },
            status: None,
        };

        let errs = validate_self_subject_access_review(&obj);
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_validate_local_subject_access_review_namespace_mismatch() {
        let obj = LocalSubjectAccessReview {
            type_meta: Default::default(),
            metadata: Some(ObjectMeta {
                namespace: Some("ns-a".to_string()),
                ..ObjectMeta::default()
            }),
            spec: SubjectAccessReviewSpec {
                resource_attributes: Some(ResourceAttributes {
                    namespace: "ns-b".to_string(),
                    ..ResourceAttributes::default()
                }),
                non_resource_attributes: None,
                user: "alice".to_string(),
                groups: vec![],
                extra: Default::default(),
                uid: String::new(),
            },
            status: None,
        };

        let errs = validate_local_subject_access_review(&obj);
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_validate_local_subject_access_review_non_resource_disallowed() {
        let obj = LocalSubjectAccessReview {
            type_meta: Default::default(),
            metadata: Some(ObjectMeta {
                namespace: Some("ns".to_string()),
                ..ObjectMeta::default()
            }),
            spec: SubjectAccessReviewSpec {
                resource_attributes: Some(ResourceAttributes::default()),
                non_resource_attributes: Some(NonResourceAttributes::default()),
                user: "alice".to_string(),
                groups: vec![],
                extra: Default::default(),
                uid: String::new(),
            },
            status: None,
        };

        let errs = validate_local_subject_access_review(&obj);
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_validate_selector_raw_and_requirements_mutually_exclusive() {
        let selector = FieldSelectorAttributes {
            raw_selector: "metadata.name=demo".to_string(),
            requirements: vec![crate::common::FieldSelectorRequirement {
                key: "metadata.name".to_string(),
                operator: field_selector_operator::IN.to_string(),
                values: vec!["demo".to_string()],
            }],
        };

        let attrs = ResourceAttributes {
            field_selector: Some(selector),
            ..ResourceAttributes::default()
        };

        let errs = validate_resource_attributes(
            Some(&attrs),
            &Path::new("spec").child("resourceAttributes"),
        );
        assert!(!errs.is_empty());
    }

    #[test]
    fn test_validate_label_selector_requirement_values() {
        let selector = LabelSelectorAttributes {
            raw_selector: String::new(),
            requirements: vec![crate::common::LabelSelectorRequirement {
                key: "app".to_string(),
                operator: label_selector_operator::IN.to_string(),
                values: vec![],
            }],
        };

        let attrs = ResourceAttributes {
            label_selector: Some(selector),
            ..ResourceAttributes::default()
        };

        let errs = validate_resource_attributes(
            Some(&attrs),
            &Path::new("spec").child("resourceAttributes"),
        );
        assert!(!errs.is_empty());
    }
}
