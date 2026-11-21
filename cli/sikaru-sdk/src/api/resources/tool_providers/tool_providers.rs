use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ToolProvidersClient {
    pub http_client: HttpClient,
}

impl ToolProvidersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// The project's tool providers, optionally narrowed to one capability prefix or status.
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
    ///         .tool_providers
    ///         .list_tool_providers(
    ///             &"project_id".to_string(),
    ///             &ListToolProvidersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_tool_providers(
        &self,
        project_id: &str,
        request: &ListToolProvidersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ToolProvider>, ApiError> {
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
                &format!("v1/projects/{}/tool-providers", project_id),
                None,
                QueryBuilder::new()
                    .serialize("capability_prefix", request.capability_prefix.clone())
                    .serialize("status", request.status.clone())
                    .build(),
                options,
            )
            .await
    }

    /// One tool provider with its delivery mode and secret state.
    ///
    /// # Arguments
    ///
    /// * `capability_prefix` - The provider's capability prefix; it never changes.
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
    ///         .tool_providers
    ///         .get_tool_provider(
    ///             &"project_id".to_string(),
    ///             &"capability_prefix".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_tool_provider(
        &self,
        project_id: &str,
        capability_prefix: &str,
        options: Option<RequestOptions>,
    ) -> Result<ToolProvider, ApiError> {
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
                    "v1/projects/{}/tool-providers/{}",
                    project_id, capability_prefix
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Create the provider for this capability prefix, or converge it to the request.
    ///
    /// Re-sending the same declaration changes nothing. Changing the provider type or
    /// credential owner is refused with `state_conflict`.
    ///
    /// # Arguments
    ///
    /// * `capability_prefix` - The provider's capability prefix; it never changes.
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
    ///         .tool_providers
    ///         .apply_tool_provider(
    ///             &"project_id".to_string(),
    ///             &"capability_prefix".to_string(),
    ///             &ApplyToolProviderRequest {
    ///                 broker_endpoint_ref: "broker_endpoint_ref".to_string(),
    ///                 display_name: "display_name".to_string(),
    ///                 provider_type: "provider_type".to_string(),
    ///                 tool_catalog_ref: "tool_catalog_ref".to_string(),
    ///                 approval_policy_ref: None,
    ///                 credential_owner: None,
    ///                 secret: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn apply_tool_provider(
        &self,
        project_id: &str,
        capability_prefix: &str,
        request: &ApplyToolProviderRequest,
        options: Option<RequestOptions>,
    ) -> Result<AppliedToolProvider, ApiError> {
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
                    "v1/projects/{}/tool-providers/{}",
                    project_id, capability_prefix
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Stop accepting the previous delivery secret now. Without one this changes nothing.
    ///
    /// # Arguments
    ///
    /// * `capability_prefix` - The provider's capability prefix; it never changes.
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
    ///         .tool_providers
    ///         .expire_previous_tool_provider_secret(
    ///             &"project_id".to_string(),
    ///             &"capability_prefix".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn expire_previous_tool_provider_secret(
        &self,
        project_id: &str,
        capability_prefix: &str,
        options: Option<RequestOptions>,
    ) -> Result<ToolProvider, ApiError> {
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
                    "v1/projects/{}/tool-providers/{}/secret/previous",
                    project_id, capability_prefix
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Make a new delivery secret current; the old one stays valid for the overlap.
    ///
    /// # Arguments
    ///
    /// * `capability_prefix` - The provider's capability prefix; it never changes.
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
    ///         .tool_providers
    ///         .rotate_tool_provider_secret(
    ///             &"project_id".to_string(),
    ///             &"capability_prefix".to_string(),
    ///             &RotateToolProviderSecretRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn rotate_tool_provider_secret(
        &self,
        project_id: &str,
        capability_prefix: &str,
        request: &RotateToolProviderSecretRequest,
        options: Option<RequestOptions>,
    ) -> Result<AppliedToolProvider, ApiError> {
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
                    "v1/projects/{}/tool-providers/{}/secret/rotate",
                    project_id, capability_prefix
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
    ///         .tool_providers
    ///         .attach_source_tool_skill(
    ///             &"project_id".to_string(),
    ///             &"capability_prefix".to_string(),
    ///             &AttachSourceToolSkillRequest {
    ///                 capability_refs: vec!["capability_refs".to_string()],
    ///                 description: "description".to_string(),
    ///                 source: ToolSkillSourceRequest {
    ///                     kind: "kind".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn attach_source_tool_skill(
        &self,
        project_id: &str,
        capability_prefix: &str,
        request: &AttachSourceToolSkillRequest,
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
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/tool-providers/{}/skills",
                    project_id, capability_prefix
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
