pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskSettlement {
    #[serde(rename = "modelCostMicroUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_cost_micro_usd: Option<i64>,
    #[serde(rename = "modelTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_tokens: Option<TaskModelTokens>,
    pub status: TaskSettlementStatus,
}

impl TaskSettlement {
    pub fn builder() -> TaskSettlementBuilder {
        <TaskSettlementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSettlementBuilder {
    model_cost_micro_usd: Option<i64>,
    model_tokens: Option<TaskModelTokens>,
    status: Option<TaskSettlementStatus>,
}

impl TaskSettlementBuilder {
    pub fn model_cost_micro_usd(mut self, value: i64) -> Self {
        self.model_cost_micro_usd = Some(value);
        self
    }

    pub fn model_tokens(mut self, value: TaskModelTokens) -> Self {
        self.model_tokens = Some(value);
        self
    }

    pub fn status(mut self, value: TaskSettlementStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskSettlement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](TaskSettlementBuilder::status)
    pub fn build(self) -> Result<TaskSettlement, BuildError> {
        Ok(TaskSettlement {
            model_cost_micro_usd: self.model_cost_micro_usd,
            model_tokens: self.model_tokens,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
