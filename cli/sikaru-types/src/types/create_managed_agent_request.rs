pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateManagedAgentRequest {
    #[serde(rename = "activeHarnessVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_harness_version_id: Option<String>,
    #[serde(rename = "agentSlug")]
    #[serde(default)]
    pub agent_slug: String,
    /// Share of maxConcurrentTurns that background turns may use; omit to keep the current share.
    #[serde(rename = "backgroundTurnShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_turn_share: Option<f64>,
    #[serde(rename = "compatibilityProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_profile_id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Fixed at creation. Each end user (tenantId, userId) of the agent gets a private cell: their memory, corrections and learning stay theirs. Omit it to keep an existing agent's choice.
    #[serde(rename = "endUserCells")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_cells: Option<bool>,
    #[serde(rename = "harnessId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_id: Option<String>,
    #[serde(rename = "initialChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_channel: Option<CreateManagedAgentRequestInitialChannel>,
    /// Most turns the agent runs at once; omit to keep the current limit.
    #[serde(rename = "maxConcurrentTurns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_turns: Option<i64>,
    /// Fixed at creation; new agents are shared unless set. A personal agent's conversations stay private to the person who runs them. Omit it to keep an existing agent's choice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CreateManagedAgentSourceRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateManagedAgentRequestStatus>,
}

impl CreateManagedAgentRequest {
    pub fn builder() -> CreateManagedAgentRequestBuilder {
        <CreateManagedAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateManagedAgentRequestBuilder {
    active_harness_version_id: Option<String>,
    agent_slug: Option<String>,
    background_turn_share: Option<f64>,
    compatibility_profile_id: Option<String>,
    display_name: Option<String>,
    end_user_cells: Option<bool>,
    harness_id: Option<String>,
    initial_channel: Option<CreateManagedAgentRequestInitialChannel>,
    max_concurrent_turns: Option<i64>,
    personal: Option<bool>,
    source: Option<CreateManagedAgentSourceRequest>,
    status: Option<CreateManagedAgentRequestStatus>,
}

impl CreateManagedAgentRequestBuilder {
    pub fn active_harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.active_harness_version_id = Some(value.into());
        self
    }

    pub fn agent_slug(mut self, value: impl Into<String>) -> Self {
        self.agent_slug = Some(value.into());
        self
    }

    pub fn background_turn_share(mut self, value: f64) -> Self {
        self.background_turn_share = Some(value);
        self
    }

    pub fn compatibility_profile_id(mut self, value: impl Into<String>) -> Self {
        self.compatibility_profile_id = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn end_user_cells(mut self, value: bool) -> Self {
        self.end_user_cells = Some(value);
        self
    }

    pub fn harness_id(mut self, value: impl Into<String>) -> Self {
        self.harness_id = Some(value.into());
        self
    }

    pub fn initial_channel(mut self, value: CreateManagedAgentRequestInitialChannel) -> Self {
        self.initial_channel = Some(value);
        self
    }

    pub fn max_concurrent_turns(mut self, value: i64) -> Self {
        self.max_concurrent_turns = Some(value);
        self
    }

    pub fn personal(mut self, value: bool) -> Self {
        self.personal = Some(value);
        self
    }

    pub fn source(mut self, value: CreateManagedAgentSourceRequest) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(mut self, value: CreateManagedAgentRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateManagedAgentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_slug`](CreateManagedAgentRequestBuilder::agent_slug)
    pub fn build(self) -> Result<CreateManagedAgentRequest, BuildError> {
        Ok(CreateManagedAgentRequest {
            active_harness_version_id: self.active_harness_version_id,
            agent_slug: self.agent_slug.ok_or_else(|| BuildError::missing_field("agent_slug"))?,
            background_turn_share: self.background_turn_share,
            compatibility_profile_id: self.compatibility_profile_id,
            display_name: self.display_name,
            end_user_cells: self.end_user_cells,
            harness_id: self.harness_id,
            initial_channel: self.initial_channel,
            max_concurrent_turns: self.max_concurrent_turns,
            personal: self.personal,
            source: self.source,
            status: self.status,
        })
    }
}

