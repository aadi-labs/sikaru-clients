pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GraderDetailStatus {
    Live,
    Provisional,
    Uncalibrated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GraderDetailStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Live => serializer.serialize_str("live"),
            Self::Provisional => serializer.serialize_str("provisional"),
            Self::Uncalibrated => serializer.serialize_str("uncalibrated"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GraderDetailStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "live" => Ok(Self::Live),
            "provisional" => Ok(Self::Provisional),
            "uncalibrated" => Ok(Self::Uncalibrated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GraderDetailStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Live => write!(f, "live"),
            Self::Provisional => write!(f, "provisional"),
            Self::Uncalibrated => write!(f, "uncalibrated"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
