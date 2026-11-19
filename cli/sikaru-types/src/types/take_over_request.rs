pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TakeOverRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab: Option<String>,
}

impl TakeOverRequest {
    pub fn builder() -> TakeOverRequestBuilder {
        <TakeOverRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TakeOverRequestBuilder {
    tab: Option<String>,
}

impl TakeOverRequestBuilder {
    pub fn tab(mut self, value: impl Into<String>) -> Self {
        self.tab = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TakeOverRequest`].
    pub fn build(self) -> Result<TakeOverRequest, BuildError> {
        Ok(TakeOverRequest {
            tab: self.tab,
        })
    }
}
