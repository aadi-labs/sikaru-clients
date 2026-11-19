pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MemoryHead {
    /// The current version of the directory; pass it as base when writing. Null for an empty directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<String>,
}

impl MemoryHead {
    pub fn builder() -> MemoryHeadBuilder {
        <MemoryHeadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemoryHeadBuilder {
    head: Option<String>,
}

impl MemoryHeadBuilder {
    pub fn head(mut self, value: impl Into<String>) -> Self {
        self.head = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MemoryHead`].
    pub fn build(self) -> Result<MemoryHead, BuildError> {
        Ok(MemoryHead {
            head: self.head,
        })
    }
}
