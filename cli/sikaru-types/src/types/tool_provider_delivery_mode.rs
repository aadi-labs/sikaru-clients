pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// push: Sikaru signs and sends tool requests to the HTTPS endpoint. poll: the product reads pending actions.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolProviderDeliveryMode {
    Push,
    Poll,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolProviderDeliveryMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Push => serializer.serialize_str("push"),
            Self::Poll => serializer.serialize_str("poll"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolProviderDeliveryMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "push" => Ok(Self::Push),
            "poll" => Ok(Self::Poll),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolProviderDeliveryMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push => write!(f, "push"),
            Self::Poll => write!(f, "poll"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
