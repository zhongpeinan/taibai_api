use std::collections::{HashMap, HashSet};

use crate::apiserverinternal::internal;
use crate::common::validation::{
    BadValue, Error, ErrorList, Path, duplicate, invalid, is_dns1035_label, is_dns1123_subdomain,
    name_is_dns_subdomain, required, validate_object_meta, validate_qualified_name,
};

const DNS1035_LABEL_FMT: &str = "[a-z]([-a-z0-9]*[a-z0-9])?";

pub fn validate_storage_version(sv: &internal::StorageVersion) -> ErrorList {
    let mut all_errs = validate_object_meta(
        &sv.metadata,
        false,
        validate_storage_version_name,
        &Path::new("metadata"),
    );
    all_errs.extend(validate_storage_version_status(
        &sv.status,
        &Path::new("status"),
    ));
    all_errs
}

pub fn validate_storage_version_name(name: &str, _prefix: bool) -> Vec<String> {
    let mut all_errs = Vec::new();
    let idx = name.rfind('.');
    let (group, resource) = match idx {
        Some(index) => (&name[..index], &name[index + 1..]),
        None => {
            all_errs.push("name must be in the form of <group>.<resource>".to_string());
            return all_errs;
        }
    };

    for msg in is_dns1123_subdomain(group) {
        all_errs.push(format!("the group segment {}", msg));
    }
    for msg in is_dns1035_label(resource) {
        all_errs.push(format!("the resource segment {}", msg));
    }

    all_errs
}

pub fn validate_storage_version_update(
    _sv: &internal::StorageVersion,
    _old_sv: &internal::StorageVersion,
) -> ErrorList {
    // Intentionally empty per upstream: StorageVersion update validation
    // only checks metadata, which is handled by validate_object_meta_update
    ErrorList::new()
}

pub fn validate_storage_version_status_update(
    sv: &internal::StorageVersion,
    _old_sv: &internal::StorageVersion,
) -> ErrorList {
    validate_storage_version_status(&sv.status, &Path::new("status"))
}

pub fn validate_storage_version_status(
    status: &internal::StorageVersionStatus,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut all_api_server_ids = HashSet::new();
    let storage_versions_path = fld_path.child("storageVersions");

    for (i, ssv) in status.storage_versions.iter().enumerate() {
        let ssv_path = storage_versions_path.index(i);
        if !all_api_server_ids.insert(ssv.api_server_id.clone()) {
            all_errs.push(duplicate(
                &ssv_path.child("apiServerID"),
                BadValue::String(ssv.api_server_id.clone()),
            ));
        }
        all_errs.extend(validate_server_storage_version(ssv, &ssv_path));
    }

    if let Some(err) = validate_common_version(status, fld_path) {
        all_errs.push(err);
    }
    all_errs.extend(validate_storage_version_condition(
        &status.conditions,
        fld_path,
    ));
    all_errs
}

fn validate_server_storage_version(
    ssv: &internal::ServerStorageVersion,
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for msg in name_is_dns_subdomain(&ssv.api_server_id, false) {
        all_errs.push(invalid(
            &fld_path.child("apiServerID"),
            BadValue::String(ssv.api_server_id.clone()),
            &msg,
        ));
    }

    let errs = is_valid_api_version(&ssv.encoding_version);
    if !errs.is_empty() {
        all_errs.push(invalid(
            &fld_path.child("encodingVersion"),
            BadValue::String(ssv.encoding_version.clone()),
            &errs.join(","),
        ));
    }

    let mut found_encoding_version = false;
    for (i, dv) in ssv.decodable_versions.iter().enumerate() {
        let errs = is_valid_api_version(dv);
        if !errs.is_empty() {
            all_errs.push(invalid(
                &fld_path.child("decodableVersions").index(i),
                BadValue::String(dv.clone()),
                &errs.join(","),
            ));
        }
        if dv == &ssv.encoding_version {
            found_encoding_version = true;
        }
    }
    if !found_encoding_version {
        all_errs.push(invalid(
            &fld_path.child("decodableVersions"),
            BadValue::String(format!("{:?}", ssv.decodable_versions)),
            &format!(
                "decodableVersions must include encodingVersion {}",
                ssv.encoding_version
            ),
        ));
    }

    for (i, sv) in ssv.served_versions.iter().enumerate() {
        let errs = is_valid_api_version(sv);
        if !errs.is_empty() {
            all_errs.push(invalid(
                &fld_path.child("servedVersions").index(i),
                BadValue::String(sv.clone()),
                &errs.join(","),
            ));
        }
        if !ssv.decodable_versions.iter().any(|dv| dv == sv) {
            all_errs.push(invalid(
                &fld_path.child("servedVersions").index(i),
                BadValue::String(sv.clone()),
                &format!(
                    "individual served version : {} must be included in decodableVersions : {:?}",
                    sv, ssv.decodable_versions
                ),
            ));
        }
    }

    all_errs
}

