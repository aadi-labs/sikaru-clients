pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Labeled {
    #[serde(rename = "conversationKey")]
    #[serde(default)]
    pub conversation_key: String,
    /// The Grader's own verdict, shown once the owner has answered.
    #[serde(rename = "graderVerdict")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grader_verdict: Option<LabeledGraderVerdict>,
    pub label: LabelView,
    #[serde(default)]
    pub labels: LabelCounts,
}

impl Labeled {
    pub fn builder() -> LabeledBuilder {
        <LabeledBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LabeledBuilder {
    conversation_key: Option<String>,
    grader_verdict: Option<LabeledGraderVerdict>,
    label: Option<LabelView>,
    labels: Option<LabelCounts>,
}

impl LabeledBuilder {
    pub fn conversation_key(mut self, value: impl Into<String>) -> Self {
        self.conversation_key = Some(value.into());
        self
    }

    pub fn grader_verdict(mut self, value: LabeledGraderVerdict) -> Self {
        self.grader_verdict = Some(value);
        self
    }

    pub fn label(mut self, value: LabelView) -> Self {
        self.label = Some(value);
        self
    }

    pub fn labels(mut self, value: LabelCounts) -> Self {
        self.labels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Labeled`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_key`](LabeledBuilder::conversation_key)
    /// - [`label`](LabeledBuilder::label)
    /// - [`labels`](LabeledBuilder::labels)
    pub fn build(self) -> Result<Labeled, BuildError> {
        Ok(Labeled {
            conversation_key: self.conversation_key.ok_or_else(|| BuildError::missing_field("conversation_key"))?,
            grader_verdict: self.grader_verdict,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
        })
    }
}
