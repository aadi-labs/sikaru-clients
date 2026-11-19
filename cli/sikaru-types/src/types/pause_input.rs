pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PauseInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub paused: bool,
}

impl PauseInput {
    pub fn builder() -> PauseInputBuilder {
        <PauseInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PauseInputBuilder {
    expected_revision: Option<i64>,
    idempotency_key: Option<String>,
    paused: Option<bool>,
}

impl PauseInputBuilder {
    pub fn expected_revision(mut self, value: i64) -> Self {
        self.expected_revision = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn paused(mut self, value: bool) -> Self {
        self.paused = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PauseInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paused`](PauseInputBuilder::paused)
    pub fn build(self) -> Result<PauseInput, BuildError> {
        Ok(PauseInput {
            expected_revision: self.expected_revision,
            idempotency_key: self.idempotency_key,
            paused: self.paused.ok_or_else(|| BuildError::missing_field("paused"))?,
        })
    }
}