fn common_version(storage_versions: &[internal::ServerStorageVersion]) -> Option<String> {
    if storage_versions.is_empty() {
        return None;
    }
    let common = storage_versions[0].encoding_version.clone();
    for version in &storage_versions[1..] {
        if version.encoding_version != common {
            return None;
        }
    }
    Some(common)
}

fn validate_common_version(
    status: &internal::StorageVersionStatus,
    fld_path: &Path,
) -> Option<Error> {
    let actual_common_version = common_version(&status.storage_versions);
    match (actual_common_version, &status.common_encoding_version) {
        (None, None) => None,
        (None, Some(value)) => Some(invalid(
            &fld_path.child("commonEncodingVersion"),
            BadValue::String(value.clone()),
            "should be nil if servers do not agree on the same encoding version, or if there is no server reporting the supported versions yet",
        )),
        (Some(actual), None) => Some(invalid(
            &fld_path.child("commonEncodingVersion"),
            BadValue::String(String::new()),
            &format!("the common encoding version is {}", actual),
        )),
        (Some(actual), Some(value)) if &actual != value => Some(invalid(
            &fld_path.child("commonEncodingVersion"),
            BadValue::String(value.clone()),
            &format!("the actual common encoding version is {}", actual),
        )),
        _ => None,
    }
}

fn validate_storage_version_condition(
    conditions: &[internal::StorageVersionCondition],
    fld_path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut seen_type: HashMap<String, usize> = HashMap::new();

    for (i, condition) in conditions.iter().enumerate() {
        let condition_path = fld_path.index(i);
        let type_value = condition.type_.as_ref().to_string();
        if let Some(previous) = seen_type.insert(type_value.clone(), i) {
            all_errs.push(invalid(
                &condition_path.child("type"),
                BadValue::String(type_value.clone()),
                &format!(
                    "the type of the condition is not unique, it also appears in conditions[{}]",
                    previous
                ),
            ));
        }

        all_errs.extend(validate_qualified_name(
            condition.type_.as_ref(),
            &condition_path.child("type"),
        ));
        all_errs.extend(validate_qualified_name(
            condition.status.as_ref(),
            &condition_path.child("status"),
        ));

        if condition.reason.is_empty() {
            all_errs.push(required(&condition_path.child("reason"), ""));
        }
        if condition.message.is_empty() {
            all_errs.push(required(&condition_path.child("message"), ""));
        }
    }

    all_errs
}

fn is_valid_api_version(api_version: &str) -> Vec<String> {
    let mut errs = Vec::new();
    let parts: Vec<&str> = api_version.split('/').collect();

    let version = match parts.len() {
        1 => parts[0],
        2 => {
            let group = parts[0];
            let version = parts[1];
            if group.is_empty() {
                errs.push(format!("group part: {}", empty_error()));
            } else {
                let msgs = is_dns1123_subdomain(group);
                if !msgs.is_empty() {
                    errs.extend(prefix_each(msgs, "group part: "));
                }
            }
            version
        }
        _ => {
            errs.push(format!(
                "an apiVersion is {} with an optional DNS subdomain prefix and '/' (e.g. 'example.com/MyVersion')",
                regex_error(
                    crate::common::validation::DNS1035_LABEL_ERROR_MSG,
                    DNS1035_LABEL_FMT,
                    &["my-name", "abc-123"],
                )
            ));
            return errs;
        }
    };

    if version.is_empty() {
        errs.push(format!("version part: {}", empty_error()));
    } else {
        let msgs = is_dns1035_label(version);
        if !msgs.is_empty() {
            errs.extend(prefix_each(msgs, "version part: "));
        }
    }

    errs
}

