pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Request for forget (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForgetRequest {
    /// The shared agent directory, or one end user's directory.
    #[serde(skip)]
    pub directory: Option<ForgetMemoryRequestDirectory>,
    #[serde(skip)]
    pub environment: Option<ForgetMemoryRequestEnvironment>,
    #[serde(rename = "tenantId")]
    #[serde(skip)]
    pub tenant_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub body: ForgetMemory,
}

impl ForgetRequest {
    pub fn builder() -> ForgetRequestBuilder {
        <ForgetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForgetRequestBuilder {
    directory: Option<ForgetMemoryRequestDirectory>,
    environment: Option<ForgetMemoryRequestEnvironment>,
    tenant_id: Option<String>,
    user_id: Option<String>,
    body: Option<ForgetMemory>,
}

impl ForgetRequestBuilder {
    pub fn directory(mut self, value: ForgetMemoryRequestDirectory) -> Self {
        self.directory = Some(value);
        self
    }

    pub fn environment(mut self, value: ForgetMemoryRequestEnvironment) -> Self {
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

    pub fn body(mut self, value: ForgetMemory) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForgetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](ForgetRequestBuilder::body)
    pub fn build(self) -> Result<ForgetRequest, BuildError> {
        Ok(ForgetRequest {
            directory: self.directory,
            environment: self.environment,
            tenant_id: self.tenant_id,
            user_id: self.user_id,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}

