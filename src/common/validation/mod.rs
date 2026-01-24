//! Field validation utilities for Kubernetes API types.
//!
//! This module provides:
//! - `Path`: Represents a path from root to a field (similar to Go's field.Path)
//! - `Error` / `ErrorList`: Validation error types
//! - DNS validation functions: `is_dns1123_label`, `is_dns1123_subdomain`, `is_dns1035_label`

pub mod dns;
pub mod errors;
pub mod object_meta;
pub mod path;
pub mod qualified_name;

pub use dns::{
    DNS1035_LABEL_ERROR_MSG, DNS1123_LABEL_ERROR_MSG, DNS1123_SUBDOMAIN_ERROR_MSG,
    is_dns1035_label, is_dns1123_label, is_dns1123_subdomain, is_dns1123_subdomain_with_underscore,
};
pub use errors::{
    BadValue, Error, ErrorList, ErrorType, duplicate, forbidden, internal_error, invalid,
    not_found, not_supported, required, too_long, too_many, type_invalid,
};
pub use object_meta::{
    ValidateNameFunc, name_is_dns_label, name_is_dns_subdomain, validate_object_meta,
};
pub use path::Path;
pub use qualified_name::{is_qualified_name, validate_qualified_name};
