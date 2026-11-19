pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdoptSuggestion {
    #[serde(rename = "baseRevision")]
    #[serde(default)]
    pub base_revision: i64,
}

impl AdoptSuggestion {
    pub fn builder() -> AdoptSuggestionBuilder {
        <AdoptSuggestionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdoptSuggestionBuilder {
    base_revision: Option<i64>,
}

impl AdoptSuggestionBuilder {
    pub fn base_revision(mut self, value: i64) -> Self {
        self.base_revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdoptSuggestion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`base_revision`](AdoptSuggestionBuilder::base_revision)
    pub fn build(self) -> Result<AdoptSuggestion, BuildError> {
        Ok(AdoptSuggestion {
            base_revision: self.base_revision.ok_or_else(|| BuildError::missing_field("base_revision"))?,
        })
    }
}

