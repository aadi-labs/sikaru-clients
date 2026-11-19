pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GradingViewStatus {
    On,
    Off,
    Disabled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GradingViewStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::On => serializer.serialize_str("on"),
            Self::Off => serializer.serialize_str("off"),
            Self::Disabled => serializer.serialize_str("disabled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GradingViewStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "disabled" => Ok(Self::Disabled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GradingViewStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
            Self::Disabled => write!(f, "disabled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
