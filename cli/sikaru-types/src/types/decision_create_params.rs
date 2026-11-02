pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub questions: HashMap<String, HashMap<String, serde_json::Value>>,
    pub state: SystemOneRequestState,
}

impl DecisionCreateParams {
    pub fn builder() -> DecisionCreateParamsBuilder {
        <DecisionCreateParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DecisionCreateParamsBuilder {
    model: Option<String>,
    options: Option<HashMap<String, serde_json::Value>>,
    questions: Option<HashMap<String, HashMap<String, serde_json::Value>>>,
    state: Option<SystemOneRequestState>,
}

impl DecisionCreateParamsBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn options(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn questions(mut self, value: HashMap<String, HashMap<String, serde_json::Value>>) -> Self {
        self.questions = Some(value);
        self
    }

    pub fn state(mut self, value: SystemOneRequestState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DecisionCreateParams`].
    /// This method will fail if any of the following fields are not set:
    /// - [`questions`](DecisionCreateParamsBuilder::questions)
    /// - [`state`](DecisionCreateParamsBuilder::state)
    pub fn build(self) -> Result<DecisionCreateParams, BuildError> {
        Ok(DecisionCreateParams {
            model: self.model,
            options: self.options,
            questions: self.questions.ok_or_else(|| BuildError::missing_field("questions"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}

