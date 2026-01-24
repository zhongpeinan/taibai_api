//! Field validation utilities for Kubernetes API types.
//!
//! This module provides:
//! - `Path`: Represents a path from root to a field (similar to Go's field.Path)
//! - `Error` / `ErrorList`: Validation error types
//! - DNS validation functions: `is_dns1123_label`, `is_dns1123_subdomain`, `is_dns1035_label`

pub mod dns;
pub mod errors;
pub mod path;

pub use dns::{
    DNS1035_LABEL_ERROR_MSG, DNS1123_LABEL_ERROR_MSG, DNS1123_SUBDOMAIN_ERROR_MSG,
    is_dns1035_label, is_dns1123_label, is_dns1123_subdomain, is_dns1123_subdomain_with_underscore,
};
pub use errors::{
    BadValue, Error, ErrorList, ErrorType, duplicate, forbidden, internal_error, invalid,
    not_found, not_supported, required, too_long, too_many, type_invalid,
};
pub use path::Path;
