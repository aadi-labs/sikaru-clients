pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEndUserFact {
    #[serde(default)]
    pub content: String,
    /// Refuse with 409 unless the fact still has this etag.
    #[serde(rename = "expectedEtag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_etag: Option<String>,
    /// USER.md or memories/<name>.md.
    #[serde(default)]
    pub path: String,
}

impl UpdateEndUserFact {
    pub fn builder() -> UpdateEndUserFactBuilder {
        <UpdateEndUserFactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEndUserFactBuilder {
    content: Option<String>,
    expected_etag: Option<String>,
    path: Option<String>,
}

impl UpdateEndUserFactBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn expected_etag(mut self, value: impl Into<String>) -> Self {
        self.expected_etag = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateEndUserFact`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](UpdateEndUserFactBuilder::content)
    /// - [`path`](UpdateEndUserFactBuilder::path)
    pub fn build(self) -> Result<UpdateEndUserFact, BuildError> {
        Ok(UpdateEndUserFact {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            expected_etag: self.expected_etag,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}

