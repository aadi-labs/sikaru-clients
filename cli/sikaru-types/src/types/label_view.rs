pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LabelView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub verdict: LabelViewVerdict,
}

impl LabelView {
    pub fn builder() -> LabelViewBuilder {
        <LabelViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LabelViewBuilder {
    reason: Option<String>,
    verdict: Option<LabelViewVerdict>,
}

impl LabelViewBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn verdict(mut self, value: LabelViewVerdict) -> Self {
        self.verdict = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LabelView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`verdict`](LabelViewBuilder::verdict)
    pub fn build(self) -> Result<LabelView, BuildError> {
        Ok(LabelView {
            reason: self.reason,
            verdict: self.verdict.ok_or_else(|| BuildError::missing_field("verdict"))?,
        })
    }
}
