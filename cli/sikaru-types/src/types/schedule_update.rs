pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScheduleUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(default)]
    pub expected_revision: i64,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl ScheduleUpdate {
    pub fn builder() -> ScheduleUpdateBuilder {
        <ScheduleUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleUpdateBuilder {
    at: Option<String>,
    cron: Option<String>,
    expected_revision: Option<i64>,
    idempotency_key: Option<String>,
    input: Option<HashMap<String, serde_json::Value>>,
    interval_seconds: Option<i64>,
    timezone: Option<String>,
}

impl ScheduleUpdateBuilder {
    pub fn at(mut self, value: impl Into<String>) -> Self {
        self.at = Some(value.into());
        self
    }

    pub fn cron(mut self, value: impl Into<String>) -> Self {
        self.cron = Some(value.into());
        self
    }

    pub fn expected_revision(mut self, value: i64) -> Self {
        self.expected_revision = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn input(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn interval_seconds(mut self, value: i64) -> Self {
        self.interval_seconds = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScheduleUpdate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expected_revision`](ScheduleUpdateBuilder::expected_revision)
    /// - [`idempotency_key`](ScheduleUpdateBuilder::idempotency_key)
    pub fn build(self) -> Result<ScheduleUpdate, BuildError> {
        Ok(ScheduleUpdate {
            at: self.at,
            cron: self.cron,
            expected_revision: self.expected_revision.ok_or_else(|| BuildError::missing_field("expected_revision"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            input: self.input,
            interval_seconds: self.interval_seconds,
            timezone: self.timezone,
        })
    }
}

