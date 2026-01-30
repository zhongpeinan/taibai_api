//! Validation for Kubernetes APIRegistration API types
//!
//! Ported from k8s.io/kube-aggregator/pkg/apis/apiregistration/validation/validation.go

use crate::apiregistration::internal;
use crate::common::ObjectMeta;
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, is_dns1035_label, is_dns1123_subdomain,
    not_supported, required, validate_object_meta_update,
};

// ============================================================================
// Public Validation Entry Points
// ============================================================================

pub fn validate_api_service(api_service: &internal::APIService) -> ErrorList {
    let required_name = format!("{}.{}", api_service.spec.version, api_service.spec.group);
    let mut all_errs = validate_api_service_object_meta(
        &api_service.metadata,
        &required_name,
        &Path::new("metadata"),
    );

    if api_service.spec.group.is_empty() && api_service.spec.version != "v1" {
        all_errs.push(required(
            &Path::new("spec").child("group"),
            "only v1 may have an empty group and it better be legacy kube",
        ));
    }
    if !api_service.spec.group.is_empty() {
        for msg in is_dns1123_subdomain(&api_service.spec.group) {
            all_errs.push(invalid(
                &Path::new("spec").child("group"),
                BadValue::String(api_service.spec.group.clone()),
                &msg,
            ));
        }
    }

    for msg in is_dns1035_label(&api_service.spec.version) {
        all_errs.push(invalid(
            &Path::new("spec").child("version"),
            BadValue::String(api_service.spec.version.clone()),
            &msg,
        ));
    }

    if api_service.spec.group_priority_minimum <= 0
        || api_service.spec.group_priority_minimum > 20000
    {
        all_errs.push(invalid(
            &Path::new("spec").child("groupPriorityMinimum"),
            BadValue::Int(api_service.spec.group_priority_minimum as i64),
            "must be positive and less than 20000",
        ));
    }
    if api_service.spec.version_priority <= 0 || api_service.spec.version_priority > 1000 {
        all_errs.push(invalid(
            &Path::new("spec").child("versionPriority"),
            BadValue::Int(api_service.spec.version_priority as i64),
            "must be positive and less than 1000",
        ));
    }

    if api_service.spec.service.is_none() {
        if !api_service.spec.ca_bundle.0.is_empty() {
            all_errs.push(invalid(
                &Path::new("spec").child("caBundle"),
                BadValue::String(format!("{} bytes", api_service.spec.ca_bundle.0.len())),
                "local APIServices may not have a caBundle",
            ));
        }
        if api_service.spec.insecure_skip_tls_verify {
            all_errs.push(invalid(
                &Path::new("spec").child("insecureSkipTLSVerify"),
                BadValue::Bool(api_service.spec.insecure_skip_tls_verify),
                "local APIServices may not have insecureSkipTLSVerify",
            ));
        }
        return all_errs;
    }

    let service = api_service.spec.service.as_ref().unwrap();
    if service.namespace.is_empty() {
        all_errs.push(required(
            &Path::new("spec").child("service").child("namespace"),
            "",
        ));
    }
    if service.name.is_empty() {
        all_errs.push(required(
            &Path::new("spec").child("service").child("name"),
            "",
        ));
    }
    let port_errs = is_valid_port_num(service.port);
    if !port_errs.is_empty() {
        all_errs.push(invalid(
            &Path::new("spec").child("service").child("port"),
            BadValue::Int(service.port as i64),
            &format!("port is not valid: {}", port_errs.join(", ")),
        ));
    }
    if api_service.spec.insecure_skip_tls_verify && !api_service.spec.ca_bundle.0.is_empty() {
        all_errs.push(invalid(
            &Path::new("spec").child("insecureSkipTLSVerify"),
            BadValue::Bool(api_service.spec.insecure_skip_tls_verify),
            "may not be true if caBundle is present",
        ));
    }

    all_errs
}

pub fn validate_api_service_update(
    new_api_service: &internal::APIService,
    old_api_service: &internal::APIService,
) -> ErrorList {
    let mut all_errs = validate_object_meta_update(
        &new_api_service.metadata,
        &old_api_service.metadata,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_api_service(new_api_service));
    all_errs
}

