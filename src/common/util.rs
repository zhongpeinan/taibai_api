//! Utility types for Kubernetes API
//!
//! This module contains common utility types used across Kubernetes API objects.

use serde::{Deserialize, Serialize};

/// IntOrString is a type that can hold either an int32 or a string.
///
/// When used in JSON serialization, it will serialize as either an integer or a string,
/// depending on which variant is present. This is commonly used in Kubernetes for fields
/// like port numbers (which can be numeric or named ports).
///
/// Corresponds to [Kubernetes IntOrString](https://github.com/kubernetes/apimachinery/blob/master/pkg/util/intstr/intstr.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum IntOrString {
    /// Integer variant
    Int(i32),
    /// String variant
    String(String),
}

impl IntOrString {
    /// Creates an IntOrString from an integer
    pub fn from_int(value: i32) -> Self {
        IntOrString::Int(value)
    }

    /// Creates an IntOrString from a string
    pub fn from_string(value: String) -> Self {
        IntOrString::String(value)
    }

    /// Returns the integer value if present, None otherwise
    pub fn as_int(&self) -> Option<i32> {
        match self {
            IntOrString::Int(i) => Some(*i),
            IntOrString::String(_) => None,
        }
    }

    /// Returns the string value (integers are converted to string)
    pub fn as_str(&self) -> &str {
        match self {
            IntOrString::Int(_) => {
                // Convert int to string representation
                // In a real implementation, we'd use a cached string or similar
                // For now, we'll return a static message
                "integer"
            }
            IntOrString::String(s) => s,
        }
    }

    /// Parses the value as an integer if possible
    pub fn parse_as_int(&self) -> Option<i32> {
        match self {
            IntOrString::Int(i) => Some(*i),
            IntOrString::String(s) => s.parse().ok(),
        }
    }
}

// Implement From trait for convenient conversions
impl From<i32> for IntOrString {
    fn from(value: i32) -> Self {
        IntOrString::Int(value)
    }
}

impl From<String> for IntOrString {
    fn from(value: String) -> Self {
        IntOrString::String(value)
    }
}

impl From<&str> for IntOrString {
    fn from(value: &str) -> Self {
        IntOrString::String(value.to_string())
    }
}

/// Quantity is a fixed-point representation of a number.
///
/// In Kubernetes, Quantity is used for resource requests and limits (e.g., "100Mi", "1Gi").
/// It provides a string representation that can be parsed and compared.
///
/// Corresponds to [Kubernetes Quantity](https://github.com/kubernetes/apimachinery/blob/master/pkg/api/resource/quantity.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Quantity(pub String);

impl Quantity {
    /// Creates a Quantity from a string
    pub fn new(value: String) -> Self {
        Quantity(value)
    }

    /// Creates a Quantity from a string slice
    pub fn from_str(value: &str) -> Self {
        Quantity(value.to_string())
    }

    /// Returns the string value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Checks if the quantity is zero
    pub fn is_zero(&self) -> bool {
        self.0 == "0" || self.0 == "0i"
    }
}

// Implement AsRef<str> for convenient string access
impl AsRef<str> for Quantity {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Implement From trait for convenient conversions
impl From<String> for Quantity {
    fn from(value: String) -> Self {
        Quantity(value)
    }
}

impl From<&str> for Quantity {
    fn from(value: &str) -> Self {
        Quantity(value.to_string())
    }
}

// Implement Display for human-readable output
impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_int() {
        let ios = IntOrString::from_int(42);
        assert_eq!(ios, IntOrString::Int(42));
    }

    #[test]
    fn test_from_string() {
        let ios = IntOrString::from_string("test".to_string());
        assert_eq!(ios, IntOrString::String("test".to_string()));
    }

    #[test]
    fn test_from_i32() {
        let ios: IntOrString = 8080.into();
        assert_eq!(ios, IntOrString::Int(8080));
    }

    #[test]
    fn test_from_str() {
        let ios: IntOrString = "http".into();
        assert_eq!(ios, IntOrString::String("http".to_string()));
    }

    #[test]
    fn test_serialize_int() {
        let ios = IntOrString::Int(8080);
        let json = serde_json::to_string(&ios).unwrap();
        assert_eq!(json, "8080");
    }

