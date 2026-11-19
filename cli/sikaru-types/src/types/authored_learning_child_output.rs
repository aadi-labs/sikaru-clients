pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthoredLearningChildOutput {
    #[serde(rename = "completedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub completed_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "productOutputs")]
    #[serde(default)]
    pub product_outputs: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub workflow: String,
}

impl AuthoredLearningChildOutput {
    pub fn builder() -> AuthoredLearningChildOutputBuilder {
        <AuthoredLearningChildOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthoredLearningChildOutputBuilder {
    completed_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    product_outputs: Option<HashMap<String, serde_json::Value>>,
    role: Option<String>,
    workflow: Option<String>,
}

impl AuthoredLearningChildOutputBuilder {
    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn product_outputs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.product_outputs = Some(value);
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn workflow(mut self, value: impl Into<String>) -> Self {
        self.workflow = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthoredLearningChildOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completed_at`](AuthoredLearningChildOutputBuilder::completed_at)
    /// - [`id`](AuthoredLearningChildOutputBuilder::id)
    /// - [`product_outputs`](AuthoredLearningChildOutputBuilder::product_outputs)
    /// - [`role`](AuthoredLearningChildOutputBuilder::role)
    /// - [`workflow`](AuthoredLearningChildOutputBuilder::workflow)
    pub fn build(self) -> Result<AuthoredLearningChildOutput, BuildError> {
        Ok(AuthoredLearningChildOutput {
            completed_at: self.completed_at.ok_or_else(|| BuildError::missing_field("completed_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            product_outputs: self.product_outputs.ok_or_else(|| BuildError::missing_field("product_outputs"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            workflow: self.workflow.ok_or_else(|| BuildError::missing_field("workflow"))?,
        })
    }
}
