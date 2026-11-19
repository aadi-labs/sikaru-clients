pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CellImport {
    #[serde(default)]
    pub digest: String,
    #[serde(default)]
    pub files: Vec<ImportFile>,
    #[serde(rename = "importId")]
    #[serde(default)]
    pub import_id: String,
    #[serde(rename = "sourceRevision")]
    #[serde(default)]
    pub source_revision: String,
}

impl CellImport {
    pub fn builder() -> CellImportBuilder {
        <CellImportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CellImportBuilder {
    digest: Option<String>,
    files: Option<Vec<ImportFile>>,
    import_id: Option<String>,
    source_revision: Option<String>,
}

impl CellImportBuilder {
    pub fn digest(mut self, value: impl Into<String>) -> Self {
        self.digest = Some(value.into());
        self
    }

    pub fn files(mut self, value: Vec<ImportFile>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn import_id(mut self, value: impl Into<String>) -> Self {
        self.import_id = Some(value.into());
        self
    }

    pub fn source_revision(mut self, value: impl Into<String>) -> Self {
        self.source_revision = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CellImport`].
    /// This method will fail if any of the following fields are not set:
    /// - [`digest`](CellImportBuilder::digest)
    /// - [`files`](CellImportBuilder::files)
    /// - [`import_id`](CellImportBuilder::import_id)
    /// - [`source_revision`](CellImportBuilder::source_revision)
    pub fn build(self) -> Result<CellImport, BuildError> {
        Ok(CellImport {
            digest: self.digest.ok_or_else(|| BuildError::missing_field("digest"))?,
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
            import_id: self.import_id.ok_or_else(|| BuildError::missing_field("import_id"))?,
            source_revision: self.source_revision.ok_or_else(|| BuildError::missing_field("source_revision"))?,
        })
    }
}

