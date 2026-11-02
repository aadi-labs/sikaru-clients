pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EndUserDeletion {
    #[serde(rename = "jobIds")]
    #[serde(default)]
    pub job_ids: Vec<String>,
}

impl EndUserDeletion {
    pub fn builder() -> EndUserDeletionBuilder {
        <EndUserDeletionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EndUserDeletionBuilder {
    job_ids: Option<Vec<String>>,
}

impl EndUserDeletionBuilder {
    pub fn job_ids(mut self, value: Vec<String>) -> Self {
        self.job_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EndUserDeletion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`job_ids`](EndUserDeletionBuilder::job_ids)
    pub fn build(self) -> Result<EndUserDeletion, BuildError> {
        Ok(EndUserDeletion {
            job_ids: self.job_ids.ok_or_else(|| BuildError::missing_field("job_ids"))?,
        })
    }
}
