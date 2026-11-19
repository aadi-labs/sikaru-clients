pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteQueryRequest {
    /// Permanently reject cell creation and new work for this end user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,
}

impl DeleteQueryRequest {
    pub fn builder() -> DeleteQueryRequestBuilder {
        <DeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteQueryRequestBuilder {
    terminal: Option<bool>,
}

impl DeleteQueryRequestBuilder {
    pub fn terminal(mut self, value: bool) -> Self {
        self.terminal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteQueryRequest`].
    pub fn build(self) -> Result<DeleteQueryRequest, BuildError> {
        Ok(DeleteQueryRequest {
            terminal: self.terminal,
        })
    }
}

