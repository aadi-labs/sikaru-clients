use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::process::{Command, Output};
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

// Native `sikaru agent` verbs against a mock gateway: what each verb sends, what it prints, and
// which verbs default help lists.
async fn sikaru(server: &MockServer, args: &[&str]) -> (Output, tempfile::TempDir) {
    let home = tempfile::tempdir().unwrap();
    let mut args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    args.extend([
        "--base-url".into(),
        server.uri(),
        "--format".into(),
        "json".into(),
    ]);
    let (dir, home_path) = (home.path().to_owned(), home.path().to_owned());
    let output = tokio::task::spawn_blocking(move || {
        Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .args(args)
            .current_dir(dir)
            .env("HOME", home_path)
            .env_remove("SIKARU_PROJECT")
            .env_remove("SIKARU_AGENT")
            .env("SIKARU_API_KEY", "test-only")
            .output()
            .unwrap()
    })
    .await
    .unwrap();
    (output, home)
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn agent_summary(slug: &str) -> Value {
    json!({"id": format!("agent_{slug}"), "organizationId": "org", "projectId": "p", "agentSlug": slug,
        "displayName": "Support Bot", "harnessId": slug, "activeHarnessVersionId": "hv_1",
        "compatibilityProfileId": "support-bot-v0", "status": "active", "personal": false})
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_prints_the_agent() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/managed-agents/support-bot"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"managedAgent": agent_summary("support-bot")})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "get", "support-bot", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown["agentSlug"], "support-bot");
    assert_eq!(shown["displayName"], "Support Bot");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_of_an_unknown_agent_fails_with_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/managed-agents/missing"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(json!({"detail": "Managed agent not found"})),
        )
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "get", "missing", "--project", "p"]).await;

    assert!(!output.status.success());
    let shown = format!("{}{}", text(&output.stdout), text(&output.stderr));
    assert!(shown.contains("Managed agent not found"), "{shown}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn list_prints_the_projects_agents() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/managed-agents"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"managedAgents": [agent_summary("support-bot"), agent_summary("sales-bot")]}),
        ))
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "list", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    let slugs: Vec<&str> = shown
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["agentSlug"].as_str().unwrap())
        .collect();
    assert_eq!(slugs, ["support-bot", "sales-bot"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn create_sends_the_agent_document() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/managed-agents"))
        .respond_with(ResponseTemplate::new(201).set_body_json(
            json!({"managedAgent": agent_summary("support-bot"), "harnessVersion": null}),
        ))
        .expect(1)
        .mount(&server)
        .await;
    let document = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(document.path(), "# Support bot\n").unwrap();

    let (output, _home) = sikaru(
        &server,
        &[
            "agent",
            "create",
            "support-bot",
            "-p",
            "p",
            "--file",
            document.path().to_str().unwrap(),
            "--personal",
        ],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let requests = server.received_requests().await.unwrap();
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["agentSlug"], "support-bot");
    assert_eq!(body["personal"], true);
    assert_eq!(body["source"]["sourceKind"], "agent_document");
    assert_eq!(body["source"]["document"], "# Support bot\n");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn publish_sends_the_reviewed_revision_to_the_document_publish_route() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/p/managed-agents/support-bot/document/publish",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/p/managed-agents/support-bot/document/review",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "revision":3,"accessDigest":"digest-1","liveVersionId":"hv_1",
            "accessDelta":{"added":[],"removed":[],"changed":[]}})))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(
        &server,
        &[
            "agent",
            "publish",
            "support-bot",
            "-p",
            "p",
            "--revision",
            "3",
            "--access-digest",
            "digest-1",
            "--live-version",
            "hv_1",
            "--acknowledge-widening",
            "--yes",
        ],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let requests = server.received_requests().await.unwrap();
    let published = requests
        .iter()
        .find(|request| request.url.path().ends_with("/publish"))
        .unwrap();
    let body: Value = serde_json::from_slice(&published.body).unwrap();
    assert_eq!(body["revision"], 3);
    assert_eq!(body["expectedAccessDigest"], "digest-1");
    assert_eq!(body["expectedLiveVersionId"], "hv_1");
    assert_eq!(body["acknowledgeWidening"], true);
    assert_eq!(body["acknowledgeRemovals"], false);
}

