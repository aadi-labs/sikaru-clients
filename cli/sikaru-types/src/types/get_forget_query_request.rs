pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_forget
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetForgetQueryRequest {
    /// The shared agent directory, or one end user's directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<GetForgetMemoryRequestDirectory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<GetForgetMemoryRequestEnvironment>,
    #[serde(rename = "tenantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl GetForgetQueryRequest {
    pub fn builder() -> GetForgetQueryRequestBuilder {
        <GetForgetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetForgetQueryRequestBuilder {
    directory: Option<GetForgetMemoryRequestDirectory>,
    environment: Option<GetForgetMemoryRequestEnvironment>,
    tenant_id: Option<String>,
    user_id: Option<String>,
}

impl GetForgetQueryRequestBuilder {
    pub fn directory(mut self, value: GetForgetMemoryRequestDirectory) -> Self {
        self.directory = Some(value);
        self
    }

    pub fn environment(mut self, value: GetForgetMemoryRequestEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetForgetQueryRequest`].
    pub fn build(self) -> Result<GetForgetQueryRequest, BuildError> {
        Ok(GetForgetQueryRequest {
            directory: self.directory,
            environment: self.environment,
            tenant_id: self.tenant_id,
            user_id: self.user_id,
        })
    }
}

