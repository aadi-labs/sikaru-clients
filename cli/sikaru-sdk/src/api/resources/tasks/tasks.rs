use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct TasksClient {
    pub http_client: HttpClient,
}

impl TasksClient {
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
    ///         .tasks
    ///         .list(&"project_id".to_string(), &"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        project_id: &str,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Tasks, ApiError> {
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
                    "v1/projects/{}/execution-sessions/{}/tasks",
                    project_id, session_id
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
    ///         .tasks
    ///         .stop_all(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &TaskCancel {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stop_all(
        &self,
        project_id: &str,
        session_id: &str,
        request: &TaskCancel,
        options: Option<RequestOptions>,
    ) -> Result<TaskReceipt, ApiError> {
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
                    "v1/projects/{}/execution-sessions/{}/tasks/stop-all",
                    project_id, session_id
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
    ///         .tasks
    ///         .get(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"task_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        session_id: &str,
        task_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
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
                    "v1/projects/{}/execution-sessions/{}/tasks/{}",
                    project_id, session_id, task_id
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
    ///         .tasks
    ///         .actions(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"task_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn actions(
        &self,
        project_id: &str,
        session_id: &str,
        task_id: &str,
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
                &format!(
                    "v1/projects/{}/execution-sessions/{}/tasks/{}/actions",
                    project_id, session_id, task_id
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
    ///         .tasks
    ///         .approve_action(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"task_id".to_string(),
    ///             &"action_id".to_string(),
    ///             &ApprovalInput {
    ///                 always: None,
    ///                 cap: None,
    ///                 decision: ApprovalInputDecision::Approved,
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn approve_action(
        &self,
        project_id: &str,
        session_id: &str,
        task_id: &str,
        action_id: &str,
        request: &ApprovalInput,
        options: Option<RequestOptions>,
    ) -> Result<TaskApprovalReceipt, ApiError> {
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
                    "v1/projects/{}/execution-sessions/{}/tasks/{}/actions/{}/approval",
                    project_id, session_id, task_id, action_id
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
    ///         .tasks
    ///         .cancel(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"task_id".to_string(),
    ///             &TaskCancel {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        project_id: &str,
        session_id: &str,
        task_id: &str,
        request: &TaskCancel,
        options: Option<RequestOptions>,
    ) -> Result<TaskReceipt, ApiError> {
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
                    "v1/projects/{}/execution-sessions/{}/tasks/{}/cancel",
                    project_id, session_id, task_id
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
    ///         .tasks
    ///         .changes(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"task_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn changes(
        &self,
        project_id: &str,
        session_id: &str,
        task_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TaskChanges, ApiError> {
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
                    "v1/projects/{}/execution-sessions/{}/tasks/{}/changes",
                    project_id, session_id, task_id
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
    ///         .tasks
    ///         .integrations(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"task_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn integrations(
        &self,
        project_id: &str,
        session_id: &str,
        task_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TaskIntegrationReceipts, ApiError> {
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
                    "v1/projects/{}/execution-sessions/{}/tasks/{}/integrations",
                    project_id, session_id, task_id
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
    ///         .tasks
    ///         .integrate(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"task_id".to_string(),
    ///             &TaskIntegration {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 revision: 1,
    ///                 paths: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn integrate(
        &self,
        project_id: &str,
        session_id: &str,
        task_id: &str,
        request: &TaskIntegration,
        options: Option<RequestOptions>,
    ) -> Result<TaskIntegrationReceipt, ApiError> {
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
                    "v1/projects/{}/execution-sessions/{}/tasks/{}/integrations",
                    project_id, session_id, task_id
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
    ///         .tasks
    ///         .message(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"task_id".to_string(),
    ///             &TaskMessage {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 input: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///                 delivery_mode: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn message(
        &self,
        project_id: &str,
        session_id: &str,
        task_id: &str,
        request: &TaskMessage,
        options: Option<RequestOptions>,
    ) -> Result<TaskReceipt, ApiError> {
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
                    "v1/projects/{}/execution-sessions/{}/tasks/{}/messages",
                    project_id, session_id, task_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
