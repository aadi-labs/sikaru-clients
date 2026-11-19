pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReleaseCheckReceiptStatus {
    Passed,
    Overridden,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReleaseCheckReceiptStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Passed => serializer.serialize_str("passed"),
            Self::Overridden => serializer.serialize_str("overridden"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReleaseCheckReceiptStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "passed" => Ok(Self::Passed),
            "overridden" => Ok(Self::Overridden),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReleaseCheckReceiptStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Passed => write!(f, "passed"),
            Self::Overridden => write!(f, "overridden"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
