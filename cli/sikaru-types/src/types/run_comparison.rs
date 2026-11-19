pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunComparison {
    #[serde(rename = "learnedRun")]
    #[serde(default)]
    pub learned_run: RunTally,
    #[serde(rename = "thisRun")]
    #[serde(default)]
    pub this_run: RunTally,
}

impl RunComparison {
    pub fn builder() -> RunComparisonBuilder {
        <RunComparisonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunComparisonBuilder {
    learned_run: Option<RunTally>,
    this_run: Option<RunTally>,
}

impl RunComparisonBuilder {
    pub fn learned_run(mut self, value: RunTally) -> Self {
        self.learned_run = Some(value);
        self
    }

    pub fn this_run(mut self, value: RunTally) -> Self {
        self.this_run = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunComparison`].
    /// This method will fail if any of the following fields are not set:
    /// - [`learned_run`](RunComparisonBuilder::learned_run)
    /// - [`this_run`](RunComparisonBuilder::this_run)
    pub fn build(self) -> Result<RunComparison, BuildError> {
        Ok(RunComparison {
            learned_run: self.learned_run.ok_or_else(|| BuildError::missing_field("learned_run"))?,
            this_run: self.this_run.ok_or_else(|| BuildError::missing_field("this_run"))?,
        })
    }
}
