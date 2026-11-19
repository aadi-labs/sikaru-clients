//! `sikaru agent grader` and `agent publish`'s release-check follow against a mock gateway: what each
//! verb sends, what it prints, and the exit code a script branches on (0 ok, 1 blocked or API refusal,
//! 2 usage, sign-in or no data).
#[path = "support/cli.rs"]
mod support;

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use support::{listed_commands, text};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, Request, ResponseTemplate,
};

const GRADERS: &str = "/v1/projects/p/agents/agent_support-bot/graders";
const GRADER_FILE: &str = "description = \"Refunds\"\njudge = \"decision-model\" # the default\n\n\
    [release_check]\nenabled = true\nceiling = 0.25\n\n\
    [[criteria]]\nname = \"refund-not-exchange\"\nquestion = \"Does the agent refund when the customer asked for an \\\"exchange\\\"?\"\n\n\
    [[criteria]]\nquestion = 'Does the agent skip the order lookup?'\n";

/// A pulled agent folder: agent.md, one Grader file, and the metadata naming project `p`.
fn agent_folder() -> (tempfile::TempDir, PathBuf) {
    let home = tempfile::tempdir().unwrap();
    let folder = home.path().join("support-bot");
    std::fs::create_dir_all(folder.join("graders")).unwrap();
    std::fs::write(folder.join("agent.md"), "# Support\n").unwrap();
    std::fs::write(folder.join("graders/refunds.toml"), GRADER_FILE).unwrap();
    std::fs::write(
        folder.join(".sikaru-agent.json"),
        json!({"project": "p", "agent": "support-bot", "baseRevision": 3, "baseLiveVersionId": "hv_1",
            "paths": ["agent.md", "graders/refunds.toml"], "baseHashes": {}})
        .to_string(),
    )
    .unwrap();
    (home, folder)
}

/// The CLI run inside `dir` with no project or agent flags, as an owner would in their folder.
async fn sikaru_in(server: &MockServer, dir: &Path, args: &[&str], env: &[(&str, &str)]) -> Output {
    let mut args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    args.extend(["--base-url".into(), server.uri()]);
    let (dir, home) = (dir.to_owned(), dir.parent().unwrap().to_owned());
    let env: Vec<(String, String)> = env
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    tokio::task::spawn_blocking(move || {
        Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .args(args)
            .current_dir(&dir)
            .env("HOME", &home)
            .env_remove("SIKARU_PROJECT")
            .env_remove("SIKARU_AGENT")
            .env("SIKARU_API_KEY", "test-only")
            .envs(env)
            .output()
            .unwrap()
    })
    .await
    .unwrap()
}

fn agent(active: &str) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"managedAgent": {"id": "agent_support-bot",
        "organizationId": "org", "projectId": "p", "agentSlug": "support-bot", "displayName": "Support Bot",
        "harnessId": "support-bot", "activeHarnessVersionId": active, "compatibilityProfileId": "support-bot-v0",
        "status": "active", "personal": false}}))
}

async fn mount_agent(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/managed-agents/support-bot"))
        .respond_with(agent("hv_1"))
        .mount(server)
        .await;
}

fn summary(id: &str, name: &str, status: &str) -> Value {
    json!({"id": id, "name": name, "description": "", "origin": "owner", "failureId": null, "status": status,
        "refitting": false, "current": {"version": 2, "status": "live", "agreement": 0.91, "heldOut": 40,
        "labelCount": 52, "createdAt": null}, "latest": null, "releaseCheck": {"enabled": true, "ceiling": 0.25},
        "file": {"path": format!("graders/{name}.toml"), "retired": false}})
}

async fn mount_list(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path(GRADERS))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "graders": [summary("grd_1", "refunds", "live"), summary("grd_preset", "Wrong tool choice", "uncalibrated")],
            "grading": {"status": "on", "message": null}, "pause": null, "staleSince": null})))
        .mount(server)
        .await;
}

