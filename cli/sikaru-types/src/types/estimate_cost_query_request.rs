pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for estimate_cost
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EstimateCostQueryRequest {
    pub judge: EstimateCostGradersRequestJudge,
    #[serde(rename = "judgeAgentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub judge_agent_id: Option<String>,
}

impl EstimateCostQueryRequest {
    pub fn builder() -> EstimateCostQueryRequestBuilder {
        <EstimateCostQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EstimateCostQueryRequestBuilder {
    judge: Option<EstimateCostGradersRequestJudge>,
    judge_agent_id: Option<String>,
}

impl EstimateCostQueryRequestBuilder {
    pub fn judge(mut self, value: EstimateCostGradersRequestJudge) -> Self {
        self.judge = Some(value);
        self
    }

    pub fn judge_agent_id(mut self, value: impl Into<String>) -> Self {
        self.judge_agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EstimateCostQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`judge`](EstimateCostQueryRequestBuilder::judge)
    pub fn build(self) -> Result<EstimateCostQueryRequest, BuildError> {
        Ok(EstimateCostQueryRequest {
            judge: self.judge.ok_or_else(|| BuildError::missing_field("judge"))?,
            judge_agent_id: self.judge_agent_id,
        })
    }
}

