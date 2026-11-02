pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecordedCorrection {
    #[serde(rename = "caseId")]
    #[serde(default)]
    pub case_id: String,
}

impl RecordedCorrection {
    pub fn builder() -> RecordedCorrectionBuilder {
        <RecordedCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordedCorrectionBuilder {
    case_id: Option<String>,
}

impl RecordedCorrectionBuilder {
    pub fn case_id(mut self, value: impl Into<String>) -> Self {
        self.case_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecordedCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`case_id`](RecordedCorrectionBuilder::case_id)
    pub fn build(self) -> Result<RecordedCorrection, BuildError> {
        Ok(RecordedCorrection {
            case_id: self.case_id.ok_or_else(|| BuildError::missing_field("case_id"))?,
        })
    }
}
