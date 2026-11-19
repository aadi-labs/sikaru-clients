pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MemoryDream {
    #[serde(rename = "dreamedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dreamed_at: Option<f64>,
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "requestedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub requested_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MemoryDreamStatus>,
}

impl MemoryDream {
    pub fn builder() -> MemoryDreamBuilder {
        <MemoryDreamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemoryDreamBuilder {
    dreamed_at: Option<f64>,
    job_id: Option<String>,
    requested_at: Option<f64>,
    status: Option<MemoryDreamStatus>,
}

impl MemoryDreamBuilder {
    pub fn dreamed_at(mut self, value: f64) -> Self {
        self.dreamed_at = Some(value);
        self
    }

    pub fn job_id(mut self, value: impl Into<String>) -> Self {
        self.job_id = Some(value.into());
        self
    }

    pub fn requested_at(mut self, value: f64) -> Self {
        self.requested_at = Some(value);
        self
    }

    pub fn status(mut self, value: MemoryDreamStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemoryDream`].
    pub fn build(self) -> Result<MemoryDream, BuildError> {
        Ok(MemoryDream {
            dreamed_at: self.dreamed_at,
            job_id: self.job_id,
            requested_at: self.requested_at,
            status: self.status,
        })
    }
}
