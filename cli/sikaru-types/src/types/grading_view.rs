pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GradingView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub status: GradingViewStatus,
}

impl GradingView {
    pub fn builder() -> GradingViewBuilder {
        <GradingViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GradingViewBuilder {
    message: Option<String>,
    status: Option<GradingViewStatus>,
}

impl GradingViewBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn status(mut self, value: GradingViewStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GradingView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](GradingViewBuilder::status)
    pub fn build(self) -> Result<GradingView, BuildError> {
        Ok(GradingView {
            message: self.message,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
