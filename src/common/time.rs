//! Kubernetes common time-related types
//!
//! This module contains time-related types used across Kubernetes API objects.

use crate::impl_unimplemented_prost_message;
#[allow(unused_imports)]
use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// Timestamp is a wrapper around DateTime<Utc> representing a timestamp in RFC3339 format.
///
/// This NewType pattern provides type safety and makes the intent clearer than using raw strings.
/// Timestamps in Kubernetes are always in RFC3339 format and UTC, for example: "2024-01-15T10:00:00Z"
///
/// # Example
/// ```ignore
/// use taibai_api::common::time::Timestamp;
///
/// // Create from RFC3339 string
/// let ts = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
///
/// // Create from chrono DateTime
/// let dt = Utc::now();
/// let ts = Timestamp::from_datetime(dt);
///
/// // Access the underlying DateTime
/// assert_eq!(ts.to_rfc3339(), "2024-01-15T10:00:00Z");
/// ```
#[derive(Clone, Debug, Eq)]
pub struct Timestamp(pub DateTime<Utc>);

impl Timestamp {
    /// Creates a new Timestamp from a DateTime<Utc>.
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    /// Creates a new Timestamp from an RFC3339 formatted string.
    ///
    /// Returns an error if the string is not a valid RFC3339 timestamp.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, chrono::ParseError> {
        Ok(Self(DateTime::parse_from_rfc3339(s)?.with_timezone(&Utc)))
    }

    /// Returns the timestamp as an RFC3339 formatted string with 'Z' suffix for UTC.
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    }

    /// Returns a reference to the inner DateTime<Utc>.
    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }

    /// Consumes the Timestamp and returns the inner DateTime<Utc>.
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    /// Returns the current time as a Timestamp.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Adds a `std::time::Duration` to the timestamp.
    pub fn add(&self, d: std::time::Duration) -> Self {
        Self(self.0 + d)
    }

    /// Adds a `chrono::TimeDelta` to the timestamp.
    pub fn add_delta(&self, d: TimeDelta) -> Self {
        Self(self.0 + d)
    }

    /// Checks if the time is zero value (corresponding to Go time.Time zero: 0001-01-01 00:00:00 +0000 UTC).
    pub fn is_zero(&self) -> bool {
        // Go time.Time zero value is 0001-01-01 00:00:00 +0000 UTC
        let zero_time = chrono::NaiveDate::from_ymd_opt(1, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();
        self.0 == zero_time
    }

    /// Creates a zero timestamp.
    pub fn zero() -> Self {
        let zero_time = chrono::NaiveDate::from_ymd_opt(1, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();
        Self(zero_time)
    }

    /// Returns the `TimeDelta` since the given timestamp.
    pub fn since(&self, t: &Self) -> TimeDelta {
        Self::now().0 - t.0
    }
}

impl_unimplemented_prost_message!(Timestamp);

impl Deref for Timestamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<DateTime<Utc>> for Timestamp {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }
}

impl From<Timestamp> for DateTime<Utc> {
    fn from(ts: Timestamp) -> Self {
        ts.0
    }
}

// TryFrom for string conversion (can fail)
impl TryFrom<&str> for Timestamp {
    type Error = chrono::ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl TryFrom<String> for Timestamp {
    type Error = chrono::ParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(&s)
    }
}

// Custom serde serialization - serialize as RFC3339 string with 'Z' suffix for UTC
impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            .serialize(serializer)
    }
}

