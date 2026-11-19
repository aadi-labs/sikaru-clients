use serde_json::{json, Value};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[path = "support/cli.rs"]
mod cli;
use cli::{listed_commands, request_bodies, sikaru, text};

// `sikaru traces` and `sikaru trajectories` against a mock gateway: upload detects the file's format and posts one trace
// stream request; get prints a run's trajectory.
async fn accepting_gateway() -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/trace-streams"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({"accepted": true})))
        .mount(&server)
        .await;
    server
}

async fn upload(server: &MockServer, name: &str, content: &str) -> std::process::Output {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join(name);
    std::fs::write(&file, content).unwrap();
    let (output, _home) = sikaru(
        server,
        &["traces", "upload", file.to_str().unwrap(), "-p", "p"],
    )
    .await;
    output
}

#[test]
fn traces_help_lists_the_curated_verbs_only() {
    let listed: Vec<String> = listed_commands(&["traces"]).into_iter().collect();
    assert_eq!(listed, ["upload"]);
    let listed: Vec<String> = listed_commands(&["trajectories"]).into_iter().collect();
    assert_eq!(listed, ["get"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn jsonl_spans_upload_as_openinference() {
    let server = accepting_gateway().await;
    let spans = "{\"trace_id\": \"t1\", \"span_id\": \"s1\", \"name\": \"llm\"}\n\n\
                 {\"trace_id\": \"t1\", \"span_id\": \"s2\", \"parent_span_id\": \"s1\"}\n";

    let output = upload(&server, "trace.jsonl", spans).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let sent = &request_bodies(&server, "POST").await[0];
    assert_eq!(sent["format"], "openinference.v1");
    let ids: Vec<&str> = sent["spans"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["span_id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, ["s1", "s2"]);
    assert_eq!(sent["metadata"]["project_id"], "p");
    assert_eq!(sent["metadata"]["source"], "cli");
    assert!(sent.get("dataset").is_none(), "{sent}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn otlp_and_atif_files_upload_in_their_formats() {
    let otlp = json!({"resourceSpans": [{"resource": {}, "scopeSpans": []}]}).to_string();
    let atif = json!({"schema_version": "1.0", "steps": [{"source": "user", "message": "hi"}]})
        .to_string();
    let wrapped = json!({"trajectories": [{"steps": []}, {"steps": []}]}).to_string();
    for (content, format, field, count) in [
        (otlp, "opentelemetry.otlp.v1", "resourceSpans", 1),
        (atif, "harbor.atif.v1", "trajectories", 1),
        (wrapped, "harbor.atif.v1", "trajectories", 2),
    ] {
        let server = accepting_gateway().await;

        let output = upload(&server, "trace.json", &content).await;

        assert!(output.status.success(), "{}", text(&output.stderr));
        let sent = &request_bodies(&server, "POST").await[0];
        assert_eq!(sent["format"], format, "{sent}");
        assert_eq!(sent[field].as_array().unwrap().len(), count, "{sent}");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mixed_or_unknown_files_are_refused_before_sending() {
    let mixed = "{\"trace_id\": \"t1\", \"span_id\": \"s1\"}\n{\"steps\": []}\n";
    for content in [mixed, "{\"hello\": \"world\"}"] {
        let server = accepting_gateway().await;

        let output = upload(&server, "trace.jsonl", content).await;

        assert!(!output.status.success(), "{content}");
        assert!(server.received_requests().await.unwrap().is_empty());
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn trajectories_get_prints_the_runs_trajectory() {
    let server = MockServer::start().await;
    let trajectory = json!({"schema_version": "1.0", "session_id": "run_1",
        "steps": [{"step_id": 1, "source": "user", "message": "summarize open tickets"}]});
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/run_1/trajectory"))
        .respond_with(ResponseTemplate::new(200).set_body_json(trajectory.clone()))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["trajectories", "get", "run_1", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown, trajectory);
}

fn langfuse_row(id: &str, run: Option<&str>, parent: Option<&str>, padding: usize) -> String {
    let mut metadata = serde_json::Map::new();
    if let Some(run) = run {
        metadata.insert("agent_run_id".into(), json!(run));
    }
    if let Some(parent) = parent {
        metadata.insert("parent_agent_run_id".into(), json!(parent));
    }
    json!({"id": id, "traceId": format!("t-{id}"), "type": "GENERATION",
        "startTime": "2026-08-20 10:00:00.000000", "metadata": metadata,
        "input": "x".repeat(padding)})
    .to_string()
}

fn sent_ids(body: &Value) -> Vec<String> {
    body["records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["id"].as_str().unwrap().to_owned())
        .collect()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn small_langfuse_export_uploads_as_one_observations_batch() {
    let server = accepting_gateway().await;
    let export = [
        langfuse_row("o1", Some("ia"), None, 10),
        langfuse_row("o2", Some("ea"), Some("ia"), 10),
        langfuse_row("o3", None, None, 10),
    ]
    .join("\n");

    let output = upload(&server, "export.jsonl", &export).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let bodies = request_bodies(&server, "POST").await;
    assert_eq!(bodies.len(), 1);
    assert_eq!(bodies[0]["format"], "langfuse.observations.v1");
    assert_eq!(sent_ids(&bodies[0]), ["o1", "o2", "o3"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn large_langfuse_export_splits_between_runs_never_inside_one() {
    let server = accepting_gateway().await;
    // Two runs of ~3 MB each: together over one batch, so they must travel separately, and
    // run "a"'s sub-agent row (listed last) must still travel with run "a".
    let mb = 1024 * 1024;
    let export = [
        langfuse_row("a1", Some("a"), None, 3 * mb / 2),
        langfuse_row("b1", Some("b"), None, 3 * mb),
        langfuse_row("a2", Some("a"), None, mb),
        langfuse_row("a-sub", Some("a-child"), Some("a"), 10),
    ]
    .join("\n");

    let output = upload(&server, "export.jsonl", &export).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let batches: Vec<Vec<String>> = request_bodies(&server, "POST").await.iter().map(sent_ids).collect();
    assert_eq!(batches, [vec!["a1", "a2", "a-sub"], vec!["b1"]]);
    let keys: std::collections::HashSet<String> = server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .map(|r| r.headers.get("Idempotency-Key").unwrap().to_str().unwrap().to_owned())
        .collect();
    assert_eq!(keys.len(), 2, "each batch carries its own idempotency key");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn langsmith_and_braintrust_exports_upload_whole_traces_in_their_formats() {
    let langsmith = [
        json!({"id": "r1", "trace_id": "t1", "run_type": "chain", "parent_run_id": null}),
        json!({"id": "r2", "trace_id": "t2", "run_type": "chain", "parent_run_id": null}),
        json!({"id": "r3", "trace_id": "t1", "run_type": "llm", "parent_run_id": "r1"}),
    ];
    let braintrust = [
        json!({"id": "s1", "span_id": "s1", "root_span_id": "s1", "span_parents": []}),
        json!({"id": "s2", "span_id": "s2", "root_span_id": "s1", "span_parents": ["s1"]}),
    ];
    for (rows, format, ids) in [
        (&langsmith[..], "langsmith.runs.v1", vec!["r1", "r3", "r2"]),
        (&braintrust[..], "braintrust.spans.v1", vec!["s1", "s2"]),
    ] {
        let server = accepting_gateway().await;
        let export: Vec<String> = rows.iter().map(Value::to_string).collect();

        let output = upload(&server, "export.jsonl", &export.join("\n")).await;

        assert!(output.status.success(), "{}", text(&output.stderr));
        let bodies = request_bodies(&server, "POST").await;
        assert_eq!(bodies.len(), 1, "{format}");
        assert_eq!(bodies[0]["format"], format);
        assert_eq!(sent_ids(&bodies[0]), ids, "rows of one trace travel together");
    }
}
