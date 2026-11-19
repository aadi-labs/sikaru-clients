pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SavedLogins {
    #[serde(default)]
    pub logins: Vec<SavedLogin>,
}

impl SavedLogins {
    pub fn builder() -> SavedLoginsBuilder {
        <SavedLoginsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SavedLoginsBuilder {
    logins: Option<Vec<SavedLogin>>,
}

impl SavedLoginsBuilder {
    pub fn logins(mut self, value: Vec<SavedLogin>) -> Self {
        self.logins = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SavedLogins`].
    /// This method will fail if any of the following fields are not set:
    /// - [`logins`](SavedLoginsBuilder::logins)
    pub fn build(self) -> Result<SavedLogins, BuildError> {
        Ok(SavedLogins {
            logins: self.logins.ok_or_else(|| BuildError::missing_field("logins"))?,
        })
    }
}
