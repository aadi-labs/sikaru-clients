pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ManagedAgentView {
    #[serde(rename = "activeHarnessVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_harness_version_id: Option<String>,
    #[serde(rename = "agentSlug")]
    #[serde(default)]
    pub agent_slug: String,
    /// Share of the turn limit background turns may use; null follows the default.
    #[serde(rename = "backgroundTurnShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub background_turn_share: Option<f64>,
    #[serde(rename = "compatibilityProfileId")]
    #[serde(default)]
    pub compatibility_profile_id: String,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    /// Fixed at creation: each end user of the agent has a private cell.
    #[serde(rename = "endUserCells")]
    #[serde(default)]
    pub end_user_cells: bool,
    #[serde(rename = "harnessId")]
    #[serde(default)]
    pub harness_id: String,
    #[serde(default)]
    pub id: String,
    /// True while learning is held until the agent passes its behavior suite.
    #[serde(rename = "learningLocked")]
    #[serde(default)]
    pub learning_locked: bool,
    /// Most turns the agent runs at once; null follows the default.
    #[serde(rename = "maxConcurrentTurns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_turns: Option<i64>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub personal: bool,
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub status: String,
}

impl ManagedAgentView {
    pub fn builder() -> ManagedAgentViewBuilder {
        <ManagedAgentViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManagedAgentViewBuilder {
    active_harness_version_id: Option<String>,
    agent_slug: Option<String>,
    background_turn_share: Option<f64>,
    compatibility_profile_id: Option<String>,
    display_name: Option<String>,
    end_user_cells: Option<bool>,
    harness_id: Option<String>,
    id: Option<String>,
    learning_locked: Option<bool>,
    max_concurrent_turns: Option<i64>,
    organization_id: Option<String>,
    personal: Option<bool>,
    project_id: Option<String>,
    status: Option<String>,
}

impl ManagedAgentViewBuilder {
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn learning_locked(mut self, value: bool) -> Self {
        self.learning_locked = Some(value);
        self
    }

    pub fn max_concurrent_turns(mut self, value: i64) -> Self {
        self.max_concurrent_turns = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn personal(mut self, value: bool) -> Self {
        self.personal = Some(value);
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ManagedAgentView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_slug`](ManagedAgentViewBuilder::agent_slug)
    /// - [`compatibility_profile_id`](ManagedAgentViewBuilder::compatibility_profile_id)
    /// - [`display_name`](ManagedAgentViewBuilder::display_name)
    /// - [`end_user_cells`](ManagedAgentViewBuilder::end_user_cells)
    /// - [`harness_id`](ManagedAgentViewBuilder::harness_id)
    /// - [`id`](ManagedAgentViewBuilder::id)
    /// - [`learning_locked`](ManagedAgentViewBuilder::learning_locked)
    /// - [`organization_id`](ManagedAgentViewBuilder::organization_id)
    /// - [`personal`](ManagedAgentViewBuilder::personal)
    /// - [`project_id`](ManagedAgentViewBuilder::project_id)
    /// - [`status`](ManagedAgentViewBuilder::status)
    pub fn build(self) -> Result<ManagedAgentView, BuildError> {
        Ok(ManagedAgentView {
            active_harness_version_id: self.active_harness_version_id,
            agent_slug: self.agent_slug.ok_or_else(|| BuildError::missing_field("agent_slug"))?,
            background_turn_share: self.background_turn_share,
            compatibility_profile_id: self.compatibility_profile_id.ok_or_else(|| BuildError::missing_field("compatibility_profile_id"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            end_user_cells: self.end_user_cells.ok_or_else(|| BuildError::missing_field("end_user_cells"))?,
            harness_id: self.harness_id.ok_or_else(|| BuildError::missing_field("harness_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            learning_locked: self.learning_locked.ok_or_else(|| BuildError::missing_field("learning_locked"))?,
            max_concurrent_turns: self.max_concurrent_turns,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            personal: self.personal.ok_or_else(|| BuildError::missing_field("personal"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
