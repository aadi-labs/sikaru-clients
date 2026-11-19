pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthoredToolProviderRef {
    #[serde(default)]
    pub capability_prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_skill_refs: Option<Vec<String>>,
    #[serde(default)]
    pub tool_provider_id: String,
}

impl AuthoredToolProviderRef {
    pub fn builder() -> AuthoredToolProviderRefBuilder {
        <AuthoredToolProviderRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthoredToolProviderRefBuilder {
    capability_prefix: Option<String>,
    source_skill_refs: Option<Vec<String>>,
    tool_provider_id: Option<String>,
}

impl AuthoredToolProviderRefBuilder {
    pub fn capability_prefix(mut self, value: impl Into<String>) -> Self {
        self.capability_prefix = Some(value.into());
        self
    }

    pub fn source_skill_refs(mut self, value: Vec<String>) -> Self {
        self.source_skill_refs = Some(value);
        self
    }

    pub fn tool_provider_id(mut self, value: impl Into<String>) -> Self {
        self.tool_provider_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthoredToolProviderRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capability_prefix`](AuthoredToolProviderRefBuilder::capability_prefix)
    /// - [`tool_provider_id`](AuthoredToolProviderRefBuilder::tool_provider_id)
    pub fn build(self) -> Result<AuthoredToolProviderRef, BuildError> {
        Ok(AuthoredToolProviderRef {
            capability_prefix: self.capability_prefix.ok_or_else(|| BuildError::missing_field("capability_prefix"))?,
            source_skill_refs: self.source_skill_refs,
            tool_provider_id: self.tool_provider_id.ok_or_else(|| BuildError::missing_field("tool_provider_id"))?,
        })
    }
}
