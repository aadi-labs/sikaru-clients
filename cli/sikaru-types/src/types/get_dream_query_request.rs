pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_dream
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDreamQueryRequest {
    /// The shared agent directory, or one end user's directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<GetDreamMemoryRequestDirectory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<GetDreamMemoryRequestEnvironment>,
    #[serde(rename = "tenantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl GetDreamQueryRequest {
    pub fn builder() -> GetDreamQueryRequestBuilder {
        <GetDreamQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDreamQueryRequestBuilder {
    directory: Option<GetDreamMemoryRequestDirectory>,
    environment: Option<GetDreamMemoryRequestEnvironment>,
    tenant_id: Option<String>,
    user_id: Option<String>,
}

impl GetDreamQueryRequestBuilder {
    pub fn directory(mut self, value: GetDreamMemoryRequestDirectory) -> Self {
        self.directory = Some(value);
        self
    }

    pub fn environment(mut self, value: GetDreamMemoryRequestEnvironment) -> Self {
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

    /// Consumes the builder and constructs a [`GetDreamQueryRequest`].
    pub fn build(self) -> Result<GetDreamQueryRequest, BuildError> {
        Ok(GetDreamQueryRequest {
            directory: self.directory,
            environment: self.environment,
            tenant_id: self.tenant_id,
            user_id: self.user_id,
        })
    }
}

