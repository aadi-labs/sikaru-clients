pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraderList {
    #[serde(default)]
    pub graders: Vec<GraderSummary>,
    pub grading: GradingView,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause: Option<PauseView>,
    #[serde(rename = "staleSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_since: Option<String>,
}

impl GraderList {
    pub fn builder() -> GraderListBuilder {
        <GraderListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraderListBuilder {
    graders: Option<Vec<GraderSummary>>,
    grading: Option<GradingView>,
    pause: Option<PauseView>,
    stale_since: Option<String>,
}

impl GraderListBuilder {
    pub fn graders(mut self, value: Vec<GraderSummary>) -> Self {
        self.graders = Some(value);
        self
    }

    pub fn grading(mut self, value: GradingView) -> Self {
        self.grading = Some(value);
        self
    }

    pub fn pause(mut self, value: PauseView) -> Self {
        self.pause = Some(value);
        self
    }

    pub fn stale_since(mut self, value: impl Into<String>) -> Self {
        self.stale_since = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GraderList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`graders`](GraderListBuilder::graders)
    /// - [`grading`](GraderListBuilder::grading)
    pub fn build(self) -> Result<GraderList, BuildError> {
        Ok(GraderList {
            graders: self.graders.ok_or_else(|| BuildError::missing_field("graders"))?,
            grading: self.grading.ok_or_else(|| BuildError::missing_field("grading"))?,
            pause: self.pause,
            stale_since: self.stale_since,
        })
    }
}
