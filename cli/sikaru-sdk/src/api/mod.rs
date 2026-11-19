//! API client and types for the Sikaru API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints

pub mod resources;

pub use resources::{
    ActivationClient, AgentDocumentsClient, AgentImportsClient, AgentsClient, ApiClient,
    AuthClient, CapabilityCeilingsClient, ChangesetsClient, ChannelsClient, ChatClient,
    ChecksClient, CompletionsClient, ComputeAttachmentsClient, ComputeCredentialsClient,
    ComputeEnvironmentsClient, ComputeOperationsClient, ComputeWorkspacesClient, ConnectionsClient,
    ContextRegistryClient, ConversationsClient, CreditTopUpsClient, DatasetsClient,
    DecisionsClient, DeploymentsClient, EndUsersClient, EnvironmentsClient,
    EvaluationComparisonsClient, EvaluationCriteriaClient, EvaluationJobsClient,
    EvaluationResultsClient, EvaluatorRunsClient, ExecutionObjectivesClient,
    ExecutionSessionsClient, ExecutionsClient, FeedbackClient, GitCredentialsClient, GradersClient,
    HarnessesClient, HttpChannelsClient, ImportSessionsClient, IssueClustersClient,
    ManagedAgentsClient, MemoryClient, ModelGatewayClient, ModelSettingsClient, ModelsClient,
    PersonalChannelsClient, ProjectsClient, ReleaseWatchesClient, ResponsesClient,
    RetentionPoliciesClient, ReviewQueueClient, RunReferencesClient, RunSchedulesClient,
    RunWebhooksClient, RunsClient, SessionsClient, TasksClient, ToolProvidersClient,
    TraceImportConnectionsClient, TraceImportsClient, TraceStreamsClient, WorkflowIntentsClient,
    WorkflowRunsClient, WorkflowsClient,
};

pub use sikaru_types::*;
