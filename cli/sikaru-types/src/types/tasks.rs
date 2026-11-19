pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Tasks {
    #[serde(default)]
    pub items: Vec<Task>,
}

impl Tasks {
    pub fn builder() -> TasksBuilder {
        <TasksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TasksBuilder {
    items: Option<Vec<Task>>,
}

impl TasksBuilder {
    pub fn items(mut self, value: Vec<Task>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Tasks`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](TasksBuilder::items)
    pub fn build(self) -> Result<Tasks, BuildError> {
        Ok(Tasks {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
