pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplyToolProviderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_policy_ref: Option<String>,
    /// A public HTTPS URL receives signed pushes; any other ref is poll-only.
    #[serde(default)]
    pub broker_endpoint_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_owner: Option<String>,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub provider_type: String,
    /// A Standard Webhooks secret: 24-64 random bytes, base64-encoded, with or without the whsec_ prefix. Omit to keep the current secret, or to have Sikaru generate one for a new HTTPS provider. A different secret rotates with a 24-hour overlap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(default)]
    pub tool_catalog_ref: String,
}

impl ApplyToolProviderRequest {
    pub fn builder() -> ApplyToolProviderRequestBuilder {
        <ApplyToolProviderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplyToolProviderRequestBuilder {
    approval_policy_ref: Option<String>,
    broker_endpoint_ref: Option<String>,
    credential_owner: Option<String>,
    display_name: Option<String>,
    provider_type: Option<String>,
    secret: Option<String>,
    tool_catalog_ref: Option<String>,
}

impl ApplyToolProviderRequestBuilder {
    pub fn approval_policy_ref(mut self, value: impl Into<String>) -> Self {
        self.approval_policy_ref = Some(value.into());
        self
    }

    pub fn broker_endpoint_ref(mut self, value: impl Into<String>) -> Self {
        self.broker_endpoint_ref = Some(value.into());
        self
    }

    pub fn credential_owner(mut self, value: impl Into<String>) -> Self {
        self.credential_owner = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn provider_type(mut self, value: impl Into<String>) -> Self {
        self.provider_type = Some(value.into());
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    pub fn tool_catalog_ref(mut self, value: impl Into<String>) -> Self {
        self.tool_catalog_ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApplyToolProviderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`broker_endpoint_ref`](ApplyToolProviderRequestBuilder::broker_endpoint_ref)
    /// - [`display_name`](ApplyToolProviderRequestBuilder::display_name)
    /// - [`provider_type`](ApplyToolProviderRequestBuilder::provider_type)
    /// - [`tool_catalog_ref`](ApplyToolProviderRequestBuilder::tool_catalog_ref)
    pub fn build(self) -> Result<ApplyToolProviderRequest, BuildError> {
        Ok(ApplyToolProviderRequest {
            approval_policy_ref: self.approval_policy_ref,
            broker_endpoint_ref: self.broker_endpoint_ref.ok_or_else(|| BuildError::missing_field("broker_endpoint_ref"))?,
            credential_owner: self.credential_owner,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            provider_type: self.provider_type.ok_or_else(|| BuildError::missing_field("provider_type"))?,
            secret: self.secret,
            tool_catalog_ref: self.tool_catalog_ref.ok_or_else(|| BuildError::missing_field("tool_catalog_ref"))?,
        })
    }
}

