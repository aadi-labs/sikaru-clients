pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GraderVersionViewStatus {
    Provisional,
    Live,
    Retired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GraderVersionViewStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Provisional => serializer.serialize_str("provisional"),
            Self::Live => serializer.serialize_str("live"),
            Self::Retired => serializer.serialize_str("retired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GraderVersionViewStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "provisional" => Ok(Self::Provisional),
            "live" => Ok(Self::Live),
            "retired" => Ok(Self::Retired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GraderVersionViewStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Provisional => write!(f, "provisional"),
            Self::Live => write!(f, "live"),
            Self::Retired => write!(f, "retired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
