pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReleaseCheckReceipt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#override: Option<ReleaseCheckOverride>,
    pub status: ReleaseCheckReceiptStatus,
}

impl ReleaseCheckReceipt {
    pub fn builder() -> ReleaseCheckReceiptBuilder {
        <ReleaseCheckReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleaseCheckReceiptBuilder {
    r#override: Option<ReleaseCheckOverride>,
    status: Option<ReleaseCheckReceiptStatus>,
}

impl ReleaseCheckReceiptBuilder {
    pub fn r#override(mut self, value: ReleaseCheckOverride) -> Self {
        self.r#override = Some(value);
        self
    }

    pub fn status(mut self, value: ReleaseCheckReceiptStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReleaseCheckReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ReleaseCheckReceiptBuilder::status)
    pub fn build(self) -> Result<ReleaseCheckReceipt, BuildError> {
        Ok(ReleaseCheckReceipt {
            r#override: self.r#override,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
