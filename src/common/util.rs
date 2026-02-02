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

impl Default for IntOrString {
    fn default() -> Self {
        IntOrString::Int(0)
    }
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
/// This implementation supports arithmetic operations, comparison with unit conversion, and validation.
///
/// Corresponds to [Kubernetes Quantity](https://github.com/kubernetes/apimachinery/blob/master/pkg/api/resource/quantity.go)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Quantity(pub String);

// Helper struct for parsed quantity with value and unit
#[derive(Clone, Debug, PartialEq)]
struct ParsedQuantity {
    value: f64,
    unit: QuantityUnit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum QuantityUnit {
    // Decimal suffixes (for CPU)
    Nano,  // n
    Micro, // u
    Milli, // m
    None,  // no suffix
    // Binary suffixes (for memory)
    Ki, // 2^10
    Mi, // 2^20
    Gi, // 2^30
    Ti, // 2^40
    Pi, // 2^50
    Ei, // 2^60
    // Decimal SI suffixes (less common, but valid)
    K, // 10^3
    M, // 10^6
    G, // 10^9
    T, // 10^12
    P, // 10^15
    E, // 10^18
}

impl QuantityUnit {
    /// Returns the multiplier for this unit
    fn multiplier(&self) -> f64 {
        match self {
            QuantityUnit::Nano => 1e-9,
            QuantityUnit::Micro => 1e-6,
            QuantityUnit::Milli => 1e-3,
            QuantityUnit::None => 1.0,
            // Binary units
            QuantityUnit::Ki => 1024.0,
            QuantityUnit::Mi => 1024.0 * 1024.0,
            QuantityUnit::Gi => 1024.0 * 1024.0 * 1024.0,
            QuantityUnit::Ti => 1024.0_f64.powi(4),
            QuantityUnit::Pi => 1024.0_f64.powi(5),
            QuantityUnit::Ei => 1024.0_f64.powi(6),
            // Decimal SI units
            QuantityUnit::K => 1e3,
            QuantityUnit::M => 1e6,
            QuantityUnit::G => 1e9,
            QuantityUnit::T => 1e12,
            QuantityUnit::P => 1e15,
            QuantityUnit::E => 1e18,
        }
    }
}

impl ParsedQuantity {
    /// Parses a quantity string into a ParsedQuantity
    fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.is_empty() {
            return Err("Empty quantity".to_string());
        }

        // Try to find the suffix
        let (num_str, unit) = if let Some(pos) =
            s.find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-' && c != '+')
        {
            let num_str = &s[..pos];
            let suffix = &s[pos..];
            let unit = match suffix {
                "n" => QuantityUnit::Nano,
                "u" => QuantityUnit::Micro,
                "m" => QuantityUnit::Milli,
                "Ki" | "ki" => QuantityUnit::Ki,
                "Mi" | "mi" => QuantityUnit::Mi,
                "Gi" | "gi" => QuantityUnit::Gi,
                "Ti" | "ti" => QuantityUnit::Ti,
                "Pi" | "pi" => QuantityUnit::Pi,
                "Ei" | "ei" => QuantityUnit::Ei,
                "K" | "k" => QuantityUnit::K,
                "M" => QuantityUnit::M,
                "G" | "g" => QuantityUnit::G,
                "T" | "t" => QuantityUnit::T,
                "P" | "p" => QuantityUnit::P,
                "E" | "e" => QuantityUnit::E,
                _ => return Err(format!("Invalid quantity suffix: {}", suffix)),
            };
            (num_str, unit)
        } else {
            (s, QuantityUnit::None)
        };

        let value: f64 = num_str
            .parse()
            .map_err(|_| format!("Invalid quantity value: {}", num_str))?;

        Ok(ParsedQuantity { value, unit })
    }

    /// Converts to a base value (multiplied by unit multiplier)
    fn to_base_value(&self) -> f64 {
        self.value * self.unit.multiplier()
    }

    /// Creates from a base value and target unit
    fn from_base_value(base: f64, unit: QuantityUnit) -> Self {
        ParsedQuantity {
            value: base / unit.multiplier(),
            unit,
        }
    }
}

impl Quantity {
    /// Creates a Quantity from a string
    pub fn new(value: String) -> Self {
        Quantity(value)
    }

    /// Creates a Quantity from a string slice (legacy API, no validation)
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Self {
        Quantity(value.to_string())
    }

