pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MemoryFile {
    #[serde(default)]
    pub content: String,
    /// The current version of the directory; pass it as base when writing. Null for an empty directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<String>,
    #[serde(default)]
    pub path: String,
}

impl MemoryFile {
    pub fn builder() -> MemoryFileBuilder {
        <MemoryFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemoryFileBuilder {
    content: Option<String>,
    head: Option<String>,
    path: Option<String>,
}

impl MemoryFileBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn head(mut self, value: impl Into<String>) -> Self {
        self.head = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MemoryFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](MemoryFileBuilder::content)
    /// - [`path`](MemoryFileBuilder::path)
    pub fn build(self) -> Result<MemoryFile, BuildError> {
        Ok(MemoryFile {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            head: self.head,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
