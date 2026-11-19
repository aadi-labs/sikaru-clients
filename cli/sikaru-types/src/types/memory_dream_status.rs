pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MemoryDreamStatus {
    Requested,
    Running,
    Complete,
    Failed,
    Cancelled,
    Disabled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MemoryDreamStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Requested => serializer.serialize_str("requested"),
            Self::Running => serializer.serialize_str("running"),
            Self::Complete => serializer.serialize_str("complete"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Disabled => serializer.serialize_str("disabled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MemoryDreamStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "requested" => Ok(Self::Requested),
            "running" => Ok(Self::Running),
            "complete" => Ok(Self::Complete),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "disabled" => Ok(Self::Disabled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MemoryDreamStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requested => write!(f, "requested"),
            Self::Running => write!(f, "running"),
            Self::Complete => write!(f, "complete"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Disabled => write!(f, "disabled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
