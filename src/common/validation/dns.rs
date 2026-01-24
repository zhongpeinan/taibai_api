//! DNS validation utility functions.
//!
//! Ported from k8s.io/apimachinery/pkg/util/validation/validation.go

use regex::Regex;
use std::sync::OnceLock;

// DNS1123 label: [a-z0-9]([-a-z0-9]*[a-z0-9])?
static DNS1123_LABEL_RE: OnceLock<Regex> = OnceLock::new();

// DNS1123 subdomain: dns1123Label(\.dns1123Label)*
static DNS1123_SUBDOMAIN_RE: OnceLock<Regex> = OnceLock::new();

// DNS1035 label: [a-z]([-a-z0-9]*[a-z0-9])?
static DNS1035_LABEL_RE: OnceLock<Regex> = OnceLock::new();

// DNS1123 subdomain with underscore: _?[a-z0-9]([-_a-z0-9]*[a-z0-9])?(\._?[a-z0-9]([-_a-z0-9]*[a-z0-9])?)*$
pub static DNS1123_SUBDOMAIN_WITH_UNDERSCORE_RE: OnceLock<Regex> = OnceLock::new();

const DNS1123_LABEL_MAX_LEN: usize = 63;
const DNS1123_SUBDOMAIN_MAX_LEN: usize = 253;
const DNS1035_LABEL_MAX_LEN: usize = 63;

/// DNS1123LabelErrorMessage is error message for invalid DNS1123 labels.
pub const DNS1123_LABEL_ERROR_MSG: &str = "a lowercase RFC 1123 label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character";

/// DNS1123SubdomainErrorMessage is error message for invalid DNS1123 subdomains.
pub const DNS1123_SUBDOMAIN_ERROR_MSG: &str = "a lowercase RFC 1123 subdomain must consist of lower case alphanumeric characters, '-' or '.', and must start and end with an alphanumeric character";

/// DNS1035LabelErrorMessage is error message for invalid DNS1035 labels.
pub const DNS1035_LABEL_ERROR_MSG: &str = "a DNS-1035 label must consist of lower case alphanumeric characters or '-', start with an alphabetic character, and end with an alphanumeric character";

/// IsDNS1123Label tests for a string that conforms to definition of a label in DNS (RFC 1123).
///
/// # Examples
/// ```
/// # use taibai_api::common::validation::dns::is_dns1123_label;
/// assert!(is_dns1123_label("my-name").is_empty());
/// assert!(is_dns1123_label("abc-123").is_empty());
/// assert!(!is_dns1123_label("My-Name").is_empty());
/// assert!(!is_dns1123_label("my.name").is_empty());
/// ```
pub fn is_dns1123_label(value: &str) -> Vec<String> {
    let mut errs = Vec::new();

    if value.len() > DNS1123_LABEL_MAX_LEN {
        errs.push(format!(
            "must be no more than {} characters",
            DNS1123_LABEL_MAX_LEN
        ));
    }

    let re =
        DNS1123_LABEL_RE.get_or_init(|| Regex::new("^[a-z0-9]([-a-z0-9]*[a-z0-9])?$").unwrap());

    if !re.is_match(value) {
        // Check if it was a valid subdomain with dots
        let subdomain_re = DNS1123_SUBDOMAIN_RE.get_or_init(|| {
            Regex::new(r"^[a-z0-9]([-a-z0-9]*[a-z0-9])?(\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*$")
                .unwrap()
        });
        if subdomain_re.is_match(value) {
            // It was a valid subdomain but not a valid label, so must contain dots
            errs.push("must not contain dots".to_string());
        } else {
            errs.push(format!(
                "{} (regex used for validation is '[a-z0-9]([-a-z0-9]*[a-z0-9])?')",
                DNS1123_LABEL_ERROR_MSG
            ));
        }
    }

    errs
}