// Custom serde deserialization - deserialize from RFC3339 string
impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| Timestamp(dt.with_timezone(&Utc)))
            .map_err(serde::de::Error::custom)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self(DateTime::UNIX_EPOCH)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        )
    }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for Timestamp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash based on timestamp for proper equality semantics
        self.0.timestamp().hash(state);
        self.0.timestamp_subsec_nanos().hash(state);
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
    fn test_timestamp_from_datetime() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let ts = Timestamp::from_datetime(dt);
        assert_eq!(ts.to_rfc3339(), "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_from_str() {
        let ts = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
        assert_eq!(ts.to_rfc3339(), "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_into_inner() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let ts = Timestamp::from_datetime(dt);
        let inner_dt: DateTime<Utc> = ts.into_inner();
        assert_eq!(inner_dt, dt);
    }

    #[test]
    fn test_timestamp_from_chrono() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let ts: Timestamp = dt.into();
        assert_eq!(ts.to_rfc3339(), "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_to_chrono() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let ts = Timestamp::from_datetime(dt);
        let converted_dt: DateTime<Utc> = ts.into();
        assert_eq!(converted_dt, dt);
    }

    #[test]
    fn test_timestamp_try_from_str() {
        let s = "2024-01-15T10:00:00Z";
        let ts: Timestamp = s.try_into().unwrap();
        assert_eq!(ts.to_rfc3339(), "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_now() {
        let before = Utc::now();
        let ts = Timestamp::now();
        let after = Utc::now();
        assert!(ts.0 >= before && ts.0 <= after);
    }

    #[test]
    fn test_timestamp_default() {
        let ts = Timestamp::default();
        // Unix epoch
        assert_eq!(ts.timestamp(), 0);
    }

    #[test]
    fn test_timestamp_display() {
        let ts = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
        assert_eq!(format!("{}", ts), "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_deref() {
        let ts = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
        // Can access DateTime methods through Deref
        // Unix timestamp for 2024-01-15 10:00:00 UTC
        assert_eq!(ts.timestamp(), 1705312800);
    }

    #[test]
    fn test_timestamp_as_ref() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let ts = Timestamp::from_datetime(dt);
        let dt_ref: &DateTime<Utc> = ts.as_ref();
        assert_eq!(dt_ref, &dt);
    }

    #[test]
    fn test_timestamp_equality() {
        let ts1 = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
        let ts2 = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
        let ts3 = Timestamp::from_str("2024-01-15T11:00:00Z").unwrap();

        assert_eq!(ts1, ts2);
        assert_ne!(ts1, ts3);
    }

    #[test]
    fn test_timestamp_ord() {
        let ts1 = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
        let ts2 = Timestamp::from_str("2024-01-15T11:00:00Z").unwrap();

        assert!(ts1 < ts2);
        assert!(ts2 > ts1);
    }

    #[test]
    fn test_timestamp_serialization() {
        let ts = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
        let json = serde_json::to_string(&ts).unwrap();
        assert_eq!(json, "\"2024-01-15T10:00:00Z\"");
    }

    #[test]
    fn test_timestamp_deserialization() {
        let json = "\"2024-01-15T10:00:00Z\"";
        let ts: Timestamp = serde_json::from_str(json).unwrap();
        assert_eq!(ts.to_rfc3339(), "2024-01-15T10:00:00Z");
    }

    #[test]
    fn test_timestamp_round_trip() {
        let original = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Timestamp = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_timestamp_hash() {
        use std::collections::HashSet;
        let ts1 = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
        let ts2 = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
        let ts3 = Timestamp::from_str("2024-01-15T11:00:00Z").unwrap();

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
        let ts1 = Timestamp::from_str("2024-01-15T10:00:00Z").unwrap();
        let ts2 = ts1.clone();
        assert_eq!(ts1, ts2);
    }
}

/// MicroTime is a wrapper around DateTime<Utc> representing a timestamp with microsecond precision.
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
/// // Create from RFC3339 string with microseconds
/// let mt = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
///
/// // Access the underlying value
/// assert!(mt.to_rfc3339_opts(chrono::SecondsFormat::Micros, true).contains("123456"));
/// ```
#[derive(Clone, Debug, Eq)]
pub struct MicroTime(pub DateTime<Utc>);

impl MicroTime {
    /// Creates a new MicroTime from a DateTime<Utc>.
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    /// Creates a new MicroTime from an RFC3339 formatted string.
    ///
    /// Returns an error if the string is not a valid RFC3339 timestamp.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, chrono::ParseError> {
        Ok(Self(DateTime::parse_from_rfc3339(s)?.with_timezone(&Utc)))
    }

    /// Returns the timestamp as an RFC3339 formatted string with microseconds and 'Z' suffix for UTC.
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339_opts(chrono::SecondsFormat::Micros, true)
    }

    /// Returns a reference to the inner DateTime<Utc>.
    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }

    /// Consumes the MicroTime and returns the inner DateTime<Utc>.
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    /// Returns the current time as a MicroTime.
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl Deref for MicroTime {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<DateTime<Utc>> for MicroTime {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl From<DateTime<Utc>> for MicroTime {
    fn from(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }
}

impl From<MicroTime> for DateTime<Utc> {
    fn from(mt: MicroTime) -> Self {
        mt.0
    }
}

impl TryFrom<&str> for MicroTime {
    type Error = chrono::ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl TryFrom<String> for MicroTime {
    type Error = chrono::ParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(&s)
    }
}

impl Serialize for MicroTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0
            .to_rfc3339_opts(chrono::SecondsFormat::Micros, true)
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MicroTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| MicroTime(dt.with_timezone(&Utc)))
            .map_err(serde::de::Error::custom)
    }
}

