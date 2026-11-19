pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthoredLearningWorkflow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "productOutputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_output_schema: Option<HashMap<String, serde_json::Value>>,
}

impl AuthoredLearningWorkflow {
    pub fn builder() -> AuthoredLearningWorkflowBuilder {
        <AuthoredLearningWorkflowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthoredLearningWorkflowBuilder {
    instructions: Option<String>,
    name: Option<String>,
    product_output_schema: Option<HashMap<String, serde_json::Value>>,
}

impl AuthoredLearningWorkflowBuilder {
    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn product_output_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.product_output_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthoredLearningWorkflow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AuthoredLearningWorkflowBuilder::name)
    pub fn build(self) -> Result<AuthoredLearningWorkflow, BuildError> {
        Ok(AuthoredLearningWorkflow {
            instructions: self.instructions,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            product_output_schema: self.product_output_schema,
        })
    }
}
