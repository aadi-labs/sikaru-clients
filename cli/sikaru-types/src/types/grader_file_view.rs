pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraderFileView {
    /// graders/<name>.toml in the agent folder.
    #[serde(default)]
    pub path: String,
    /// The file was removed: the Grader no longer scores or checks; labels stay.
    #[serde(default)]
    pub retired: bool,
}

impl GraderFileView {
    pub fn builder() -> GraderFileViewBuilder {
        <GraderFileViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraderFileViewBuilder {
    path: Option<String>,
    retired: Option<bool>,
}

impl GraderFileViewBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn retired(mut self, value: bool) -> Self {
        self.retired = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraderFileView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](GraderFileViewBuilder::path)
    /// - [`retired`](GraderFileViewBuilder::retired)
    pub fn build(self) -> Result<GraderFileView, BuildError> {
        Ok(GraderFileView {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            retired: self.retired.ok_or_else(|| BuildError::missing_field("retired"))?,
        })
    }
}
