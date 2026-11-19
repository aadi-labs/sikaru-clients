use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EndUsersClient {
    pub http_client: HttpClient,
}

impl EndUsersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// What the agent knows about this end user: the persona summary, never the persona prompt.
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
    ///         .end_users
    ///         .get(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"tenant_id".to_string(),
    ///             &"user_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        agent_slug: &str,
        tenant_id: &str,
        user_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<EndUser, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/end-users/{}/{}",
                    project_id, agent_slug, tenant_id, user_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Create this end user's private cell, or update its timezone and population opt-in.
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
    ///         .end_users
    ///         .upsert(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"tenant_id".to_string(),
    ///             &"user_id".to_string(),
    ///             &UpsertEndUser {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert(
        &self,
        project_id: &str,
        agent_slug: &str,
        tenant_id: &str,
        user_id: &str,
        request: &UpsertEndUser,
        options: Option<RequestOptions>,
    ) -> Result<EndUser, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/end-users/{}/{}",
                    project_id, agent_slug, tenant_id, user_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Purge this user from every agent of the product, with every trace or eval case they contributed.
    ///
    /// # Arguments
    ///
    /// * `terminal` - Permanently reject cell creation and new work for this end user.
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
    ///         .end_users
    ///         .delete(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"tenant_id".to_string(),
    ///             &"user_id".to_string(),
    ///             &DeleteQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        project_id: &str,
        agent_slug: &str,
        tenant_id: &str,
        user_id: &str,
        request: &DeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<EndUserDeletion, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/end-users/{}/{}",
                    project_id, agent_slug, tenant_id, user_id
                ),
                None,
                QueryBuilder::new()
                    .bool("terminal", request.terminal.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Import a versioned product snapshot atomically, preserving correction provenance.
    ///
    /// The cell must already exist. Every file requires expectedEtag (null for an
    /// absent destination). Reusing an importId requires the identical manifest.
    /// The digest is SHA-256 of canonical JSON {sourceRevision, files}, with sorted
    /// keys, compact separators and the supplied file order. Private layers are
    /// accepted as input but are never returned in the receipt or end-user view.
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
    ///         .end_users
    ///         .import_state(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"tenant_id".to_string(),
    ///             &"user_id".to_string(),
    ///             &CellImport {
    ///                 digest: "digest".to_string(),
    ///                 files: vec![ImportFile {
    ///                     content: "content".to_string(),
    ///                     expected_etag: None,
    ///                     path: "path".to_string(),
    ///                     provenance: "provenance".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 import_id: "importId".to_string(),
    ///                 source_revision: "sourceRevision".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn import_state(
        &self,
        project_id: &str,
        agent_slug: &str,
        tenant_id: &str,
        user_id: &str,
        request: &CellImport,
        options: Option<RequestOptions>,
    ) -> Result<ImportReceipt, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/end-users/{}/{}/imports",
                    project_id, agent_slug, tenant_id, user_id
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
    ///         .end_users
    ///         .start_learning_run(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"tenant_id".to_string(),
    ///             &"user_id".to_string(),
    ///             &StartAuthoredLearning {
    ///                 evidence: vec![ProductLearningEvidence {
    ///                     content: "content".to_string(),
    ///                     digest: "digest".to_string(),
    ///                     learning_eligible: true,
    ///                     provenance: ProductLearningEvidenceProvenance::UserMessage,
    ///                     revocation_source_id: None,
    ///                     source_id: "sourceId".to_string(),
    ///                 }],
    ///                 invocation_id: "invocationId".to_string(),
    ///                 source_revision: "sourceRevision".to_string(),
    ///                 workflow: AuthoredLearningWorkflow {
    ///                     name: "name".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 capability_grants: None,
    ///                 context: None,
    ///                 tool_provider_refs: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn start_learning_run(
        &self,
        project_id: &str,
        agent_slug: &str,
        tenant_id: &str,
        user_id: &str,
        request: &StartAuthoredLearning,
        options: Option<RequestOptions>,
    ) -> Result<AuthoredLearningReceipt, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/end-users/{}/{}/learning-runs",
                    project_id, agent_slug, tenant_id, user_id
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
    ///         .end_users
    ///         .get_learning_run(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"tenant_id".to_string(),
    ///             &"user_id".to_string(),
    ///             &"invocation_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_learning_run(
        &self,
        project_id: &str,
        agent_slug: &str,
        tenant_id: &str,
        user_id: &str,
        invocation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AuthoredLearningReceipt, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/end-users/{}/{}/learning-runs/{}",
                    project_id, agent_slug, tenant_id, user_id, invocation_id
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
    ///         .end_users
    ///         .revoke_learning_source(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"tenant_id".to_string(),
    ///             &"user_id".to_string(),
    ///             &"source_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn revoke_learning_source(
        &self,
        project_id: &str,
        agent_slug: &str,
        tenant_id: &str,
        user_id: &str,
        source_id: &str,
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
                Method::POST,
                &format!(
                    "v1/projects/{}/managed-agents/{}/end-users/{}/{}/learning-sources/{}/revoke",
                    project_id, agent_slug, tenant_id, user_id, source_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Correct the persona summary. The correction applies from the next turn and is never relearned away.
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
    ///         .end_users
    ///         .correct_persona_summary(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"tenant_id".to_string(),
    ///             &"user_id".to_string(),
    ///             &PersonaSummaryCorrection {
    ///                 text: "text".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn correct_persona_summary(
        &self,
        project_id: &str,
        agent_slug: &str,
        tenant_id: &str,
        user_id: &str,
        request: &PersonaSummaryCorrection,
        options: Option<RequestOptions>,
    ) -> Result<RecordedCorrection, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/end-users/{}/{}/persona-summary/corrections",
                    project_id, agent_slug, tenant_id, user_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