fn regex_error(msg: &str, fmt: &str, examples: &[&str]) -> String {
    if examples.is_empty() {
        return format!("{} (regex used for validation is '{}')", msg, fmt);
    }

    let mut out = format!("{} (e.g. ", msg);
    for (i, example) in examples.iter().enumerate() {
        if i > 0 {
            out.push_str(" or ");
        }
        out.push('\'');
        out.push_str(example);
        out.push_str("', ");
    }
    out.push_str(&format!("regex used for validation is '{}')", fmt));
    out
}

fn empty_error() -> &'static str {
    "must be non-empty"
}

fn prefix_each(mut msgs: Vec<String>, prefix: &str) -> Vec<String> {
    for msg in &mut msgs {
        *msg = format!("{}{}", prefix, msg);
    }
    msgs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ObjectMeta, Timestamp, TypeMeta};

    #[test]
    fn validate_storage_version_accepts_valid_object() {
        let sv = internal::StorageVersion {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("apps.deployments".to_string()),
                ..ObjectMeta::default()
            },
            status: internal::StorageVersionStatus {
                storage_versions: vec![internal::ServerStorageVersion {
                    api_server_id: "server-1".to_string(),
                    encoding_version: "v1".to_string(),
                    decodable_versions: vec!["v1".to_string()],
                    served_versions: vec!["v1".to_string()],
                }],
                common_encoding_version: Some("v1".to_string()),
                conditions: vec![internal::StorageVersionCondition {
                    type_: internal::StorageVersionConditionType::from(
                        internal::StorageVersionConditionType::ALL_ENCODING_VERSIONS_EQUAL,
                    ),
                    status: internal::ConditionStatus::from(internal::ConditionStatus::TRUE),
                    observed_generation: 1,
                    last_transition_time: Timestamp::zero(),
                    reason: "AllEqual".to_string(),
                    message: "All servers agree".to_string(),
                }],
            },
            ..internal::StorageVersion::default()
        };

        let errs = validate_storage_version(&sv);
        assert!(errs.is_empty(), "expected no errors, got {:?}", errs);
    }

    #[test]
    fn validate_storage_version_reports_errors() {
        let sv = internal::StorageVersion {
            type_meta: TypeMeta::default(),
            metadata: ObjectMeta {
                name: Some("invalidname".to_string()),
                ..ObjectMeta::default()
            },
            status: internal::StorageVersionStatus {
                storage_versions: vec![
                    internal::ServerStorageVersion {
                        api_server_id: "bad$id".to_string(),
                        encoding_version: "BadVersion".to_string(),
                        decodable_versions: Vec::new(),
                        served_versions: vec!["v1".to_string()],
                    },
                    internal::ServerStorageVersion {
                        api_server_id: "bad$id".to_string(),
                        encoding_version: "v2".to_string(),
                        decodable_versions: vec!["v1".to_string()],
                        served_versions: vec!["v2".to_string()],
                    },
                ],
                common_encoding_version: Some("v1".to_string()),
                conditions: vec![internal::StorageVersionCondition {
                    type_: internal::StorageVersionConditionType::from("bad$type"),
                    status: internal::ConditionStatus::from("bad status"),
                    observed_generation: 0,
                    last_transition_time: Timestamp::zero(),
                    reason: String::new(),
                    message: String::new(),
                }],
            },
            ..internal::StorageVersion::default()
        };

        let errs = validate_storage_version(&sv);
        assert!(!errs.is_empty());

        let fields: Vec<&str> = errs.errors.iter().map(|err| err.field.as_str()).collect();
        assert!(fields.contains(&"metadata.name"));
        assert!(fields.contains(&"status.storageVersions[1].apiServerID"));
        assert!(fields.contains(&"status.commonEncodingVersion"));
    }
}
