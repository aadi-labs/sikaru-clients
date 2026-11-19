pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskCancel {
    #[serde(default)]
    pub idempotency_key: String,
}

impl TaskCancel {
    pub fn builder() -> TaskCancelBuilder {
        <TaskCancelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskCancelBuilder {
    idempotency_key: Option<String>,
}

impl TaskCancelBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskCancel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](TaskCancelBuilder::idempotency_key)
    pub fn build(self) -> Result<TaskCancel, BuildError> {
        Ok(TaskCancel {
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
        })
    }
}
