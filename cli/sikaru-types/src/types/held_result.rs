pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HeldResult {
    /// Up to 10 conversation keys flagged only for this version.
    #[serde(default)]
    pub flagged: Vec<String>,
    #[serde(rename = "flaggedCount")]
    #[serde(default)]
    pub flagged_count: i64,
    #[serde(rename = "graderId")]
    #[serde(default)]
    pub grader_id: String,
    /// Up to 10 conversation keys whose replay was inconclusive on either side: neither flagged nor fine.
    #[serde(default)]
    pub inconclusive: Vec<String>,
    #[serde(rename = "inconclusiveCount")]
    #[serde(default)]
    pub inconclusive_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub outcome: HeldResultOutcome,
}

impl HeldResult {
    pub fn builder() -> HeldResultBuilder {
        <HeldResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HeldResultBuilder {
    flagged: Option<Vec<String>>,
    flagged_count: Option<i64>,
    grader_id: Option<String>,
    inconclusive: Option<Vec<String>>,
    inconclusive_count: Option<i64>,
    name: Option<String>,
    outcome: Option<HeldResultOutcome>,
}

impl HeldResultBuilder {
    pub fn flagged(mut self, value: Vec<String>) -> Self {
        self.flagged = Some(value);
        self
    }

    pub fn flagged_count(mut self, value: i64) -> Self {
        self.flagged_count = Some(value);
        self
    }

    pub fn grader_id(mut self, value: impl Into<String>) -> Self {
        self.grader_id = Some(value.into());
        self
    }

    pub fn inconclusive(mut self, value: Vec<String>) -> Self {
        self.inconclusive = Some(value);
        self
    }

    pub fn inconclusive_count(mut self, value: i64) -> Self {
        self.inconclusive_count = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn outcome(mut self, value: HeldResultOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HeldResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`flagged`](HeldResultBuilder::flagged)
    /// - [`flagged_count`](HeldResultBuilder::flagged_count)
    /// - [`grader_id`](HeldResultBuilder::grader_id)
    /// - [`inconclusive`](HeldResultBuilder::inconclusive)
    /// - [`inconclusive_count`](HeldResultBuilder::inconclusive_count)
    /// - [`outcome`](HeldResultBuilder::outcome)
    pub fn build(self) -> Result<HeldResult, BuildError> {
        Ok(HeldResult {
            flagged: self.flagged.ok_or_else(|| BuildError::missing_field("flagged"))?,
            flagged_count: self.flagged_count.ok_or_else(|| BuildError::missing_field("flagged_count"))?,
            grader_id: self.grader_id.ok_or_else(|| BuildError::missing_field("grader_id"))?,
            inconclusive: self.inconclusive.ok_or_else(|| BuildError::missing_field("inconclusive"))?,
            inconclusive_count: self.inconclusive_count.ok_or_else(|| BuildError::missing_field("inconclusive_count"))?,
            name: self.name,
            outcome: self.outcome.ok_or_else(|| BuildError::missing_field("outcome"))?,
        })
    }
}
