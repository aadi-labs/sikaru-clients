pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CriterionView {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub name: String,
}

impl CriterionView {
    pub fn builder() -> CriterionViewBuilder {
        <CriterionViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CriterionViewBuilder {
    description: Option<String>,
    kind: Option<String>,
    name: Option<String>,
}

impl CriterionViewBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CriterionView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](CriterionViewBuilder::description)
    /// - [`kind`](CriterionViewBuilder::kind)
    /// - [`name`](CriterionViewBuilder::name)
    pub fn build(self) -> Result<CriterionView, BuildError> {
        Ok(CriterionView {
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
