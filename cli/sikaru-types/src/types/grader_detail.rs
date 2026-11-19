pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraderDetail {
    #[serde(default)]
    pub bar: Bar,
    #[serde(default)]
    pub criteria: Vec<CriterionView>,
    #[serde(rename = "criteriaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria_version: Option<i64>,
    /// The live version, else the newest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<GraderVersionView>,
    #[serde(default)]
    pub description: String,
    /// Uncertain conversations go to the LLM judge (decision model only).
    #[serde(default)]
    pub escalate: bool,
    #[serde(rename = "failureId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_id: Option<String>,
    /// The agent-folder file that defines this Grader, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<GraderFileView>,
    #[serde(default)]
    pub id: String,
    pub judge: JudgeView,
    #[serde(default)]
    pub labels: LabelCounts,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<GraderVersionView>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause: Option<PauseView>,
    #[serde(default)]
    pub refitting: bool,
    #[serde(rename = "releaseCheck")]
    #[serde(default)]
    pub release_check: ReleaseCheckView,
    #[serde(rename = "staleSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_since: Option<String>,
    pub status: GraderDetailStatus,
    /// Newest first.
    #[serde(default)]
    pub versions: Vec<GraderVersionView>,
}

impl GraderDetail {
    pub fn builder() -> GraderDetailBuilder {
        <GraderDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraderDetailBuilder {
    bar: Option<Bar>,
    criteria: Option<Vec<CriterionView>>,
    criteria_version: Option<i64>,
    current: Option<GraderVersionView>,
    description: Option<String>,
    escalate: Option<bool>,
    failure_id: Option<String>,
    file: Option<GraderFileView>,
    id: Option<String>,
    judge: Option<JudgeView>,
    labels: Option<LabelCounts>,
    latest: Option<GraderVersionView>,
    name: Option<String>,
    origin: Option<String>,
    pause: Option<PauseView>,
    refitting: Option<bool>,
    release_check: Option<ReleaseCheckView>,
    stale_since: Option<String>,
    status: Option<GraderDetailStatus>,
    versions: Option<Vec<GraderVersionView>>,
}

impl GraderDetailBuilder {
    pub fn bar(mut self, value: Bar) -> Self {
        self.bar = Some(value);
        self
    }

    pub fn criteria(mut self, value: Vec<CriterionView>) -> Self {
        self.criteria = Some(value);
        self
    }

    pub fn criteria_version(mut self, value: i64) -> Self {
        self.criteria_version = Some(value);
        self
    }

    pub fn current(mut self, value: GraderVersionView) -> Self {
        self.current = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn escalate(mut self, value: bool) -> Self {
        self.escalate = Some(value);
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

    pub fn judge(mut self, value: JudgeView) -> Self {
        self.judge = Some(value);
        self
    }

    pub fn labels(mut self, value: LabelCounts) -> Self {
        self.labels = Some(value);
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

    pub fn pause(mut self, value: PauseView) -> Self {
        self.pause = Some(value);
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

    pub fn stale_since(mut self, value: impl Into<String>) -> Self {
        self.stale_since = Some(value.into());
        self
    }

    pub fn status(mut self, value: GraderDetailStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn versions(mut self, value: Vec<GraderVersionView>) -> Self {
        self.versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraderDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bar`](GraderDetailBuilder::bar)
    /// - [`criteria`](GraderDetailBuilder::criteria)
    /// - [`description`](GraderDetailBuilder::description)
    /// - [`escalate`](GraderDetailBuilder::escalate)
    /// - [`id`](GraderDetailBuilder::id)
    /// - [`judge`](GraderDetailBuilder::judge)
    /// - [`labels`](GraderDetailBuilder::labels)
    /// - [`name`](GraderDetailBuilder::name)
    /// - [`origin`](GraderDetailBuilder::origin)
    /// - [`refitting`](GraderDetailBuilder::refitting)
    /// - [`release_check`](GraderDetailBuilder::release_check)
    /// - [`status`](GraderDetailBuilder::status)
    /// - [`versions`](GraderDetailBuilder::versions)
    pub fn build(self) -> Result<GraderDetail, BuildError> {
        Ok(GraderDetail {
            bar: self.bar.ok_or_else(|| BuildError::missing_field("bar"))?,
            criteria: self.criteria.ok_or_else(|| BuildError::missing_field("criteria"))?,
            criteria_version: self.criteria_version,
            current: self.current,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            escalate: self.escalate.ok_or_else(|| BuildError::missing_field("escalate"))?,
            failure_id: self.failure_id,
            file: self.file,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            judge: self.judge.ok_or_else(|| BuildError::missing_field("judge"))?,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
            latest: self.latest,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            origin: self.origin.ok_or_else(|| BuildError::missing_field("origin"))?,
            pause: self.pause,
            refitting: self.refitting.ok_or_else(|| BuildError::missing_field("refitting"))?,
            release_check: self.release_check.ok_or_else(|| BuildError::missing_field("release_check"))?,
            stale_since: self.stale_since,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            versions: self.versions.ok_or_else(|| BuildError::missing_field("versions"))?,
        })
    }
}
