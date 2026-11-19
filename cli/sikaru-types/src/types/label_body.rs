pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LabelBody {
    #[serde(rename = "conversationKey")]
    #[serde(default)]
    pub conversation_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub verdict: LabelBodyVerdict,
}

impl LabelBody {
    pub fn builder() -> LabelBodyBuilder {
        <LabelBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LabelBodyBuilder {
    conversation_key: Option<String>,
    reason: Option<String>,
    verdict: Option<LabelBodyVerdict>,
}

impl LabelBodyBuilder {
    pub fn conversation_key(mut self, value: impl Into<String>) -> Self {
        self.conversation_key = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn verdict(mut self, value: LabelBodyVerdict) -> Self {
        self.verdict = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LabelBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_key`](LabelBodyBuilder::conversation_key)
    /// - [`verdict`](LabelBodyBuilder::verdict)
    pub fn build(self) -> Result<LabelBody, BuildError> {
        Ok(LabelBody {
            conversation_key: self.conversation_key.ok_or_else(|| BuildError::missing_field("conversation_key"))?,
            reason: self.reason,
            verdict: self.verdict.ok_or_else(|| BuildError::missing_field("verdict"))?,
        })
    }
}

