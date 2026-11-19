pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete_file
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteFileQueryRequest {
    /// Path inside the directory, such as MEMORY.md or notes/trip.md.
    #[serde(default)]
    pub path: String,
    /// The head the deletion was made from.
    #[serde(default)]
    pub base: String,
    /// The shared agent directory, or one end user's directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<DeleteFileMemoryRequestDirectory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<DeleteFileMemoryRequestEnvironment>,
    #[serde(rename = "tenantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl DeleteFileQueryRequest {
    pub fn builder() -> DeleteFileQueryRequestBuilder {
        <DeleteFileQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteFileQueryRequestBuilder {
    path: Option<String>,
    base: Option<String>,
    directory: Option<DeleteFileMemoryRequestDirectory>,
    environment: Option<DeleteFileMemoryRequestEnvironment>,
    tenant_id: Option<String>,
    user_id: Option<String>,
}

impl DeleteFileQueryRequestBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.base = Some(value.into());
        self
    }

    pub fn directory(mut self, value: DeleteFileMemoryRequestDirectory) -> Self {
        self.directory = Some(value);
        self
    }

    pub fn environment(mut self, value: DeleteFileMemoryRequestEnvironment) -> Self {
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

    /// Consumes the builder and constructs a [`DeleteFileQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](DeleteFileQueryRequestBuilder::path)
    /// - [`base`](DeleteFileQueryRequestBuilder::base)
    pub fn build(self) -> Result<DeleteFileQueryRequest, BuildError> {
        Ok(DeleteFileQueryRequest {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            base: self.base.ok_or_else(|| BuildError::missing_field("base"))?,
            directory: self.directory,
            environment: self.environment,
            tenant_id: self.tenant_id,
            user_id: self.user_id,
        })
    }
}

