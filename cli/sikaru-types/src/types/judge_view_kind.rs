pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JudgeViewKind {
    DecisionModel,
    Llm,
    Agent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for JudgeViewKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DecisionModel => serializer.serialize_str("decision-model"),
            Self::Llm => serializer.serialize_str("llm"),
            Self::Agent => serializer.serialize_str("agent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for JudgeViewKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "decision-model" => Ok(Self::DecisionModel),
            "llm" => Ok(Self::Llm),
            "agent" => Ok(Self::Agent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for JudgeViewKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecisionModel => write!(f, "decision-model"),
            Self::Llm => write!(f, "llm"),
            Self::Agent => write!(f, "agent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
