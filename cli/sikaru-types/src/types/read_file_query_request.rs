pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for read_file
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadFileQueryRequest {
    /// Path inside the directory, such as MEMORY.md or notes/trip.md.
    #[serde(default)]
    pub path: String,
    /// The shared agent directory, or one end user's directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<ReadFileMemoryRequestDirectory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ReadFileMemoryRequestEnvironment>,
    #[serde(rename = "tenantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ReadFileQueryRequest {
    pub fn builder() -> ReadFileQueryRequestBuilder {
        <ReadFileQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadFileQueryRequestBuilder {
    path: Option<String>,
    directory: Option<ReadFileMemoryRequestDirectory>,
    environment: Option<ReadFileMemoryRequestEnvironment>,
    tenant_id: Option<String>,
    user_id: Option<String>,
}

impl ReadFileQueryRequestBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn directory(mut self, value: ReadFileMemoryRequestDirectory) -> Self {
        self.directory = Some(value);
        self
    }

    pub fn environment(mut self, value: ReadFileMemoryRequestEnvironment) -> Self {
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

    /// Consumes the builder and constructs a [`ReadFileQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](ReadFileQueryRequestBuilder::path)
    pub fn build(self) -> Result<ReadFileQueryRequest, BuildError> {
        Ok(ReadFileQueryRequest {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            directory: self.directory,
            environment: self.environment,
            tenant_id: self.tenant_id,
            user_id: self.user_id,
        })
    }
}