pub fn validate_api_service_status(
    status: &internal::APIServiceStatus,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, condition) in status.conditions.iter().enumerate() {
        if condition.status != internal::ConditionStatus::True
            && condition.status != internal::ConditionStatus::False
            && condition.status != internal::ConditionStatus::Unknown
        {
            all_errs.push(not_supported(
                &fld_path.child("conditions").index(i).child("status"),
                BadValue::String(format!("{:?}", condition.status)),
                &["True", "False", "Unknown"],
            ));
        }
    }

    all_errs
}

pub fn validate_api_service_status_update(
    new_api_service: &internal::APIService,
    old_api_service: &internal::APIService,
) -> ErrorList {
    let mut all_errs = validate_object_meta_update(
        &new_api_service.metadata,
        &old_api_service.metadata,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_api_service_status(
        &new_api_service.status,
        &Path::new("status"),
    ));
    all_errs
}

// ============================================================================
// Helper Functions
// ============================================================================

fn validate_api_service_object_meta(
    meta: &ObjectMeta,
    required_name: &str,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(generate_name) = meta.generate_name.as_deref()
        && !generate_name.is_empty()
    {
        for msg in validate_path_segment_name(generate_name, true) {
            all_errs.push(invalid(
                &fld_path.child("generateName"),
                BadValue::String(generate_name.to_string()),
                &msg,
            ));
        }
        if generate_name != required_name {
            all_errs.push(invalid(
                &fld_path.child("generateName"),
                BadValue::String(generate_name.to_string()),
                &format!(
                    "must be `spec.version+\".\"+spec.group`: \"{}\"",
                    required_name
                ),
            ));
        }
    }

    let name = meta.name.as_deref().unwrap_or("");
    if name.is_empty() {
        all_errs.push(required(
            &fld_path.child("name"),
            "name or generateName is required",
        ));
    } else {
        for msg in validate_path_segment_name(name, false) {
            all_errs.push(invalid(
                &fld_path.child("name"),
                BadValue::String(name.to_string()),
                &msg,
            ));
        }
        if name != required_name {
            all_errs.push(invalid(
                &fld_path.child("name"),
                BadValue::String(name.to_string()),
                &format!(
                    "must be `spec.version+\".\"+spec.group`: \"{}\"",
                    required_name
                ),
            ));
        }
    }

    if let Some(namespace) = meta.namespace.as_deref()
        && !namespace.is_empty()
    {
        all_errs.push(forbidden(
            &fld_path.child("namespace"),
            "not allowed on this type",
        ));
    }

    if let Some(generation) = meta.generation
        && generation < 0
    {
        all_errs.push(invalid(
            &fld_path.child("generation"),
            BadValue::Int(generation),
            "must be greater than or equal to 0",
        ));
    }

    all_errs
}

fn validate_path_segment_name(name: &str, _prefix: bool) -> Vec<String> {
    let mut errs = Vec::new();
    if name == "." || name == ".." {
        errs.push(format!("may not be '{}'", name));
    }
    if name.contains('/') {
        errs.push("may not contain '/'".to_string());
    }
    if name.contains('%') {
        errs.push("may not contain '%'".to_string());
    }
    errs
}

fn is_valid_port_num(port: i32) -> Vec<String> {
    if (1..=65535).contains(&port) {
        return Vec::new();
    }
    vec![format!("must be between {} and {}, inclusive", 1, 65535)]
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Timestamp, TypeMeta};

    #[test]
    fn validate_api_service_rejects_invalid_name() {
        let api_service = internal::APIService {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("wrong.name".to_string()),
                ..Default::default()
            },
            spec: internal::APIServiceSpec {
                group: "apps".to_string(),
                version: "v1".to_string(),
                group_priority_minimum: 1000,
                version_priority: 20,
                ..Default::default()
            },
            status: internal::APIServiceStatus {
                conditions: vec![internal::APIServiceCondition {
                    type_: internal::APIServiceConditionType::Available,
                    status: internal::ConditionStatus::True,
                    last_transition_time: Timestamp::zero(),
                    reason: String::new(),
                    message: String::new(),
                }],
            },
        };

        let errs = validate_api_service(&api_service);
        assert!(!errs.is_empty());
    }
}
