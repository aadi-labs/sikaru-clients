//! Stable advice excludes arbitrary remote bodies, tokens and URLs. Draft admission exposes only allowlisted reach fields.
use super::{input::InvalidInput, state, transport::TransportFailure};
use serde_json::{json, Value};

#[derive(Debug, thiserror::Error)]
#[error("agent_release_unavailable")]
pub struct UnavailableRelease;

/// Only session admission can prove this rejection happened before execution.
pub async fn session_admission<T>(
    matches: &clap::ArgMatches,
    future: impl std::future::Future<Output = Result<T, sikaru_sdk::ApiError>>,
) -> anyhow::Result<T> {
    tokio::time::timeout(std::time::Duration::from_secs(10), future)
        .await
        .map_err(|_| anyhow::Error::new(TransportFailure::Transient))?
        .map_err(|error| admission_error(error, matches))
}

#[derive(Debug, thiserror::Error)]
#[error("draft_access_requires_review")]
struct DraftAccessRequired {
    delta: Value,
    next: String,
    digest: String,
}

fn admission_error(error: sikaru_sdk::ApiError, matches: &clap::ArgMatches) -> anyhow::Error {
    if let Some(body) = refusal_body(&error) {
        if let Some((delta, digest)) = draft_access_delta(body) {
            return anyhow::Error::new(DraftAccessRequired {
                delta,
                next: draft_retry(matches, &digest),
                digest,
            });
        }
        if unavailable_release_code(body) {
            return anyhow::Error::new(UnavailableRelease);
        }
    }
    if matches!(&error, sikaru_sdk::ApiError::ConflictError {code:Some(code),..} if code == "release_unavailable")
    {
        return anyhow::Error::new(UnavailableRelease);
    }
    super::transport::classify(error)
}
fn refusal_body(error: &sikaru_sdk::ApiError) -> Option<&str> {
    use fern_cli_sdk::sdk_executor::SdkError;
    match error {
        sikaru_sdk::ApiError::Http {
            status: 409,
            message,
        } => Some(message),
        sikaru_sdk::ApiError::Executor(error) => match error.downcast_ref::<SdkError>() {
            Some(SdkError::Http { status: 409, body }) => Some(body),
            _ => None,
        },
        _ => None,
    }
}
fn safe_name(value: &Value) -> bool {
    value.as_str().is_some_and(|name| {
        name.len() <= 256
            && name
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"@/_-.".contains(&c))
    })
}
fn draft_access_delta(body: &str) -> Option<(Value, String)> {
    let body: Value = serde_json::from_str(body).ok()?;
    let detail = &body["detail"];
    if detail["message"] != "Acknowledge the added or changed draft access" {
        return None;
    }
    let digest = detail.get("accessDigest")?.as_str()?;
    if digest.len() != 64 || !digest.bytes().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let delta = detail.get("accessDelta")?.as_object()?;
    let mut result = json!({});
    for key in ["added", "changed", "removed"] {
        let names: Vec<_> = delta
            .get(key)?
            .as_array()?
            .iter()
            .filter(|v| safe_name(v))
            .take(256)
            .cloned()
            .collect();
        result[key] = json!(names);
    }
    result["capabilities"] = safe_capabilities(delta.get("capabilities"));
    Some((result, digest.to_owned()))
}
fn safe_capabilities(value: Option<&Value>) -> Value {
    let Some(items) = value.and_then(Value::as_array) else {
        return json!([]);
    };
    let changes: Vec<_> = items
        .iter()
        .filter(|item| safe_name(&item["path"]))
        .take(256)
        .map(|item| {
            let mut safe = json!({"path":item["path"]});
            for key in ["before", "after"] {
                match safe_capability_value(item["path"].as_str().unwrap(), &item[key]) {
                    Some(value) => safe[key] = value,
                    None => safe["detailsOmitted"] = json!(true),
                }
            }
            safe
        })
        .collect();
    json!(changes)
}
fn safe_capability_value(path: &str, value: &Value) -> Option<Value> {
    if value.is_null() || value.is_boolean() || value.is_number() {
        return Some(value.clone());
    }
    if path.ends_with(".policy")
        && matches!(value.as_str(), Some("allow" | "deny" | "require_approval"))
    {
        return Some(value.clone());
    }
    if !path.ends_with("allow_domains") && !path.ends_with("block_domains") {
        return None;
    }
    let domains = value.as_array()?;
    if domains.len() > 128 || !domains.iter().all(safe_domain) {
        return None;
    }
    Some(value.clone())
}
fn safe_domain(value: &Value) -> bool {
    value.as_str().is_some_and(|host| {
        host.len() <= 253
            && (host.contains('.') || host == "localhost")
            && host
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b".-*".contains(&c))
    })
}

