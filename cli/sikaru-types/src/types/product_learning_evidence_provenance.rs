pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProductLearningEvidenceProvenance {
    UserMessage,
    AssistantMessage,
    ToolResult,
    UserCorrection,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProductLearningEvidenceProvenance {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UserMessage => serializer.serialize_str("user_message"),
            Self::AssistantMessage => serializer.serialize_str("assistant_message"),
            Self::ToolResult => serializer.serialize_str("tool_result"),
            Self::UserCorrection => serializer.serialize_str("user_correction"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProductLearningEvidenceProvenance {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user_message" => Ok(Self::UserMessage),
            "assistant_message" => Ok(Self::AssistantMessage),
            "tool_result" => Ok(Self::ToolResult),
            "user_correction" => Ok(Self::UserCorrection),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProductLearningEvidenceProvenance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserMessage => write!(f, "user_message"),
            Self::AssistantMessage => write!(f, "assistant_message"),
            Self::ToolResult => write!(f, "tool_result"),
            Self::UserCorrection => write!(f, "user_correction"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
