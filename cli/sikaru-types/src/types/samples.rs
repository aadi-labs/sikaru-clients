pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Samples {
    #[serde(default)]
    pub samples: Vec<Sample>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl Samples {
    pub fn builder() -> SamplesBuilder {
        <SamplesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SamplesBuilder {
    samples: Option<Vec<Sample>>,
    version: Option<i64>,
}

impl SamplesBuilder {
    pub fn samples(mut self, value: Vec<Sample>) -> Self {
        self.samples = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Samples`].
    /// This method will fail if any of the following fields are not set:
    /// - [`samples`](SamplesBuilder::samples)
    pub fn build(self) -> Result<Samples, BuildError> {
        Ok(Samples {
            samples: self.samples.ok_or_else(|| BuildError::missing_field("samples"))?,
            version: self.version,
        })
    }
}
