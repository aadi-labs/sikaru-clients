pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// `ready`: credits fund work; `no_card`: the organization has not saved a card, so nothing funds work yet; `complimentary`: usage is covered and needs no credits.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreditBalanceState {
    Ready,
    NoCard,
    Complimentary,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreditBalanceState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ready => serializer.serialize_str("ready"),
            Self::NoCard => serializer.serialize_str("no_card"),
            Self::Complimentary => serializer.serialize_str("complimentary"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreditBalanceState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ready" => Ok(Self::Ready),
            "no_card" => Ok(Self::NoCard),
            "complimentary" => Ok(Self::Complimentary),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreditBalanceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ready => write!(f, "ready"),
            Self::NoCard => write!(f, "no_card"),
            Self::Complimentary => write!(f, "complimentary"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
