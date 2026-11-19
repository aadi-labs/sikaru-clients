pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Playbook {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub site: String,
    #[serde(default)]
    pub slots: Vec<String>,
    #[serde(default)]
    pub stale: bool,
    #[serde(default)]
    pub steps: i64,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub uses: i64,
}

impl Playbook {
    pub fn builder() -> PlaybookBuilder {
        <PlaybookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlaybookBuilder {
    id: Option<String>,
    name: Option<String>,
    site: Option<String>,
    slots: Option<Vec<String>>,
    stale: Option<bool>,
    steps: Option<i64>,
    updated_at: Option<String>,
    uses: Option<i64>,
}

impl PlaybookBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn site(mut self, value: impl Into<String>) -> Self {
        self.site = Some(value.into());
        self
    }

    pub fn slots(mut self, value: Vec<String>) -> Self {
        self.slots = Some(value);
        self
    }

    pub fn stale(mut self, value: bool) -> Self {
        self.stale = Some(value);
        self
    }

    pub fn steps(mut self, value: i64) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn uses(mut self, value: i64) -> Self {
        self.uses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Playbook`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PlaybookBuilder::id)
    /// - [`name`](PlaybookBuilder::name)
    /// - [`site`](PlaybookBuilder::site)
    /// - [`slots`](PlaybookBuilder::slots)
    /// - [`stale`](PlaybookBuilder::stale)
    /// - [`steps`](PlaybookBuilder::steps)
    /// - [`updated_at`](PlaybookBuilder::updated_at)
    /// - [`uses`](PlaybookBuilder::uses)
    pub fn build(self) -> Result<Playbook, BuildError> {
        Ok(Playbook {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            site: self.site.ok_or_else(|| BuildError::missing_field("site"))?,
            slots: self.slots.ok_or_else(|| BuildError::missing_field("slots"))?,
            stale: self.stale.ok_or_else(|| BuildError::missing_field("stale"))?,
            steps: self.steps.ok_or_else(|| BuildError::missing_field("steps"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            uses: self.uses.ok_or_else(|| BuildError::missing_field("uses"))?,
        })
    }
}
