pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DecisionUsage {
    #[serde(default)]
    pub input_tokens: i64,
    #[serde(default)]
    pub output_tokens: i64,
}

impl DecisionUsage {
    pub fn builder() -> DecisionUsageBuilder {
        <DecisionUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DecisionUsageBuilder {
    input_tokens: Option<i64>,
    output_tokens: Option<i64>,
}

impl DecisionUsageBuilder {
    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DecisionUsage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input_tokens`](DecisionUsageBuilder::input_tokens)
    /// - [`output_tokens`](DecisionUsageBuilder::output_tokens)
    pub fn build(self) -> Result<DecisionUsage, BuildError> {
        Ok(DecisionUsage {
            input_tokens: self.input_tokens.ok_or_else(|| BuildError::missing_field("input_tokens"))?,
            output_tokens: self.output_tokens.ok_or_else(|| BuildError::missing_field("output_tokens"))?,
        })
    }
}
