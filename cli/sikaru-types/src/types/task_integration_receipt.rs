pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskIntegrationReceipt {
    #[serde(default)]
    pub conflicts: Vec<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub revision: i64,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub task_id: String,
}

impl TaskIntegrationReceipt {
    pub fn builder() -> TaskIntegrationReceiptBuilder {
        <TaskIntegrationReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskIntegrationReceiptBuilder {
    conflicts: Option<Vec<String>>,
    id: Option<String>,
    revision: Option<i64>,
    status: Option<String>,
    task_id: Option<String>,
}

impl TaskIntegrationReceiptBuilder {
    pub fn conflicts(mut self, value: Vec<String>) -> Self {
        self.conflicts = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskIntegrationReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conflicts`](TaskIntegrationReceiptBuilder::conflicts)
    /// - [`id`](TaskIntegrationReceiptBuilder::id)
    /// - [`revision`](TaskIntegrationReceiptBuilder::revision)
    /// - [`status`](TaskIntegrationReceiptBuilder::status)
    /// - [`task_id`](TaskIntegrationReceiptBuilder::task_id)
    pub fn build(self) -> Result<TaskIntegrationReceipt, BuildError> {
        Ok(TaskIntegrationReceipt {
            conflicts: self.conflicts.ok_or_else(|| BuildError::missing_field("conflicts"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            task_id: self.task_id.ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
