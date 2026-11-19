pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DiscardDocument {
    #[serde(rename = "baseRevision")]
    #[serde(default)]
    pub base_revision: i64,
    #[serde(rename = "expectedLiveVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_live_version_id: Option<String>,
}

impl DiscardDocument {
    pub fn builder() -> DiscardDocumentBuilder {
        <DiscardDocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DiscardDocumentBuilder {
    base_revision: Option<i64>,
    expected_live_version_id: Option<String>,
}

impl DiscardDocumentBuilder {
    pub fn base_revision(mut self, value: i64) -> Self {
        self.base_revision = Some(value);
        self
    }

    pub fn expected_live_version_id(mut self, value: impl Into<String>) -> Self {
        self.expected_live_version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DiscardDocument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`base_revision`](DiscardDocumentBuilder::base_revision)
    pub fn build(self) -> Result<DiscardDocument, BuildError> {
        Ok(DiscardDocument {
            base_revision: self.base_revision.ok_or_else(|| BuildError::missing_field("base_revision"))?,
            expected_live_version_id: self.expected_live_version_id,
        })
    }
}