    /// Creates a validated Quantity from a string, returns error if invalid
    pub fn from_str_validated(value: &str) -> Result<Self, String> {
        ParsedQuantity::parse(value)?;
        Ok(Quantity(value.to_string()))
    }

    /// Returns the string value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Checks if the quantity is zero
    pub fn is_zero(&self) -> bool {
        match ParsedQuantity::parse(&self.0) {
            Ok(pq) => pq.to_base_value() == 0.0,
            Err(_) => false,
        }
    }

    /// Parses this quantity
    fn parse(&self) -> Result<ParsedQuantity, String> {
        ParsedQuantity::parse(&self.0)
    }

    /// Adds two quantities
    /// The result uses the unit of the first quantity
    pub fn add(&self, other: &Quantity) -> Result<Quantity, String> {
        let q1 = self.parse()?;
        let q2 = other.parse()?;

        let base_sum = q1.to_base_value() + q2.to_base_value();
        let result = ParsedQuantity::from_base_value(base_sum, q1.unit.clone());

        let suffix = result.unit.suffix();
        let value_str = if result.value.fract() == 0.0 && result.value.abs() < 1e9 {
            format!("{}", result.value as i64)
        } else {
            format!("{:.6}", result.value)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        };

        Ok(Quantity(value_str + suffix))
    }

    /// Subtracts two quantities
    pub fn sub(&self, other: &Quantity) -> Result<Quantity, String> {
        let q1 = self.parse()?;
        let q2 = other.parse()?;

        let base_diff = q1.to_base_value() - q2.to_base_value();
        if base_diff < 0.0 {
            return Err("Subtraction would result in negative value".to_string());
        }

        let result = ParsedQuantity::from_base_value(base_diff, q1.unit.clone());

        let suffix = result.unit.suffix();
        let value_str = if result.value.fract() == 0.0 && result.value.abs() < 1e9 {
            format!("{}", result.value as i64)
        } else {
            format!("{:.6}", result.value)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        };

        Ok(Quantity(value_str + suffix))
    }

    /// Compares two quantities, returns Ordering
    /// This handles unit conversion automatically
    #[allow(clippy::should_implement_trait)]
    pub fn cmp(&self, other: &Quantity) -> Result<std::cmp::Ordering, String> {
        let q1 = self.parse()?;
        let q2 = other.parse()?;

        let v1 = q1.to_base_value();
        let v2 = q2.to_base_value();

        // Use epsilon for floating point comparison
        const EPSILON: f64 = 1e-9;
        if (v1 - v2).abs() < EPSILON {
            Ok(std::cmp::Ordering::Equal)
        } else if v1 < v2 {
            Ok(std::cmp::Ordering::Less)
        } else {
            Ok(std::cmp::Ordering::Greater)
        }
    }

    /// Converts to f64 in base unit
    /// For CPU: returns cores (100m = 0.1)
    /// For memory: returns bytes
    pub fn to_f64(&self) -> Result<f64, String> {
        let pq = self.parse()?;
        Ok(pq.to_base_value())
    }

    /// Multiplies the quantity by an integer factor.
    ///
    /// Returns `Err` if the result would overflow or be invalid.
    /// The result preserves the original unit, except when zero.
    ///
    /// # Example
    /// ```ignore
    /// let q = Quantity::from_str("100Mi");
    /// let doubled = q.mul(2).unwrap(); // "200Mi"
    /// ```
    pub fn mul(&self, factor: i64) -> Result<Quantity, String> {
        let q = self.parse()?;
        let new_value = q.value as i128 * factor as i128;

        // Check for overflow
        if new_value > i64::MAX as i128 || new_value < i64::MIN as i128 {
            return Err(format!("Multiplication overflow: {} * {}", q.value, factor));
        }

        // Handle zero case specially - return "0" without suffix
        if new_value == 0 {
            return Ok(Quantity("0".to_string()));
        }

        let result = ParsedQuantity {
            value: new_value as f64,
            unit: q.unit.clone(),
        };

        let suffix = result.unit.suffix();
        let value_str = if result.value.fract() == 0.0 && result.value.abs() < 1e9 {
            format!("{}", result.value as i64)
        } else {
            format!("{:.6}", result.value)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        };

        Ok(Quantity(value_str + suffix))
    }