/// IsDNS1123Subdomain tests for a string that conforms to definition of a subdomain in DNS (RFC 1123).
///
/// # Examples
/// ```
/// # use taibai_api::common::validation::dns::is_dns1123_subdomain;
/// assert!(is_dns1123_subdomain("example.com").is_empty());
/// assert!(is_dns1123_subdomain("my-service.example.com").is_empty());
/// assert!(!is_dns1123_subdomain("Example.com").is_empty());
/// assert!(!is_dns1123_subdomain("my_service").is_empty());
/// ```
pub fn is_dns1123_subdomain(value: &str) -> Vec<String> {
    let mut errs = Vec::new();

    if value.len() > DNS1123_SUBDOMAIN_MAX_LEN {
        errs.push(format!(
            "must be no more than {} characters",
            DNS1123_SUBDOMAIN_MAX_LEN
        ));
    }

    let re = DNS1123_SUBDOMAIN_RE.get_or_init(|| {
        Regex::new(r"^[a-z0-9]([-a-z0-9]*[a-z0-9])?(\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*$").unwrap()
    });

    if !re.is_match(value) {
        errs.push(format!(
            "{} (regex used for validation is '[a-z0-9]([-a-z0-9]*[a-z0-9])?(\\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*$')",
            DNS1123_SUBDOMAIN_ERROR_MSG
        ));
    }

    errs
}

/// IsDNS1123SubdomainWithUnderscore tests for a string that conforms to definition of a subdomain in DNS (RFC 1123),
/// but allows use of an underscore in string.
///
/// # Examples
/// ```
/// # use taibai_api::common::validation::dns::is_dns1123_subdomain_with_underscore;
/// assert!(is_dns1123_subdomain_with_underscore("example.com").is_empty());
/// assert!(is_dns1123_subdomain_with_underscore("my_service.example.com").is_empty());
/// ```
pub fn is_dns1123_subdomain_with_underscore(value: &str) -> Vec<String> {
    let mut errs = Vec::new();

    if value.len() > DNS1123_SUBDOMAIN_MAX_LEN {
        errs.push(format!(
            "must be no more than {} characters",
            DNS1123_SUBDOMAIN_MAX_LEN
        ));
    }

    let re = DNS1123_SUBDOMAIN_WITH_UNDERSCORE_RE.get_or_init(|| {
        Regex::new(r"^_?[a-z0-9]([-_a-z0-9]*[a-z0-9])?(\._?[a-z0-9]([-_a-z0-9]*[a-z0-9])?)*$")
            .unwrap()
    });

    if !re.is_match(value) {
        errs.push(format!(
            "{} (regex used for validation is '^_?[a-z0-9]([-_a-z0-9]*[a-z0-9])?(\\._?[a-z0-9]([-_a-z0-9]*[a-z0-9])?)*$')",
            "a lowercase RFC 1123 subdomain must consist of lower case alphanumeric characters, '_', '-' or '.', and must start and end with an alphanumeric character"
        ));
    }

    errs
}

