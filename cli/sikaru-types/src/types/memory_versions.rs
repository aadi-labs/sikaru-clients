pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MemoryVersions {
    /// The current version of the directory; pass it as base when writing. Null for an empty directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<String>,
    /// Retained versions, newest first.
    #[serde(default)]
    pub versions: Vec<MemoryVersion>,
}

impl MemoryVersions {
    pub fn builder() -> MemoryVersionsBuilder {
        <MemoryVersionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemoryVersionsBuilder {
    head: Option<String>,
    versions: Option<Vec<MemoryVersion>>,
}

impl MemoryVersionsBuilder {
    pub fn head(mut self, value: impl Into<String>) -> Self {
        self.head = Some(value.into());
        self
    }

    pub fn versions(mut self, value: Vec<MemoryVersion>) -> Self {
        self.versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemoryVersions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`versions`](MemoryVersionsBuilder::versions)
    pub fn build(self) -> Result<MemoryVersions, BuildError> {
        Ok(MemoryVersions {
            head: self.head,
            versions: self.versions.ok_or_else(|| BuildError::missing_field("versions"))?,
        })
    }
}
