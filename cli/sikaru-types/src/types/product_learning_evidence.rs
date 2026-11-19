pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProductLearningEvidence {
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub digest: String,
    #[serde(rename = "learningEligible")]
    #[serde(default)]
    pub learning_eligible: bool,
    pub provenance: ProductLearningEvidenceProvenance,
    #[serde(rename = "revocationSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_source_id: Option<String>,
    #[serde(rename = "sourceId")]
    #[serde(default)]
    pub source_id: String,
}

impl ProductLearningEvidence {
    pub fn builder() -> ProductLearningEvidenceBuilder {
        <ProductLearningEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductLearningEvidenceBuilder {
    content: Option<String>,
    digest: Option<String>,
    learning_eligible: Option<bool>,
    provenance: Option<ProductLearningEvidenceProvenance>,
    revocation_source_id: Option<String>,
    source_id: Option<String>,
}

impl ProductLearningEvidenceBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn digest(mut self, value: impl Into<String>) -> Self {
        self.digest = Some(value.into());
        self
    }

    pub fn learning_eligible(mut self, value: bool) -> Self {
        self.learning_eligible = Some(value);
        self
    }

    pub fn provenance(mut self, value: ProductLearningEvidenceProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn revocation_source_id(mut self, value: impl Into<String>) -> Self {
        self.revocation_source_id = Some(value.into());
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProductLearningEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](ProductLearningEvidenceBuilder::content)
    /// - [`digest`](ProductLearningEvidenceBuilder::digest)
    /// - [`learning_eligible`](ProductLearningEvidenceBuilder::learning_eligible)
    /// - [`provenance`](ProductLearningEvidenceBuilder::provenance)
    /// - [`source_id`](ProductLearningEvidenceBuilder::source_id)
    pub fn build(self) -> Result<ProductLearningEvidence, BuildError> {
        Ok(ProductLearningEvidence {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            digest: self.digest.ok_or_else(|| BuildError::missing_field("digest"))?,
            learning_eligible: self.learning_eligible.ok_or_else(|| BuildError::missing_field("learning_eligible"))?,
            provenance: self.provenance.ok_or_else(|| BuildError::missing_field("provenance"))?,
            revocation_source_id: self.revocation_source_id,
            source_id: self.source_id.ok_or_else(|| BuildError::missing_field("source_id"))?,
        })
    }
}
