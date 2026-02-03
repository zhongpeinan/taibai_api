//! Helper validation wrappers for internal core API types.

use crate::common::validation::{BadValue, ErrorList, Path, invalid, is_dns1123_label};

pub fn validate_container_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    for msg in is_dns1123_label(name) {
        all_errs.push(invalid(path, BadValue::String(name.to_string()), &msg));
    }
    all_errs
}
