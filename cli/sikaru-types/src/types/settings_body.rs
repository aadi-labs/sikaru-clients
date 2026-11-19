pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SettingsBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub judge: Option<JudgeChoice>,
    /// Why a check that blocks a release is turned off or loosened; required then, and recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "releaseCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_check: Option<CheckChoice>,
}

impl SettingsBody {
    pub fn builder() -> SettingsBodyBuilder {
        <SettingsBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsBodyBuilder {
    description: Option<String>,
    escalate: Option<bool>,
    judge: Option<JudgeChoice>,
    reason: Option<String>,
    release_check: Option<CheckChoice>,
}

impl SettingsBodyBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn escalate(mut self, value: bool) -> Self {
        self.escalate = Some(value);
        self
    }

    pub fn judge(mut self, value: JudgeChoice) -> Self {
        self.judge = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn release_check(mut self, value: CheckChoice) -> Self {
        self.release_check = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsBody`].
    pub fn build(self) -> Result<SettingsBody, BuildError> {
        Ok(SettingsBody {
            description: self.description,
            escalate: self.escalate,
            judge: self.judge,
            reason: self.reason,
            release_check: self.release_check,
        })
    }
}

