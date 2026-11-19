pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportFile {
    #[serde(default)]
    pub content: String,
    #[serde(rename = "expectedEtag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_etag: Option<String>,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub provenance: String,
}

impl ImportFile {
    pub fn builder() -> ImportFileBuilder {
        <ImportFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportFileBuilder {
    content: Option<String>,
    expected_etag: Option<String>,
    path: Option<String>,
    provenance: Option<String>,
}

impl ImportFileBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn expected_etag(mut self, value: impl Into<String>) -> Self {
        self.expected_etag = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn provenance(mut self, value: impl Into<String>) -> Self {
        self.provenance = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](ImportFileBuilder::content)
    /// - [`path`](ImportFileBuilder::path)
    /// - [`provenance`](ImportFileBuilder::provenance)
    pub fn build(self) -> Result<ImportFile, BuildError> {
        Ok(ImportFile {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            expected_etag: self.expected_etag,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            provenance: self.provenance.ok_or_else(|| BuildError::missing_field("provenance"))?,
        })
    }
}
