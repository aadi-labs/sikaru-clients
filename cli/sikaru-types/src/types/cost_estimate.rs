pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostEstimate {
    #[serde(rename = "conversationsPerDay")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub conversations_per_day: f64,
    pub judge: CostEstimateJudge,
    #[serde(rename = "perConversationUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub per_conversation_usd: Option<f64>,
    #[serde(rename = "perDayUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub per_day_usd: Option<f64>,
    /// The one-time refit that calibrates the new judge.
    #[serde(rename = "refitUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub refit_usd: Option<f64>,
}

impl CostEstimate {
    pub fn builder() -> CostEstimateBuilder {
        <CostEstimateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostEstimateBuilder {
    conversations_per_day: Option<f64>,
    judge: Option<CostEstimateJudge>,
    per_conversation_usd: Option<f64>,
    per_day_usd: Option<f64>,
    refit_usd: Option<f64>,
}

impl CostEstimateBuilder {
    pub fn conversations_per_day(mut self, value: f64) -> Self {
        self.conversations_per_day = Some(value);
        self
    }

    pub fn judge(mut self, value: CostEstimateJudge) -> Self {
        self.judge = Some(value);
        self
    }

    pub fn per_conversation_usd(mut self, value: f64) -> Self {
        self.per_conversation_usd = Some(value);
        self
    }

    pub fn per_day_usd(mut self, value: f64) -> Self {
        self.per_day_usd = Some(value);
        self
    }

    pub fn refit_usd(mut self, value: f64) -> Self {
        self.refit_usd = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CostEstimate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversations_per_day`](CostEstimateBuilder::conversations_per_day)
    /// - [`judge`](CostEstimateBuilder::judge)
    pub fn build(self) -> Result<CostEstimate, BuildError> {
        Ok(CostEstimate {
            conversations_per_day: self.conversations_per_day.ok_or_else(|| BuildError::missing_field("conversations_per_day"))?,
            judge: self.judge.ok_or_else(|| BuildError::missing_field("judge"))?,
            per_conversation_usd: self.per_conversation_usd,
            per_day_usd: self.per_day_usd,
            refit_usd: self.refit_usd,
        })
    }
}
