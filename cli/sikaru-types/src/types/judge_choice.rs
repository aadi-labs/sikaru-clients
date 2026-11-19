pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct JudgeChoice {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    pub kind: JudgeChoiceKind,
}

impl JudgeChoice {
    pub fn builder() -> JudgeChoiceBuilder {
        <JudgeChoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeChoiceBuilder {
    agent_id: Option<String>,
    kind: Option<JudgeChoiceKind>,
}

impl JudgeChoiceBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: JudgeChoiceKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JudgeChoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](JudgeChoiceBuilder::kind)
    pub fn build(self) -> Result<JudgeChoice, BuildError> {
        Ok(JudgeChoice {
            agent_id: self.agent_id,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
