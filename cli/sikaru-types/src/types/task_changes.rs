pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskChanges {
    #[serde(default)]
    pub changes: Vec<TaskChange>,
    #[serde(default)]
    pub revision: i64,
    #[serde(default)]
    pub task_id: String,
}

impl TaskChanges {
    pub fn builder() -> TaskChangesBuilder {
        <TaskChangesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskChangesBuilder {
    changes: Option<Vec<TaskChange>>,
    revision: Option<i64>,
    task_id: Option<String>,
}

impl TaskChangesBuilder {
    pub fn changes(mut self, value: Vec<TaskChange>) -> Self {
        self.changes = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskChanges`].
    /// This method will fail if any of the following fields are not set:
    /// - [`changes`](TaskChangesBuilder::changes)
    /// - [`revision`](TaskChangesBuilder::revision)
    /// - [`task_id`](TaskChangesBuilder::task_id)
    pub fn build(self) -> Result<TaskChanges, BuildError> {
        Ok(TaskChanges {
            changes: self.changes.ok_or_else(|| BuildError::missing_field("changes"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            task_id: self.task_id.ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
