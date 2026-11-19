pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Remembered {
    /// Finished forgets of this target that no longer filter search and dreams.
    #[serde(default)]
    pub reversed: i64,
}

impl Remembered {
    pub fn builder() -> RememberedBuilder {
        <RememberedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RememberedBuilder {
    reversed: Option<i64>,
}

impl RememberedBuilder {
    pub fn reversed(mut self, value: i64) -> Self {
        self.reversed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Remembered`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reversed`](RememberedBuilder::reversed)
    pub fn build(self) -> Result<Remembered, BuildError> {
        Ok(Remembered {
            reversed: self.reversed.ok_or_else(|| BuildError::missing_field("reversed"))?,
        })
    }
}
