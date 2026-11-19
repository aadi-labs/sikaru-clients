pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The shared agent directory, or one end user's directory.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListFilesMemoryRequestDirectory {
    Agent,
    User,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListFilesMemoryRequestDirectory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Agent => serializer.serialize_str("agent"),
            Self::User => serializer.serialize_str("user"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListFilesMemoryRequestDirectory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "agent" => Ok(Self::Agent),
            "user" => Ok(Self::User),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListFilesMemoryRequestDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Agent => write!(f, "agent"),
            Self::User => write!(f, "user"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
