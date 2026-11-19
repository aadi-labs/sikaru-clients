pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BrowserRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub control: String,
    #[serde(rename = "grantedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granted_by: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lastUsedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playbook: Option<String>,
    pub scope: BrowserRuleScope,
    #[serde(default)]
    pub site: String,
    pub state: BrowserRuleState,
    #[serde(default)]
    pub step: i64,
    #[serde(default)]
    pub uses: i64,
}

impl BrowserRule {
    pub fn builder() -> BrowserRuleBuilder {
        <BrowserRuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserRuleBuilder {
    cap: Option<HashMap<String, serde_json::Value>>,
    control: Option<String>,
    granted_by: Option<String>,
    id: Option<String>,
    last_used_at: Option<String>,
    playbook: Option<String>,
    scope: Option<BrowserRuleScope>,
    site: Option<String>,
    state: Option<BrowserRuleState>,
    step: Option<i64>,
    uses: Option<i64>,
}

impl BrowserRuleBuilder {
    pub fn cap(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.cap = Some(value);
        self
    }

    pub fn control(mut self, value: impl Into<String>) -> Self {
        self.control = Some(value.into());
        self
    }

    pub fn granted_by(mut self, value: impl Into<String>) -> Self {
        self.granted_by = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_used_at(mut self, value: impl Into<String>) -> Self {
        self.last_used_at = Some(value.into());
        self
    }

    pub fn playbook(mut self, value: impl Into<String>) -> Self {
        self.playbook = Some(value.into());
        self
    }

    pub fn scope(mut self, value: BrowserRuleScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn site(mut self, value: impl Into<String>) -> Self {
        self.site = Some(value.into());
        self
    }

    pub fn state(mut self, value: BrowserRuleState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn step(mut self, value: i64) -> Self {
        self.step = Some(value);
        self
    }

    pub fn uses(mut self, value: i64) -> Self {
        self.uses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrowserRule`].
    /// This method will fail if any of the following fields are not set:
    /// - [`control`](BrowserRuleBuilder::control)
    /// - [`id`](BrowserRuleBuilder::id)
    /// - [`scope`](BrowserRuleBuilder::scope)
    /// - [`site`](BrowserRuleBuilder::site)
    /// - [`state`](BrowserRuleBuilder::state)
    /// - [`step`](BrowserRuleBuilder::step)
    /// - [`uses`](BrowserRuleBuilder::uses)
    pub fn build(self) -> Result<BrowserRule, BuildError> {
        Ok(BrowserRule {
            cap: self.cap,
            control: self.control.ok_or_else(|| BuildError::missing_field("control"))?,
            granted_by: self.granted_by,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_used_at: self.last_used_at,
            playbook: self.playbook,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            site: self.site.ok_or_else(|| BuildError::missing_field("site"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            step: self.step.ok_or_else(|| BuildError::missing_field("step"))?,
            uses: self.uses.ok_or_else(|| BuildError::missing_field("uses"))?,
        })
    }
}
