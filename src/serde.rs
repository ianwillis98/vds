//! Serde serialization support for [`VDChar`](crate::VDChar) and [`VDString`](crate::VDString).
//!
//! This module is only available when the `serde` feature is enabled.
//!
//! # Representation
//!
//! - [`VDChar`] is serialized as a single `char`, e.g. `'A'`
//! - [`VDString`] is serialized as a `str`, e.g. `"ABC234"`
//!
//! These formats are human-friendly, compact, and interoperable with
//! other text-based formats like JSON, TOML, and YAML.
//!
//! Invalid deserialization inputs will produce an error at runtime.

use crate::{VDChar, VDString};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serializes a [`VDChar`] as a single `char`.
///
/// # Example (JSON)
/// ```json
/// "X"
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for VDChar {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_char(self.as_char())
    }
}

/// Deserializes a [`VDChar`] from a `char`.
///
/// Returns an error if the character is not in the visibly distinguishable set.
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for VDChar {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let c = <char>::deserialize(deserializer)?;
        VDChar::new(c).ok_or_else(|| serde::de::Error::custom(format_args!("invalid VDChar: {}", c)))
    }
}

/// Serializes a [`VDString`] as a `str`, e.g. `"ABC29"`.
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for VDString {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self)
    }
}

/// Deserializes a [`VDString`] from a `str`.
///
/// Returns an error if any character is not in the allowed set.
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for VDString {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&str>::deserialize(deserializer)?;
        s.parse().map_err(|_| serde::de::Error::custom("invalid VDString"))
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    use super::*;
    use serde_json;

    fn vd(c: char) -> VDChar {
        VDChar::new(c).unwrap()
    }

    #[test]
    fn vdchar_roundtrip_json() {
        let c = vd('M');
        let serialized = serde_json::to_string(&c).unwrap();
        assert_eq!(serialized, "\"M\"");

        let deserialized: VDChar = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, c);
    }

    #[test]
    fn vdstring_roundtrip_json() {
        let original: VDString = "K2Z7".parse().unwrap();
        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(json, "\"K2Z7\"");

        let decoded: VDString = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn invalid_vdchar_fails() {
        let err = serde_json::from_str::<VDChar>("\"O\"");
        assert!(err.is_err());

        let err2 = serde_json::from_str::<VDChar>("\"!\"");
        assert!(err2.is_err());
    }

    #[test]
    fn invalid_vdstring_fails() {
        let err = serde_json::from_str::<VDString>("\"ABCO\"");
        assert!(err.is_err());

        let err2 = serde_json::from_str::<VDString>("\"abc\"");
        assert!(err2.is_err());
    }
}
