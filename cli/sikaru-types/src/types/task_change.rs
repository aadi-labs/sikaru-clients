pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskChange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub path: String,
}

impl TaskChange {
    pub fn builder() -> TaskChangeBuilder {
        <TaskChangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskChangeBuilder {
    after: Option<HashMap<String, serde_json::Value>>,
    before: Option<HashMap<String, serde_json::Value>>,
    path: Option<String>,
}

impl TaskChangeBuilder {
    pub fn after(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.after = Some(value);
        self
    }

    pub fn before(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.before = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskChange`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](TaskChangeBuilder::path)
    pub fn build(self) -> Result<TaskChange, BuildError> {
        Ok(TaskChange {
            after: self.after,
            before: self.before,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