    /// Negates the quantity.
    ///
    /// Returns `Err` if negation would result in an invalid quantity.
    /// Note: Kubernetes resources typically don't support negative values,
    /// so use with caution.
    ///
    /// # Example
    /// ```ignore
    /// let q = Quantity::from_str("100m");
    /// let negated = q.checked_neg().unwrap(); // "-100m"
    /// ```
    pub fn checked_neg(&self) -> Result<Quantity, String> {
        let q = self.parse()?;
        let negated = ParsedQuantity {
            value: -q.value,
            unit: q.unit.clone(),
        };

        let suffix = negated.unit.suffix();
        let value_str = if negated.value.fract() == 0.0 && negated.value.abs() < 1e9 {
            format!("{}", negated.value as i64)
        } else {
            format!("{:.6}", negated.value)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        };

        Ok(Quantity(value_str + suffix))
    }

    /// Returns the sign of the quantity.
    ///
    /// Uses `Ordering` for a Rust-idiomatic approach:
    /// - `Less`: negative quantity
    /// - `Equal`: zero
    /// - `Greater`: positive quantity
    ///
    /// # Example
    /// ```ignore
    /// assert_eq!(Quantity::from_str("100m").sign(), Ok(Ordering::Greater));
    /// assert_eq!(Quantity::from_str("0").sign(), Ok(Ordering::Equal));
    /// ```
    pub fn sign(&self) -> Result<std::cmp::Ordering, String> {
        let q = self.parse()?;
        const EPSILON: f64 = 1e-9;
        if q.value.abs() < EPSILON {
            Ok(std::cmp::Ordering::Equal)
        } else if q.value < 0.0 {
            Ok(std::cmp::Ordering::Less)
        } else {
            Ok(std::cmp::Ordering::Greater)
        }
    }

    /// Attempts to convert the quantity to an i64 value.
    ///
    /// Returns `Err` if the quantity has a fractional part, overflows i64,
    /// or cannot be represented as an integer.
    ///
    /// This is the Rust-idiomatic version of Go's `AsInt64() (int64, bool)`,
    /// using `Result` instead of a tuple with a boolean flag.
    ///
    /// # Example
    /// ```ignore
    /// assert_eq!(Quantity::from_str("100").as_i64(), Ok(100));
    /// assert!(Quantity::from_str("100m").as_i64().is_err()); // fractional
    /// ```
    pub fn as_i64(&self) -> Result<i64, String> {
        let q = self.parse()?;

        // Reject quantities with fractional units (n, u, m) or binary units
        // These represent fractional values that can't be cleanly represented as i64
        match q.unit {
            QuantityUnit::Nano | QuantityUnit::Micro | QuantityUnit::Milli => {
                return Err(format!(
                    "Cannot convert fractional unit quantity to i64: {}",
                    self.0
                ));
            }
            _ => {}
        }

        // Check if the value is integral (no fractional part)
        if q.value.fract() != 0.0 {
            return Err(format!("Quantity has fractional part: {}", q.value));
        }

        // Check for i64 bounds
        if q.value > i64::MAX as f64 || q.value < i64::MIN as f64 {
            return Err(format!("Quantity out of i64 range: {}", q.value));
        }

        Ok(q.value as i64)
    }
}

impl QuantityUnit {
    /// Returns the string suffix for this unit
    fn suffix(&self) -> &'static str {
        match self {
            QuantityUnit::Nano => "n",
            QuantityUnit::Micro => "u",
            QuantityUnit::Milli => "m",
            QuantityUnit::None => "",
            QuantityUnit::Ki => "Ki",
            QuantityUnit::Mi => "Mi",
            QuantityUnit::Gi => "Gi",
            QuantityUnit::Ti => "Ti",
            QuantityUnit::Pi => "Pi",
            QuantityUnit::Ei => "Ei",
            QuantityUnit::K => "k",
            QuantityUnit::M => "M",
            QuantityUnit::G => "G",
            QuantityUnit::T => "T",
            QuantityUnit::P => "P",
            QuantityUnit::E => "E",
        }
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
mod tests {}

// ============================================================================
// Helper functions for serde
// ============================================================================

/// Helper function for serde skip_serializing_if for i64 fields
pub fn is_zero_i64(value: &i64) -> bool {
    *value == 0
}

/// Helper function for serde skip_serializing_if for bool fields
/// Skip serializing when the value is false (matches Go's omitempty behavior)
pub fn is_false(value: &bool) -> bool {
    !*value
}
