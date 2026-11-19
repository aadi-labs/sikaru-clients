pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BrowserRuleList {
    #[serde(default)]
    pub rules: Vec<BrowserRule>,
}

impl BrowserRuleList {
    pub fn builder() -> BrowserRuleListBuilder {
        <BrowserRuleListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrowserRuleListBuilder {
    rules: Option<Vec<BrowserRule>>,
}

impl BrowserRuleListBuilder {
    pub fn rules(mut self, value: Vec<BrowserRule>) -> Self {
        self.rules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BrowserRuleList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rules`](BrowserRuleListBuilder::rules)
    pub fn build(self) -> Result<BrowserRuleList, BuildError> {
        Ok(BrowserRuleList {
            rules: self.rules.ok_or_else(|| BuildError::missing_field("rules"))?,
        })
    }
}
