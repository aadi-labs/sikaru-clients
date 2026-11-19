pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraderSummary {
    /// The live version, else the newest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<GraderVersionView>,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "failureId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_id: Option<String>,
    /// The agent-folder file that defines this Grader, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<GraderFileView>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<GraderVersionView>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub origin: String,
    #[serde(default)]
    pub refitting: bool,
    #[serde(rename = "releaseCheck")]
    #[serde(default)]
    pub release_check: ReleaseCheckView,
    pub status: GraderSummaryStatus,
}

impl GraderSummary {
    pub fn builder() -> GraderSummaryBuilder {
        <GraderSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraderSummaryBuilder {
    current: Option<GraderVersionView>,
    description: Option<String>,
    failure_id: Option<String>,
    file: Option<GraderFileView>,
    id: Option<String>,
    latest: Option<GraderVersionView>,
    name: Option<String>,
    origin: Option<String>,
    refitting: Option<bool>,
    release_check: Option<ReleaseCheckView>,
    status: Option<GraderSummaryStatus>,
}

impl GraderSummaryBuilder {
    pub fn current(mut self, value: GraderVersionView) -> Self {
        self.current = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn failure_id(mut self, value: impl Into<String>) -> Self {
        self.failure_id = Some(value.into());
        self
    }

    pub fn file(mut self, value: GraderFileView) -> Self {
        self.file = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn latest(mut self, value: GraderVersionView) -> Self {
        self.latest = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn origin(mut self, value: impl Into<String>) -> Self {
        self.origin = Some(value.into());
        self
    }

    pub fn refitting(mut self, value: bool) -> Self {
        self.refitting = Some(value);
        self
    }

    pub fn release_check(mut self, value: ReleaseCheckView) -> Self {
        self.release_check = Some(value);
        self
    }

    pub fn status(mut self, value: GraderSummaryStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraderSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](GraderSummaryBuilder::description)
    /// - [`id`](GraderSummaryBuilder::id)
    /// - [`name`](GraderSummaryBuilder::name)
    /// - [`origin`](GraderSummaryBuilder::origin)
    /// - [`refitting`](GraderSummaryBuilder::refitting)
    /// - [`release_check`](GraderSummaryBuilder::release_check)
    /// - [`status`](GraderSummaryBuilder::status)
    pub fn build(self) -> Result<GraderSummary, BuildError> {
        Ok(GraderSummary {
            current: self.current,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            failure_id: self.failure_id,
            file: self.file,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            latest: self.latest,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            origin: self.origin.ok_or_else(|| BuildError::missing_field("origin"))?,
            refitting: self.refitting.ok_or_else(|| BuildError::missing_field("refitting"))?,
            release_check: self.release_check.ok_or_else(|| BuildError::missing_field("release_check"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
