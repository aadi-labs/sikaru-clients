pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateSkillSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "expectedRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision: Option<String>,
    #[serde(rename = "operationId")]
    #[serde(default)]
    pub operation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<HashMap<String, String>>,
    #[serde(rename = "sourceRevision")]
    #[serde(default)]
    pub source_revision: String,
}

impl UpdateSkillSource {
    pub fn builder() -> UpdateSkillSourceBuilder {
        <UpdateSkillSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSkillSourceBuilder {
    content: Option<String>,
    enabled: Option<bool>,
    expected_revision: Option<String>,
    operation_id: Option<String>,
    resources: Option<HashMap<String, String>>,
    source_revision: Option<String>,
}

impl UpdateSkillSourceBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn expected_revision(mut self, value: impl Into<String>) -> Self {
        self.expected_revision = Some(value.into());
        self
    }

    pub fn operation_id(mut self, value: impl Into<String>) -> Self {
        self.operation_id = Some(value.into());
        self
    }

    pub fn resources(mut self, value: HashMap<String, String>) -> Self {
        self.resources = Some(value);
        self
    }

    pub fn source_revision(mut self, value: impl Into<String>) -> Self {
        self.source_revision = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateSkillSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](UpdateSkillSourceBuilder::enabled)
    /// - [`operation_id`](UpdateSkillSourceBuilder::operation_id)
    /// - [`source_revision`](UpdateSkillSourceBuilder::source_revision)
    pub fn build(self) -> Result<UpdateSkillSource, BuildError> {
        Ok(UpdateSkillSource {
            content: self.content,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            expected_revision: self.expected_revision,
            operation_id: self.operation_id.ok_or_else(|| BuildError::missing_field("operation_id"))?,
            resources: self.resources,
            source_revision: self.source_revision.ok_or_else(|| BuildError::missing_field("source_revision"))?,
        })
    }
}

