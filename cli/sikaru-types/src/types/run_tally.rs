pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunTally {
    #[serde(rename = "costUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_usd: Option<f64>,
    #[serde(rename = "modelCalls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_calls: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taps: Option<i64>,
}

impl RunTally {
    pub fn builder() -> RunTallyBuilder {
        <RunTallyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunTallyBuilder {
    cost_usd: Option<f64>,
    model_calls: Option<i64>,
    steps: Option<i64>,
    taps: Option<i64>,
}

impl RunTallyBuilder {
    pub fn cost_usd(mut self, value: f64) -> Self {
        self.cost_usd = Some(value);
        self
    }

    pub fn model_calls(mut self, value: i64) -> Self {
        self.model_calls = Some(value);
        self
    }

    pub fn steps(mut self, value: i64) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn taps(mut self, value: i64) -> Self {
        self.taps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunTally`].
    pub fn build(self) -> Result<RunTally, BuildError> {
        Ok(RunTally {
            cost_usd: self.cost_usd,
            model_calls: self.model_calls,
            steps: self.steps,
            taps: self.taps,
        })
    }
}
