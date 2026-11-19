pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WriteMemoryFile {
    /// The head this change was made from; 409 if another writer changed the same file since.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub path: String,
    /// The shared agent directory, or one end user's directory.
    #[serde(skip)]
    pub directory: Option<WriteFileMemoryRequestDirectory>,
    #[serde(skip)]
    pub environment: Option<WriteFileMemoryRequestEnvironment>,
    #[serde(rename = "tenantId")]
    #[serde(skip)]
    pub tenant_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip)]
    pub user_id: Option<String>,
}

impl WriteMemoryFile {
    pub fn builder() -> WriteMemoryFileBuilder {
        <WriteMemoryFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WriteMemoryFileBuilder {
    base: Option<String>,
    content: Option<String>,
    path: Option<String>,
    directory: Option<WriteFileMemoryRequestDirectory>,
    environment: Option<WriteFileMemoryRequestEnvironment>,
    tenant_id: Option<String>,
    user_id: Option<String>,
}

impl WriteMemoryFileBuilder {
    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.base = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn directory(mut self, value: WriteFileMemoryRequestDirectory) -> Self {
        self.directory = Some(value);
        self
    }

    pub fn environment(mut self, value: WriteFileMemoryRequestEnvironment) -> Self {
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

    /// Consumes the builder and constructs a [`WriteMemoryFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](WriteMemoryFileBuilder::content)
    /// - [`path`](WriteMemoryFileBuilder::path)
    pub fn build(self) -> Result<WriteMemoryFile, BuildError> {
        Ok(WriteMemoryFile {
            base: self.base,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            directory: self.directory,
            environment: self.environment,
            tenant_id: self.tenant_id,
            user_id: self.user_id,
        })
    }
}

