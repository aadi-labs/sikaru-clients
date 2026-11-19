pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskIntegrationReceipts {
    #[serde(default)]
    pub items: Vec<TaskIntegrationReceipt>,
}

impl TaskIntegrationReceipts {
    pub fn builder() -> TaskIntegrationReceiptsBuilder {
        <TaskIntegrationReceiptsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskIntegrationReceiptsBuilder {
    items: Option<Vec<TaskIntegrationReceipt>>,
}

impl TaskIntegrationReceiptsBuilder {
    pub fn items(mut self, value: Vec<TaskIntegrationReceipt>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskIntegrationReceipts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](TaskIntegrationReceiptsBuilder::items)
    pub fn build(self) -> Result<TaskIntegrationReceipts, BuildError> {
        Ok(TaskIntegrationReceipts {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
