pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TopUpInput {
    /// Whole cents, for example "20.00".
    #[serde(rename = "amountUsd")]
    #[serde(default)]
    pub amount_usd: String,
}

impl TopUpInput {
    pub fn builder() -> TopUpInputBuilder {
        <TopUpInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TopUpInputBuilder {
    amount_usd: Option<String>,
}

impl TopUpInputBuilder {
    pub fn amount_usd(mut self, value: impl Into<String>) -> Self {
        self.amount_usd = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TopUpInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_usd`](TopUpInputBuilder::amount_usd)
    pub fn build(self) -> Result<TopUpInput, BuildError> {
        Ok(TopUpInput {
            amount_usd: self.amount_usd.ok_or_else(|| BuildError::missing_field("amount_usd"))?,
        })
    }
}

