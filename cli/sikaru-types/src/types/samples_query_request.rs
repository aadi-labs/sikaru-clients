pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for samples
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SamplesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl SamplesQueryRequest {
    pub fn builder() -> SamplesQueryRequestBuilder {
        <SamplesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SamplesQueryRequestBuilder {
    limit: Option<i64>,
}

impl SamplesQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SamplesQueryRequest`].
    pub fn build(self) -> Result<SamplesQueryRequest, BuildError> {
        Ok(SamplesQueryRequest {
            limit: self.limit,
        })
    }
}

