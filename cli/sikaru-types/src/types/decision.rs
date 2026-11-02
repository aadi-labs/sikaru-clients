pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Answers to the requested questions. A question the model could not answer is absent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Decision {
    #[serde(default)]
    pub answers: HashMap<String, DecisionAnswersValue>,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub usage: DecisionUsage,
}

impl Decision {
    pub fn builder() -> DecisionBuilder {
        <DecisionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DecisionBuilder {
    answers: Option<HashMap<String, DecisionAnswersValue>>,
    model: Option<String>,
    usage: Option<DecisionUsage>,
}

impl DecisionBuilder {
    pub fn answers(mut self, value: HashMap<String, DecisionAnswersValue>) -> Self {
        self.answers = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn usage(mut self, value: DecisionUsage) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Decision`].
    /// This method will fail if any of the following fields are not set:
    /// - [`answers`](DecisionBuilder::answers)
    /// - [`model`](DecisionBuilder::model)
    /// - [`usage`](DecisionBuilder::usage)
    pub fn build(self) -> Result<Decision, BuildError> {
        Ok(Decision {
            answers: self.answers.ok_or_else(|| BuildError::missing_field("answers"))?,
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
            usage: self.usage.ok_or_else(|| BuildError::missing_field("usage"))?,
        })
    }
}
