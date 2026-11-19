pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MemoryFileEntry {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub size: i64,
}

impl MemoryFileEntry {
    pub fn builder() -> MemoryFileEntryBuilder {
        <MemoryFileEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemoryFileEntryBuilder {
    path: Option<String>,
    size: Option<i64>,
}

impl MemoryFileEntryBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemoryFileEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](MemoryFileEntryBuilder::path)
    /// - [`size`](MemoryFileEntryBuilder::size)
    pub fn build(self) -> Result<MemoryFileEntry, BuildError> {
        Ok(MemoryFileEntry {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
        })
    }
}
