//! Field-level validation errors.
//!
//! Ported from k8s.io/apimachinery/pkg/util/validation/field/errors.go

use std::fmt;

use super::path::Path;

/// Error is a field-level validation error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    /// Type of error
    pub error_type: ErrorType,
    /// Field path (e.g., "metadata.name")
    pub field: String,
    /// Bad value that caused the error
    pub bad_value: Option<BadValue>,
    /// Additional detail about the error
    pub detail: String,
    /// Origin uniquely identifies where this error was generated
    pub origin: Option<String>,
    /// CoveredByDeclarative is true when covered by declarative validation
    pub covered_by_declarative: bool,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.error_body())
    }
}

impl Error {
    /// Returns error message without the field name.
    pub fn error_body(&self) -> String {
        let mut s = match self.error_type {
            ErrorType::Required
            | ErrorType::Forbidden
            | ErrorType::TooLong
            | ErrorType::Internal => self.error_type.to_string(),
            ErrorType::Invalid
            | ErrorType::TypeInvalid
            | ErrorType::NotSupported
            | ErrorType::NotFound
            | ErrorType::Duplicate
            | ErrorType::TooMany => {
                if let Some(ref value) = self.bad_value {
                    format!("{}: {}", self.error_type, value)
                } else {
                    self.error_type.to_string()
                }
            }
        };

        if !self.detail.is_empty() {
            s.push_str(&format!(": {}", self.detail));
        }
        s
    }

    /// Sets the origin for this error.
    pub fn with_origin(mut self, origin: &str) -> Self {
        self.origin = Some(origin.to_string());
        self
    }

    /// Marks this error as covered by declarative validation.
    pub fn mark_covered_by_declarative(mut self) -> Self {
        self.covered_by_declarative = true;
        self
    }
}

/// Wrapper for bad value in errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BadValue {
    String(String),
    Int(i64),
    Bool(bool),
    // Add more variants as needed
}

fn quote_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
}

impl fmt::Display for BadValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BadValue::String(s) => write!(f, "{}", quote_string(s)),
            BadValue::Int(i) => write!(f, "{}", i),
            BadValue::Bool(b) => write!(f, "{}", b),
        }
    }
}

/// ErrorType is a machine readable value providing more detail about why
/// a field is invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ErrorType {
    /// Failure to find a requested value
    NotFound,
    /// Required values that are not provided
    Required,
    /// Collisions of values that must be unique
    Duplicate,
    /// Malformed values (e.g., failed regex match, too long, out of bounds)
    Invalid,
    /// Unknown values for enumerated fields
    NotSupported,
    /// Valid values not permitted by current conditions
    Forbidden,
    /// Value is too long
    TooLong,
    /// Too many items in a list
    TooMany,
    /// Internal error (not related to user input)
    Internal,
    /// Value did not match schema type
    TypeInvalid,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ErrorType::NotFound => "Not found",
            ErrorType::Required => "Required value",
            ErrorType::Duplicate => "Duplicate value",
            ErrorType::Invalid => "Invalid value",
            ErrorType::NotSupported => "Unsupported value",
            ErrorType::Forbidden => "Forbidden",
            ErrorType::TooLong => "Too long",
            ErrorType::TooMany => "Too many",
            ErrorType::Internal => "Internal error",
            ErrorType::TypeInvalid => "Invalid value",
        };
        write!(f, "{}", s)
    }
}

/// ErrorList holds a set of Errors.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ErrorList {
    pub errors: Vec<Error>,
}

impl ErrorList {
    /// Creates a new empty ErrorList
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends an error to the list
    pub fn push(&mut self, error: Error) {
        self.errors.push(error);
    }

    /// Extends this ErrorList with another
    pub fn extend(&mut self, other: ErrorList) {
        self.errors.extend(other.errors);
    }

    /// Returns true if there are no errors
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the number of errors
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Sets origin for all errors in the list
    pub fn with_origin(mut self, origin: &str) -> Self {
        for err in &mut self.errors {
            err.origin = Some(origin.to_string());
        }
        self
    }

    /// Marks all errors as covered by declarative validation
    pub fn mark_covered_by_declarative(mut self) -> Self {
        for err in &mut self.errors {
            err.covered_by_declarative = true;
        }
        self
    }
}

impl fmt::Display for ErrorList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, err) in self.errors.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

/// Error constructors

/// NotFound returns an Error indicating "value not found".
pub fn not_found(field: &Path, value: BadValue) -> Error {
    Error {
        error_type: ErrorType::NotFound,
        field: field.to_string(),
        bad_value: Some(value),
        detail: String::new(),
        origin: None,
        covered_by_declarative: false,
    }
}

