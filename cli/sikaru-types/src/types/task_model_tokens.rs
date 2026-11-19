pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskModelTokens {
    #[serde(default)]
    pub cached_input_tokens: i64,
    #[serde(default)]
    pub input_tokens: i64,
    #[serde(default)]
    pub output_tokens: i64,
}

impl TaskModelTokens {
    pub fn builder() -> TaskModelTokensBuilder {
        <TaskModelTokensBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskModelTokensBuilder {
    cached_input_tokens: Option<i64>,
    input_tokens: Option<i64>,
    output_tokens: Option<i64>,
}

impl TaskModelTokensBuilder {
    pub fn cached_input_tokens(mut self, value: i64) -> Self {
        self.cached_input_tokens = Some(value);
        self
    }

    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskModelTokens`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cached_input_tokens`](TaskModelTokensBuilder::cached_input_tokens)
    /// - [`input_tokens`](TaskModelTokensBuilder::input_tokens)
    /// - [`output_tokens`](TaskModelTokensBuilder::output_tokens)
    pub fn build(self) -> Result<TaskModelTokens, BuildError> {
        Ok(TaskModelTokens {
            cached_input_tokens: self.cached_input_tokens.ok_or_else(|| BuildError::missing_field("cached_input_tokens"))?,
            input_tokens: self.input_tokens.ok_or_else(|| BuildError::missing_field("input_tokens"))?,
            output_tokens: self.output_tokens.ok_or_else(|| BuildError::missing_field("output_tokens"))?,
        })
    }
}
