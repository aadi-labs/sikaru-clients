pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Which delivery secret is current and how long the previous one stays valid. Never secret bytes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ToolProviderSecretState {
    /// First 16 hex characters of SHA-256 over the decoded secret.
    #[serde(default)]
    pub fingerprint: String,
    #[serde(rename = "previousExpiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub previous_expires_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "previousFingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_fingerprint: Option<String>,
    #[serde(rename = "setAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub set_at: DateTime<FixedOffset>,
    pub source: ToolProviderSecretStateSource,
    #[serde(default)]
    pub version: i64,
}

impl ToolProviderSecretState {
    pub fn builder() -> ToolProviderSecretStateBuilder {
        <ToolProviderSecretStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolProviderSecretStateBuilder {
    fingerprint: Option<String>,
    previous_expires_at: Option<DateTime<FixedOffset>>,
    previous_fingerprint: Option<String>,
    set_at: Option<DateTime<FixedOffset>>,
    source: Option<ToolProviderSecretStateSource>,
    version: Option<i64>,
}

impl ToolProviderSecretStateBuilder {
    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn previous_expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.previous_expires_at = Some(value);
        self
    }

    pub fn previous_fingerprint(mut self, value: impl Into<String>) -> Self {
        self.previous_fingerprint = Some(value.into());
        self
    }

    pub fn set_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.set_at = Some(value);
        self
    }

    pub fn source(mut self, value: ToolProviderSecretStateSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolProviderSecretState`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fingerprint`](ToolProviderSecretStateBuilder::fingerprint)
    /// - [`set_at`](ToolProviderSecretStateBuilder::set_at)
    /// - [`source`](ToolProviderSecretStateBuilder::source)
    /// - [`version`](ToolProviderSecretStateBuilder::version)
    pub fn build(self) -> Result<ToolProviderSecretState, BuildError> {
        Ok(ToolProviderSecretState {
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            previous_expires_at: self.previous_expires_at,
            previous_fingerprint: self.previous_fingerprint,
            set_at: self.set_at.ok_or_else(|| BuildError::missing_field("set_at"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
