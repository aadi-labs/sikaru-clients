pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueueItem {
    #[serde(rename = "conversationKey")]
    #[serde(default)]
    pub conversation_key: String,
    /// The conversation's last steps; the verdict is not shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<Vec<ExcerptStep>>,
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

impl QueueItem {
    pub fn builder() -> QueueItemBuilder {
        <QueueItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueueItemBuilder {
    conversation_key: Option<String>,
    excerpt: Option<Vec<ExcerptStep>>,
    run_id: Option<String>,
}

impl QueueItemBuilder {
    pub fn conversation_key(mut self, value: impl Into<String>) -> Self {
        self.conversation_key = Some(value.into());
        self
    }

    pub fn excerpt(mut self, value: Vec<ExcerptStep>) -> Self {
        self.excerpt = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueueItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_key`](QueueItemBuilder::conversation_key)
    pub fn build(self) -> Result<QueueItem, BuildError> {
        Ok(QueueItem {
            conversation_key: self.conversation_key.ok_or_else(|| BuildError::missing_field("conversation_key"))?,
            excerpt: self.excerpt,
            run_id: self.run_id,
        })
    }
}
