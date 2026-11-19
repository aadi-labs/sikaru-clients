use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MemoryClient {
    pub http_client: HttpClient,
}

impl MemoryClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// The directory's latest dream job.
    ///
    /// # Arguments
    ///
    /// * `directory` - The shared agent directory, or one end user's directory.
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
    ///         .memory
    ///         .get_dream(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &GetDreamQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_dream(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &GetDreamQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemoryDream, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/dreams",
                    project_id, agent_slug
                ),
                None,
                QueryBuilder::new()
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Dream now: consolidate this directory without waiting for idle time, even with automatic dreaming off.
    ///
    /// # Arguments
    ///
    /// * `directory` - The shared agent directory, or one end user's directory.
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
    ///         .memory
    ///         .dream(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &DreamQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn dream(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &DreamQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemoryDream, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/dreams",
                    project_id, agent_slug
                ),
                None,
                QueryBuilder::new()
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// The directory's files and its current head.
    ///
    /// # Arguments
    ///
    /// * `directory` - The shared agent directory, or one end user's directory.
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
    ///         .memory
    ///         .list_files(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &ListFilesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_files(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &ListFilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemoryFiles, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/files",
                    project_id, agent_slug
                ),
                None,
                QueryBuilder::new()
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
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
    ///         .memory
    ///         .read_file(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &ReadFileQueryRequest {
    ///                 path: "path".to_string(),
    ///                 directory: None,
    ///                 environment: None,
    ///                 tenant_id: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn read_file(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &ReadFileQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemoryFile, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/files/content",
                    project_id, agent_slug
                ),
                None,
                QueryBuilder::new()
                    .string("path", request.path.clone())
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create or replace one file.
    ///
    /// # Arguments
    ///
    /// * `directory` - The shared agent directory, or one end user's directory.
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
    ///         .memory
    ///         .write_file(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &WriteMemoryFile {
    ///                 content: "content".to_string(),
    ///                 path: "path".to_string(),
    ///                 directory: None,
    ///                 environment: None,
    ///                 tenant_id: None,
    ///                 user_id: None,
    ///                 base: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn write_file(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &WriteMemoryFile,
        options: Option<RequestOptions>,
    ) -> Result<MemoryHead, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/files/content",
                    project_id, agent_slug
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
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
    ///         .memory
    ///         .delete_file(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &DeleteFileQueryRequest {
    ///                 path: "path".to_string(),
    ///                 base: "base".to_string(),
    ///                 directory: None,
    ///                 environment: None,
    ///                 tenant_id: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_file(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &DeleteFileQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemoryHead, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/files/content",
                    project_id, agent_slug
                ),
                None,
                QueryBuilder::new()
                    .string("path", request.path.clone())
                    .string("base", request.base.clone())
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Move or rename one file; the destination must not exist.
    ///
    /// # Arguments
    ///
    /// * `directory` - The shared agent directory, or one end user's directory.
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
    ///         .memory
    ///         .move_file(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &MoveMemoryFile {
    ///                 path: "path".to_string(),
    ///                 to: "to".to_string(),
    ///                 directory: None,
    ///                 environment: None,
    ///                 tenant_id: None,
    ///                 user_id: None,
    ///                 base: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn move_file(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &MoveMemoryFile,
        options: Option<RequestOptions>,
    ) -> Result<MemoryHead, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/files/move",
                    project_id, agent_slug
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
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
    ///         .memory
    ///         .list_forgets(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &ListForgetsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_forgets(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &ListForgetsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemoryForgets, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/forgets",
                    project_id, agent_slug
                ),
                None,
                QueryBuilder::new()
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Remove something from this user's memory, its retained history and past-session search, in the background.
    ///
    /// # Arguments
    ///
    /// * `directory` - The shared agent directory, or one end user's directory.
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
    ///         .memory
    ///         .forget(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &ForgetRequest {
    ///                 body: ForgetMemory {
    ///                     target: "target".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 directory: None,
    ///                 environment: None,
    ///                 tenant_id: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn forget(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &ForgetRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemoryForget, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/forgets",
                    project_id, agent_slug
                ),
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
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
    ///         .memory
    ///         .get_forget(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"job_id".to_string(),
    ///             &GetForgetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_forget(
        &self,
        project_id: &str,
        agent_slug: &str,
        job_id: &str,
        request: &GetForgetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemoryForget, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/forgets/{}",
                    project_id, agent_slug, job_id
                ),
                None,
                QueryBuilder::new()
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Remember again: finished forgets of this target stop hiding it from future search and dreams.
    ///
    /// # Arguments
    ///
    /// * `directory` - The shared agent directory, or one end user's directory.
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
    ///         .memory
    ///         .remember(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &RememberRequest {
    ///                 body: ForgetMemory {
    ///                     target: "target".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 directory: None,
    ///                 environment: None,
    ///                 tenant_id: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn remember(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &RememberRequest,
        options: Option<RequestOptions>,
    ) -> Result<Remembered, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/remember",
                    project_id, agent_slug
                ),
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
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
    ///         .memory
    ///         .list_versions(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &ListVersionsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_versions(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &ListVersionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemoryVersions, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/versions",
                    project_id, agent_slug
                ),
                None,
                QueryBuilder::new()
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Make a retained version the head again; the rollback is kept as the newest version.
    ///
    /// # Arguments
    ///
    /// * `directory` - The shared agent directory, or one end user's directory.
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
    ///         .memory
    ///         .rollback(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             1,
    ///             &RollbackQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn rollback(
        &self,
        project_id: &str,
        agent_slug: &str,
        version: i64,
        request: &RollbackQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MemoryHead, ApiError> {
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
                    "v1/projects/{}/managed-agents/{}/memory/versions/{}/rollback",
                    project_id, agent_slug, version
                ),
                None,
                QueryBuilder::new()
                    .serialize("directory", request.directory.clone())
                    .serialize("environment", request.environment.clone())
                    .serialize("tenantId", request.tenant_id.clone())
                    .serialize("userId", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
