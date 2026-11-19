pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Receipt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<RunComparison>,
    #[serde(default)]
    pub lines: Vec<ReceiptLine>,
}

impl Receipt {
    pub fn builder() -> ReceiptBuilder {
        <ReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptBuilder {
    comparison: Option<RunComparison>,
    lines: Option<Vec<ReceiptLine>>,
}

impl ReceiptBuilder {
    pub fn comparison(mut self, value: RunComparison) -> Self {
        self.comparison = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<ReceiptLine>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Receipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lines`](ReceiptBuilder::lines)
    pub fn build(self) -> Result<Receipt, BuildError> {
        Ok(Receipt {
            comparison: self.comparison,
            lines: self.lines.ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
