pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LiveLink {
    #[serde(rename = "liveUrl")]
    #[serde(default)]
    pub live_url: String,
}

impl LiveLink {
    pub fn builder() -> LiveLinkBuilder {
        <LiveLinkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LiveLinkBuilder {
    live_url: Option<String>,
}

impl LiveLinkBuilder {
    pub fn live_url(mut self, value: impl Into<String>) -> Self {
        self.live_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LiveLink`].
    /// This method will fail if any of the following fields are not set:
    /// - [`live_url`](LiveLinkBuilder::live_url)
    pub fn build(self) -> Result<LiveLink, BuildError> {
        Ok(LiveLink {
            live_url: self.live_url.ok_or_else(|| BuildError::missing_field("live_url"))?,
        })
    }
}