/// Required returns an Error indicating "value required".
pub fn required(field: &Path, detail: &str) -> Error {
    Error {
        error_type: ErrorType::Required,
        field: field.to_string(),
        bad_value: None,
        detail: detail.to_string(),
        origin: None,
        covered_by_declarative: false,
    }
}

/// Duplicate returns an Error indicating "duplicate value".
pub fn duplicate(field: &Path, value: BadValue) -> Error {
    Error {
        error_type: ErrorType::Duplicate,
        field: field.to_string(),
        bad_value: Some(value),
        detail: String::new(),
        origin: None,
        covered_by_declarative: false,
    }
}

/// Invalid returns an Error indicating "invalid value".
pub fn invalid(field: &Path, value: BadValue, detail: &str) -> Error {
    Error {
        error_type: ErrorType::Invalid,
        field: field.to_string(),
        bad_value: Some(value),
        detail: detail.to_string(),
        origin: None,
        covered_by_declarative: false,
    }
}

/// NotSupported returns an Error indicating "unsupported value".
pub fn not_supported(field: &Path, value: BadValue, valid_values: &[&str]) -> Error {
    let detail = if valid_values.is_empty() {
        String::new()
    } else {
        format!(
            "supported values: {}",
            valid_values
                .iter()
                .map(|v| quote_string(v))
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    Error {
        error_type: ErrorType::NotSupported,
        field: field.to_string(),
        bad_value: Some(value),
        detail,
        origin: None,
        covered_by_declarative: false,
    }
}

/// Forbidden returns an Error indicating "forbidden".
pub fn forbidden(field: &Path, detail: &str) -> Error {
    Error {
        error_type: ErrorType::Forbidden,
        field: field.to_string(),
        bad_value: None,
        detail: detail.to_string(),
        origin: None,
        covered_by_declarative: false,
    }
}

/// TooLong returns an Error indicating "too long".
pub fn too_long(field: &Path, max_length: usize) -> Error {
    let unit = if max_length == 1 { "byte" } else { "bytes" };
    let detail = format!("may not be more than {} {}", max_length, unit);
    Error {
        error_type: ErrorType::TooLong,
        field: field.to_string(),
        bad_value: Some(BadValue::String("<value omitted>".to_string())),
        detail,
        origin: None,
        covered_by_declarative: false,
    }
}

/// TooMany returns an Error indicating "too many".
pub fn too_many(field: &Path, actual: Option<usize>, max: usize) -> Error {
    let msg = if max == 1 {
        "must have at most 1 item".to_string()
    } else {
        format!("must have at most {} items", max)
    };
    Error {
        error_type: ErrorType::TooMany,
        field: field.to_string(),
        bad_value: actual.map(|value| BadValue::Int(value as i64)),
        detail: msg,
        origin: None,
        covered_by_declarative: false,
    }
}

/// InternalError returns an Error indicating "internal error".
pub fn internal_error(field: &Path, err: &str) -> Error {
    Error {
        error_type: ErrorType::Internal,
        field: field.to_string(),
        bad_value: None,
        detail: err.to_string(),
        origin: None,
        covered_by_declarative: false,
    }
}

/// TypeInvalid returns an Error indicating "type is invalid".
pub fn type_invalid(field: &Path, value: BadValue, detail: &str) -> Error {
    Error {
        error_type: ErrorType::TypeInvalid,
        field: field.to_string(),
        bad_value: Some(value),
        detail: detail.to_string(),
        origin: None,
        covered_by_declarative: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_required() {
        let p = Path::new("metadata").child("name");
        let err = required(&p, "name is required");
        assert_eq!(
            err.to_string(),
            "metadata.name: Required value: name is required"
        );
    }

    #[test]
    fn test_error_invalid() {
        let p = Path::new("metadata").child("name");
        let err = invalid(
            &p,
            BadValue::String("".to_string()),
            "name must be non-empty",
        );
        assert_eq!(
            err.to_string(),
            "metadata.name: Invalid value: \"\": name must be non-empty"
        );
    }

    #[test]
    fn test_error_duplicate() {
        let p = Path::new("spec").child("containers").index(0).child("name");
        let err = duplicate(&p, BadValue::String("container".to_string()));
        assert!(err.to_string().contains("Duplicate value"));
    }

    #[test]
    fn test_error_list() {
        let mut list = ErrorList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        let p1 = Path::new("metadata").child("name");
        list.push(required(&p1, "name is required"));

        let p2 = Path::new("spec").child("replicas");
        list.push(invalid(&p2, BadValue::Int(-1), "must be non-negative"));

        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
    }
}
