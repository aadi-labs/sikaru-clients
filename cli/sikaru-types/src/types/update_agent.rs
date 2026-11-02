pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAgent {
    /// Share of maxConcurrentTurns that background turns may use; null restores the default.
    #[serde(rename = "backgroundTurnShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_turn_share: Option<f64>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Most turns the agent runs at once; null restores the default.
    #[serde(rename = "maxConcurrentTurns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_turns: Option<i64>,
}

impl UpdateAgent {
    pub fn builder() -> UpdateAgentBuilder {
        <UpdateAgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAgentBuilder {
    background_turn_share: Option<f64>,
    display_name: Option<String>,
    max_concurrent_turns: Option<i64>,
}

impl UpdateAgentBuilder {
    pub fn background_turn_share(mut self, value: f64) -> Self {
        self.background_turn_share = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn max_concurrent_turns(mut self, value: i64) -> Self {
        self.max_concurrent_turns = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAgent`].
    pub fn build(self) -> Result<UpdateAgent, BuildError> {
        Ok(UpdateAgent {
            background_turn_share: self.background_turn_share,
            display_name: self.display_name,
            max_concurrent_turns: self.max_concurrent_turns,
        })
    }
}

