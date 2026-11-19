pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScheduleRecord {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<f64>,
    #[serde(rename = "callbackUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(rename = "declaredPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exhausted: Option<bool>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "intervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_seconds: Option<i64>,
    #[serde(rename = "nextAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "sessionMode")]
    pub session_mode: ScheduleRecordSessionMode,
    #[serde(default)]
    pub timezone: String,
    #[serde(rename = "toolProviderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_provider_id: Option<String>,
}

impl ScheduleRecord {
    pub fn builder() -> ScheduleRecordBuilder {
        <ScheduleRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleRecordBuilder {
    agent_id: Option<String>,
    at: Option<f64>,
    callback_url: Option<String>,
    creator: Option<String>,
    cron: Option<String>,
    declared_path: Option<String>,
    deleted: Option<bool>,
    destination: Option<String>,
    environment: Option<String>,
    exhausted: Option<bool>,
    id: Option<String>,
    input: Option<HashMap<String, serde_json::Value>>,
    interval_seconds: Option<i64>,
    next_at: Option<f64>,
    paused: Option<bool>,
    revision: Option<i64>,
    session_id: Option<String>,
    session_mode: Option<ScheduleRecordSessionMode>,
    timezone: Option<String>,
    tool_provider_id: Option<String>,
}

impl ScheduleRecordBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn at(mut self, value: f64) -> Self {
        self.at = Some(value);
        self
    }

    pub fn callback_url(mut self, value: impl Into<String>) -> Self {
        self.callback_url = Some(value.into());
        self
    }

    pub fn creator(mut self, value: impl Into<String>) -> Self {
        self.creator = Some(value.into());
        self
    }

    pub fn cron(mut self, value: impl Into<String>) -> Self {
        self.cron = Some(value.into());
        self
    }

    pub fn declared_path(mut self, value: impl Into<String>) -> Self {
        self.declared_path = Some(value.into());
        self
    }

    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn destination(mut self, value: impl Into<String>) -> Self {
        self.destination = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn exhausted(mut self, value: bool) -> Self {
        self.exhausted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn next_at(mut self, value: f64) -> Self {
        self.next_at = Some(value);
        self
    }

    pub fn paused(mut self, value: bool) -> Self {
        self.paused = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn session_mode(mut self, value: ScheduleRecordSessionMode) -> Self {
        self.session_mode = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn tool_provider_id(mut self, value: impl Into<String>) -> Self {
        self.tool_provider_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScheduleRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ScheduleRecordBuilder::id)
    /// - [`session_mode`](ScheduleRecordBuilder::session_mode)
    /// - [`timezone`](ScheduleRecordBuilder::timezone)
    pub fn build(self) -> Result<ScheduleRecord, BuildError> {
        Ok(ScheduleRecord {
            agent_id: self.agent_id,
            at: self.at,
            callback_url: self.callback_url,
            creator: self.creator,
            cron: self.cron,
            declared_path: self.declared_path,
            deleted: self.deleted,
            destination: self.destination,
            environment: self.environment,
            exhausted: self.exhausted,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            input: self.input,
            interval_seconds: self.interval_seconds,
            next_at: self.next_at,
            paused: self.paused,
            revision: self.revision,
            session_id: self.session_id,
            session_mode: self.session_mode.ok_or_else(|| BuildError::missing_field("session_mode"))?,
            timezone: self.timezone.ok_or_else(|| BuildError::missing_field("timezone"))?,
            tool_provider_id: self.tool_provider_id,
        })
    }
}
