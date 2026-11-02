pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EndUser {
    /// The profile (USER.md) and long-term memories (memories/*.md).
    #[serde(default)]
    pub facts: Vec<EndUserFile>,
    /// Whether this user contributes traces and eval cases to population learning.
    #[serde(rename = "optIn")]
    #[serde(default)]
    pub opt_in: bool,
    /// A plain-language summary of how the agent sees this user.
    #[serde(rename = "personaSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona_summary: Option<EndUserFile>,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
}

impl EndUser {
    pub fn builder() -> EndUserBuilder {
        <EndUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EndUserBuilder {
    facts: Option<Vec<EndUserFile>>,
    opt_in: Option<bool>,
    persona_summary: Option<EndUserFile>,
    tenant_id: Option<String>,
    timezone: Option<String>,
    user_id: Option<String>,
}

impl EndUserBuilder {
    pub fn facts(mut self, value: Vec<EndUserFile>) -> Self {
        self.facts = Some(value);
        self
    }

    pub fn opt_in(mut self, value: bool) -> Self {
        self.opt_in = Some(value);
        self
    }

    pub fn persona_summary(mut self, value: EndUserFile) -> Self {
        self.persona_summary = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EndUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`facts`](EndUserBuilder::facts)
    /// - [`opt_in`](EndUserBuilder::opt_in)
    /// - [`tenant_id`](EndUserBuilder::tenant_id)
    /// - [`user_id`](EndUserBuilder::user_id)
    pub fn build(self) -> Result<EndUser, BuildError> {
        Ok(EndUser {
            facts: self.facts.ok_or_else(|| BuildError::missing_field("facts"))?,
            opt_in: self.opt_in.ok_or_else(|| BuildError::missing_field("opt_in"))?,
            persona_summary: self.persona_summary,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            timezone: self.timezone,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
