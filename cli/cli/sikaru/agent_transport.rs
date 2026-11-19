//! Preserve structured gateway refusals before the generated SDK narrows error schemas.
//! Requests still use Fern's authentication, headers, retries and transport.
use fern_cli_sdk::{
    openapi::AppContext,
    sdk_executor::{CliExecutor, SdkError, SdkRequestExecutor},
};
use sikaru_sdk::{api::ApiClient, ClientConfig, HttpClient, RequestExecutor};
use std::{future::Future, pin::Pin, sync::Arc};

struct FolderExecutor(Arc<CliExecutor>);
impl RequestExecutor for FolderExecutor {
    fn execute(
        &self,
        request: reqwest::Request,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<reqwest::Response, Box<dyn std::error::Error + Send + Sync>>>
                + Send
                + '_,
        >,
    > {
        Box::pin(async move {
            let response = SdkRequestExecutor::execute(&*self.0, request)
                .await
                .map_err(boxed)?;
            if response.status().is_success() {
                return Ok(response);
            }
            let status = response.status().as_u16();
            let body = response
                .text()
                .await
                .map_err(|error| boxed(SdkError::from(error)))?;
            Err(boxed(SdkError::Http { status, body }))
        })
    }
}
fn boxed(error: SdkError) -> Box<dyn std::error::Error + Send + Sync> {
    Box::new(error)
}

pub fn client(ctx: &AppContext) -> ApiClient {
    let mut client = crate::sdk::client(ctx);
    let http = HttpClient::with_executor(
        Arc::new(FolderExecutor(ctx.build_sdk_executor())),
        ClientConfig {
            base_url: ctx.effective_base_url(),
            ..Default::default()
        },
    );
    client.agent_documents.http_client = http.clone();
    client.managed_agents.http_client = http.clone();
    client.channels.http_client = http.clone();
    client.execution_sessions.http_client = http;
    client
}
