pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReleaseCheckView {
    /// The flag rate above which a release is blocked, 0 to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ceiling: Option<f64>,
    #[serde(default)]
    pub enabled: bool,
}

impl ReleaseCheckView {
    pub fn builder() -> ReleaseCheckViewBuilder {
        <ReleaseCheckViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleaseCheckViewBuilder {
    ceiling: Option<f64>,
    enabled: Option<bool>,
}

impl ReleaseCheckViewBuilder {
    pub fn ceiling(mut self, value: f64) -> Self {
        self.ceiling = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReleaseCheckView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](ReleaseCheckViewBuilder::enabled)
    pub fn build(self) -> Result<ReleaseCheckView, BuildError> {
        Ok(ReleaseCheckView {
            ceiling: self.ceiling,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}
