pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete_schedule
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteScheduleQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

impl DeleteScheduleQueryRequest {
    pub fn builder() -> DeleteScheduleQueryRequestBuilder {
        <DeleteScheduleQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteScheduleQueryRequestBuilder {
    expected_revision: Option<i64>,
    idempotency_key: Option<String>,
}

impl DeleteScheduleQueryRequestBuilder {
    pub fn expected_revision(mut self, value: i64) -> Self {
        self.expected_revision = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteScheduleQueryRequest`].
    pub fn build(self) -> Result<DeleteScheduleQueryRequest, BuildError> {
        Ok(DeleteScheduleQueryRequest {
            expected_revision: self.expected_revision,
            idempotency_key: self.idempotency_key,
        })
    }
}

