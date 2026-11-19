pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LabelCounts {
    #[serde(default)]
    pub flagged: i64,
    #[serde(rename = "looksFine")]
    #[serde(default)]
    pub looks_fine: i64,
    #[serde(default)]
    pub total: i64,
}

impl LabelCounts {
    pub fn builder() -> LabelCountsBuilder {
        <LabelCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LabelCountsBuilder {
    flagged: Option<i64>,
    looks_fine: Option<i64>,
    total: Option<i64>,
}

impl LabelCountsBuilder {
    pub fn flagged(mut self, value: i64) -> Self {
        self.flagged = Some(value);
        self
    }

    pub fn looks_fine(mut self, value: i64) -> Self {
        self.looks_fine = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LabelCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`flagged`](LabelCountsBuilder::flagged)
    /// - [`looks_fine`](LabelCountsBuilder::looks_fine)
    /// - [`total`](LabelCountsBuilder::total)
    pub fn build(self) -> Result<LabelCounts, BuildError> {
        Ok(LabelCounts {
            flagged: self.flagged.ok_or_else(|| BuildError::missing_field("flagged"))?,
            looks_fine: self.looks_fine.ok_or_else(|| BuildError::missing_field("looks_fine"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