fn sse(events: &[Value]) -> ResponseTemplate {
    let body: String = events
        .iter()
        .map(|event| format!("id: {}\ndata: {event}\n\n", event["sequence"]))
        .collect();
    ResponseTemplate::new(200).set_body_raw(body, "text/event-stream")
}

fn event(sequence: i64, event_type: &str) -> Value {
    json!({"id": format!("e{sequence}"), "sequence": sequence, "eventType": event_type,
        "createdAt": "2026-10-25T00:00:00Z", "payload": {"text": format!("step {sequence}")}})
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn logs_follow_resumes_a_closed_stream_until_a_terminal_event() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r/events/stream"))
        .and(query_param("after", "0"))
        .respond_with(sse(&[event(1, "run.started")]))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r/events/stream"))
        .and(query_param("after", "1"))
        .respond_with(sse(&[event(2, "run.completed")]))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "logs", "r", "-p", "p", "-f"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let printed: Vec<Value> = text(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let types: Vec<&str> = printed
        .iter()
        .map(|e| e[0]["eventType"].as_str().unwrap())
        .collect();
    assert_eq!(types, ["run.started", "run.completed"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn logs_follow_stops_at_a_personal_session_it_cannot_read() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r/events/stream"))
        .respond_with(sse(&[event(0, "session.personal")]))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "logs", "r", "-p", "p", "--follow"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert!(text(&output.stdout).contains("session.personal"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn logs_without_follow_prints_recorded_events_once() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r/events"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"events": [event(1, "run.started"), event(2, "run.tool_call.completed")], "nextAfter": 2}),
        ))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "logs", "r", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert!(text(&output.stdout).contains("run.tool_call.completed"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stop_cancels_the_run() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/runs/r/cancel"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "cancelled"})))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "stop", "r", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert!(text(&output.stdout).contains("cancelled"));
}

fn help(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_sikaru"))
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{args:?}: {}",
        text(&output.stderr)
    );
    text(&output.stdout)
}

fn listed_commands(help: &str) -> BTreeSet<String> {
    help.lines()
        .skip_while(|line| !line.starts_with("Commands:"))
        .skip(1)
        .take_while(|line| line.starts_with("  "))
        .filter_map(|line| line.split_whitespace().next())
        .map(str::to_owned)
        .collect()
}

#[test]
fn agent_help_lists_the_curated_verbs_only() {
    let listed = listed_commands(&help(&["agent", "--help"]));
    for verb in [
        "list", "get", "create", "run", "logs", "stop", "pull", "push", "validate", "review",
        "publish", "init", "add", "info",
    ] {
        assert!(listed.contains(verb), "{verb} missing from {listed:?}");
    }
    assert!(!listed.contains("snippets"), "{listed:?}");
    help(&["agent", "snippets", "--help"]);
}

// `agent run` on Sikaru: a session, a turn without a compute attachment, the run's events on
// stderr, and one JSON result whose status sets the exit code.
async fn managed_gateway(final_event: &str, status: &str) -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/p/harnesses/support-bot/execution-sessions",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"session": {
            "id": "s1", "projectId": "p", "harnessId": "support-bot",
            "harnessVersionId": "hv_1", "createdAt": "2026-10-25T00:00:00Z", "nextTurn": 0}})))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/execution-sessions/s1/turns"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "input": {"id": "i1", "idempotencyKey": "k", "mode": "immediate",
                "status": "accepted", "runId": "r1"},
            "run": {"id": "r1", "inputId": "i1", "sessionId": "s1", "status": "queued",
                "harnessVersionId": "hv_1", "traceId": null}})))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r1/events/stream"))
        .respond_with(sse(&[event(1, "run.started"), event(2, final_event)]))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r1/events"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"events": [
            {"id": "e2", "sequence": 2, "eventType": "run.completed",
             "createdAt": "2026-10-25T00:00:00Z", "payload": {"output": "3 tickets open"}}],
            "nextAfter": 2})))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"runId": "r1",
            "status": status, "eventsUrl": "events", "harnessId": "support-bot",
            "harnessVersionId": "hv_1"})),
        )
        .mount(&server)
        .await;
    server
}

