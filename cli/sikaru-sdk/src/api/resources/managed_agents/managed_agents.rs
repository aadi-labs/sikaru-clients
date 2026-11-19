use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ManagedAgentsClient {
    pub http_client: HttpClient,
}

impl ManagedAgentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .list_managed_agents(&"project_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_managed_agents(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/managed-agents", project_id),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .create_managed_agent(
    ///             &"project_id".to_string(),
    ///             &CreateManagedAgentRequest {
    ///                 agent_slug: "agentSlug".to_string(),
    ///                 active_harness_version_id: None,
    ///                 background_turn_share: None,
    ///                 compatibility_profile_id: None,
    ///                 display_name: None,
    ///                 end_user_cells: None,
    ///                 harness_id: None,
    ///                 initial_channel: None,
    ///                 max_concurrent_turns: None,
    ///                 memory_dream_auto: None,
    ///                 memory_dream_idle_seconds: None,
    ///                 personal: None,
    ///                 source: None,
    ///                 status: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_managed_agent(
        &self,
        project_id: &str,
        request: &CreateManagedAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatedManagedAgent, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/managed-agents", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// One managed agent in the project: the same public summary the list returns.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .get(&"project_id".to_string(), &"agent_slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        agent_slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<ManagedAgentRead, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/managed-agents/{}", project_id, agent_slug),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete an agent that has never gone live, with its drafts, checks and schedules.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .delete_managed_agent(&"project_id".to_string(), &"agent_slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_managed_agent(
        &self,
        project_id: &str,
        agent_slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/projects/{}/managed-agents/{}", project_id, agent_slug),
                None,
                None,
                options,
            )
            .await
    }

    /// Change the agent's display name, turn capacity or memory dreaming. The slug and every reference to the agent stay the same.
    ///
    /// Personal and end-user-cell choices are fixed at creation.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .update(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &UpdateAgent {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &UpdateAgent,
        options: Option<RequestOptions>,
    ) -> Result<RenamedManagedAgent, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/projects/{}/managed-agents/{}", project_id, agent_slug),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .list_agent_logins(&"project_id".to_string(), &"agent_slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_agent_logins(
        &self,
        project_id: &str,
        agent_slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<SavedLogins, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/managed-agents/{}/browser-logins",
                    project_id, agent_slug
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// An operator signs the agent in to a company account once; every run then uses it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .add_agent_login(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &LoginRequest {
    ///                 url: "url".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_agent_login(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &LoginRequest,
        options: Option<RequestOptions>,
    ) -> Result<LiveLink, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/managed-agents/{}/browser-logins",
                    project_id, agent_slug
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .delete_agent_login(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"login_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_agent_login(
        &self,
        project_id: &str,
        agent_slug: &str,
        login_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/projects/{}/managed-agents/{}/browser-logins/{}",
                    project_id, agent_slug, login_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .list_playbooks(&"project_id".to_string(), &"agent_slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_playbooks(
        &self,
        project_id: &str,
        agent_slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<PlaybookList, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/managed-agents/{}/browser-playbooks",
                    project_id, agent_slug
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .delete_playbook(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"playbook_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_playbook(
        &self,
        project_id: &str,
        agent_slug: &str,
        playbook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/projects/{}/managed-agents/{}/browser-playbooks/{}",
                    project_id, agent_slug, playbook_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Every always-allow rule on the agent; a rule personal to one end user does not say whose.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .list_rules(&"project_id".to_string(), &"agent_slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_rules(
        &self,
        project_id: &str,
        agent_slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<BrowserRuleList, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/managed-agents/{}/browser-rules",
                    project_id, agent_slug
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .delete_rule(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"rule_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_rule(
        &self,
        project_id: &str,
        agent_slug: &str,
        rule_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/projects/{}/managed-agents/{}/browser-rules/{}",
                    project_id, agent_slug, rule_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .get_definition_revision(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"changeset_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_definition_revision(
        &self,
        project_id: &str,
        agent_slug: &str,
        changeset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DefinitionRevisionView, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/managed-agents/{}/definition-revisions/{}",
                    project_id, agent_slug, changeset_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Replace or enable a customer-authored source using its own revision CAS.
    ///
    /// Private optimized methodology and optimizer digests are never returned.
    /// Replaying an operation returns its original receipt without reverting later work.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .managed_agents
    ///         .update_end_user_skill_source(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"tenant_id".to_string(),
    ///             &"user_id".to_string(),
    ///             &"skill_id".to_string(),
    ///             &UpdateSkillSource {
    ///                 enabled: true,
    ///                 operation_id: "operationId".to_string(),
    ///                 source_revision: "sourceRevision".to_string(),
    ///                 content: None,
    ///                 expected_revision: None,
    ///                 resources: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_end_user_skill_source(
        &self,
        project_id: &str,
        agent_slug: &str,
        tenant_id: &str,
        user_id: &str,
        skill_id: &str,
        request: &UpdateSkillSource,
        options: Option<RequestOptions>,
    ) -> Result<SkillSourceReceipt, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/projects/{}/managed-agents/{}/end-users/{}/{}/skill-sources/{}",
                    project_id, agent_slug, tenant_id, user_id, skill_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
