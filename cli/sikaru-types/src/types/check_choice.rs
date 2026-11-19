pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CheckChoice {
    /// The flag-rate ceiling; null removes it, and leaving it out keeps the current one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ceiling: Option<f64>,
    #[serde(default)]
    pub enabled: bool,
}

impl CheckChoice {
    pub fn builder() -> CheckChoiceBuilder {
        <CheckChoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckChoiceBuilder {
    ceiling: Option<f64>,
    enabled: Option<bool>,
}

impl CheckChoiceBuilder {
    pub fn ceiling(mut self, value: f64) -> Self {
        self.ceiling = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckChoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](CheckChoiceBuilder::enabled)
    pub fn build(self) -> Result<CheckChoice, BuildError> {
        Ok(CheckChoice {
            ceiling: self.ceiling,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}
