pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SkillSourceReceipt {
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "operationId")]
    #[serde(default)]
    pub operation_id: String,
    #[serde(rename = "sourceRevision")]
    #[serde(default)]
    pub source_revision: String,
}

impl SkillSourceReceipt {
    pub fn builder() -> SkillSourceReceiptBuilder {
        <SkillSourceReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillSourceReceiptBuilder {
    enabled: Option<bool>,
    operation_id: Option<String>,
    source_revision: Option<String>,
}

impl SkillSourceReceiptBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn operation_id(mut self, value: impl Into<String>) -> Self {
        self.operation_id = Some(value.into());
        self
    }

    pub fn source_revision(mut self, value: impl Into<String>) -> Self {
        self.source_revision = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SkillSourceReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](SkillSourceReceiptBuilder::enabled)
    /// - [`operation_id`](SkillSourceReceiptBuilder::operation_id)
    /// - [`source_revision`](SkillSourceReceiptBuilder::source_revision)
    pub fn build(self) -> Result<SkillSourceReceipt, BuildError> {
        Ok(SkillSourceReceipt {
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            operation_id: self.operation_id.ok_or_else(|| BuildError::missing_field("operation_id"))?,
            source_revision: self.source_revision.ok_or_else(|| BuildError::missing_field("source_revision"))?,
        })
    }
}
