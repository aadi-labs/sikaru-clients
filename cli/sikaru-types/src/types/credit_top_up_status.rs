pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreditTopUpStatus {
    Offered,
    Charging,
    PendingAction,
    Paid,
    Credited,
    Expired,
    Failed,
    Disputed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreditTopUpStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Offered => serializer.serialize_str("offered"),
            Self::Charging => serializer.serialize_str("charging"),
            Self::PendingAction => serializer.serialize_str("pending_action"),
            Self::Paid => serializer.serialize_str("paid"),
            Self::Credited => serializer.serialize_str("credited"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Disputed => serializer.serialize_str("disputed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreditTopUpStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "offered" => Ok(Self::Offered),
            "charging" => Ok(Self::Charging),
            "pending_action" => Ok(Self::PendingAction),
            "paid" => Ok(Self::Paid),
            "credited" => Ok(Self::Credited),
            "expired" => Ok(Self::Expired),
            "failed" => Ok(Self::Failed),
            "disputed" => Ok(Self::Disputed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreditTopUpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Offered => write!(f, "offered"),
            Self::Charging => write!(f, "charging"),
            Self::PendingAction => write!(f, "pending_action"),
            Self::Paid => write!(f, "paid"),
            Self::Credited => write!(f, "credited"),
            Self::Expired => write!(f, "expired"),
            Self::Failed => write!(f, "failed"),
            Self::Disputed => write!(f, "disputed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
