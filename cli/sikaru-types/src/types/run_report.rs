pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RunReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusive: Option<i64>,
    #[serde(default)]
    pub conversations: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "estimatedUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flagged: Option<i64>,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    pub status: RunReportStatus,
}

impl RunReport {
    pub fn builder() -> RunReportBuilder {
        <RunReportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunReportBuilder {
    conclusive: Option<i64>,
    conversations: Option<i64>,
    error: Option<String>,
    estimated_usd: Option<String>,
    flagged: Option<i64>,
    run_id: Option<String>,
    status: Option<RunReportStatus>,
}

impl RunReportBuilder {
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

    pub fn estimated_usd(mut self, value: impl Into<String>) -> Self {
        self.estimated_usd = Some(value.into());
        self
    }

    pub fn flagged(mut self, value: i64) -> Self {
        self.flagged = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: RunReportStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunReport`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversations`](RunReportBuilder::conversations)
    /// - [`run_id`](RunReportBuilder::run_id)
    /// - [`status`](RunReportBuilder::status)
    pub fn build(self) -> Result<RunReport, BuildError> {
        Ok(RunReport {
            conclusive: self.conclusive,
            conversations: self.conversations.ok_or_else(|| BuildError::missing_field("conversations"))?,
            error: self.error,
            estimated_usd: self.estimated_usd,
            flagged: self.flagged,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
