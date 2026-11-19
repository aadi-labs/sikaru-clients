pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScheduleDeleted {
    #[serde(default)]
    pub deleted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}

impl ScheduleDeleted {
    pub fn builder() -> ScheduleDeletedBuilder {
        <ScheduleDeletedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleDeletedBuilder {
    deleted: Option<bool>,
    revision: Option<i64>,
}

impl ScheduleDeletedBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleDeleted`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](ScheduleDeletedBuilder::deleted)
    pub fn build(self) -> Result<ScheduleDeleted, BuildError> {
        Ok(ScheduleDeleted {
            deleted: self.deleted.ok_or_else(|| BuildError::missing_field("deleted"))?,
            revision: self.revision,
        })
    }
}
