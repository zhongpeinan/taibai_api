//! Helper validation wrappers for internal core API types.

use crate::common::validation::{ErrorList, Path};

pub fn validate_container_name(name: &str, path: &Path) -> ErrorList {
    crate::core::v1::validation::helpers::validate_container_name(name, path)
}
