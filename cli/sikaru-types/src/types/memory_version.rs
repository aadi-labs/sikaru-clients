pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MemoryVersion {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(default)]
    pub head: String,
    /// The conversation or memory run that produced it; null for a builder edit.
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(default)]
    pub version: i64,
}

impl MemoryVersion {
    pub fn builder() -> MemoryVersionBuilder {
        <MemoryVersionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemoryVersionBuilder {
    created_at: Option<f64>,
    head: Option<String>,
    run_id: Option<String>,
    version: Option<i64>,
}

impl MemoryVersionBuilder {
    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn head(mut self, value: impl Into<String>) -> Self {
        self.head = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemoryVersion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](MemoryVersionBuilder::created_at)
    /// - [`head`](MemoryVersionBuilder::head)
    /// - [`version`](MemoryVersionBuilder::version)
    pub fn build(self) -> Result<MemoryVersion, BuildError> {
        Ok(MemoryVersion {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            head: self.head.ok_or_else(|| BuildError::missing_field("head"))?,
            run_id: self.run_id,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
