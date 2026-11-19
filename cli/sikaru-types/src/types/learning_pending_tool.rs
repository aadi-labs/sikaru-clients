pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LearningPendingTool {
    #[serde(default)]
    pub arguments: HashMap<String, serde_json::Value>,
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    pub capability_name: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(rename = "toolCallId")]
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(rename = "toolProviderId")]
    #[serde(default)]
    pub tool_provider_id: String,
}

impl LearningPendingTool {
    pub fn builder() -> LearningPendingToolBuilder {
        <LearningPendingToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LearningPendingToolBuilder {
    arguments: Option<HashMap<String, serde_json::Value>>,
    capability_name: Option<String>,
    request_id: Option<String>,
    run_id: Option<String>,
    tool_call_id: Option<String>,
    tool_provider_id: Option<String>,
}

impl LearningPendingToolBuilder {
    pub fn arguments(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn capability_name(mut self, value: impl Into<String>) -> Self {
        self.capability_name = Some(value.into());
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn tool_provider_id(mut self, value: impl Into<String>) -> Self {
        self.tool_provider_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LearningPendingTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`arguments`](LearningPendingToolBuilder::arguments)
    /// - [`capability_name`](LearningPendingToolBuilder::capability_name)
    /// - [`request_id`](LearningPendingToolBuilder::request_id)
    /// - [`run_id`](LearningPendingToolBuilder::run_id)
    /// - [`tool_call_id`](LearningPendingToolBuilder::tool_call_id)
    /// - [`tool_provider_id`](LearningPendingToolBuilder::tool_provider_id)
    pub fn build(self) -> Result<LearningPendingTool, BuildError> {
        Ok(LearningPendingTool {
            arguments: self.arguments.ok_or_else(|| BuildError::missing_field("arguments"))?,
            capability_name: self.capability_name.ok_or_else(|| BuildError::missing_field("capability_name"))?,
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_provider_id: self.tool_provider_id.ok_or_else(|| BuildError::missing_field("tool_provider_id"))?,
        })
    }
}
