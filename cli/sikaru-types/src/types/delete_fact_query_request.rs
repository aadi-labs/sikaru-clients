pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete_fact
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteFactQueryRequest {
    /// USER.md or memories/<name>.md.
    #[serde(default)]
    pub path: String,
}

impl DeleteFactQueryRequest {
    pub fn builder() -> DeleteFactQueryRequestBuilder {
        <DeleteFactQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteFactQueryRequestBuilder {
    path: Option<String>,
}

impl DeleteFactQueryRequestBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteFactQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](DeleteFactQueryRequestBuilder::path)
    pub fn build(self) -> Result<DeleteFactQueryRequest, BuildError> {
        Ok(DeleteFactQueryRequest {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}

