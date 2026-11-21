pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The provider after a declaration or rotation; carries a secret only when Sikaru generated one.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AppliedToolProvider {
    #[serde(rename = "approvalPolicyRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_policy_ref: Option<String>,
    #[serde(rename = "brokerEndpointRef")]
    #[serde(default)]
    pub broker_endpoint_ref: String,
    #[serde(rename = "capabilityPrefix")]
    #[serde(default)]
    pub capability_prefix: String,
    #[serde(rename = "credentialOwner")]
    #[serde(default)]
    pub credential_owner: String,
    /// push: Sikaru signs and sends tool requests to the HTTPS endpoint. poll: the product reads pending actions.
    #[serde(rename = "deliveryMode")]
    pub delivery_mode: AppliedToolProviderDeliveryMode,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "providerType")]
    #[serde(default)]
    pub provider_type: String,
    /// Present only when Sikaru generated a secret in this call; it is never shown again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "secretState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_state: Option<ToolProviderSecretState>,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "toolCatalogRef")]
    #[serde(default)]
    pub tool_catalog_ref: String,
}

impl AppliedToolProvider {
    pub fn builder() -> AppliedToolProviderBuilder {
        <AppliedToolProviderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppliedToolProviderBuilder {
    approval_policy_ref: Option<String>,
    broker_endpoint_ref: Option<String>,
    capability_prefix: Option<String>,
    credential_owner: Option<String>,
    delivery_mode: Option<AppliedToolProviderDeliveryMode>,
    display_name: Option<String>,
    id: Option<String>,
    provider_type: Option<String>,
    secret: Option<String>,
    secret_state: Option<ToolProviderSecretState>,
    status: Option<String>,
    tool_catalog_ref: Option<String>,
}

impl AppliedToolProviderBuilder {
    pub fn approval_policy_ref(mut self, value: impl Into<String>) -> Self {
        self.approval_policy_ref = Some(value.into());
        self
    }

    pub fn broker_endpoint_ref(mut self, value: impl Into<String>) -> Self {
        self.broker_endpoint_ref = Some(value.into());
        self
    }

    pub fn capability_prefix(mut self, value: impl Into<String>) -> Self {
        self.capability_prefix = Some(value.into());
        self
    }

    pub fn credential_owner(mut self, value: impl Into<String>) -> Self {
        self.credential_owner = Some(value.into());
        self
    }

    pub fn delivery_mode(mut self, value: AppliedToolProviderDeliveryMode) -> Self {
        self.delivery_mode = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn secret_state(mut self, value: ToolProviderSecretState) -> Self {
        self.secret_state = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn tool_catalog_ref(mut self, value: impl Into<String>) -> Self {
        self.tool_catalog_ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AppliedToolProvider`].
    /// This method will fail if any of the following fields are not set:
    /// - [`broker_endpoint_ref`](AppliedToolProviderBuilder::broker_endpoint_ref)
    /// - [`capability_prefix`](AppliedToolProviderBuilder::capability_prefix)
    /// - [`credential_owner`](AppliedToolProviderBuilder::credential_owner)
    /// - [`delivery_mode`](AppliedToolProviderBuilder::delivery_mode)
    /// - [`display_name`](AppliedToolProviderBuilder::display_name)
    /// - [`id`](AppliedToolProviderBuilder::id)
    /// - [`provider_type`](AppliedToolProviderBuilder::provider_type)
    /// - [`status`](AppliedToolProviderBuilder::status)
    /// - [`tool_catalog_ref`](AppliedToolProviderBuilder::tool_catalog_ref)
    pub fn build(self) -> Result<AppliedToolProvider, BuildError> {
        Ok(AppliedToolProvider {
            approval_policy_ref: self.approval_policy_ref,
            broker_endpoint_ref: self.broker_endpoint_ref.ok_or_else(|| BuildError::missing_field("broker_endpoint_ref"))?,
            capability_prefix: self.capability_prefix.ok_or_else(|| BuildError::missing_field("capability_prefix"))?,
            credential_owner: self.credential_owner.ok_or_else(|| BuildError::missing_field("credential_owner"))?,
            delivery_mode: self.delivery_mode.ok_or_else(|| BuildError::missing_field("delivery_mode"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            provider_type: self.provider_type.ok_or_else(|| BuildError::missing_field("provider_type"))?,
            secret: self.secret,
            secret_state: self.secret_state,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            tool_catalog_ref: self.tool_catalog_ref.ok_or_else(|| BuildError::missing_field("tool_catalog_ref"))?,
        })
    }
}
