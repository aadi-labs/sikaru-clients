pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoginRequest {
    #[serde(default)]
    pub url: String,
}

impl LoginRequest {
    pub fn builder() -> LoginRequestBuilder {
        <LoginRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoginRequestBuilder {
    url: Option<String>,
}

impl LoginRequestBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LoginRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](LoginRequestBuilder::url)
    pub fn build(self) -> Result<LoginRequest, BuildError> {
        Ok(LoginRequest {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

