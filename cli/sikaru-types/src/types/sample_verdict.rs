pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SampleVerdict {
    Flagged,
    LooksFine,
    Inconclusive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SampleVerdict {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Flagged => serializer.serialize_str("flagged"),
            Self::LooksFine => serializer.serialize_str("looks_fine"),
            Self::Inconclusive => serializer.serialize_str("inconclusive"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SampleVerdict {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "flagged" => Ok(Self::Flagged),
            "looks_fine" => Ok(Self::LooksFine),
            "inconclusive" => Ok(Self::Inconclusive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SampleVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Flagged => write!(f, "flagged"),
            Self::LooksFine => write!(f, "looks_fine"),
            Self::Inconclusive => write!(f, "inconclusive"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