/// IsDNS1035Label tests for a string that conforms to definition of a label in DNS (RFC 1035).
///
/// # Examples
/// ```
/// # use taibai_api::common::validation::dns::is_dns1035_label;
/// assert!(is_dns1035_label("my-name").is_empty());
/// assert!(is_dns1035_label("abc-123").is_empty());
/// assert!(!is_dns1035_label("1name").is_empty());  // Can't start with number
/// assert!(!is_dns1035_label("My-Name").is_empty());  // Must be lowercase
/// ```
pub fn is_dns1035_label(value: &str) -> Vec<String> {
    let mut errs = Vec::new();

    if value.len() > DNS1035_LABEL_MAX_LEN {
        errs.push(format!(
            "must be no more than {} characters",
            DNS1035_LABEL_MAX_LEN
        ));
    }

    let re = DNS1035_LABEL_RE.get_or_init(|| Regex::new("^[a-z]([-a-z0-9]*[a-z0-9])?$").unwrap());

    if !re.is_match(value) {
        errs.push(format!(
            "{} (regex used for validation is '[a-z]([-a-z0-9]*[a-z0-9])?')",
            DNS1035_LABEL_ERROR_MSG
        ));
    }

    errs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dns1123_label_valid() {
        assert!(is_dns1123_label("a").is_empty());
        assert!(is_dns1123_label("my-name").is_empty());
        assert!(is_dns1123_label("abc-123").is_empty());
        assert!(is_dns1123_label("0").is_empty());
        assert!(is_dns1123_label("123").is_empty());
        assert!(is_dns1123_label("my-name-123").is_empty());
    }

    #[test]
    fn test_is_dns1123_label_invalid() {
        assert!(!is_dns1123_label("").is_empty()); // Empty
        assert!(!is_dns1123_label("My-Name").is_empty()); // Uppercase
        assert!(!is_dns1123_label("my.name").is_empty()); // Dot not allowed
        assert!(!is_dns1123_label("-myname").is_empty()); // Can't start with hyphen
        assert!(!is_dns1123_label("myname-").is_empty()); // Can't end with hyphen
        // "my--name" is actually VALID according to Go's IsDNS1123Label
        // The regex [a-z0-9]([-a-z0-9]*[a-z0-9])? allows consecutive hyphens
        assert!(is_dns1123_label("my--name").is_empty());
        // "myname" is valid (single word, all lowercase alnum)
        assert!(is_dns1123_label("myname").is_empty());
    }

    #[test]
    fn test_is_dns1123_label_max_length() {
        let valid = "a".repeat(DNS1123_LABEL_MAX_LEN);
        assert!(is_dns1123_label(&valid).is_empty());

        let invalid = "a".repeat(DNS1123_LABEL_MAX_LEN + 1);
        assert!(!is_dns1123_label(&invalid).is_empty());
    }

    #[test]
    fn test_is_dns1123_subdomain_valid() {
        assert!(is_dns1123_subdomain("example.com").is_empty());
        assert!(is_dns1123_subdomain("my-service.example.com").is_empty());
        assert!(is_dns1123_subdomain("a").is_empty());
        assert!(is_dns1123_subdomain("a.b.c.d").is_empty());
    }

    #[test]
    fn test_is_dns1123_subdomain_invalid() {
        assert!(!is_dns1123_subdomain("").is_empty());
        assert!(!is_dns1123_subdomain("Example.com").is_empty());
        assert!(!is_dns1123_subdomain("-example.com").is_empty());
        assert!(!is_dns1123_subdomain("example-.com").is_empty());
        assert!(!is_dns1123_subdomain("my_service.example.com").is_empty());
    }

    #[test]
    fn test_is_dns1123_subdomain_max_length() {
        let valid = "a.b";
        assert!(valid.len() <= DNS1123_SUBDOMAIN_MAX_LEN);
        assert!(is_dns1123_subdomain(&valid).is_empty());
    }

    #[test]
    fn test_is_dns1035_label_valid() {
        assert!(is_dns1035_label("a").is_empty());
        assert!(is_dns1035_label("my-name").is_empty());
        assert!(is_dns1035_label("abc-123").is_empty());
        assert!(is_dns1035_label("ab").is_empty());
    }

    #[test]
    fn test_is_dns1035_label_invalid() {
        assert!(!is_dns1035_label("").is_empty());
        assert!(!is_dns1035_label("1name").is_empty()); // Can't start with number
        assert!(!is_dns1035_label("My-Name").is_empty()); // Must be lowercase
        assert!(!is_dns1035_label("-my").is_empty()); // Can't start with hyphen
        assert!(!is_dns1035_label("my-").is_empty()); // Can't end with hyphen
        assert!(!is_dns1035_label("my.name").is_empty()); // Dot not allowed
    }

    #[test]
    fn test_is_dns1123_subdomain_with_underscore_valid() {
        assert!(is_dns1123_subdomain_with_underscore("example.com").is_empty());
        assert!(is_dns1123_subdomain_with_underscore("my_service.example.com").is_empty());
        assert!(is_dns1123_subdomain_with_underscore("_my-service.example.com").is_empty());
    }
}
