pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreditTopUp {
    #[serde(rename = "amountUsd")]
    #[serde(default)]
    pub amount_usd: String,
    /// Unix seconds; the pay URL refuses payment after it.
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: i64,
    #[serde(default)]
    pub id: String,
    /// Pay with an agent wallet through HTTP 402 machine payment.
    #[serde(rename = "payUrl")]
    #[serde(default)]
    pub pay_url: String,
    pub status: CreditTopUpStatus,
}

impl CreditTopUp {
    pub fn builder() -> CreditTopUpBuilder {
        <CreditTopUpBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditTopUpBuilder {
    amount_usd: Option<String>,
    expires_at: Option<i64>,
    id: Option<String>,
    pay_url: Option<String>,
    status: Option<CreditTopUpStatus>,
}

impl CreditTopUpBuilder {
    pub fn amount_usd(mut self, value: impl Into<String>) -> Self {
        self.amount_usd = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: i64) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn pay_url(mut self, value: impl Into<String>) -> Self {
        self.pay_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreditTopUpStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreditTopUp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_usd`](CreditTopUpBuilder::amount_usd)
    /// - [`expires_at`](CreditTopUpBuilder::expires_at)
    /// - [`id`](CreditTopUpBuilder::id)
    /// - [`pay_url`](CreditTopUpBuilder::pay_url)
    /// - [`status`](CreditTopUpBuilder::status)
    pub fn build(self) -> Result<CreditTopUp, BuildError> {
        Ok(CreditTopUp {
            amount_usd: self.amount_usd.ok_or_else(|| BuildError::missing_field("amount_usd"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            pay_url: self.pay_url.ok_or_else(|| BuildError::missing_field("pay_url"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
