pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskApprovalReceipt {
    #[serde(default)]
    pub decision: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
}

impl TaskApprovalReceipt {
    pub fn builder() -> TaskApprovalReceiptBuilder {
        <TaskApprovalReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskApprovalReceiptBuilder {
    decision: Option<String>,
    id: Option<String>,
    status: Option<String>,
}

impl TaskApprovalReceiptBuilder {
    pub fn decision(mut self, value: impl Into<String>) -> Self {
        self.decision = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskApprovalReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`decision`](TaskApprovalReceiptBuilder::decision)
    /// - [`id`](TaskApprovalReceiptBuilder::id)
    /// - [`status`](TaskApprovalReceiptBuilder::status)
    pub fn build(self) -> Result<TaskApprovalReceipt, BuildError> {
        Ok(TaskApprovalReceipt {
            decision: self.decision.ok_or_else(|| BuildError::missing_field("decision"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