fn report(flagged: usize, total: usize, would_block: bool) -> ResponseTemplate {
    let mut results: Vec<Value> = (0..flagged)
        .map(|i| {
            json!({"conversationKey": format!("bad_{i}"), "runId": format!("run_bad_{i}"),
            "verdict": "flagged", "reason": "Refunded when an exchange was asked for"})
        })
        .collect();
    results.extend((flagged..total).map(|i| {
        json!({"conversationKey": format!("ok_{i}"),
        "runId": format!("run_ok_{i}"), "verdict": "looks_fine", "reason": null})
    }));
    let rate = if total == 0 {
        Value::Null
    } else {
        json!(flagged as f64 / total as f64)
    };
    ResponseTemplate::new(200).set_body_json(json!({"status": if total == 0 {"no_data"} else {"scored"},
        "conversations": total, "conclusive": total, "flagged": flagged, "flagRate": rate, "ceiling": 0.25,
        "wouldBlock": would_block, "results": results}))
}

async fn bodies(server: &MockServer, suffix: &str) -> Vec<Value> {
    let requests: Vec<Request> = server.received_requests().await.unwrap();
    requests
        .iter()
        .filter(|r| r.url.path().ends_with(suffix))
        .map(|r| serde_json::from_slice(&r.body).unwrap_or(Value::Null))
        .collect()
}

#[test]
fn agent_grader_help_lists_the_seven_verbs() {
    let listed = listed_commands(&["agent", "grader", "--help"]);
    for verb in ["init", "test", "label", "list", "get", "samples", "run"] {
        assert!(listed.contains(verb), "{verb} missing from {listed:?}");
    }
    assert!(listed_commands(&["agent", "--help"]).contains("grader"));
}

