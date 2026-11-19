pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// `pending` while its release check runs; `passed` once the check cleared, before the publish is sent again to release it; `held` when the check blocked it or couldn't verify it (Publish Anyway can release it); `released` once its version is live; `ended` when it ended any other way.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PublicationStateStatus {
    Pending,
    Passed,
    Held,
    Released,
    Ended,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PublicationStateStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Passed => serializer.serialize_str("passed"),
            Self::Held => serializer.serialize_str("held"),
            Self::Released => serializer.serialize_str("released"),
            Self::Ended => serializer.serialize_str("ended"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PublicationStateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "passed" => Ok(Self::Passed),
            "held" => Ok(Self::Held),
            "released" => Ok(Self::Released),
            "ended" => Ok(Self::Ended),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PublicationStateStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Passed => write!(f, "passed"),
            Self::Held => write!(f, "held"),
            Self::Released => write!(f, "released"),
            Self::Ended => write!(f, "ended"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
