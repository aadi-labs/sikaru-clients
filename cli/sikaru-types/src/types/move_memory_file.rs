pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MoveMemoryFile {
    /// The head this change was made from; 409 if another writer changed the same file since.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub to: String,
    /// The shared agent directory, or one end user's directory.
    #[serde(skip)]
    pub directory: Option<MoveFileMemoryRequestDirectory>,
    #[serde(skip)]
    pub environment: Option<MoveFileMemoryRequestEnvironment>,
    #[serde(rename = "tenantId")]
    #[serde(skip)]
    pub tenant_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip)]
    pub user_id: Option<String>,
}

impl MoveMemoryFile {
    pub fn builder() -> MoveMemoryFileBuilder {
        <MoveMemoryFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MoveMemoryFileBuilder {
    base: Option<String>,
    path: Option<String>,
    to: Option<String>,
    directory: Option<MoveFileMemoryRequestDirectory>,
    environment: Option<MoveFileMemoryRequestEnvironment>,
    tenant_id: Option<String>,
    user_id: Option<String>,
}

impl MoveMemoryFileBuilder {
    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.base = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn directory(mut self, value: MoveFileMemoryRequestDirectory) -> Self {
        self.directory = Some(value);
        self
    }

    pub fn environment(mut self, value: MoveFileMemoryRequestEnvironment) -> Self {
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

    /// Consumes the builder and constructs a [`MoveMemoryFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](MoveMemoryFileBuilder::path)
    /// - [`to`](MoveMemoryFileBuilder::to)
    pub fn build(self) -> Result<MoveMemoryFile, BuildError> {
        Ok(MoveMemoryFile {
            base: self.base,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            directory: self.directory,
            environment: self.environment,
            tenant_id: self.tenant_id,
            user_id: self.user_id,
        })
    }
}

