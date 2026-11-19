pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MemoryForgets {
    #[serde(default)]
    pub forgets: Vec<MemoryForget>,
}

impl MemoryForgets {
    pub fn builder() -> MemoryForgetsBuilder {
        <MemoryForgetsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemoryForgetsBuilder {
    forgets: Option<Vec<MemoryForget>>,
}

impl MemoryForgetsBuilder {
    pub fn forgets(mut self, value: Vec<MemoryForget>) -> Self {
        self.forgets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemoryForgets`].
    /// This method will fail if any of the following fields are not set:
    /// - [`forgets`](MemoryForgetsBuilder::forgets)
    pub fn build(self) -> Result<MemoryForgets, BuildError> {
        Ok(MemoryForgets {
            forgets: self.forgets.ok_or_else(|| BuildError::missing_field("forgets"))?,
        })
    }
}
