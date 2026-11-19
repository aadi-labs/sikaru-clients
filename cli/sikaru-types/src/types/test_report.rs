pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ceiling: Option<f64>,
    #[serde(default)]
    pub conclusive: i64,
    #[serde(default)]
    pub conversations: i64,
    /// Why a failed test could not finish.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "flagRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub flag_rate: Option<f64>,
    #[serde(default)]
    pub flagged: i64,
    /// Flagged first.
    #[serde(default)]
    pub results: Vec<Tested>,
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// A pending test is still scoring; poll runId. A failed test is final: test again to start a new run.
    pub status: TestReportStatus,
    /// The flag rate is above the ceiling.
    #[serde(rename = "wouldBlock")]
    #[serde(default)]
    pub would_block: bool,
}

impl TestReport {
    pub fn builder() -> TestReportBuilder {
        <TestReportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestReportBuilder {
    ceiling: Option<f64>,
    conclusive: Option<i64>,
    conversations: Option<i64>,
    error: Option<String>,
    flag_rate: Option<f64>,
    flagged: Option<i64>,
    results: Option<Vec<Tested>>,
    run_id: Option<String>,
    status: Option<TestReportStatus>,
    would_block: Option<bool>,
}

impl TestReportBuilder {
    pub fn ceiling(mut self, value: f64) -> Self {
        self.ceiling = Some(value);
        self
    }

    pub fn conclusive(mut self, value: i64) -> Self {
        self.conclusive = Some(value);
        self
    }

    pub fn conversations(mut self, value: i64) -> Self {
        self.conversations = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn flag_rate(mut self, value: f64) -> Self {
        self.flag_rate = Some(value);
        self
    }

    pub fn flagged(mut self, value: i64) -> Self {
        self.flagged = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<Tested>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: TestReportStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn would_block(mut self, value: bool) -> Self {
        self.would_block = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TestReport`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conclusive`](TestReportBuilder::conclusive)
    /// - [`conversations`](TestReportBuilder::conversations)
    /// - [`flagged`](TestReportBuilder::flagged)
    /// - [`results`](TestReportBuilder::results)
    /// - [`status`](TestReportBuilder::status)
    /// - [`would_block`](TestReportBuilder::would_block)
    pub fn build(self) -> Result<TestReport, BuildError> {
        Ok(TestReport {
            ceiling: self.ceiling,
            conclusive: self.conclusive.ok_or_else(|| BuildError::missing_field("conclusive"))?,
            conversations: self.conversations.ok_or_else(|| BuildError::missing_field("conversations"))?,
            error: self.error,
            flag_rate: self.flag_rate,
            flagged: self.flagged.ok_or_else(|| BuildError::missing_field("flagged"))?,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
            run_id: self.run_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            would_block: self.would_block.ok_or_else(|| BuildError::missing_field("would_block"))?,
        })
    }
}
