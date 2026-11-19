pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Sample {
    #[serde(rename = "conversationKey")]
    #[serde(default)]
    pub conversation_key: String,
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "scoredAt")]
    #[serde(default)]
    pub scored_at: String,
    pub verdict: SampleVerdict,
}

impl Sample {
    pub fn builder() -> SampleBuilder {
        <SampleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SampleBuilder {
    conversation_key: Option<String>,
    run_id: Option<String>,
    scored_at: Option<String>,
    verdict: Option<SampleVerdict>,
}

impl SampleBuilder {
    pub fn conversation_key(mut self, value: impl Into<String>) -> Self {
        self.conversation_key = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn scored_at(mut self, value: impl Into<String>) -> Self {
        self.scored_at = Some(value.into());
        self
    }

    pub fn verdict(mut self, value: SampleVerdict) -> Self {
        self.verdict = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Sample`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_key`](SampleBuilder::conversation_key)
    /// - [`scored_at`](SampleBuilder::scored_at)
    /// - [`verdict`](SampleBuilder::verdict)
    pub fn build(self) -> Result<Sample, BuildError> {
        Ok(Sample {
            conversation_key: self.conversation_key.ok_or_else(|| BuildError::missing_field("conversation_key"))?,
            run_id: self.run_id,
            scored_at: self.scored_at.ok_or_else(|| BuildError::missing_field("scored_at"))?,
            verdict: self.verdict.ok_or_else(|| BuildError::missing_field("verdict"))?,
        })
    }
}
