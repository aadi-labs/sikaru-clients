pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MemoryForgetStatus {
    Pending,
    Complete,
    NothingFound,
    Failed,
    Reversed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MemoryForgetStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Complete => serializer.serialize_str("complete"),
            Self::NothingFound => serializer.serialize_str("nothing_found"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Reversed => serializer.serialize_str("reversed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MemoryForgetStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "complete" => Ok(Self::Complete),
            "nothing_found" => Ok(Self::NothingFound),
            "failed" => Ok(Self::Failed),
            "reversed" => Ok(Self::Reversed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MemoryForgetStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Complete => write!(f, "complete"),
            Self::NothingFound => write!(f, "nothing_found"),
            Self::Failed => write!(f, "failed"),
            Self::Reversed => write!(f, "reversed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
