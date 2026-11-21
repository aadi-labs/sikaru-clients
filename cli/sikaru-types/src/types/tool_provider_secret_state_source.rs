pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolProviderSecretStateSource {
    Supplied,
    Generated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolProviderSecretStateSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Supplied => serializer.serialize_str("supplied"),
            Self::Generated => serializer.serialize_str("generated"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolProviderSecretStateSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "supplied" => Ok(Self::Supplied),
            "generated" => Ok(Self::Generated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolProviderSecretStateSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Supplied => write!(f, "supplied"),
            Self::Generated => write!(f, "generated"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
