pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HeldResultOutcome {
    Blocked,
    CouldntVerify,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for HeldResultOutcome {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Blocked => serializer.serialize_str("blocked"),
            Self::CouldntVerify => serializer.serialize_str("couldnt_verify"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for HeldResultOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "blocked" => Ok(Self::Blocked),
            "couldnt_verify" => Ok(Self::CouldntVerify),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for HeldResultOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blocked => write!(f, "blocked"),
            Self::CouldntVerify => write!(f, "couldnt_verify"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
