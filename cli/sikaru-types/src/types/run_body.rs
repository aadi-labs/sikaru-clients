pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}

impl RunBody {
    pub fn builder() -> RunBodyBuilder {
        <RunBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunBodyBuilder {
    days: Option<i64>,
}

impl RunBodyBuilder {
    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunBody`].
    pub fn build(self) -> Result<RunBody, BuildError> {
        Ok(RunBody {
            days: self.days,
        })
    }
}

