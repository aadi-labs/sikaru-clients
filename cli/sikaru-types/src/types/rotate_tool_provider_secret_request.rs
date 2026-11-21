pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RotateToolProviderSecretRequest {
    /// Seconds the previous secret stays valid; 0 retires it at once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap_seconds: Option<i64>,
    /// A Standard Webhooks secret: 24-64 random bytes, base64-encoded, with or without the whsec_ prefix. Omit to have Sikaru generate one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl RotateToolProviderSecretRequest {
    pub fn builder() -> RotateToolProviderSecretRequestBuilder {
        <RotateToolProviderSecretRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RotateToolProviderSecretRequestBuilder {
    overlap_seconds: Option<i64>,
    secret: Option<String>,
}

impl RotateToolProviderSecretRequestBuilder {
    pub fn overlap_seconds(mut self, value: i64) -> Self {
        self.overlap_seconds = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RotateToolProviderSecretRequest`].
    pub fn build(self) -> Result<RotateToolProviderSecretRequest, BuildError> {
        Ok(RotateToolProviderSecretRequest {
            overlap_seconds: self.overlap_seconds,
            secret: self.secret,
        })
    }
}

