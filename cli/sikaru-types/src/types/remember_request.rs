pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Request for remember (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RememberRequest {
    /// The shared agent directory, or one end user's directory.
    #[serde(skip)]
    pub directory: Option<RememberMemoryRequestDirectory>,
    #[serde(skip)]
    pub environment: Option<RememberMemoryRequestEnvironment>,
    #[serde(rename = "tenantId")]
    #[serde(skip)]
    pub tenant_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub body: ForgetMemory,
}

impl RememberRequest {
    pub fn builder() -> RememberRequestBuilder {
        <RememberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RememberRequestBuilder {
    directory: Option<RememberMemoryRequestDirectory>,
    environment: Option<RememberMemoryRequestEnvironment>,
    tenant_id: Option<String>,
    user_id: Option<String>,
    body: Option<ForgetMemory>,
}

impl RememberRequestBuilder {
    pub fn directory(mut self, value: RememberMemoryRequestDirectory) -> Self {
        self.directory = Some(value);
        self
    }

    pub fn environment(mut self, value: RememberMemoryRequestEnvironment) -> Self {
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

    /// Consumes the builder and constructs a [`RememberRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](RememberRequestBuilder::body)
    pub fn build(self) -> Result<RememberRequest, BuildError> {
        Ok(RememberRequest {
            directory: self.directory,
            environment: self.environment,
            tenant_id: self.tenant_id,
            user_id: self.user_id,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}

