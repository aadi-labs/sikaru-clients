pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OutputPreviewSource {
    Live,
    Committed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OutputPreviewSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Live => serializer.serialize_str("live"),
            Self::Committed => serializer.serialize_str("committed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OutputPreviewSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "live" => Ok(Self::Live),
            "committed" => Ok(Self::Committed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OutputPreviewSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Live => write!(f, "live"),
            Self::Committed => write!(f, "committed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