/// Init, test --last 20 and publish inside the folder, each ending with a dashboard link.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn first_run_inits_tests_and_publishes_from_the_folder_with_links() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    Mock::given(method("POST"))
        .and(path(format!("{GRADERS}/test")))
        .respond_with(report(2, 3, false))
        .mount(&server)
        .await;
    let (_home, folder) = agent_folder();

    let init = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "init", "exchanges"],
        &[],
    )
    .await;
    assert!(init.status.success(), "{}", text(&init.stderr));
    let starter = std::fs::read_to_string(folder.join("graders/exchanges.toml")).unwrap();
    assert!(starter.contains("[[criteria]]") && starter.contains("question = "));
    assert!(text(&init.stdout)
        .trim_end()
        .lines()
        .last()
        .unwrap()
        .contains("/p/p/graders"));

    let tested = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "test", "refunds", "--last", "20"],
        &[],
    )
    .await;
    assert!(tested.status.success(), "{}", text(&tested.stderr));
    let shown = text(&tested.stdout);
    assert!(shown.contains("2 of 3 flagged"), "{shown}");
    let flagged_at = shown.find("run_bad_1").unwrap();
    assert!(
        flagged_at < shown.find("run_ok_2").unwrap(),
        "flagged runs come first: {shown}"
    );
    assert!(shown.contains("Refunded when an exchange was asked for"));
    let last = shown.trim_end().lines().last().unwrap();
    assert!(
        last.contains("/p/p/agents/agent_support-bot/graders"),
        "{last}"
    );

    // The local file is what was scored, sent as written, with the window asked for.
    let sent = bodies(&server, "/graders/test").await;
    assert_eq!(
        sent[0],
        json!({"name": "refunds", "file": GRADER_FILE, "limit": 20})
    );

    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/p/managed-agents/support-bot/document/review",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"revision": 3, "accessDigest": "d",
            "liveVersionId": "hv_1", "accessDelta": {"added": [], "removed": [], "changed": []}})),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/p/managed-agents/support-bot/document/publish",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"harnessVersionId": "hv_2",
            "publicationStatus": "completed", "releaseCheck": {"status": "passed"}})),
        )
        .mount(&server)
        .await;
    let published = sikaru_in(
        &server,
        &folder,
        &["agent", "publish", "--revision", "3", "--yes", "-o", "json"],
        &[("SIKARU_PROJECT", "p")],
    )
    .await;
    assert!(published.status.success(), "{}", text(&published.stderr));
    let receipt: Value = serde_json::from_slice(&published.stdout).unwrap();
    assert_eq!(receipt["releaseCheck"]["status"], "passed");
    assert!(receipt["dashboardUrl"]
        .as_str()
        .unwrap()
        .ends_with("/p/p/agents/agent_support-bot/releases"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_exits_one_when_the_ceiling_would_block() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    Mock::given(method("POST"))
        .and(path(format!("{GRADERS}/test")))
        .respond_with(report(3, 4, true))
        .mount(&server)
        .await;
    let (_home, folder) = agent_folder();

    let output = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "test", "refunds"],
        &[],
    )
    .await;

    assert_eq!(output.status.code(), Some(1), "{}", text(&output.stderr));
    assert!(text(&output.stdout).contains("3 of 4 flagged"));
    assert_eq!(
        bodies(&server, "/graders/test").await[0]["limit"],
        20,
        "--last defaults to 20"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_exits_two_with_no_recent_runs_and_follows_a_judged_test() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    Mock::given(method("POST"))
        .and(path(format!("{GRADERS}/test")))
        .respond_with(report(0, 0, false))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(format!("{GRADERS}/test")))
        .respond_with(
            ResponseTemplate::new(202)
                .set_body_json(json!({"status": "pending", "runId": "test_1"})),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!("{GRADERS}/tests/test_1")))
        .respond_with(report(1, 4, false))
        .mount(&server)
        .await;
    let (_home, folder) = agent_folder();

    let empty = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "test", "refunds", "-o", "json"],
        &[],
    )
    .await;
    assert_eq!(empty.status.code(), Some(2), "{}", text(&empty.stderr));
    assert_eq!(
        serde_json::from_slice::<Value>(&empty.stdout).unwrap()["status"],
        "no_data"
    );

    let llm_file = GRADER_FILE.replace("\"decision-model\"", "\"llm\"");
    std::fs::write(folder.join("graders/refunds.toml"), &llm_file).unwrap();
    let llm = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "test", "refunds"],
        &[],
    )
    .await;
    assert_eq!(llm.status.code(), Some(0), "{}", text(&llm.stderr));
    assert!(text(&llm.stdout).contains("1 of 4 flagged"));
    assert_eq!(bodies(&server, "/graders/test").await[1]["file"], llm_file);

    let missing = sikaru_in(&server, &folder, &["agent", "grader", "test", "nope"], &[]).await;
    assert_eq!(missing.status.code(), Some(2));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_exits_one_and_says_why_when_a_judged_test_fails() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    Mock::given(method("POST"))
        .and(path(format!("{GRADERS}/test")))
        .respond_with(
            ResponseTemplate::new(202)
                .set_body_json(json!({"status": "pending", "runId": "test_1"})),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!("{GRADERS}/tests/test_1")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "failed", "runId": "test_1", "conversations": 4, "conclusive": 0,
            "flagged": 0, "flagRate": null, "ceiling": null, "wouldBlock": false, "results": [],
            "error": "The judge did not finish in time. Test again to start a new run."
        })))
        .mount(&server)
        .await;
    let (_home, folder) = agent_folder();

    let failed = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "test", "refunds"],
        &[],
    )
    .await;

    assert_eq!(failed.status.code(), Some(1), "{}", text(&failed.stderr));
    let printed = text(&failed.stdout);
    assert!(
        printed.contains("The judge did not finish in time. Test again to start a new run."),
        "{printed}"
    );
    assert!(!printed.contains("0 of 4 flagged"), "{printed}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_names_the_file_and_line_the_gateway_refused() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    Mock::given(method("POST"))
        .and(path(format!("{GRADERS}/test")))
        .respond_with(ResponseTemplate::new(422).set_body_json(json!({"code": "invalid_grader_file",
            "detail": "graders/refunds.toml:4: release_check.ceiling: Input should be less than or equal to 1"})))
        .mount(&server)
        .await;
    let (_home, folder) = agent_folder();

    let output = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "test", "refunds"],
        &[],
    )
    .await;

    assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
    assert!(
        text(&output.stderr).contains("graders/refunds.toml:4: release_check.ceiling"),
        "{}",
        text(&output.stderr)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn folder_metadata_names_the_project_before_the_environment() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    mount_list(&server).await;
    let (_home, folder) = agent_folder();

    let output = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "list"],
        &[("SIKARU_PROJECT", "elsewhere")],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert!(
        text(&output.stdout).contains("Wrong tool choice"),
        "presets are listed without a file"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn label_refuses_to_run_without_a_terminal() {
    let server = MockServer::start().await;
    let (_home, folder) = agent_folder();

    let output = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "label", "refunds"],
        &[],
    )
    .await;

    assert_eq!(output.status.code(), Some(2));
    assert!(
        text(&output.stderr).contains("terminal"),
        "{}",
        text(&output.stderr)
    );
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn every_verb_prints_parseable_json_and_run_ends_with_a_link() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    mount_list(&server).await;
    Mock::given(method("GET"))
        .and(path(format!("{GRADERS}/grd_1")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "grd_1", "name": "refunds",
            "status": "live", "judge": {"kind": "decision-model", "agentId": null}, "escalate": false,
            "releaseCheck": {"enabled": true, "ceiling": 0.25}, "labels": {"total": 3, "flagged": 1, "looksFine": 2},
            "criteria": [{"name": "c", "description": "Was the refund wrong?", "kind": "judge"}]})))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!("{GRADERS}/grd_1/samples")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": 2, "samples": [
            {"conversationKey": "bad", "runId": "run_bad", "verdict": "flagged", "scoredAt": "2026-11-15"},
            {"conversationKey": "ok", "runId": null, "verdict": "looks_fine", "scoredAt": "2026-11-15"}]})))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(format!("{GRADERS}/grd_1/run")))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"runId": "regrade_1",
            "conversations": 120, "estimatedUsd": "0.40"})),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(format!("{GRADERS}/test")))
        .respond_with(report(1, 2, false))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!("{GRADERS}/grd_1/runs/regrade_1")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "runId": "regrade_1", "status": "completed", "conversations": 120,
            "conclusive": 118, "flagged": 7, "estimatedUsd": "0.40", "error": null})))
        .mount(&server)
        .await;
    let (_home, folder) = agent_folder();

    for verb in [
        &["list"][..],
        &["get", "refunds"],
        &["samples", "refunds"],
        &["run", "refunds", "--days", "7"],
        &["test", "refunds"],
        &["init", "late-refunds"],
    ] {
        let args: Vec<&str> = ["agent", "grader"]
            .iter()
            .chain(verb)
            .chain(&["-o", "json"])
            .copied()
            .collect();
        let output = sikaru_in(&server, &folder, &args, &[]).await;
        assert!(
            output.status.success(),
            "{verb:?}: {}",
            text(&output.stderr)
        );
        let parsed: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
            panic!("{verb:?} printed non-JSON ({e}): {}", text(&output.stdout))
        });
        if ["run", "test", "init"].contains(&verb[0]) {
            assert!(
                parsed["dashboardUrl"].as_str().is_some(),
                "{verb:?}: {parsed}"
            );
        }
        if verb[0] == "run" {
            assert_eq!(parsed["status"], "completed");
            assert_eq!(parsed["flagged"], 7);
            // The Grader's page lives under its agent (R21).
            assert!(
                parsed["dashboardUrl"]
                    .as_str()
                    .unwrap()
                    .ends_with("/p/p/agents/agent_support-bot/graders/grd_1"),
                "{parsed}"
            );
        }
    }
    assert_eq!(bodies(&server, "/grd_1/run").await, [json!({"days": 7})]);

    let human = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "samples", "refunds"],
        &[],
    )
    .await;
    let shown = text(&human.stdout);
    assert!(
        shown.contains("Flagged") && shown.contains("Looks Fine"),
        "{shown}"
    );
    let ran = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "run", "refunds"],
        &[],
    )
    .await;
    assert!(text(&ran.stdout)
        .trim_end()
        .lines()
        .last()
        .unwrap()
        .contains("/p/p/agents/agent_support-bot/graders/grd_1"));
}

