pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for dream
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DreamQueryRequest {
    /// The shared agent directory, or one end user's directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<DreamMemoryRequestDirectory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<DreamMemoryRequestEnvironment>,
    #[serde(rename = "tenantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl DreamQueryRequest {
    pub fn builder() -> DreamQueryRequestBuilder {
        <DreamQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DreamQueryRequestBuilder {
    directory: Option<DreamMemoryRequestDirectory>,
    environment: Option<DreamMemoryRequestEnvironment>,
    tenant_id: Option<String>,
    user_id: Option<String>,
}

impl DreamQueryRequestBuilder {
    pub fn directory(mut self, value: DreamMemoryRequestDirectory) -> Self {
        self.directory = Some(value);
        self
    }

    pub fn environment(mut self, value: DreamMemoryRequestEnvironment) -> Self {
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

    /// Consumes the builder and constructs a [`DreamQueryRequest`].
    pub fn build(self) -> Result<DreamQueryRequest, BuildError> {
        Ok(DreamQueryRequest {
            directory: self.directory,
            environment: self.environment,
            tenant_id: self.tenant_id,
            user_id: self.user_id,
        })
    }
}

