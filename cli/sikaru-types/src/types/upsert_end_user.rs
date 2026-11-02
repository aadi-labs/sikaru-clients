pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpsertEndUser {
    /// Opting out removes this user's contributed traces and eval cases from population material. Omit it to keep the current choice.
    #[serde(rename = "optIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in: Option<bool>,
    /// IANA timezone, such as Europe/Berlin. Explicit null clears it; omit it to keep the current value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl UpsertEndUser {
    pub fn builder() -> UpsertEndUserBuilder {
        <UpsertEndUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertEndUserBuilder {
    opt_in: Option<bool>,
    timezone: Option<String>,
}

impl UpsertEndUserBuilder {
    pub fn opt_in(mut self, value: bool) -> Self {
        self.opt_in = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpsertEndUser`].
    pub fn build(self) -> Result<UpsertEndUser, BuildError> {
        Ok(UpsertEndUser {
            opt_in: self.opt_in,
            timezone: self.timezone,
        })
    }
}

