pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunStarted {
    #[serde(default)]
    pub conversations: i64,
    #[serde(rename = "estimatedUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_usd: Option<String>,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
}

impl RunStarted {
    pub fn builder() -> RunStartedBuilder {
        <RunStartedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunStartedBuilder {
    conversations: Option<i64>,
    estimated_usd: Option<String>,
    run_id: Option<String>,
}

impl RunStartedBuilder {
    pub fn conversations(mut self, value: i64) -> Self {
        self.conversations = Some(value);
        self
    }

    pub fn estimated_usd(mut self, value: impl Into<String>) -> Self {
        self.estimated_usd = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunStarted`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversations`](RunStartedBuilder::conversations)
    /// - [`run_id`](RunStartedBuilder::run_id)
    pub fn build(self) -> Result<RunStarted, BuildError> {
        Ok(RunStarted {
            conversations: self.conversations.ok_or_else(|| BuildError::missing_field("conversations"))?,
            estimated_usd: self.estimated_usd,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
        })
    }
}