async fn turn_bodies(server: &MockServer) -> Vec<Value> {
    server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .filter(|r| r.url.path().ends_with("/turns"))
        .map(|r| serde_json::from_slice(&r.body).unwrap())
        .collect()
}

async fn touched_local_compute(server: &MockServer) -> bool {
    server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .any(|r| r.url.path().contains("/compute-"))
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_run_appends_a_turn_without_compute_and_exits_zero() {
    let server = managed_gateway("run.completed", "completed").await;

    let (output, _home) = sikaru(
        &server,
        &[
            "agent",
            "run",
            "support-bot",
            "summarize open tickets",
            "-p",
            "p",
        ],
    )
    .await;

    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["status"], "completed");
    assert_eq!(result["session_id"], "s1");
    assert_eq!(result["run_id"], "r1");
    assert_eq!(result["final_output"]["output"], "3 tickets open");
    let turns = turn_bodies(&server).await;
    assert_eq!(turns.len(), 1);
    assert_eq!(turns[0]["input"]["goal"], "summarize open tickets");
    assert!(
        turns[0].get("compute_attachment_id").is_none(),
        "{}",
        turns[0]
    );
    assert!(!touched_local_compute(&server).await);
    assert!(
        text(&output.stderr).contains("run.started"),
        "progress streams"
    );
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_run_waiting_for_approval_exits_two() {
    let server = managed_gateway("run.requires_approval", "waiting_for_human").await;

    let (output, _home) = sikaru(
        &server,
        &["agent", "run", "support-bot", "deploy", "-p", "p"],
    )
    .await;

    assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["status"], "approval_required");
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_run_paused_for_funding_exits_five_and_says_how_to_resume() {
    let server = managed_gateway("run.paused", "paused").await;

    let (output, _home) = sikaru(
        &server,
        &["agent", "run", "support-bot", "deploy", "-p", "p"],
    )
    .await;

    assert_eq!(output.status.code(), Some(5), "{}", text(&output.stderr));
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["status"], "paused");
    assert_eq!(
        result["next"],
        "Resolve the billing issue, then run: sikaru agent runs resume --project-id p --run-id r1"
    );
    let cancels = server.received_requests().await.unwrap();
    assert!(
        !cancels.iter().any(|r| r.url.path().ends_with("/cancel")),
        "a paused run is kept"
    );
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_run_out_of_credits_names_the_top_up_before_resume() {
    let server = managed_gateway("run.paused", "paused").await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"runId": "r1",
            "status": "paused", "eventsUrl": "events", "harnessId": "support-bot",
            "harnessVersionId": "hv_1", "pause": {"reason": "credits_exhausted",
            "message": "No credits.", "resumable": true, "requires_action": true,
            "topUp": {"operation": "credit_top_ups_create", "cli": "sikaru credits add",
                "minUsd": "5.00", "maxUsd": "500.00"}}})),
        )
        .with_priority(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(
        &server,
        &["agent", "run", "support-bot", "deploy", "-p", "p"],
    )
    .await;

    assert_eq!(output.status.code(), Some(5), "{}", text(&output.stderr));
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    let next = result["next"].as_str().unwrap();
    let add = next
        .find("sikaru credits add --amount 5.00 --project p")
        .expect(next);
    let resume = next
        .find("sikaru agent runs resume --project-id p --run-id r1")
        .expect(next);
    assert!(add < resume, "{next}");
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_dry_run_and_local_only_options_send_nothing() {
    let server = MockServer::start().await;

    let (dry, _home) = sikaru(
        &server,
        &[
            "agent",
            "run",
            "support-bot",
            "task",
            "-p",
            "p",
            "--dry-run",
        ],
    )
    .await;
    let (resumed, _home) = sikaru(
        &server,
        &[
            "agent",
            "run",
            "support-bot",
            "task",
            "-p",
            "p",
            "--resume",
            "state",
        ],
    )
    .await;

    assert!(dry.status.success(), "{}", text(&dry.stderr));
    let validated: Value = serde_json::from_slice(&dry.stdout).unwrap();
    assert_eq!(validated["dry_run"], true);
    assert_eq!(resumed.status.code(), Some(1));
    let refused: Value = serde_json::from_slice(&resumed.stdout).unwrap();
    assert_eq!(refused["reason"], "invalid_input");
    assert!(server.received_requests().await.unwrap().is_empty());
}

/// An omitted agent uses the local folder; SIKARU_AGENT never silently selects a different agent.
fn run_with_env_agent(server: &MockServer, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sikaru"))
        .args(["--base-url", &server.uri(), "agent", "run"])
        .args(args)
        .args(["-p", "p", "--dry-run", "--format", "json"])
        .env("SIKARU_AGENT", "env-bot")
        .env("SIKARU_API_KEY", "test-only")
        .output()
        .unwrap()
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn run_agent_is_positional_only() {
    let server = MockServer::start().await;

    let missing = run_with_env_agent(&server, &["--prompt", "task"]);
    let named = run_with_env_agent(&server, &["support-bot", "task"]);

    assert!(missing.status.success(), "{}", text(&missing.stderr));
    let inferred: Value = serde_json::from_slice(&missing.stdout).unwrap();
    assert_eq!(inferred["agent"], ".");
    assert!(named.status.success(), "{}", text(&named.stderr));
    let validated: Value = serde_json::from_slice(&named.stdout).unwrap();
    assert_eq!(validated["agent"], "support-bot");
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_conversation_continues_one_session() {
    use std::io::Write;
    use std::os::fd::FromRawFd;
    let server = managed_gateway("run.completed", "completed").await;
    let (mut master, mut slave) = (0, 0);
    let opened = unsafe {
        libc::openpty(
            &mut master,
            &mut slave,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    assert_eq!(opened, 0);
    let mut input = unsafe { std::fs::File::from_raw_fd(master) };
    let terminal = unsafe { std::fs::File::from_raw_fd(slave) };
    let child = tokio::process::Command::new(env!("CARGO_BIN_EXE_sikaru"))
        .args([
            "--base-url",
            &server.uri(),
            "agent",
            "run",
            "support-bot",
            "-p",
            "p",
        ])
        .env("SIKARU_API_KEY", "test-only")
        .env_remove("SIKARU_AGENT")
        .stdin(terminal)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    input
        .write_all(b"first task\nsecond task\n/exit\n")
        .unwrap();

    let output = tokio::time::timeout(std::time::Duration::from_secs(30), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown = text(&output.stderr);
    assert!(
        shown.contains("You>") && shown.contains("3 tickets open"),
        "{shown}"
    );
    let goals: Vec<String> = turn_bodies(&server)
        .await
        .iter()
        .map(|turn| turn["input"]["goal"].as_str().unwrap().trim().to_owned())
        .collect();
    assert_eq!(goals, ["first task", "second task"]);
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn folder_schedule_run_uploads_unsaved_files_and_runs_prompt_once() {
    let server = managed_gateway("run.completed", "completed").await;
    let folder = tempfile::tempdir().unwrap();
    std::fs::create_dir(folder.path().join("schedules")).unwrap();
    std::fs::write(folder.path().join("agent.md"), "# Unsaved\n").unwrap();
    std::fs::write(
        folder.path().join("schedules/brief.md"),
        "---\nevery: 1d\n---\nBrief once.\n",
    )
    .unwrap();
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/agent-documents/validate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "publishable":true,"mentions":[],"errors":[],
            "folder":{"schedules":{"brief":{"prompt":"Brief once.\n"}}}})))
        .mount(&server)
        .await;
    let (output, _) = sikaru(
        &server,
        &[
            "agent",
            "run",
            "support-bot",
            "--folder",
            folder.path().to_str().unwrap(),
            "--schedule",
            "brief",
            "-p",
            "p",
            "-o",
            "json",
        ],
    )
    .await;
    assert!(output.status.success(), "{}", text(&output.stderr));
    let requests = server.received_requests().await.unwrap();
    let session = requests
        .iter()
        .find(|r| r.url.path().ends_with("/execution-sessions"))
        .unwrap();
    let body: Value = serde_json::from_slice(&session.body).unwrap();
    assert_eq!(body["environment"], "draft");
    assert_eq!(body["draft_files"]["agent.md"], "# Unsaved\n");
    assert_eq!(body["draft_schedule"], "brief");
    assert_eq!(turn_bodies(&server).await.len(), 1);
    assert_eq!(
        turn_bodies(&server).await[0]["input"]["goal"],
        "Brief once.\n"
    );
    assert!(!requests
        .iter()
        .any(|r| r.method.as_str() == "PUT" || r.url.path().ends_with("/publish")));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn yes_never_approves_changed_reach() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/managed-agents/support-bot/document/review"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"revision":3,
            "accessDigest":"exact-review-digest","accessDelta":{"added":["browser"],"changed":[],"removed":[]}})))
        .expect(1).mount(&server).await;
    let (output, _) = sikaru(
        &server,
        &[
            "agent",
            "publish",
            "support-bot",
            "-p",
            "p",
            "--revision",
            "3",
            "--yes",
        ],
    )
    .await;
    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown["status"], "pending");
    assert!(shown["next"]
        .as_str()
        .unwrap()
        .contains("--access-digest exact-review-digest"));
    assert!(!server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .any(|r| r.url.path().ends_with("/publish")));
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn folder_admission_refusal_keeps_safe_reach_and_retry_for_both_compute_modes() {
    for compute in ["managed", "local"] {
        let server = MockServer::start().await;
        let folder = tempfile::tempdir().unwrap();
        std::fs::write(folder.path().join("agent.md"), "# Agent\n").unwrap();
        Mock::given(method("POST"))
            .and(path("/v1/projects/p/agent-documents/validate"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"publishable":true,"mentions":[],"errors":[]})),
            )
            .mount(&server)
            .await;
        Mock::given(method("POST")).and(path("/v1/projects/p/harnesses/support-bot/execution-sessions"))
            .respond_with(ResponseTemplate::new(409).set_body_json(json!({"detail":{
                "message":"Acknowledge the added or changed draft access",
                "accessDigest":"a".repeat(64),"accessDelta":{"added":["@app/calendar"],"changed":[],"removed":[],
                    "capabilities":[{"path":"web.browser.enabled","before":false,"after":true},
                        {"path":"web.allow_domains","before":["old.example"],"after":["new.example"]},
                        {"path":"web.allow_domains","before":"Bearer secret-token","after":"private-token"}]},
                "debug":"secret-token"}})))
            .expect(1).mount(&server).await;
        let folder_path = folder.path().to_str().unwrap();
        let mut args = vec![
            "agent",
            "run",
            "support-bot",
            "private user prompt",
            "-p",
            "p",
            "--folder",
            folder_path,
            "--compute",
            compute,
        ];
        if compute == "local" {
            args.extend(["--workspace", folder_path, "--no-workspace-checkpoints"]);
        }
        let (output, _) = sikaru(&server, &args).await;
        assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
        let body: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(body["reason"], "draft_access_requires_review");
        assert_eq!(body["accessDelta"]["added"], json!(["@app/calendar"]));
        assert_eq!(body["accessDelta"]["capabilities"][0]["after"], true);
        assert_eq!(
            body["accessDelta"]["capabilities"][1]["after"],
            json!(["new.example"])
        );
        let next = body["next"].as_str().unwrap();
        assert!(next.contains("--acknowledge-widening"));
        assert!(next.contains(&format!("--access-digest {}", "a".repeat(64))));
        assert!(next.contains("--prompt-file -"));
        for hidden in [
            "secret-token",
            "private-token",
            "private user prompt",
            "Inspect the saved run",
        ] {
            assert!(!text(&output.stdout).contains(hidden));
        }
        assert!(!server.received_requests().await.unwrap().iter().any(|r| r
            .url
            .path()
            .ends_with("/turns")
            || r.url.path().contains("/compute-")));
    }
}

