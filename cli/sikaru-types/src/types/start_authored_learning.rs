pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StartAuthoredLearning {
    #[serde(rename = "capabilityGrants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_grants: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub evidence: Vec<ProductLearningEvidence>,
    #[serde(rename = "invocationId")]
    #[serde(default)]
    pub invocation_id: String,
    #[serde(rename = "sourceRevision")]
    #[serde(default)]
    pub source_revision: String,
    #[serde(rename = "toolProviderRefs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_provider_refs: Option<Vec<AuthoredToolProviderRef>>,
    #[serde(default)]
    pub workflow: AuthoredLearningWorkflow,
}

impl StartAuthoredLearning {
    pub fn builder() -> StartAuthoredLearningBuilder {
        <StartAuthoredLearningBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartAuthoredLearningBuilder {
    capability_grants: Option<Vec<String>>,
    context: Option<HashMap<String, serde_json::Value>>,
    evidence: Option<Vec<ProductLearningEvidence>>,
    invocation_id: Option<String>,
    source_revision: Option<String>,
    tool_provider_refs: Option<Vec<AuthoredToolProviderRef>>,
    workflow: Option<AuthoredLearningWorkflow>,
}

impl StartAuthoredLearningBuilder {
    pub fn capability_grants(mut self, value: Vec<String>) -> Self {
        self.capability_grants = Some(value);
        self
    }

    pub fn context(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.context = Some(value);
        self
    }

    pub fn evidence(mut self, value: Vec<ProductLearningEvidence>) -> Self {
        self.evidence = Some(value);
        self
    }

    pub fn invocation_id(mut self, value: impl Into<String>) -> Self {
        self.invocation_id = Some(value.into());
        self
    }

    pub fn source_revision(mut self, value: impl Into<String>) -> Self {
        self.source_revision = Some(value.into());
        self
    }

    pub fn tool_provider_refs(mut self, value: Vec<AuthoredToolProviderRef>) -> Self {
        self.tool_provider_refs = Some(value);
        self
    }

    pub fn workflow(mut self, value: AuthoredLearningWorkflow) -> Self {
        self.workflow = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StartAuthoredLearning`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evidence`](StartAuthoredLearningBuilder::evidence)
    /// - [`invocation_id`](StartAuthoredLearningBuilder::invocation_id)
    /// - [`source_revision`](StartAuthoredLearningBuilder::source_revision)
    /// - [`workflow`](StartAuthoredLearningBuilder::workflow)
    pub fn build(self) -> Result<StartAuthoredLearning, BuildError> {
        Ok(StartAuthoredLearning {
            capability_grants: self.capability_grants,
            context: self.context,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
            invocation_id: self.invocation_id.ok_or_else(|| BuildError::missing_field("invocation_id"))?,
            source_revision: self.source_revision.ok_or_else(|| BuildError::missing_field("source_revision"))?,
            tool_provider_refs: self.tool_provider_refs,
            workflow: self.workflow.ok_or_else(|| BuildError::missing_field("workflow"))?,
        })
    }
}

