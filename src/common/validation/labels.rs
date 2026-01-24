use regex::Regex;
use std::collections::BTreeMap;
use std::sync::OnceLock;

use super::qualified_name::is_qualified_name;
use super::{BadValue, ErrorList, Path, invalid};

const LABEL_VALUE_FMT: &str = "(([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9])?";
const LABEL_VALUE_ERR_MSG: &str = "a valid label must be an empty string or consist of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character";
const LABEL_VALUE_MAX_LENGTH: usize = 63;

static LABEL_VALUE_RE: OnceLock<Regex> = OnceLock::new();

pub fn is_valid_label_value(value: &str) -> Vec<String> {
    let mut errs = Vec::new();
    if value.len() > LABEL_VALUE_MAX_LENGTH {
        errs.push(format!(
            "must be no more than {} characters",
            LABEL_VALUE_MAX_LENGTH
        ));
    }
    let re = LABEL_VALUE_RE.get_or_init(|| Regex::new(&format!("^{}$", LABEL_VALUE_FMT)).unwrap());
    if !re.is_match(value) {
        errs.push(format!(
            "{} (regex used for validation is '{}')",
            LABEL_VALUE_ERR_MSG, LABEL_VALUE_FMT
        ));
    }
    errs
}

pub fn validate_label_name(name: &str, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in is_qualified_name(name) {
        all_errs.push(invalid(fld_path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}

pub fn validate_labels(labels: &BTreeMap<String, String>, fld_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for (key, value) in labels {
        all_errs.extend(validate_label_name(key, &fld_path.key(key)));
        for msg in is_valid_label_value(value) {
            all_errs.push(invalid(
                &fld_path.key(key),
                BadValue::String(value.to_string()),
                &msg,
            ));
        }
    }
    all_errs
}
