pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportReceipt {
    #[serde(default)]
    pub digest: String,
    #[serde(rename = "evaluationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_status: Option<ImportReceiptEvaluationStatus>,
    #[serde(rename = "importId")]
    #[serde(default)]
    pub import_id: String,
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(rename = "sourceRevision")]
    #[serde(default)]
    pub source_revision: String,
}

impl ImportReceipt {
    pub fn builder() -> ImportReceiptBuilder {
        <ImportReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportReceiptBuilder {
    digest: Option<String>,
    evaluation_status: Option<ImportReceiptEvaluationStatus>,
    import_id: Option<String>,
    paths: Option<Vec<String>>,
    source_revision: Option<String>,
}

impl ImportReceiptBuilder {
    pub fn digest(mut self, value: impl Into<String>) -> Self {
        self.digest = Some(value.into());
        self
    }

    pub fn evaluation_status(mut self, value: ImportReceiptEvaluationStatus) -> Self {
        self.evaluation_status = Some(value);
        self
    }

    pub fn import_id(mut self, value: impl Into<String>) -> Self {
        self.import_id = Some(value.into());
        self
    }

    pub fn paths(mut self, value: Vec<String>) -> Self {
        self.paths = Some(value);
        self
    }

    pub fn source_revision(mut self, value: impl Into<String>) -> Self {
        self.source_revision = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`digest`](ImportReceiptBuilder::digest)
    /// - [`import_id`](ImportReceiptBuilder::import_id)
    /// - [`paths`](ImportReceiptBuilder::paths)
    /// - [`source_revision`](ImportReceiptBuilder::source_revision)
    pub fn build(self) -> Result<ImportReceipt, BuildError> {
        Ok(ImportReceipt {
            digest: self.digest.ok_or_else(|| BuildError::missing_field("digest"))?,
            evaluation_status: self.evaluation_status,
            import_id: self.import_id.ok_or_else(|| BuildError::missing_field("import_id"))?,
            paths: self.paths.ok_or_else(|| BuildError::missing_field("paths"))?,
            source_revision: self.source_revision.ok_or_else(|| BuildError::missing_field("source_revision"))?,
        })
    }
}
