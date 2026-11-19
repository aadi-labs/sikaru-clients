pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<TaskMessageDeliveryMode>,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(default)]
    pub input: HashMap<String, serde_json::Value>,
}

impl TaskMessage {
    pub fn builder() -> TaskMessageBuilder {
        <TaskMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskMessageBuilder {
    delivery_mode: Option<TaskMessageDeliveryMode>,
    idempotency_key: Option<String>,
    input: Option<HashMap<String, serde_json::Value>>,
}

impl TaskMessageBuilder {
    pub fn delivery_mode(mut self, value: TaskMessageDeliveryMode) -> Self {
        self.delivery_mode = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn input(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](TaskMessageBuilder::idempotency_key)
    /// - [`input`](TaskMessageBuilder::input)
    pub fn build(self) -> Result<TaskMessage, BuildError> {
        Ok(TaskMessage {
            delivery_mode: self.delivery_mode,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
        })
    }
}

