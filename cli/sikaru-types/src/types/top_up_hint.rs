pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An agent can add credits itself: create a top-up offer, then pay its URL.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TopUpHint {
    /// CLI command that creates the top-up offer.
    #[serde(default)]
    pub cli: String,
    #[serde(rename = "maxUsd")]
    #[serde(default)]
    pub max_usd: String,
    #[serde(rename = "minUsd")]
    #[serde(default)]
    pub min_usd: String,
    /// SDK operation that creates the top-up offer.
    #[serde(default)]
    pub operation: String,
}

impl TopUpHint {
    pub fn builder() -> TopUpHintBuilder {
        <TopUpHintBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TopUpHintBuilder {
    cli: Option<String>,
    max_usd: Option<String>,
    min_usd: Option<String>,
    operation: Option<String>,
}

impl TopUpHintBuilder {
    pub fn cli(mut self, value: impl Into<String>) -> Self {
        self.cli = Some(value.into());
        self
    }

    pub fn max_usd(mut self, value: impl Into<String>) -> Self {
        self.max_usd = Some(value.into());
        self
    }

    pub fn min_usd(mut self, value: impl Into<String>) -> Self {
        self.min_usd = Some(value.into());
        self
    }

    pub fn operation(mut self, value: impl Into<String>) -> Self {
        self.operation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TopUpHint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cli`](TopUpHintBuilder::cli)
    /// - [`max_usd`](TopUpHintBuilder::max_usd)
    /// - [`min_usd`](TopUpHintBuilder::min_usd)
    /// - [`operation`](TopUpHintBuilder::operation)
    pub fn build(self) -> Result<TopUpHint, BuildError> {
        Ok(TopUpHint {
            cli: self.cli.ok_or_else(|| BuildError::missing_field("cli"))?,
            max_usd: self.max_usd.ok_or_else(|| BuildError::missing_field("max_usd"))?,
            min_usd: self.min_usd.ok_or_else(|| BuildError::missing_field("min_usd"))?,
            operation: self.operation.ok_or_else(|| BuildError::missing_field("operation"))?,
        })
    }
}
