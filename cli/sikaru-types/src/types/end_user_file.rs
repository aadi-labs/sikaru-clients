pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EndUserFile {
    #[serde(default)]
    pub content: String,
    /// Pass as expectedEtag to replace exactly this content.
    #[serde(default)]
    pub etag: String,
    /// USER.md, memories/<name>.md or cell/PERSONA_SUMMARY.md.
    #[serde(default)]
    pub path: String,
}

impl EndUserFile {
    pub fn builder() -> EndUserFileBuilder {
        <EndUserFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EndUserFileBuilder {
    content: Option<String>,
    etag: Option<String>,
    path: Option<String>,
}

impl EndUserFileBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn etag(mut self, value: impl Into<String>) -> Self {
        self.etag = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EndUserFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](EndUserFileBuilder::content)
    /// - [`etag`](EndUserFileBuilder::etag)
    /// - [`path`](EndUserFileBuilder::path)
    pub fn build(self) -> Result<EndUserFile, BuildError> {
        Ok(EndUserFile {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            etag: self.etag.ok_or_else(|| BuildError::missing_field("etag"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
