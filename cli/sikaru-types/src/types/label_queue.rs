pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LabelQueue {
    #[serde(default)]
    pub items: Vec<QueueItem>,
    #[serde(default)]
    pub labels: LabelCounts,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl LabelQueue {
    pub fn builder() -> LabelQueueBuilder {
        <LabelQueueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LabelQueueBuilder {
    items: Option<Vec<QueueItem>>,
    labels: Option<LabelCounts>,
    version: Option<i64>,
}

impl LabelQueueBuilder {
    pub fn items(mut self, value: Vec<QueueItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn labels(mut self, value: LabelCounts) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LabelQueue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](LabelQueueBuilder::items)
    /// - [`labels`](LabelQueueBuilder::labels)
    pub fn build(self) -> Result<LabelQueue, BuildError> {
        Ok(LabelQueue {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
            version: self.version,
        })
    }
}
