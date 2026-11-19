pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthoredLearningReceipt {
    #[serde(rename = "admissionStatus")]
    #[serde(default)]
    pub admission_status: String,
    #[serde(rename = "childOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_outputs: Option<Vec<AuthoredLearningChildOutput>>,
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "invocationId")]
    #[serde(default)]
    pub invocation_id: String,
    #[serde(rename = "pendingDescendants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_descendants: Option<i64>,
    #[serde(rename = "pendingTools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_tools: Option<Vec<LearningPendingTool>>,
    #[serde(rename = "productOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_outputs: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub status: String,
}

impl AuthoredLearningReceipt {
    pub fn builder() -> AuthoredLearningReceiptBuilder {
        <AuthoredLearningReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthoredLearningReceiptBuilder {
    admission_status: Option<String>,
    child_outputs: Option<Vec<AuthoredLearningChildOutput>>,
    completed_at: Option<DateTime<FixedOffset>>,
    invocation_id: Option<String>,
    pending_descendants: Option<i64>,
    pending_tools: Option<Vec<LearningPendingTool>>,
    product_outputs: Option<HashMap<String, serde_json::Value>>,
    run_id: Option<String>,
    status: Option<String>,
}

impl AuthoredLearningReceiptBuilder {
    pub fn admission_status(mut self, value: impl Into<String>) -> Self {
        self.admission_status = Some(value.into());
        self
    }

    pub fn child_outputs(mut self, value: Vec<AuthoredLearningChildOutput>) -> Self {
        self.child_outputs = Some(value);
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn invocation_id(mut self, value: impl Into<String>) -> Self {
        self.invocation_id = Some(value.into());
        self
    }

    pub fn pending_descendants(mut self, value: i64) -> Self {
        self.pending_descendants = Some(value);
        self
    }

    pub fn pending_tools(mut self, value: Vec<LearningPendingTool>) -> Self {
        self.pending_tools = Some(value);
        self
    }

    pub fn product_outputs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.product_outputs = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthoredLearningReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`admission_status`](AuthoredLearningReceiptBuilder::admission_status)
    /// - [`invocation_id`](AuthoredLearningReceiptBuilder::invocation_id)
    /// - [`run_id`](AuthoredLearningReceiptBuilder::run_id)
    /// - [`status`](AuthoredLearningReceiptBuilder::status)
    pub fn build(self) -> Result<AuthoredLearningReceipt, BuildError> {
        Ok(AuthoredLearningReceipt {
            admission_status: self.admission_status.ok_or_else(|| BuildError::missing_field("admission_status"))?,
            child_outputs: self.child_outputs,
            completed_at: self.completed_at,
            invocation_id: self.invocation_id.ok_or_else(|| BuildError::missing_field("invocation_id"))?,
            pending_descendants: self.pending_descendants,
            pending_tools: self.pending_tools,
            product_outputs: self.product_outputs,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
