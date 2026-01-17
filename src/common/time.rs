//! Kubernetes common time-related types
//!
//! This module contains time-related types used across Kubernetes API objects.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// Timestamp is a wrapper around String representing a timestamp in RFC3339 format.
///
/// This NewType pattern provides type safety and makes the intent clearer than using raw strings.
/// Timestamps in Kubernetes are always in RFC3339 format and UTC, for example: "2024-01-15T10:00:00Z"
///
/// # Example
/// ```ignore
/// use taibai_api::common::time::Timestamp;
///
/// // Create from string
/// let ts = Timestamp::new("2024-01-15T10:00:00Z".to_string());
///
/// // Access the underlying value
/// assert_eq!(ts.as_str(), "2024-01-15T10:00:00Z");
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Eq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Timestamp(String);

impl Timestamp {
    /// Creates a new Timestamp from a String.
    ///
    /// The string should be in RFC3339 format, but this constructor doesn't validate.
    /// Use `parse_rfc3339()` if you need validation.
    pub fn new(s: String) -> Self {
        Self(s)
    }

    /// Creates a new Timestamp from a &str.
    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }

    /// Returns the timestamp as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the Timestamp and returns the inner String.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for Timestamp {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Timestamp {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for Timestamp {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<Timestamp> for String {
    fn from(ts: Timestamp) -> Self {
        ts.0
    }
}

