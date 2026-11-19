pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraderVersionView {
    /// Held-out agreement with the owner's labels, 0 to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub agreement: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Labels the agreement was measured on.
    #[serde(rename = "heldOut")]
    #[serde(default)]
    pub held_out: i64,
    #[serde(rename = "labelCount")]
    #[serde(default)]
    pub label_count: i64,
    pub status: GraderVersionViewStatus,
    #[serde(default)]
    pub version: i64,
}

impl GraderVersionView {
    pub fn builder() -> GraderVersionViewBuilder {
        <GraderVersionViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraderVersionViewBuilder {
    agreement: Option<f64>,
    created_at: Option<String>,
    held_out: Option<i64>,
    label_count: Option<i64>,
    status: Option<GraderVersionViewStatus>,
    version: Option<i64>,
}

impl GraderVersionViewBuilder {
    pub fn agreement(mut self, value: f64) -> Self {
        self.agreement = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn held_out(mut self, value: i64) -> Self {
        self.held_out = Some(value);
        self
    }

    pub fn label_count(mut self, value: i64) -> Self {
        self.label_count = Some(value);
        self
    }

    pub fn status(mut self, value: GraderVersionViewStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraderVersionView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`held_out`](GraderVersionViewBuilder::held_out)
    /// - [`label_count`](GraderVersionViewBuilder::label_count)
    /// - [`status`](GraderVersionViewBuilder::status)
    /// - [`version`](GraderVersionViewBuilder::version)
    pub fn build(self) -> Result<GraderVersionView, BuildError> {
        Ok(GraderVersionView {
            agreement: self.agreement,
            created_at: self.created_at,
            held_out: self.held_out.ok_or_else(|| BuildError::missing_field("held_out"))?,
            label_count: self.label_count.ok_or_else(|| BuildError::missing_field("label_count"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
