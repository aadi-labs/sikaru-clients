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

    /// What the agent knows about this end user: facts and the persona summary, never the persona prompt.
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
                None,
                options,
            )
            .await
    }

    /// Correct or add one fact. It serves from the next turn and becomes a held-out case for this user.
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
    ///         .update_fact(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"tenant_id".to_string(),
    ///             &"user_id".to_string(),
    ///             &UpdateEndUserFact {
    ///                 content: "content".to_string(),
    ///                 path: "path".to_string(),
    ///                 expected_etag: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_fact(
        &self,
        project_id: &str,
        agent_slug: &str,
        tenant_id: &str,
        user_id: &str,
        request: &UpdateEndUserFact,
        options: Option<RequestOptions>,
    ) -> Result<EndUserFile, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/end-users/{}/{}/facts",
                    project_id, agent_slug, tenant_id, user_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Forget one memory, or clear the profile.
    ///
    /// # Arguments
    ///
    /// * `path` - USER.md or memories/<name>.md.
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
    ///         .end_users
    ///         .delete_fact(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"tenant_id".to_string(),
    ///             &"user_id".to_string(),
    ///             &DeleteFactQueryRequest {
    ///                 path: "path".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_fact(
        &self,
        project_id: &str,
        agent_slug: &str,
        tenant_id: &str,
        user_id: &str,
        request: &DeleteFactQueryRequest,
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
                    "v1/projects/{}/managed-agents/{}/end-users/{}/{}/facts",
                    project_id, agent_slug, tenant_id, user_id
                ),
                None,
                QueryBuilder::new()
                    .string("path", request.path.clone())
                    .build(),
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
