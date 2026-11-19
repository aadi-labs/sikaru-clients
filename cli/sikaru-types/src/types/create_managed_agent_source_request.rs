pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateManagedAgentSourceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<HashMap<String, Option<String>>>,
    #[serde(rename = "sourceKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_kind: Option<CreateManagedAgentSourceRequestSourceKind>,
}

impl CreateManagedAgentSourceRequest {
    pub fn builder() -> CreateManagedAgentSourceRequestBuilder {
        <CreateManagedAgentSourceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateManagedAgentSourceRequestBuilder {
    document: Option<String>,
    files: Option<HashMap<String, Option<String>>>,
    source_kind: Option<CreateManagedAgentSourceRequestSourceKind>,
}

impl CreateManagedAgentSourceRequestBuilder {
    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
        self
    }

    pub fn files(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn source_kind(mut self, value: CreateManagedAgentSourceRequestSourceKind) -> Self {
        self.source_kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateManagedAgentSourceRequest`].
    pub fn build(self) -> Result<CreateManagedAgentSourceRequest, BuildError> {
        Ok(CreateManagedAgentSourceRequest {
            document: self.document,
            files: self.files,
            source_kind: self.source_kind,
        })
    }
}
