pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Bar {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub agreement: f64,
    #[serde(rename = "heldOut")]
    #[serde(default)]
    pub held_out: i64,
}

impl Bar {
    pub fn builder() -> BarBuilder {
        <BarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BarBuilder {
    agreement: Option<f64>,
    held_out: Option<i64>,
}

impl BarBuilder {
    pub fn agreement(mut self, value: f64) -> Self {
        self.agreement = Some(value);
        self
    }

    pub fn held_out(mut self, value: i64) -> Self {
        self.held_out = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Bar`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agreement`](BarBuilder::agreement)
    /// - [`held_out`](BarBuilder::held_out)
    pub fn build(self) -> Result<Bar, BuildError> {
        Ok(Bar {
            agreement: self.agreement.ok_or_else(|| BuildError::missing_field("agreement"))?,
            held_out: self.held_out.ok_or_else(|| BuildError::missing_field("held_out"))?,
        })
    }
}