fn review_and_publish_args() -> Vec<&'static str> {
    vec![
        "agent",
        "publish",
        "support-bot",
        "-p",
        "p",
        "--revision",
        "3",
        "--live-version",
        "hv_1",
        "--access-digest",
        "d",
        "--yes",
        "-o",
        "json",
    ]
}

async fn mount_review(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/p/managed-agents/support-bot/document/review",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"revision": 3, "accessDigest": "d",
            "liveVersionId": "hv_1", "accessDelta": {"added": [], "removed": [], "changed": []}})),
        )
        .mount(server)
        .await;
}

fn pending() -> ResponseTemplate {
    ResponseTemplate::new(202).set_body_json(
        json!({"harnessVersionId": "hv_2", "publicationStatus": "pending",
        "publicationId": "pub_1", "activationId": "document_x"}),
    )
}

const PUBLISH: &str = "/v1/projects/p/managed-agents/support-bot/document/publish";
const PUBLICATION: &str = "/v1/projects/p/managed-agents/support-bot/document/publications/pub_1";

fn publication(status: &str) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"publicationId": "pub_1", "status": status,
        "harnessVersionId": "hv_2", "code": null, "message": null, "results": []}))
}

/// Every publish after the first fails as a draft saved meanwhile makes it fail: following a pending
/// publication must read it, never publish it again.
async fn mount_pending_publish(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path(PUBLISH))
        .respond_with(pending())
        .up_to_n_times(1)
        .mount(server)
        .await;
    Mock::given(method("POST"))
        .and(path(PUBLISH))
        .respond_with(
            ResponseTemplate::new(409).set_body_json(json!({"code": "conflict",
            "detail": {"message": "Draft revision changed", "draft": {"revision": 4}}})),
        )
        .mount(server)
        .await;
}

