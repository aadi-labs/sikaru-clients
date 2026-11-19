pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PublishAnyway {
    /// Why this version goes live although its release check held it.
    #[serde(default)]
    pub reason: String,
}

impl PublishAnyway {
    pub fn builder() -> PublishAnywayBuilder {
        <PublishAnywayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublishAnywayBuilder {
    reason: Option<String>,
}

impl PublishAnywayBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PublishAnyway`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](PublishAnywayBuilder::reason)
    pub fn build(self) -> Result<PublishAnyway, BuildError> {
        Ok(PublishAnyway {
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
