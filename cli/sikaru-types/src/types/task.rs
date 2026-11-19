pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Task {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(rename = "deliveryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<String>,
    #[serde(rename = "finalText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_text: Option<String>,
    #[serde(rename = "latestSteer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_steer: Option<TaskReceipt>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<String>,
    #[serde(rename = "pauseReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement: Option<TaskSettlement>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub task_id: String,
}

impl Task {
    pub fn builder() -> TaskBuilder {
        <TaskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskBuilder {
    background: Option<bool>,
    delivery_status: Option<String>,
    final_text: Option<String>,
    latest_steer: Option<TaskReceipt>,
    name: Option<String>,
    parent_task_id: Option<String>,
    pause_reason: Option<String>,
    revision: Option<i64>,
    run_id: Option<String>,
    settlement: Option<TaskSettlement>,
    status: Option<String>,
    task_id: Option<String>,
}

impl TaskBuilder {
    pub fn background(mut self, value: bool) -> Self {
        self.background = Some(value);
        self
    }

    pub fn delivery_status(mut self, value: impl Into<String>) -> Self {
        self.delivery_status = Some(value.into());
        self
    }

    pub fn final_text(mut self, value: impl Into<String>) -> Self {
        self.final_text = Some(value.into());
        self
    }

    pub fn latest_steer(mut self, value: TaskReceipt) -> Self {
        self.latest_steer = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_task_id(mut self, value: impl Into<String>) -> Self {
        self.parent_task_id = Some(value.into());
        self
    }

    pub fn pause_reason(mut self, value: impl Into<String>) -> Self {
        self.pause_reason = Some(value.into());
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn settlement(mut self, value: TaskSettlement) -> Self {
        self.settlement = Some(value);
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

    /// Consumes the builder and constructs a [`Task`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TaskBuilder::name)
    /// - [`run_id`](TaskBuilder::run_id)
    /// - [`status`](TaskBuilder::status)
    /// - [`task_id`](TaskBuilder::task_id)
    pub fn build(self) -> Result<Task, BuildError> {
        Ok(Task {
            background: self.background,
            delivery_status: self.delivery_status,
            final_text: self.final_text,
            latest_steer: self.latest_steer,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_task_id: self.parent_task_id,
            pause_reason: self.pause_reason,
            revision: self.revision,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            settlement: self.settlement,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            task_id: self.task_id.ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
