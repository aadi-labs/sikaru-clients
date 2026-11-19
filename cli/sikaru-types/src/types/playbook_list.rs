pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlaybookList {
    #[serde(default)]
    pub playbooks: Vec<Playbook>,
}

impl PlaybookList {
    pub fn builder() -> PlaybookListBuilder {
        <PlaybookListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlaybookListBuilder {
    playbooks: Option<Vec<Playbook>>,
}

impl PlaybookListBuilder {
    pub fn playbooks(mut self, value: Vec<Playbook>) -> Self {
        self.playbooks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlaybookList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`playbooks`](PlaybookListBuilder::playbooks)
    pub fn build(self) -> Result<PlaybookList, BuildError> {
        Ok(PlaybookList {
            playbooks: self.playbooks.ok_or_else(|| BuildError::missing_field("playbooks"))?,
        })
    }
}
