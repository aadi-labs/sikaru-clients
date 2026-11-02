pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The latest snapshot of one output message. A newer snapshot of the same messageId replaces it; a terminal snapshot is the saved result or marks an unsaved partial failed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OutputPreview {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<i64>,
    /// Set for code execution output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<OutputPreviewKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<OutputPreviewLanguage>,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    #[serde(default)]
    pub revision: String,
    pub source: OutputPreviewSource,
    pub state: OutputPreviewState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,
    #[serde(default)]
    pub text: String,
    #[serde(rename = "toolCalls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<OutputPreviewToolCallsItem>>,
}

impl OutputPreview {
    pub fn builder() -> OutputPreviewBuilder {
        <OutputPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutputPreviewBuilder {
    epoch: Option<i64>,
    kind: Option<OutputPreviewKind>,
    language: Option<OutputPreviewLanguage>,
    message_id: Option<String>,
    reasoning: Option<String>,
    revision: Option<String>,
    source: Option<OutputPreviewSource>,
    state: Option<OutputPreviewState>,
    terminal: Option<bool>,
    text: Option<String>,
    tool_calls: Option<Vec<OutputPreviewToolCallsItem>>,
}

impl OutputPreviewBuilder {
    pub fn epoch(mut self, value: i64) -> Self {
        self.epoch = Some(value);
        self
    }

    pub fn kind(mut self, value: OutputPreviewKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn language(mut self, value: OutputPreviewLanguage) -> Self {
        self.language = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn reasoning(mut self, value: impl Into<String>) -> Self {
        self.reasoning = Some(value.into());
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn source(mut self, value: OutputPreviewSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn state(mut self, value: OutputPreviewState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn terminal(mut self, value: bool) -> Self {
        self.terminal = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn tool_calls(mut self, value: Vec<OutputPreviewToolCallsItem>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OutputPreview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](OutputPreviewBuilder::message_id)
    /// - [`revision`](OutputPreviewBuilder::revision)
    /// - [`source`](OutputPreviewBuilder::source)
    /// - [`state`](OutputPreviewBuilder::state)
    /// - [`text`](OutputPreviewBuilder::text)
    pub fn build(self) -> Result<OutputPreview, BuildError> {
        Ok(OutputPreview {
            epoch: self.epoch,
            kind: self.kind,
            language: self.language,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            reasoning: self.reasoning,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            terminal: self.terminal,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            tool_calls: self.tool_calls,
        })
    }
}
