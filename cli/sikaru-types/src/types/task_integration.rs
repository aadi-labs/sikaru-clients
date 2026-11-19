pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskIntegration {
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    #[serde(default)]
    pub revision: i64,
}

impl TaskIntegration {
    pub fn builder() -> TaskIntegrationBuilder {
        <TaskIntegrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskIntegrationBuilder {
    idempotency_key: Option<String>,
    paths: Option<Vec<String>>,
    revision: Option<i64>,
}

impl TaskIntegrationBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn paths(mut self, value: Vec<String>) -> Self {
        self.paths = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskIntegration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](TaskIntegrationBuilder::idempotency_key)
    /// - [`revision`](TaskIntegrationBuilder::revision)
    pub fn build(self) -> Result<TaskIntegration, BuildError> {
        Ok(TaskIntegration {
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            paths: self.paths,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}

