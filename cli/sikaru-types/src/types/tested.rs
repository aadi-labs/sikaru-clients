pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Tested {
    #[serde(rename = "conversationKey")]
    #[serde(default)]
    pub conversation_key: String,
    /// The criterion that flagged it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    pub verdict: TestedVerdict,
}

impl Tested {
    pub fn builder() -> TestedBuilder {
        <TestedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestedBuilder {
    conversation_key: Option<String>,
    reason: Option<String>,
    run_id: Option<String>,
    verdict: Option<TestedVerdict>,
}

impl TestedBuilder {
    pub fn conversation_key(mut self, value: impl Into<String>) -> Self {
        self.conversation_key = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn verdict(mut self, value: TestedVerdict) -> Self {
        self.verdict = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Tested`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_key`](TestedBuilder::conversation_key)
    /// - [`verdict`](TestedBuilder::verdict)
    pub fn build(self) -> Result<Tested, BuildError> {
        Ok(Tested {
            conversation_key: self.conversation_key.ok_or_else(|| BuildError::missing_field("conversation_key"))?,
            reason: self.reason,
            run_id: self.run_id,
            verdict: self.verdict.ok_or_else(|| BuildError::missing_field("verdict"))?,
        })
    }
}