/// The publication reads pending once, then `then`.
async fn mount_publication(server: &MockServer, then: ResponseTemplate) {
    Mock::given(method("GET"))
        .and(path(PUBLICATION))
        .respond_with(publication("pending"))
        .up_to_n_times(1)
        .mount(server)
        .await;
    Mock::given(method("GET"))
        .and(path(PUBLICATION))
        .respond_with(then)
        .mount(server)
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn publish_follows_a_running_check_to_its_block_and_names_publish_anyway() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    mount_review(&server).await;
    mount_pending_publish(&server).await;
    mount_publication(
        &server,
        ResponseTemplate::new(200).set_body_json(
            json!({"publicationId": "pub_1", "status": "held",
            "harnessVersionId": "hv_2", "code": "release_check_blocked",
            "message": "A release check blocked this version.",
            "results": [{"graderId": "grd_1", "name": "refunds", "outcome": "blocked",
                "flagged": ["conv_a", "conv_b"], "flaggedCount": 14}]}),
        ),
    )
    .await;
    let (_home, folder) = agent_folder();

    let output = sikaru_in(&server, &folder, &review_and_publish_args(), &[]).await;

    assert_eq!(output.status.code(), Some(1), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        (
            shown["status"].clone(),
            shown["code"].clone(),
            shown["publicationId"].clone()
        ),
        (
            json!("blocked"),
            json!("release_check_blocked"),
            json!("pub_1")
        )
    );
    assert_eq!(shown["results"][0]["flagged"], json!(["conv_a", "conv_b"]));
    // The link opens the agent's editor following this publication, where its hold and Publish Anyway are shown.
    assert!(
        shown["dashboardUrl"]
            .as_str()
            .unwrap()
            .ends_with("/p/p/agents/agent_support-bot/configure?view=settings&publication=pub_1"),
        "{shown}"
    );
    let explained = text(&output.stderr);
    assert!(
        explained.contains(
            "refunds: blocked; flagged only for this version: conv_a, conv_b and 12 more"
        ),
        "{explained}"
    );
    assert!(
        explained
            .contains("sikaru agent publish 'support-bot' -p 'p' --publication 'pub_1' --anyway"),
        "{explained}"
    );
    assert_eq!(
        bodies(&server, "/document/publish").await.len(),
        1,
        "a pending publication is read, not published again"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn publish_held_on_the_first_ask_prints_an_unverified_check() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    mount_review(&server).await;
    Mock::given(method("POST"))
        .and(path(PUBLISH))
        .respond_with(ResponseTemplate::new(409).set_body_json(
            json!({"code": "release_check_unverified",
            "detail": {"message": "A release check couldn't verify this version.", "publicationId": "pub_1",
                "results": [{"graderId": "grd_1", "name": null, "outcome": "couldnt_verify",
                    "flagged": [], "flaggedCount": 0}]}}),
        ))
        .mount(&server)
        .await;
    let (_home, folder) = agent_folder();

    let output = sikaru_in(&server, &folder, &review_and_publish_args(), &[]).await;

    assert_eq!(output.status.code(), Some(1), "{}", text(&output.stderr));
    let explained = text(&output.stderr);
    assert!(
        explained.contains("grd_1: couldn't verify; flagged only for this version: none"),
        "{explained}"
    );
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown["code"], "release_check_unverified");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn publish_reports_a_release_without_publishing_again() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    mount_review(&server).await;
    mount_pending_publish(&server).await;
    mount_publication(&server, publication("released")).await;
    let (_home, folder) = agent_folder();

    let output = sikaru_in(&server, &folder, &review_and_publish_args(), &[]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown["publicationStatus"], "completed");
    assert_eq!(bodies(&server, "/document/publish").await.len(), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn publish_sends_the_reviewed_publish_once_more_after_its_check_passes() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    mount_review(&server).await;
    Mock::given(method("POST"))
        .and(path(PUBLISH))
        .respond_with(pending())
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(PUBLISH))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"harnessVersionId": "hv_2",
            "publicationStatus": "completed", "releaseCheck": {"status": "passed"}})),
        )
        .mount(&server)
        .await;
    mount_publication(&server, publication("passed")).await;
    let (_home, folder) = agent_folder();

    let output = sikaru_in(&server, &folder, &review_and_publish_args(), &[]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown["releaseCheck"]["status"], "passed");
    let sent = bodies(&server, "/document/publish").await;
    assert_eq!(sent.len(), 2);
    assert_eq!(sent[0], sent[1], "the same reviewed publish");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn publish_no_wait_returns_at_pending() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    mount_review(&server).await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/p/managed-agents/support-bot/document/publish",
        ))
        .respond_with(pending())
        .expect(1)
        .mount(&server)
        .await;
    let (_home, folder) = agent_folder();
    let mut args = review_and_publish_args();
    args.push("--no-wait");

    let output = sikaru_in(&server, &folder, &args, &[]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        (
            shown["publicationStatus"].clone(),
            shown["publicationId"].clone()
        ),
        (json!("pending"), json!("pub_1"))
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn publish_anyway_sends_the_reason_to_the_held_publication() {
    let server = MockServer::start().await;
    mount_agent(&server).await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/p/managed-agents/support-bot/document/publications/pub_1/publish-anyway",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"harnessVersionId": "hv_2",
            "publicationStatus": "completed", "releaseCheck": {"status": "overridden"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let (_home, folder) = agent_folder();

    let output = sikaru_in(
        &server,
        &folder,
        &[
            "agent",
            "publish",
            "support-bot",
            "-p",
            "p",
            "--publication",
            "pub_1",
            "--anyway",
            "Flags are the known holiday spike",
            "-o",
            "json",
        ],
        &[],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert_eq!(
        bodies(&server, "/publish-anyway").await,
        [json!({"reason": "Flags are the known holiday spike"})]
    );
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown["releaseCheck"]["status"], "overridden");
    assert!(server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .all(|r| !r.url.path().ends_with("/document/publish")));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn init_refuses_a_name_a_grader_file_cannot_have_as_a_usage_error() {
    let server = MockServer::start().await;
    let (_home, folder) = agent_folder();

    let output = sikaru_in(
        &server,
        &folder,
        &["agent", "grader", "init", "Bad_Name"],
        &[],
    )
    .await;

    assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
    assert!(!folder.join("graders/Bad_Name.toml").exists());
}
