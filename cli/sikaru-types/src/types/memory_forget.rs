pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryForget {
    #[serde(default)]
    pub attempts: i64,
    /// Conversations dropped from past-session search.
    #[serde(rename = "conversationsHidden")]
    #[serde(default)]
    pub conversations_hidden: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(rename = "finishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub finished_at: Option<f64>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    /// Passages removed from memory.
    #[serde(default)]
    pub removed: i64,
    pub status: MemoryForgetStatus,
}

impl MemoryForget {
    pub fn builder() -> MemoryForgetBuilder {
        <MemoryForgetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemoryForgetBuilder {
    attempts: Option<i64>,
    conversations_hidden: Option<i64>,
    created_at: Option<f64>,
    finished_at: Option<f64>,
    job_id: Option<String>,
    removed: Option<i64>,
    status: Option<MemoryForgetStatus>,
}

impl MemoryForgetBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn conversations_hidden(mut self, value: i64) -> Self {
        self.conversations_hidden = Some(value);
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn finished_at(mut self, value: f64) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn job_id(mut self, value: impl Into<String>) -> Self {
        self.job_id = Some(value.into());
        self
    }

    pub fn removed(mut self, value: i64) -> Self {
        self.removed = Some(value);
        self
    }

    pub fn status(mut self, value: MemoryForgetStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemoryForget`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](MemoryForgetBuilder::attempts)
    /// - [`conversations_hidden`](MemoryForgetBuilder::conversations_hidden)
    /// - [`created_at`](MemoryForgetBuilder::created_at)
    /// - [`job_id`](MemoryForgetBuilder::job_id)
    /// - [`removed`](MemoryForgetBuilder::removed)
    /// - [`status`](MemoryForgetBuilder::status)
    pub fn build(self) -> Result<MemoryForget, BuildError> {
        Ok(MemoryForget {
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            conversations_hidden: self.conversations_hidden.ok_or_else(|| BuildError::missing_field("conversations_hidden"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            finished_at: self.finished_at,
            job_id: self.job_id.ok_or_else(|| BuildError::missing_field("job_id"))?,
            removed: self.removed.ok_or_else(|| BuildError::missing_field("removed"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
