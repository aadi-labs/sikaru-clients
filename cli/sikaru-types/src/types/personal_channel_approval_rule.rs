pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PersonalChannelApprovalRule {
    Saved,
    LimitReached,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PersonalChannelApprovalRule {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Saved => serializer.serialize_str("saved"),
            Self::LimitReached => serializer.serialize_str("limit_reached"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PersonalChannelApprovalRule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "saved" => Ok(Self::Saved),
            "limit_reached" => Ok(Self::LimitReached),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PersonalChannelApprovalRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Saved => write!(f, "saved"),
            Self::LimitReached => write!(f, "limit_reached"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
