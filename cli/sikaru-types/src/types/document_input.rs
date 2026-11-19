pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<HashMap<String, Option<String>>>,
}

impl DocumentInput {
    pub fn builder() -> DocumentInputBuilder {
        <DocumentInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentInputBuilder {
    document: Option<String>,
    files: Option<HashMap<String, Option<String>>>,
}

impl DocumentInputBuilder {
    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
        self
    }

    pub fn files(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentInput`].
    pub fn build(self) -> Result<DocumentInput, BuildError> {
        Ok(DocumentInput {
            document: self.document,
            files: self.files,
        })
    }
}