async fn sikaru_in(server: &MockServer, dir: &std::path::Path, args: &[&str]) -> Output {
    let mut args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    args.extend(["--base-url".into(), server.uri()]);
    let dir = dir.to_owned();
    tokio::task::spawn_blocking(move || {
        Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .args(args)
            .current_dir(&dir)
            .env("HOME", &dir)
            .env_remove("SIKARU_PROJECT")
            .env_remove("SIKARU_AGENT")
            .env("SIKARU_API_KEY", "test-only")
            .output()
            .unwrap()
    })
    .await
    .unwrap()
}

const GRADER: &str = "description = \"Refunds\"\n\n[[criteria]]\nquestion = \"Was the refund wrong?\"\n";

fn grader_folder() -> (tempfile::TempDir, std::path::PathBuf) {
    let home = tempfile::tempdir().unwrap();
    let folder = home.path().join("support-bot");
    std::fs::create_dir_all(folder.join("graders")).unwrap();
    std::fs::write(folder.join("agent.md"), "# Support\n").unwrap();
    std::fs::write(folder.join("graders/refunds.toml"), GRADER).unwrap();
    (home, folder)
}

fn draft(revision: i64, grader: &str) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"revision": revision, "document": "# Support\n",
        "files": {"agent.md": "# Support\n", "graders/refunds.toml": grader}}))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn push_sends_grader_files_with_the_folder() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v1/projects/p/managed-agents/support-bot/document"))
        .respond_with(draft(2, GRADER))
        .expect(1)
        .mount(&server)
        .await;
    let (home, folder) = grader_folder();
    let folder_arg = folder.to_string_lossy().into_owned();

    let output = sikaru_in(&server, home.path(), &["agent", "push", "support-bot", "-p", "p", "--revision", "1",
        "--folder", &folder_arg]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let requests = server.received_requests().await.unwrap();
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["files"]["graders/refunds.toml"], GRADER);
    assert_eq!(body["files"]["agent.md"], "# Support\n");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pull_brings_a_dashboard_grader_edit_down_and_refuses_over_a_local_edit() {
    let server = MockServer::start().await;
    let edited = GRADER.replace("Refunds", "Refunds issued wrongly");
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/managed-agents/support-bot/document"))
        .respond_with(draft(3, &edited))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/managed-agents/support-bot/document"))
        .respond_with(draft(4, &GRADER.replace("Refunds", "Refunds, again")))
        .mount(&server)
        .await;
    let (home, folder) = grader_folder();
    let folder_arg = folder.to_string_lossy().into_owned();
    let pull = ["agent", "pull", "support-bot", "-p", "p", "--folder", &folder_arg];
    std::fs::remove_file(folder.join("graders/refunds.toml")).unwrap(); // a folder never pulled has no Grader file

    let first = sikaru_in(&server, home.path(), &pull).await;
    assert!(first.status.success(), "{}", text(&first.stderr));
    assert_eq!(std::fs::read_to_string(folder.join("graders/refunds.toml")).unwrap(), edited);

    let mine = edited.replace("wrongly", "without an order");
    std::fs::write(folder.join("graders/refunds.toml"), &mine).unwrap();
    let second = sikaru_in(&server, home.path(), &pull).await;

    assert!(!second.status.success());
    assert!(text(&second.stdout).contains("graders/refunds.toml") || text(&second.stderr).contains("graders/refunds.toml"));
    assert_eq!(std::fs::read_to_string(folder.join("graders/refunds.toml")).unwrap(), mine);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn add_scaffolds_a_grader_file() {
    let server = MockServer::start().await;
    let home = tempfile::tempdir().unwrap();
    std::fs::write(home.path().join("agent.md"), "# Support\n").unwrap();

    let output = sikaru_in(&server, home.path(), &["agent", "add", "grader/refunds", "-o", "json"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let written = std::fs::read_to_string(home.path().join("graders/refunds.toml")).unwrap();
    assert!(written.contains("[[criteria]]") && written.contains("question = "));
}
