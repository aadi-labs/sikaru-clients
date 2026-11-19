pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SaveDocument {
    #[serde(rename = "baseRevision")]
    #[serde(default)]
    pub base_revision: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<HashMap<String, Option<String>>>,
}

impl SaveDocument {
    pub fn builder() -> SaveDocumentBuilder {
        <SaveDocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SaveDocumentBuilder {
    base_revision: Option<i64>,
    document: Option<String>,
    files: Option<HashMap<String, Option<String>>>,
}

impl SaveDocumentBuilder {
    pub fn base_revision(mut self, value: i64) -> Self {
        self.base_revision = Some(value);
        self
    }

    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
        self
    }

    pub fn files(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SaveDocument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`base_revision`](SaveDocumentBuilder::base_revision)
    pub fn build(self) -> Result<SaveDocument, BuildError> {
        Ok(SaveDocument {
            base_revision: self.base_revision.ok_or_else(|| BuildError::missing_field("base_revision"))?,
            document: self.document,
            files: self.files,
        })
    }
}

