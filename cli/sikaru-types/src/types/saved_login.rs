pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SavedLogin {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lastUsedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub site: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl SavedLogin {
    pub fn builder() -> SavedLoginBuilder {
        <SavedLoginBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SavedLoginBuilder {
    id: Option<String>,
    last_used_at: Option<String>,
    owner: Option<String>,
    site: Option<String>,
    updated_at: Option<String>,
}

impl SavedLoginBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_used_at(mut self, value: impl Into<String>) -> Self {
        self.last_used_at = Some(value.into());
        self
    }

    pub fn owner(mut self, value: impl Into<String>) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn site(mut self, value: impl Into<String>) -> Self {
        self.site = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SavedLogin`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SavedLoginBuilder::id)
    /// - [`owner`](SavedLoginBuilder::owner)
    /// - [`site`](SavedLoginBuilder::site)
    /// - [`updated_at`](SavedLoginBuilder::updated_at)
    pub fn build(self) -> Result<SavedLogin, BuildError> {
        Ok(SavedLogin {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_used_at: self.last_used_at,
            owner: self.owner.ok_or_else(|| BuildError::missing_field("owner"))?,
            site: self.site.ok_or_else(|| BuildError::missing_field("site"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
