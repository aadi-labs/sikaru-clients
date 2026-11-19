pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvocationPolicy {
    /// Allow this session to supply context to other sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_session_recall: Option<bool>,
    /// Allow this session to supply memory, persona, skill and evaluation learning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning: Option<bool>,
    /// Disabled runs only contextual model inference, without a code kernel or tool dispatch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<InvocationPolicyTools>,
}

impl InvocationPolicy {
    pub fn builder() -> InvocationPolicyBuilder {
        <InvocationPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvocationPolicyBuilder {
    cross_session_recall: Option<bool>,
    learning: Option<bool>,
    tools: Option<InvocationPolicyTools>,
}

impl InvocationPolicyBuilder {
    pub fn cross_session_recall(mut self, value: bool) -> Self {
        self.cross_session_recall = Some(value);
        self
    }

    pub fn learning(mut self, value: bool) -> Self {
        self.learning = Some(value);
        self
    }

    pub fn tools(mut self, value: InvocationPolicyTools) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvocationPolicy`].
    pub fn build(self) -> Result<InvocationPolicy, BuildError> {
        Ok(InvocationPolicy {
            cross_session_recall: self.cross_session_recall,
            learning: self.learning,
            tools: self.tools,
        })
    }
}
