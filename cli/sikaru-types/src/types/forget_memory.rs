pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForgetMemory {
    /// What to forget, in plain words.
    #[serde(default)]
    pub target: String,
}

impl ForgetMemory {
    pub fn builder() -> ForgetMemoryBuilder {
        <ForgetMemoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForgetMemoryBuilder {
    target: Option<String>,
}

impl ForgetMemoryBuilder {
    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ForgetMemory`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target`](ForgetMemoryBuilder::target)
    pub fn build(self) -> Result<ForgetMemory, BuildError> {
        Ok(ForgetMemory {
            target: self.target.ok_or_else(|| BuildError::missing_field("target"))?,
        })
    }
}
