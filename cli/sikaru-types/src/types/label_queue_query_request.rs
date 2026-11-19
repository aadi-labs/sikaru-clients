pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for label_queue
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LabelQueueQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl LabelQueueQueryRequest {
    pub fn builder() -> LabelQueueQueryRequestBuilder {
        <LabelQueueQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LabelQueueQueryRequestBuilder {
    limit: Option<i64>,
}

impl LabelQueueQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LabelQueueQueryRequest`].
    pub fn build(self) -> Result<LabelQueueQueryRequest, BuildError> {
        Ok(LabelQueueQueryRequest {
            limit: self.limit,
        })
    }
}

