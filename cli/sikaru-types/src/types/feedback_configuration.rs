pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeedbackConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emojis: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl FeedbackConfiguration {
    pub fn builder() -> FeedbackConfigurationBuilder {
        <FeedbackConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedbackConfigurationBuilder {
    emojis: Option<HashMap<String, String>>,
    enabled: Option<bool>,
}

impl FeedbackConfigurationBuilder {
    pub fn emojis(mut self, value: HashMap<String, String>) -> Self {
        self.emojis = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedbackConfiguration`].
    pub fn build(self) -> Result<FeedbackConfiguration, BuildError> {
        Ok(FeedbackConfiguration {
            emojis: self.emojis,
            enabled: self.enabled,
        })
    }
}
