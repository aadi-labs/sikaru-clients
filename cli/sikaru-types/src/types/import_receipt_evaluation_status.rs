pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImportReceiptEvaluationStatus {
    Ready,
    Pending,
    Unjudgeable,
    Superseded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ImportReceiptEvaluationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ready => serializer.serialize_str("ready"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Unjudgeable => serializer.serialize_str("unjudgeable"),
            Self::Superseded => serializer.serialize_str("superseded"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ImportReceiptEvaluationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ready" => Ok(Self::Ready),
            "pending" => Ok(Self::Pending),
            "unjudgeable" => Ok(Self::Unjudgeable),
            "superseded" => Ok(Self::Superseded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ImportReceiptEvaluationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ready => write!(f, "ready"),
            Self::Pending => write!(f, "pending"),
            Self::Unjudgeable => write!(f, "unjudgeable"),
            Self::Superseded => write!(f, "superseded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
