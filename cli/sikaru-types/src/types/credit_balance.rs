pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreditBalance {
    /// Credits available for new work now, after amounts held for work in progress; null unless `ready`.
    #[serde(rename = "availableUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_usd: Option<String>,
    /// `ready`: credits fund work; `no_card`: the organization has not saved a card, so nothing funds work yet; `complimentary`: usage is covered and needs no credits.
    pub state: CreditBalanceState,
    /// Largest top-up offer an agent can create now; null while agent top-ups are off or unavailable.
    #[serde(rename = "topUpMaxUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_up_max_usd: Option<String>,
}

impl CreditBalance {
    pub fn builder() -> CreditBalanceBuilder {
        <CreditBalanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditBalanceBuilder {
    available_usd: Option<String>,
    state: Option<CreditBalanceState>,
    top_up_max_usd: Option<String>,
}

impl CreditBalanceBuilder {
    pub fn available_usd(mut self, value: impl Into<String>) -> Self {
        self.available_usd = Some(value.into());
        self
    }

    pub fn state(mut self, value: CreditBalanceState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn top_up_max_usd(mut self, value: impl Into<String>) -> Self {
        self.top_up_max_usd = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreditBalance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](CreditBalanceBuilder::state)
    pub fn build(self) -> Result<CreditBalance, BuildError> {
        Ok(CreditBalance {
            available_usd: self.available_usd,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            top_up_max_usd: self.top_up_max_usd,
        })
    }
}
