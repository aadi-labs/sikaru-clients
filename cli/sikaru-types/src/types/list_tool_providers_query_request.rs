pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_tool_providers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListToolProvidersQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListToolProvidersToolProvidersRequestStatus>,
}

impl ListToolProvidersQueryRequest {
    pub fn builder() -> ListToolProvidersQueryRequestBuilder {
        <ListToolProvidersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListToolProvidersQueryRequestBuilder {
    capability_prefix: Option<String>,
    status: Option<ListToolProvidersToolProvidersRequestStatus>,
}

impl ListToolProvidersQueryRequestBuilder {
    pub fn capability_prefix(mut self, value: impl Into<String>) -> Self {
        self.capability_prefix = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListToolProvidersToolProvidersRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListToolProvidersQueryRequest`].
    pub fn build(self) -> Result<ListToolProvidersQueryRequest, BuildError> {
        Ok(ListToolProvidersQueryRequest {
            capability_prefix: self.capability_prefix,
            status: self.status,
        })
    }
}

