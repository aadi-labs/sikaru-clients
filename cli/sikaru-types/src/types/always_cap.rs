pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The most an always-allowed commit may send in one named amount field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AlwaysCap {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub max: String,
}

impl AlwaysCap {
    pub fn builder() -> AlwaysCapBuilder {
        <AlwaysCapBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlwaysCapBuilder {
    field: Option<String>,
    max: Option<String>,
}

impl AlwaysCapBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn max(mut self, value: impl Into<String>) -> Self {
        self.max = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AlwaysCap`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AlwaysCapBuilder::field)
    /// - [`max`](AlwaysCapBuilder::max)
    pub fn build(self) -> Result<AlwaysCap, BuildError> {
        Ok(AlwaysCap {
            field: self.field.ok_or_else(|| BuildError::missing_field("field"))?,
            max: self.max.ok_or_else(|| BuildError::missing_field("max"))?,
        })
    }
}
