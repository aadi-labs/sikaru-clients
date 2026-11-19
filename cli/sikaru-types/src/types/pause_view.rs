pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PauseView {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub reason: String,
}

impl PauseView {
    pub fn builder() -> PauseViewBuilder {
        <PauseViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PauseViewBuilder {
    message: Option<String>,
    reason: Option<String>,
}

impl PauseViewBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PauseView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](PauseViewBuilder::message)
    /// - [`reason`](PauseViewBuilder::reason)
    pub fn build(self) -> Result<PauseView, BuildError> {
        Ok(PauseView {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