fn draft_retry(m: &clap::ArgMatches, digest: &str) -> String {
    use super::super::agent_folder::{optional, root, shell_word, slug};
    let mut next = format!(
        "sikaru agent run {} --project {} --folder {} --acknowledge-widening",
        shell_word(&slug(m).unwrap_or_else(|_| "AGENT".into())),
        shell_word(optional(m, "project").unwrap_or("PROJECT")),
        shell_word(
            &root(m)
                .canonicalize()
                .unwrap_or_else(|_| root(m))
                .to_string_lossy()
        )
    );
    next.push_str(&format!(" --access-digest {digest}"));
    for name in ["model", "tenant", "user"] {
        if let Some(value) = optional(m, name) {
            next.push_str(&format!(" --{name} {}", shell_word(value)));
        }
    }
    if optional(m, "compute") == Some("local") {
        next.push_str(&format!(
            " --compute local --workspace {}",
            shell_word(
                &std::path::Path::new(optional(m, "workspace").unwrap_or("."))
                    .canonicalize()
                    .unwrap_or_else(|_| std::path::PathBuf::from(
                        optional(m, "workspace").unwrap_or(".")
                    ))
                    .to_string_lossy()
            )
        ));
    }
    match optional(m, "schedule") {
        Some(name) => next.push_str(&format!(" --schedule {}", shell_word(name))),
        None => next.push_str(" --prompt-file -"),
    }
    next
}

fn unavailable_release_code(body: &str) -> bool {
    serde_json::from_str::<Value>(body)
        .ok()
        .is_some_and(|value| value["code"] == "release_unavailable")
}

pub fn failure(error: &anyhow::Error, reason: &str) -> Value {
    if let Some(refusal) = error.downcast_ref::<DraftAccessRequired>() {
        return json!({"status":"approval_required","reason":"draft_access_requires_review",
            "accessDelta":refusal.delta,"accessDigest":refusal.digest,"next":refusal.next,
            "help":"No session or run started. Review accessDelta before acknowledging; if detailsOmitted, inspect agent info first. For a task retry, pipe the original task to --prompt-file -.",
            "execution":null,"cleanup":"not_started","usage":{"available":false}});
    }
    if let Some(input) = error.downcast_ref::<InvalidInput>() {
        return json!({"status":"failed","reason":"invalid_input","message":input.0,
            "execution":null,"cleanup":"not_started","cancel_acknowledged":null,
            "usage":{"available":false}});
    }
    if error.is::<UnavailableRelease>() {
        return json!({"status":"failed","reason":"agent_release_unavailable",
            "help":"No work started. Ask an agent administrator to upgrade this agent, then retry with the saved state directory. Existing sessions retain their release.",
            "execution":null,"cleanup":"not_started","cancel_acknowledged":null,
            "usage":{"available":false}});
    }
    let mut result = state::failure(reason);
    result["help"] = json!(advice(error));
    result
}

fn advice(error: &anyhow::Error) -> &'static str {
    match error.downcast_ref::<TransportFailure>() {
        Some(TransportFailure::Rejected(401)) => "Authentication failed. Run sikaru login, then resume the saved state directory.",
        Some(TransportFailure::Rejected(402)) => "Billing blocked this request. Check the organization's credits and spending limits in Billing, then resume the saved state directory.",
        Some(TransportFailure::Rejected(403)) => "Access denied. Check that your API key has access to this project and the required scopes.",
        Some(TransportFailure::Rejected(404)) => "Resource not found. Check --project and the agent, or your saved session's availability.",
        Some(TransportFailure::Transient) => "The service could not be reached or is temporarily unavailable. Check connectivity and service status; preserve the state directory for explicit resume.",
        _ => "Preserve the original workspace and state directory. Inspect the saved run before resuming; do not start a replacement run to retry an uncertain action.",
    }
}
