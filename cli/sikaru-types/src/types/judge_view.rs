pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct JudgeView {
    /// The project agent that judges, for an agent judge.
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    pub kind: JudgeViewKind,
}

impl JudgeView {
    pub fn builder() -> JudgeViewBuilder {
        <JudgeViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeViewBuilder {
    agent_id: Option<String>,
    kind: Option<JudgeViewKind>,
}

impl JudgeViewBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: JudgeViewKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JudgeView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](JudgeViewBuilder::kind)
    pub fn build(self) -> Result<JudgeView, BuildError> {
        Ok(JudgeView {
            agent_id: self.agent_id,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
