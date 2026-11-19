pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskReceipt {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub task_id: String,
}

impl TaskReceipt {
    pub fn builder() -> TaskReceiptBuilder {
        <TaskReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskReceiptBuilder {
    id: Option<String>,
    reason: Option<String>,
    status: Option<String>,
    task_id: Option<String>,
}

impl TaskReceiptBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
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

    /// Consumes the builder and constructs a [`TaskReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaskReceiptBuilder::id)
    /// - [`status`](TaskReceiptBuilder::status)
    /// - [`task_id`](TaskReceiptBuilder::task_id)
    pub fn build(self) -> Result<TaskReceipt, BuildError> {
        Ok(TaskReceipt {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self.reason,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            task_id: self.task_id.ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
