pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MemoryFiles {
    #[serde(default)]
    pub files: Vec<MemoryFileEntry>,
    /// The current version of the directory; pass it as base when writing. Null for an empty directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<String>,
}

impl MemoryFiles {
    pub fn builder() -> MemoryFilesBuilder {
        <MemoryFilesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemoryFilesBuilder {
    files: Option<Vec<MemoryFileEntry>>,
    head: Option<String>,
}

impl MemoryFilesBuilder {
    pub fn files(mut self, value: Vec<MemoryFileEntry>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn head(mut self, value: impl Into<String>) -> Self {
        self.head = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MemoryFiles`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](MemoryFilesBuilder::files)
    pub fn build(self) -> Result<MemoryFiles, BuildError> {
        Ok(MemoryFiles {
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
            head: self.head,
        })
    }
}
