use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct GradersClient {
    pub http_client: HttpClient,
}

impl GradersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// The agent's Graders, presets included, and whether the agent is graded at all.
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
    ///         .graders
    ///         .list(&"project_id".to_string(), &"agent_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        project_id: &str,
        agent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GraderList, ApiError> {
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
                &format!("v1/projects/{}/agents/{}/graders", project_id, agent_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Score an unsaved file with its own judge. Poll a pending test; no Grader or verdict is saved.
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
    ///         .graders
    ///         .test(
    ///             &"project_id".to_string(),
    ///             &"agent_id".to_string(),
    ///             &TestBody {
    ///                 file: "file".to_string(),
    ///                 name: "name".to_string(),
    ///                 limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn test(
        &self,
        project_id: &str,
        agent_id: &str,
        request: &TestBody,
        options: Option<RequestOptions>,
    ) -> Result<TestReport, ApiError> {
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
                    "v1/projects/{}/agents/{}/graders/test",
                    project_id, agent_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read a pending, completed or failed unsaved test. Reports expire after one day.
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
    ///         .graders
    ///         .get_test(
    ///             &"project_id".to_string(),
    ///             &"agent_id".to_string(),
    ///             &"test_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_test(
        &self,
        project_id: &str,
        agent_id: &str,
        test_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TestReport, ApiError> {
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
                    "v1/projects/{}/agents/{}/graders/tests/{}",
                    project_id, agent_id, test_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// One Grader: its status, criteria, judge, release check, versions and label counts.
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
    ///         .graders
    ///         .get(
    ///             &"project_id".to_string(),
    ///             &"agent_id".to_string(),
    ///             &"grader_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        agent_id: &str,
        grader_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GraderDetail, ApiError> {
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
                    "v1/projects/{}/agents/{}/graders/{}",
                    project_id, agent_id, grader_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Change the description, judge, escalation or release check. A judge or escalation change saves a provisional
    /// version that goes live only once it clears the promotion rule; the other settings change no verdict.
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
    ///         .graders
    ///         .update(
    ///             &"project_id".to_string(),
    ///             &"agent_id".to_string(),
    ///             &"grader_id".to_string(),
    ///             &SettingsBody {
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
        agent_id: &str,
        grader_id: &str,
        request: &SettingsBody,
        options: Option<RequestOptions>,
    ) -> Result<GraderDetail, ApiError> {
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
                &format!(
                    "v1/projects/{}/agents/{}/graders/{}",
                    project_id, agent_id, grader_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// What judging every conversation with the LLM judge or a project agent is expected to cost (R4).
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
    ///         .graders
    ///         .estimate_cost(
    ///             &"project_id".to_string(),
    ///             &"agent_id".to_string(),
    ///             &"grader_id".to_string(),
    ///             &EstimateCostQueryRequest {
    ///                 judge: EstimateCostGradersRequestJudge::Llm,
    ///                 judge_agent_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn estimate_cost(
        &self,
        project_id: &str,
        agent_id: &str,
        grader_id: &str,
        request: &EstimateCostQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CostEstimate, ApiError> {
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
                    "v1/projects/{}/agents/{}/graders/{}/cost-estimate",
                    project_id, agent_id, grader_id
                ),
                None,
                QueryBuilder::new()
                    .serialize("judge", Some(request.judge.clone()))
                    .serialize("judgeAgentId", request.judge_agent_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Conversations the Grader is least sure about, unlabeled, each with an excerpt and without its verdict.
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
    ///         .graders
    ///         .label_queue(
    ///             &"project_id".to_string(),
    ///             &"agent_id".to_string(),
    ///             &"grader_id".to_string(),
    ///             &LabelQueueQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn label_queue(
        &self,
        project_id: &str,
        agent_id: &str,
        grader_id: &str,
        request: &LabelQueueQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LabelQueue, ApiError> {
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
                    "v1/projects/{}/agents/{}/graders/{}/label-queue",
                    project_id, agent_id, grader_id
                ),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Mark one conversation Flagged or Looks Fine with an optional reason; labeling it again replaces the label.
    /// The response then shows what the Grader itself said.
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
    ///         .graders
    ///         .label(
    ///             &"project_id".to_string(),
    ///             &"agent_id".to_string(),
    ///             &"grader_id".to_string(),
    ///             &LabelBody {
    ///                 conversation_key: "conversationKey".to_string(),
    ///                 verdict: LabelBodyVerdict::Flagged,
    ///                 reason: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn label(
        &self,
        project_id: &str,
        agent_id: &str,
        grader_id: &str,
        request: &LabelBody,
        options: Option<RequestOptions>,
    ) -> Result<Labeled, ApiError> {
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
                    "v1/projects/{}/agents/{}/graders/{}/labels",
                    project_id, agent_id, grader_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Re-score recent production conversations using this Grader version's own judge.
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
    ///         .graders
    ///         .run(
    ///             &"project_id".to_string(),
    ///             &"agent_id".to_string(),
    ///             &"grader_id".to_string(),
    ///             &RunBody {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn run(
        &self,
        project_id: &str,
        agent_id: &str,
        grader_id: &str,
        request: &RunBody,
        options: Option<RequestOptions>,
    ) -> Result<RunStarted, ApiError> {
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
                    "v1/projects/{}/agents/{}/graders/{}/run",
                    project_id, agent_id, grader_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Follow a recent-production re-score to completion or a useful failure message.
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
    ///         .graders
    ///         .get_run(
    ///             &"project_id".to_string(),
    ///             &"agent_id".to_string(),
    ///             &"grader_id".to_string(),
    ///             &"run_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_run(
        &self,
        project_id: &str,
        agent_id: &str,
        grader_id: &str,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RunReport, ApiError> {
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
                    "v1/projects/{}/agents/{}/graders/{}/runs/{}",
                    project_id, agent_id, grader_id, run_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// The Grader's recent verdicts, flagged conversations first.
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
    ///         .graders
    ///         .samples(
    ///             &"project_id".to_string(),
    ///             &"agent_id".to_string(),
    ///             &"grader_id".to_string(),
    ///             &SamplesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn samples(
        &self,
        project_id: &str,
        agent_id: &str,
        grader_id: &str,
        request: &SamplesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Samples, ApiError> {
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
                    "v1/projects/{}/agents/{}/graders/{}/samples",
                    project_id, agent_id, grader_id
                ),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }
}
