pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The model lab's recommended sampling; a request that omits a value runs at it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SamplingDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}

impl SamplingDefaults {
    pub fn builder() -> SamplingDefaultsBuilder {
        <SamplingDefaultsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SamplingDefaultsBuilder {
    min_p: Option<f64>,
    temperature: Option<f64>,
    top_k: Option<i64>,
    top_p: Option<f64>,
}

impl SamplingDefaultsBuilder {
    pub fn min_p(mut self, value: f64) -> Self {
        self.min_p = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    pub fn top_p(mut self, value: f64) -> Self {
        self.top_p = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SamplingDefaults`].
    pub fn build(self) -> Result<SamplingDefaults, BuildError> {
        Ok(SamplingDefaults {
            min_p: self.min_p,
            temperature: self.temperature,
            top_k: self.top_k,
            top_p: self.top_p,
        })
    }
}
