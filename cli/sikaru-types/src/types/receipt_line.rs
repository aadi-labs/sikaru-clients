pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceiptLine {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub at: String,
    #[serde(default)]
    pub by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
}

impl ReceiptLine {
    pub fn builder() -> ReceiptLineBuilder {
        <ReceiptLineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptLineBuilder {
    action: Option<String>,
    at: Option<String>,
    by: Option<String>,
    detail: Option<HashMap<String, serde_json::Value>>,
    owner: Option<String>,
    page: Option<String>,
    screenshot: Option<String>,
    site: Option<String>,
}

impl ReceiptLineBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn at(mut self, value: impl Into<String>) -> Self {
        self.at = Some(value.into());
        self
    }

    pub fn by(mut self, value: impl Into<String>) -> Self {
        self.by = Some(value.into());
        self
    }

    pub fn detail(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.detail = Some(value);
        self
    }

    pub fn owner(mut self, value: impl Into<String>) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn page(mut self, value: impl Into<String>) -> Self {
        self.page = Some(value.into());
        self
    }

    pub fn screenshot(mut self, value: impl Into<String>) -> Self {
        self.screenshot = Some(value.into());
        self
    }

    pub fn site(mut self, value: impl Into<String>) -> Self {
        self.site = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceiptLine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](ReceiptLineBuilder::action)
    /// - [`at`](ReceiptLineBuilder::at)
    /// - [`by`](ReceiptLineBuilder::by)
    pub fn build(self) -> Result<ReceiptLine, BuildError> {
        Ok(ReceiptLine {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            by: self.by.ok_or_else(|| BuildError::missing_field("by"))?,
            detail: self.detail,
            owner: self.owner,
            page: self.page,
            screenshot: self.screenshot,
            site: self.site,
        })
    }
}
