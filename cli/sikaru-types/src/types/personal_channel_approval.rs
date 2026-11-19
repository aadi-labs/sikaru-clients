pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PersonalChannelApproval {
    pub approval: PersonalChannelApprovalDecision,
    /// Set when the decision also chose to always allow a browser commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<PersonalChannelApprovalRule>,
}

impl PersonalChannelApproval {
    pub fn builder() -> PersonalChannelApprovalBuilder {
        <PersonalChannelApprovalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonalChannelApprovalBuilder {
    approval: Option<PersonalChannelApprovalDecision>,
    rule: Option<PersonalChannelApprovalRule>,
}

impl PersonalChannelApprovalBuilder {
    pub fn approval(mut self, value: PersonalChannelApprovalDecision) -> Self {
        self.approval = Some(value);
        self
    }

    pub fn rule(mut self, value: PersonalChannelApprovalRule) -> Self {
        self.rule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PersonalChannelApproval`].
    /// This method will fail if any of the following fields are not set:
    /// - [`approval`](PersonalChannelApprovalBuilder::approval)
    pub fn build(self) -> Result<PersonalChannelApproval, BuildError> {
        Ok(PersonalChannelApproval {
            approval: self.approval.ok_or_else(|| BuildError::missing_field("approval"))?,
            rule: self.rule,
        })
    }
}
