pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OutputPreviewToolCallsItemFunction {
    #[serde(default)]
    pub arguments: String,
    #[serde(default)]
    pub name: String,
}

impl OutputPreviewToolCallsItemFunction {
    pub fn builder() -> OutputPreviewToolCallsItemFunctionBuilder {
        <OutputPreviewToolCallsItemFunctionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutputPreviewToolCallsItemFunctionBuilder {
    arguments: Option<String>,
    name: Option<String>,
}

impl OutputPreviewToolCallsItemFunctionBuilder {
    pub fn arguments(mut self, value: impl Into<String>) -> Self {
        self.arguments = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OutputPreviewToolCallsItemFunction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`arguments`](OutputPreviewToolCallsItemFunctionBuilder::arguments)
    /// - [`name`](OutputPreviewToolCallsItemFunctionBuilder::name)
    pub fn build(self) -> Result<OutputPreviewToolCallsItemFunction, BuildError> {
        Ok(OutputPreviewToolCallsItemFunction {
            arguments: self.arguments.ok_or_else(|| BuildError::missing_field("arguments"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
