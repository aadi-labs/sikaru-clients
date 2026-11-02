pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PersonaSummaryCorrection {
    /// What the summary gets wrong, in the user's words.
    #[serde(default)]
    pub text: String,
}

impl PersonaSummaryCorrection {
    pub fn builder() -> PersonaSummaryCorrectionBuilder {
        <PersonaSummaryCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonaSummaryCorrectionBuilder {
    text: Option<String>,
}

impl PersonaSummaryCorrectionBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PersonaSummaryCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](PersonaSummaryCorrectionBuilder::text)
    pub fn build(self) -> Result<PersonaSummaryCorrection, BuildError> {
        Ok(PersonaSummaryCorrection {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}