impl<'a> From<&'a str> for Timestamp {
    fn from(s: &'a str) -> Self {
        Self(s.to_string())
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for Timestamp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_new() {
        let ts = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        assert_eq!(ts.as_str(), "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_from_str() {
        let ts = Timestamp::from_str("2024-01-15T10:00:00Z");
        assert_eq!(ts.as_str(), "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_into_inner() {
        let ts = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        let s: String = ts.into_inner();
        assert_eq!(s, "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_from_string() {
        let s = "2024-01-15T10:00:00Z".to_string();
        let ts: Timestamp = s.clone().into();
        assert_eq!(ts.as_str(), s);
    }

    #[test]
    fn test_timestamp_to_string() {
        let ts = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        let s: String = ts.clone().into();
        assert_eq!(s, "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_from_str_ref() {
        let s = "2024-01-15T10:00:00Z";
        let ts: Timestamp = s.into();
        assert_eq!(ts.as_str(), s);
    }

    #[test]
    fn test_timestamp_default() {
        let ts = Timestamp::default();
        assert!(ts.as_str().is_empty());
    }

    #[test]
    fn test_timestamp_display() {
        let ts = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        assert_eq!(format!("{}", ts), "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_deref() {
        let ts = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        assert_eq!(&*ts, "2024-01-15T10:00:00Z");
        assert_eq!(ts.len(), 20);
    }

    #[test]
    fn test_timestamp_as_ref() {
        let ts = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        let s: &str = ts.as_ref();
        assert_eq!(s, "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_equality() {
        let ts1 = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        let ts2 = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        let ts3 = Timestamp::new("2024-01-15T11:00:00Z".to_string());

        assert_eq!(ts1, ts2);
        assert_ne!(ts1, ts3);
    }

    #[test]
    fn test_timestamp_ord() {
        let ts1 = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        let ts2 = Timestamp::new("2024-01-15T11:00:00Z".to_string());

        assert!(ts1 < ts2);
        assert!(ts2 > ts1);
    }

    #[test]
    fn test_timestamp_serialization() {
        let ts = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        let json = serde_json::to_string(&ts).unwrap();
        assert_eq!(json, "\"2024-01-15T10:00:00Z\"");
    }

    #[test]
    fn test_timestamp_deserialization() {
        let json = "\"2024-01-15T10:00:00Z\"";
        let ts: Timestamp = serde_json::from_str(json).unwrap();
        assert_eq!(ts.as_str(), "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_round_trip() {
        let original = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Timestamp = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_timestamp_hash() {
        use std::collections::HashSet;
        let ts1 = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        let ts2 = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        let ts3 = Timestamp::new("2024-01-15T11:00:00Z".to_string());

        let mut set = HashSet::new();
        set.insert(ts1.clone());
        set.insert(ts2.clone());
        set.insert(ts3.clone());

        // ts1 and ts2 are equal, so only 2 entries in the set
        assert_eq!(set.len(), 2);
        assert!(set.contains(&ts1));
        assert!(set.contains(&ts3));
    }

    #[test]
    fn test_timestamp_clone() {
        let ts1 = Timestamp::new("2024-01-15T10:00:00Z".to_string());
        let ts2 = ts1.clone();
        assert_eq!(ts1, ts2);
    }
}

/// MicroTime is a wrapper around String representing a timestamp with microsecond precision.
///
/// This is similar to Timestamp but provides microsecond-level precision for certain
/// Kubernetes API fields that require finer granularity.
///
/// The format is RFC3339 with microsecond precision, for example: "2024-01-15T10:00:00.123456Z"
///
/// # Example
/// ```ignore
/// use taibai_api::common::time::MicroTime;
///
/// // Create from string
/// let mt = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
///
/// // Access the underlying value
/// assert_eq!(mt.as_str(), "2024-01-15T10:00:00.123456Z");
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Eq)]
#[serde(transparent)]
#[derive(Default)]
pub struct MicroTime(String);

impl MicroTime {
    /// Creates a new MicroTime from a String.
    ///
    /// The string should be in RFC3339 format with microsecond precision.
    pub fn new(s: String) -> Self {
        Self(s)
    }

    /// Creates a new MicroTime from a &str.
    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }

    /// Returns the timestamp as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the MicroTime and returns the inner String.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for MicroTime {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for MicroTime {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for MicroTime {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<MicroTime> for String {
    fn from(mt: MicroTime) -> Self {
        mt.0
    }
}

impl<'a> From<&'a str> for MicroTime {
    fn from(s: &'a str) -> Self {
        Self(s.to_string())
    }
}

impl fmt::Display for MicroTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for MicroTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for MicroTime {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl PartialOrd for MicroTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MicroTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[cfg(test)]
mod tests_micro_time {
    use super::*;

    #[test]
    fn test_micro_time_new() {
        let mt = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        assert_eq!(mt.as_str(), "2024-01-15T10:00:00.123456Z");
    }

    #[test]
    fn test_micro_time_from_str() {
        let mt = MicroTime::from_str("2024-01-15T10:00:00.123456Z");
        assert_eq!(mt.as_str(), "2024-01-15T10:00:00.123456Z");
    }

    #[test]
    fn test_micro_time_into_inner() {
        let mt = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        let s: String = mt.into_inner();
        assert_eq!(s, "2024-01-15T10:00:00.123456Z");
    }

    #[test]
    fn test_micro_time_from_string() {
        let s = "2024-01-15T10:00:00.123456Z".to_string();
        let mt: MicroTime = s.clone().into();
        assert_eq!(mt.as_str(), s);
    }

    #[test]
    fn test_micro_time_to_string() {
        let mt = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        let s: String = mt.clone().into();
        assert_eq!(s, "2024-01-15T10:00:00.123456Z");
    }

    #[test]
    fn test_micro_time_from_str_ref() {
        let s = "2024-01-15T10:00:00.123456Z";
        let mt: MicroTime = s.into();
        assert_eq!(mt.as_str(), s);
    }

    #[test]
    fn test_micro_time_default() {
        let mt = MicroTime::default();
        assert!(mt.as_str().is_empty());
    }

    #[test]
    fn test_micro_time_display() {
        let mt = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        assert_eq!(format!("{}", mt), "2024-01-15T10:00:00.123456Z");
    }

    #[test]
    fn test_micro_time_deref() {
        let mt = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        assert_eq!(&*mt, "2024-01-15T10:00:00.123456Z");
        assert_eq!(mt.len(), 28);
    }

    #[test]
    fn test_micro_time_as_ref() {
        let mt = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        let s: &str = mt.as_ref();
        assert_eq!(s, "2024-01-15T10:00:00.123456Z");
    }

    #[test]
    fn test_micro_time_equality() {
        let mt1 = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        let mt2 = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        let mt3 = MicroTime::new("2024-01-15T11:00:00.123456Z".to_string());

        assert_eq!(mt1, mt2);
        assert_ne!(mt1, mt3);
    }

    #[test]
    fn test_micro_time_ord() {
        let mt1 = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        let mt2 = MicroTime::new("2024-01-15T11:00:00.123456Z".to_string());

        assert!(mt1 < mt2);
        assert!(mt2 > mt1);
    }

    #[test]
    fn test_micro_time_serialization() {
        let mt = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        let json = serde_json::to_string(&mt).unwrap();
        assert_eq!(json, "\"2024-01-15T10:00:00.123456Z\"");
    }

    #[test]
    fn test_micro_time_deserialization() {
        let json = "\"2024-01-15T10:00:00.123456Z\"";
        let mt: MicroTime = serde_json::from_str(json).unwrap();
        assert_eq!(mt.as_str(), "2024-01-15T10:00:00.123456Z");
    }

    #[test]
    fn test_micro_time_round_trip() {
        let original = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: MicroTime = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_micro_time_hash() {
        use std::collections::HashSet;
        let mt1 = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        let mt2 = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        let mt3 = MicroTime::new("2024-01-15T11:00:00.123456Z".to_string());

        let mut set = HashSet::new();
        set.insert(mt1.clone());
        set.insert(mt2.clone());
        set.insert(mt3.clone());

        // mt1 and mt2 are equal, so only 2 entries in the set
        assert_eq!(set.len(), 2);
        assert!(set.contains(&mt1));
        assert!(set.contains(&mt3));
    }

    #[test]
    fn test_micro_time_clone() {
        let mt1 = MicroTime::new("2024-01-15T10:00:00.123456Z".to_string());
        let mt2 = mt1.clone();
        assert_eq!(mt1, mt2);
    }
}