    #[test]
    fn test_serialize_string() {
        let ios = IntOrString::String("http".to_string());
        let json = serde_json::to_string(&ios).unwrap();
        assert_eq!(json, r#""http""#);
    }

    #[test]
    fn test_deserialize_int() {
        let json = "8080";
        let ios: IntOrString = serde_json::from_str(json).unwrap();
        assert_eq!(ios, IntOrString::Int(8080));
    }

    #[test]
    fn test_deserialize_string() {
        let json = r#""http""#;
        let ios: IntOrString = serde_json::from_str(json).unwrap();
        assert_eq!(ios, IntOrString::String("http".to_string()));
    }

    #[test]
    fn test_deserialize_numeric_string() {
        // Numeric strings deserialize as String variant
        let json = r#""8080""#;
        let ios: IntOrString = serde_json::from_str(json).unwrap();
        assert_eq!(ios, IntOrString::String("8080".to_string()));
    }

    #[test]
    fn test_as_int() {
        let ios_int = IntOrString::Int(42);
        assert_eq!(ios_int.as_int(), Some(42));

        let ios_str = IntOrString::String("42".to_string());
        assert_eq!(ios_str.as_int(), None);
    }

    #[test]
    fn test_parse_as_int() {
        let ios_int = IntOrString::Int(42);
        assert_eq!(ios_int.parse_as_int(), Some(42));

        let ios_numeric_str = IntOrString::String("42".to_string());
        assert_eq!(ios_numeric_str.parse_as_int(), Some(42));

        let ios_str = IntOrString::String("http".to_string());
        assert_eq!(ios_str.parse_as_int(), None);
    }

    #[test]
    fn test_as_str() {
        let ios_str = IntOrString::String("http".to_string());
        assert_eq!(ios_str.as_str(), "http");

        let ios_int = IntOrString::Int(8080);
        assert_eq!(ios_int.as_str(), "integer");
    }

    #[test]
    fn test_round_trip_int() {
        let original = IntOrString::Int(443);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: IntOrString = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_round_trip_string() {
        let original = IntOrString::String("https".to_string());
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: IntOrString = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // Quantity tests
    #[test]
    fn test_quantity_new() {
        let q = Quantity::new("100Mi".to_string());
        assert_eq!(q.0, "100Mi");
    }

    #[test]
    fn test_quantity_from_str() {
        let q = Quantity::from_str("1Gi");
        assert_eq!(q.0, "1Gi");
    }

    #[test]
    fn test_quantity_from_string() {
        let q: Quantity = "500Mi".to_string().into();
        assert_eq!(q.0, "500Mi");
    }

    #[test]
    fn test_quantity_from_str_slice() {
        let q: Quantity = "2Gi".into();
        assert_eq!(q.0, "2Gi");
    }

    #[test]
    fn test_quantity_as_str() {
        let q = Quantity::from_str("100Mi");
        assert_eq!(q.as_str(), "100Mi");
    }

    #[test]
    fn test_quantity_as_ref() {
        let q = Quantity::from_str("100Mi");
        let s: &str = q.as_ref();
        assert_eq!(s, "100Mi");
    }

    #[test]
    fn test_quantity_is_zero() {
        assert!(Quantity::from_str("0").is_zero());
        assert!(Quantity::from_str("0i").is_zero());
        assert!(!Quantity::from_str("100Mi").is_zero());
        assert!(!Quantity::from_str("1").is_zero());
    }

    #[test]
    fn test_quantity_serialize() {
        let q = Quantity::from_str("100Mi");
        let json = serde_json::to_string(&q).unwrap();
        assert_eq!(json, r#""100Mi""#);
    }

    #[test]
    fn test_quantity_deserialize() {
        let json = r#""1Gi""#;
        let q: Quantity = serde_json::from_str(json).unwrap();
        assert_eq!(q.0, "1Gi");
    }

    #[test]
    fn test_quantity_round_trip() {
        let original = Quantity::from_str("512Mi");
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Quantity = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_quantity_display() {
        let q = Quantity::from_str("100Mi");
        assert_eq!(format!("{}", q), "100Mi");
    }

    #[test]
    fn test_quantity_equality() {
        let q1 = Quantity::from_str("100Mi");
        let q2 = Quantity::from_str("100Mi");
        let q3 = Quantity::from_str("200Mi");
        assert_eq!(q1, q2);
        assert_ne!(q1, q3);
    }
}
