pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicationState {
    /// The refusal code of a held publication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The version this publication releases.
    #[serde(rename = "harnessVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "publicationId")]
    #[serde(default)]
    pub publication_id: String,
    /// What each Grader found, for a held publication.
    #[serde(default)]
    pub results: Vec<HeldResult>,
    /// `pending` while its release check runs; `passed` once the check cleared, before the publish is sent again to release it; `held` when the check blocked it or couldn't verify it (Publish Anyway can release it); `released` once its version is live; `ended` when it ended any other way.
    pub status: PublicationStateStatus,
}

impl PublicationState {
    pub fn builder() -> PublicationStateBuilder {
        <PublicationStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicationStateBuilder {
    code: Option<String>,
    harness_version_id: Option<String>,
    message: Option<String>,
    publication_id: Option<String>,
    results: Option<Vec<HeldResult>>,
    status: Option<PublicationStateStatus>,
}

impl PublicationStateBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.harness_version_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn publication_id(mut self, value: impl Into<String>) -> Self {
        self.publication_id = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<HeldResult>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn status(mut self, value: PublicationStateStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicationState`].
    /// This method will fail if any of the following fields are not set:
    /// - [`publication_id`](PublicationStateBuilder::publication_id)
    /// - [`results`](PublicationStateBuilder::results)
    /// - [`status`](PublicationStateBuilder::status)
    pub fn build(self) -> Result<PublicationState, BuildError> {
        Ok(PublicationState {
            code: self.code,
            harness_version_id: self.harness_version_id,
            message: self.message,
            publication_id: self.publication_id.ok_or_else(|| BuildError::missing_field("publication_id"))?,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
