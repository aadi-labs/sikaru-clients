pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleaseCheckOverride {
    #[serde(default)]
    pub actor: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// Conversations flagged for this version and not for the live one.
    #[serde(default)]
    pub flagged: Vec<String>,
    #[serde(rename = "publicationId")]
    #[serde(default)]
    pub publication_id: String,
    #[serde(default)]
    pub reason: String,
}

impl ReleaseCheckOverride {
    pub fn builder() -> ReleaseCheckOverrideBuilder {
        <ReleaseCheckOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleaseCheckOverrideBuilder {
    actor: Option<String>,
    created_at: Option<String>,
    flagged: Option<Vec<String>>,
    publication_id: Option<String>,
    reason: Option<String>,
}

impl ReleaseCheckOverrideBuilder {
    pub fn actor(mut self, value: impl Into<String>) -> Self {
        self.actor = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn flagged(mut self, value: Vec<String>) -> Self {
        self.flagged = Some(value);
        self
    }

    pub fn publication_id(mut self, value: impl Into<String>) -> Self {
        self.publication_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReleaseCheckOverride`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actor`](ReleaseCheckOverrideBuilder::actor)
    /// - [`created_at`](ReleaseCheckOverrideBuilder::created_at)
    /// - [`flagged`](ReleaseCheckOverrideBuilder::flagged)
    /// - [`publication_id`](ReleaseCheckOverrideBuilder::publication_id)
    /// - [`reason`](ReleaseCheckOverrideBuilder::reason)
    pub fn build(self) -> Result<ReleaseCheckOverride, BuildError> {
        Ok(ReleaseCheckOverride {
            actor: self.actor.ok_or_else(|| BuildError::missing_field("actor"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            flagged: self.flagged.ok_or_else(|| BuildError::missing_field("flagged"))?,
            publication_id: self.publication_id.ok_or_else(|| BuildError::missing_field("publication_id"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