impl Default for MicroTime {
    fn default() -> Self {
        Self(DateTime::UNIX_EPOCH)
    }
}

impl fmt::Display for MicroTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0.to_rfc3339_opts(chrono::SecondsFormat::Micros, true)
        )
    }
}

impl PartialEq for MicroTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for MicroTime {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.timestamp().hash(state);
        self.0.timestamp_subsec_nanos().hash(state);
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
    fn test_micro_time_from_datetime() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let mt = MicroTime::from_datetime(dt);
        assert_eq!(mt.to_rfc3339(), "2024-01-15T10:00:00.000000Z");
    }

    #[test]
    fn test_micro_time_from_str() {
        let mt = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
        let formatted = mt.0.to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
        assert_eq!(formatted, "2024-01-15T10:00:00.123456Z");
    }

    #[test]
    fn test_micro_time_into_inner() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let mt = MicroTime::from_datetime(dt);
        let inner_dt: DateTime<Utc> = mt.into_inner();
        assert_eq!(inner_dt, dt);
    }

    #[test]
    fn test_micro_time_from_chrono() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let mt: MicroTime = dt.into();
        assert_eq!(mt.to_rfc3339(), "2024-01-15T10:00:00.000000Z");
    }

    #[test]
    fn test_micro_time_to_chrono() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let mt = MicroTime::from_datetime(dt);
        let converted_dt: DateTime<Utc> = mt.into();
        assert_eq!(converted_dt, dt);
    }

    #[test]
    fn test_micro_time_try_from_str() {
        let s = "2024-01-15T10:00:00.123456Z";
        let mt: MicroTime = s.try_into().unwrap();
        let formatted = mt.0.to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
        assert_eq!(formatted, "2024-01-15T10:00:00.123456Z");
    }

    #[test]
    fn test_micro_time_now() {
        let before = Utc::now();
        let mt = MicroTime::now();
        let after = Utc::now();
        assert!(mt.0 >= before && mt.0 <= after);
    }

    #[test]
    fn test_micro_time_default() {
        let mt = MicroTime::default();
        assert_eq!(mt.timestamp(), 0);
    }

    #[test]
    fn test_micro_time_display() {
        let mt = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
        let formatted = format!("{}", mt);
        assert!(formatted.contains("123456"));
    }

    #[test]
    fn test_micro_time_deref() {
        let mt = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
        assert_eq!(mt.timestamp(), 1705312800);
    }

    #[test]
    fn test_micro_time_as_ref() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let mt = MicroTime::from_datetime(dt);
        let dt_ref: &DateTime<Utc> = mt.as_ref();
        assert_eq!(dt_ref, &dt);
    }

    #[test]
    fn test_micro_time_equality() {
        let mt1 = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
        let mt2 = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
        let mt3 = MicroTime::from_str("2024-01-15T11:00:00.123456Z").unwrap();

        assert_eq!(mt1, mt2);
        assert_ne!(mt1, mt3);
    }

    #[test]
    fn test_micro_time_ord() {
        let mt1 = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
        let mt2 = MicroTime::from_str("2024-01-15T11:00:00.123456Z").unwrap();

        assert!(mt1 < mt2);
        assert!(mt2 > mt1);
    }

    #[test]
    fn test_micro_time_serialization() {
        let mt = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
        let json = serde_json::to_string(&mt).unwrap();
        assert_eq!(json, "\"2024-01-15T10:00:00.123456Z\"");
    }

    #[test]
    fn test_micro_time_deserialization() {
        let json = "\"2024-01-15T10:00:00.123456Z\"";
        let mt: MicroTime = serde_json::from_str(json).unwrap();
        let formatted = mt.0.to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
        assert_eq!(formatted, "2024-01-15T10:00:00.123456Z");
    }

    #[test]
    fn test_micro_time_round_trip() {
        let original = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: MicroTime = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_micro_time_hash() {
        use std::collections::HashSet;
        let mt1 = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
        let mt2 = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
        let mt3 = MicroTime::from_str("2024-01-15T11:00:00.123456Z").unwrap();

        let mut set = HashSet::new();
        set.insert(mt1.clone());
        set.insert(mt2.clone());
        set.insert(mt3.clone());

        assert_eq!(set.len(), 2);
        assert!(set.contains(&mt1));
        assert!(set.contains(&mt3));
    }

    #[test]
    fn test_micro_time_clone() {
        let mt1 = MicroTime::from_str("2024-01-15T10:00:00.123456Z").unwrap();
        let mt2 = mt1.clone();
        assert_eq!(mt1, mt2);
    }
}
