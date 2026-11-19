pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExcerptStep {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub source: String,
}

impl ExcerptStep {
    pub fn builder() -> ExcerptStepBuilder {
        <ExcerptStepBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExcerptStepBuilder {
    message: Option<String>,
    source: Option<String>,
}

impl ExcerptStepBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExcerptStep`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ExcerptStepBuilder::message)
    /// - [`source`](ExcerptStepBuilder::source)
    pub fn build(self) -> Result<ExcerptStep, BuildError> {
        Ok(ExcerptStep {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
