pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TestBody {
    /// The Grader file's text, as a push would read it. It is tested by its chosen judge without saving a Grader or verdict; its ceiling applies while its release check is on.
    #[serde(default)]
    pub file: String,
    /// Recent conversations to score.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The Grader file's name: graders/<name>.toml in the agent folder.
    #[serde(default)]
    pub name: String,
}

impl TestBody {
    pub fn builder() -> TestBodyBuilder {
        <TestBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestBodyBuilder {
    file: Option<String>,
    limit: Option<i64>,
    name: Option<String>,
}

impl TestBodyBuilder {
    pub fn file(mut self, value: impl Into<String>) -> Self {
        self.file = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TestBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](TestBodyBuilder::file)
    /// - [`name`](TestBodyBuilder::name)
    pub fn build(self) -> Result<TestBody, BuildError> {
        Ok(TestBody {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            limit: self.limit,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

