pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OutputPreviewToolCallsItem {
    #[serde(default)]
    pub function: OutputPreviewToolCallsItemFunction,
    #[serde(default)]
    pub id: String,
}

impl OutputPreviewToolCallsItem {
    pub fn builder() -> OutputPreviewToolCallsItemBuilder {
        <OutputPreviewToolCallsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutputPreviewToolCallsItemBuilder {
    function: Option<OutputPreviewToolCallsItemFunction>,
    id: Option<String>,
}

impl OutputPreviewToolCallsItemBuilder {
    pub fn function(mut self, value: OutputPreviewToolCallsItemFunction) -> Self {
        self.function = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OutputPreviewToolCallsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`function`](OutputPreviewToolCallsItemBuilder::function)
    /// - [`id`](OutputPreviewToolCallsItemBuilder::id)
    pub fn build(self) -> Result<OutputPreviewToolCallsItem, BuildError> {
        Ok(OutputPreviewToolCallsItem {
            function: self.function.ok_or_else(|| BuildError::missing_field("function"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
