//! `sikaru agent grader`: define a Grader as a file in the agent folder, score it on recent runs,
//! teach it with labels, and read what it flagged. Every verb works bare inside an agent folder:
//! the project and agent come from a flag, then the folder's metadata, then `SIKARU_PROJECT` and
//! `SIKARU_AGENT`. `-o json` prints one JSON value; verbs that run or change something end with a
//! dashboard link. Exit codes are listed in `TEST_EXITS`: a would-block result and a missing input
//! each have the common release-check outcome codes.
//!
//! The Grader operations are public API routes the generated SDK gains on its next regeneration;
//! until then they go through the CLI's own transport, as `credits` does.
use super::{
    agent::{invalid, value},
    agent_folder as folder,
};
use clap::{Arg, ArgMatches, Command};
use fern_cli_sdk::{
    error::CliError,
    openapi::AppContext,
    sdk_executor::{SdkError, SdkRequestExecutor},
};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use reqwest::Method;
use serde_json::{json, Value};
use std::io::{BufRead, IsTerminal, Write};
use std::path::Path;

/// Path segments keep the unreserved characters ids and slugs use.
const SEGMENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');
/// The label walk shows a conversation's last steps, each on one capped line.
const EXCERPT_STEPS: usize = 8;
const STEP_CHARS: usize = 400;

/// Invalid input: a bad name, a missing or invalid Grader file (the CLI's own validation code).
pub(super) const EXIT_USAGE: i32 = 2;
/// A release check would block (`grader test`) or held the release (`agent publish`).
pub(super) const EXIT_BLOCKED: i32 = 1;
/// `grader test` found no recent runs to score.
pub(super) const EXIT_NO_DATA: i32 = 2;
const TEST_EXITS: &str = "Exit codes:
  0  scored within the ceiling, or the file's release check is off
  1  would block, the test failed, or the API refused the request
  2  invalid input, sign-in needed, or no recent runs to score";

pub fn command() -> Command {
    let grader = || {
        Arg::new("grader")
            .required(true)
            .help("Grader name, file name or id")
    };
    let name = || {
        Arg::new("name")
            .required(true)
            .help("The Grader file's name, graders/<name>.toml")
    };
    Command::new("grader")
        .about("Define, test and teach the agent's Graders")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(verb("init", "Write a starter Grader file, graders/<name>.toml").arg(name()))
        .subcommand(
            verb(
                "test",
                "Score the local Grader file on the agent's recent runs; saves nothing",
            )
            .after_help(TEST_EXITS)
            .arg(name())
            .arg(
                Arg::new("last")
                    .long("last")
                    .value_parser(clap::value_parser!(i64).range(1..=50))
                    .default_value("20")
                    .help("How many recent runs to score"),
            ),
        )
        .subcommand(
            verb(
                "label",
                "Mark conversations Flagged or Looks Fine, one at a time",
            )
            .arg(grader()),
        )
        .subcommand(verb("list", "List the agent's Graders, presets included"))
        .subcommand(verb("get", "Show one Grader").arg(grader()))
        .subcommand(
            verb(
                "samples",
                "List the Grader's recent verdicts, flagged first",
            )
            .arg(grader()),
        )
        .subcommand(
            verb(
                "run",
                "Re-score recent production runs; datasets run with `sikaru eval run`",
            )
            .arg(grader())
            .arg(
                Arg::new("days")
                    .long("days")
                    .value_parser(clap::value_parser!(i64).range(1..=90))
                    .help("Days of production runs to re-score"),
            ),
        )
}

fn verb(name: &'static str, about: &'static str) -> Command {
    folder::options(
        Command::new(name)
            .about(about)
            .arg(
                Arg::new("project")
                    .long("project")
                    .short('p')
                    .help("Project; defaults to the folder's, then SIKARU_PROJECT"),
            )
            .arg(
                Arg::new("agent")
                    .long("agent")
                    .short('a')
                    .help("Agent slug; defaults to the folder's, then SIKARU_AGENT"),
            ),
    )
}

/// Runs a verb; an input it refuses (a bad name, an unreadable folder) is a usage error.
pub fn handle(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    match verb_of(m, ctx) {
        Err(CliError::Validation(message)) => usage(message),
        result => result,
    }
}

fn verb_of(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let (verb, m) = m.subcommand().expect("clap requires a verb");
    match verb {
        "init" => init(m, ctx),
        "test" => test(m, ctx),
        "label" => label(m, ctx),
        "list" => list(m, ctx),
        "get" => get(m, ctx),
        "samples" => samples(m, ctx),
        _ => rescore(m, ctx),
    }
}

// --- Transport, links and exits ------------------------------------------------------------------

/// A gateway reply: HTTP status and JSON body.
pub(super) type Reply = (u16, Value);

/// One gateway call through the CLI's own transport after the sign-in check. Only transport
/// failures are errors, so callers can read a refusal's code and structured detail.
pub(super) fn call(
    ctx: &AppContext,
    method: Method,
    path: &str,
    body: Option<&Value>,
) -> Result<Reply, CliError> {
    super::inference::blocking(async {
        super::user_auth::ensure_signed_in()?;
        let mutation = method != Method::GET;
        let mut request =
            super::inference::endpoint(ctx, method, path).header("Accept", "application/json");
        if mutation {
            request = request.header(
                "Idempotency-Key",
                format!("{:032x}", rand::random::<u128>()),
            );
        }
        if let Some(body) = body {
            request = request.json(body);
        }
        let request = request.build().map_err(invalid)?;
        let response = ctx
            .build_sdk_executor()
            .execute(request)
            .await
            .map_err(SdkError::into_cli_error)?;
        let status = response.status().as_u16();
        let text = response.text().await.unwrap_or_default();
        Ok((
            status,
            serde_json::from_str(&text).unwrap_or(Value::String(text)),
        ))
    })
}

/// The body of a successful reply; a refusal renders as every generated command renders one.
pub(super) fn ok((status, body): Reply) -> Result<Value, CliError> {
    if (200..300).contains(&status) {
        return Ok(body);
    }
    let text = serde_json::to_string(&body).unwrap_or_default();
    Err(fern_cli_sdk::error::api_error_from_body(status, &text))
}

/// A refusal's readable message, whether `detail` is text or an object carrying `message`.
pub(super) fn message(body: &Value) -> String {
    match &body["detail"] {
        Value::String(text) => text.clone(),
        Value::Object(detail) => detail
            .get("message")
            .and_then(Value::as_str)
            .map_or_else(|| body["detail"].to_string(), str::to_owned),
        Value::Null => body
            .as_str()
            .map_or_else(|| body.to_string(), str::to_owned),
        detail => detail.to_string(),
    }
}

pub(super) fn seg(value: &str) -> String {
    utf8_percent_encode(value, SEGMENT).to_string()
}

/// The body of a successful GET.
fn fetch(ctx: &AppContext, path: &str) -> Result<Value, CliError> {
    ok(call(ctx, Method::GET, path, None)?)
}

/// A managed agent by slug; its `id` names it in Grader routes and dashboard links.
pub(super) fn managed_agent(
    ctx: &AppContext,
    project: &str,
    slug: &str,
) -> Result<Value, CliError> {
    let path = format!("v1/projects/{}/managed-agents/{}", seg(project), seg(slug));
    Ok(fetch(ctx, &path)?["managedAgent"].take())
}

/// A managed agent's id by slug; `None` when it cannot be read.
pub(super) fn agent_id(ctx: &AppContext, project: &str, slug: &str) -> Option<String> {
    managed_agent(ctx, project, slug)
        .ok()
        .and_then(|agent| agent["id"].as_str().map(str::to_owned))
}

/// A dashboard page on the platform host that serves the API host's project (api. -> app.).
pub(super) fn dashboard(ctx: &AppContext, page: &str) -> String {
    let base = ctx.effective_base_url().to_string();
    let origin = reqwest::Url::parse(&base).map_or(base, |mut url| {
        let app = url
            .host_str()
            .and_then(|host| host.strip_prefix("api."))
            .map(|rest| format!("app.{rest}"));
        if let Some(host) = app {
            let _ = url.set_host(Some(&host));
        }
        url.origin().ascii_serialization()
    });
    format!("{origin}/{page}")
}

/// One agent tab, such as `graders` or `releases`.
pub(super) fn agent_page(ctx: &AppContext, project: &str, agent_id: &str, tab: &str) -> String {
    dashboard(
        ctx,
        &format!("p/{}/agents/{}/{tab}", seg(project), seg(agent_id)),
    )
}

/// Exits with `code` after flushing what was printed.
pub(super) fn exit(code: i32) -> ! {
    let _ = std::io::stdout().flush();
    std::process::exit(code)
}

/// A usage or missing-input refusal.
fn usage(message: impl std::fmt::Display) -> ! {
    eprintln!("error: {message}");
    exit(EXIT_USAGE)
}

fn json_mode(m: &ArgMatches) -> bool {
    folder::optional(m, "folder-output") == Some("json")
        || folder::optional(m, "format") == Some("json")
}

/// Prints a verb's result: its JSON with `-o json`, else `human`'s lines; a dashboard link is last.
fn finish(
    m: &ArgMatches,
    mut result: Value,
    link: Option<String>,
    human: impl FnOnce(&Value),
) -> Result<(), CliError> {
    if json_mode(m) {
        if let Some(link) = link {
            result["dashboardUrl"] = json!(link);
        }
        println!("{}", serde_json::to_string(&result).map_err(invalid)?);
        return Ok(());
    }
    human(&result);
    if let Some(link) = link {
        println!("Open in dashboard: {link}");
    }
    Ok(())
}

// --- The agent and its Graders ------------------------------------------------------------------

struct Target {
    project: String,
    id: String,
}

/// A flag, then the folder's metadata, then the environment (which holds the saved project).
fn given(m: &ArgMatches, flag: &str, saved: &str, env: &str) -> Option<String> {
    folder::optional(m, flag)
        .map(str::to_owned)
        .or_else(|| Some(saved.to_owned()).filter(|value| !value.is_empty()))
        .or_else(|| std::env::var(env).ok().filter(|value| !value.is_empty()))
}

fn target(m: &ArgMatches, ctx: &AppContext) -> Result<Target, CliError> {
    let root = folder::root(m);
    let saved = folder::metadata(&root)?;
    let project = given(m, "project", &saved.project, "SIKARU_PROJECT").unwrap_or_else(|| {
        usage("Name the project: run inside a pulled agent folder, pass --project, or set SIKARU_PROJECT")
    });
    let slug =
        given(m, "agent", &saved.agent, "SIKARU_AGENT").unwrap_or_else(|| folder_name(&root));
    let agent = managed_agent(ctx, &project, &slug)?;
    let id = agent["id"]
        .as_str()
        .ok_or_else(|| invalid("The agent has no id"))?;
    Ok(Target {
        id: id.to_owned(),
        project,
    })
}

/// An agent folder not yet pulled is named for its directory, as `agent publish` names it.
fn folder_name(root: &Path) -> String {
    let name = root
        .canonicalize()
        .ok()
        .filter(|path| path.join("agent.md").is_file())
        .and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_default();
    if folder::require_name(&name).is_err() {
        usage("Name the agent: run inside its folder, pass --agent, or set SIKARU_AGENT")
    }
    name
}

/// An agent's Grader routes, by project and agent id.
pub(super) fn graders_route(project: &str, agent_id: &str, rest: &str) -> String {
    format!(
        "v1/projects/{}/agents/{}/graders{rest}",
        seg(project),
        seg(agent_id)
    )
}

fn graders(t: &Target, rest: &str) -> String {
    graders_route(&t.project, &t.id, rest)
}

fn grader_route(t: &Target, grader: &Value, rest: &str) -> String {
    graders(
        t,
        &format!("/{}{rest}", seg(grader["id"].as_str().unwrap_or_default())),
    )
}

/// The Grader's page, under its agent's Graders tab (R21).
fn grader_page(ctx: &AppContext, t: &Target, grader: &Value) -> String {
    let id = grader["id"].as_str().unwrap_or_default();
    agent_page(ctx, &t.project, &t.id, &format!("graders/{}", seg(id)))
}

/// The verb's agent and its Grader argument.
fn chosen(m: &ArgMatches, ctx: &AppContext) -> Result<(Target, Value), CliError> {
    let t = target(m, ctx)?;
    let grader = find(ctx, &t, value(m, "grader"))?;
    Ok((t, grader))
}

/// A Grader by id, file name or name (presets have no file).
fn find(ctx: &AppContext, t: &Target, wanted: &str) -> Result<Value, CliError> {
    let listed = fetch(ctx, &graders(t, ""))?;
    let file = format!("graders/{wanted}.toml");
    let named = |grader: &&Value| {
        grader["id"] == wanted
            || grader["file"]["path"] == file.as_str()
            || grader["name"]
                .as_str()
                .is_some_and(|name| name.eq_ignore_ascii_case(wanted))
    };
    match listed["graders"]
        .as_array()
        .into_iter()
        .flatten()
        .find(named)
    {
        Some(grader) => Ok(grader.clone()),
        None => usage(format!(
            "No Grader named {wanted}; see `sikaru agent grader list`"
        )),
    }
}

// --- Words --------------------------------------------------------------------------------------

fn verdict_word(verdict: &Value) -> &'static str {
    match verdict.as_str() {
        Some("flagged") => "Flagged",
        Some("looks_fine") => "Looks Fine",
        _ => "Inconclusive",
    }
}

fn percent(share: f64) -> String {
    format!("{:.0}%", share * 100.0)
}

fn judge_word(judge: &Value) -> String {
    match judge["kind"].as_str() {
        Some("llm") => "LLM judge".into(),
        Some("agent") => format!("Agent judge ({})", judge["agentId"].as_str().unwrap_or("")),
        _ => "Decision model".into(),
    }
}

fn summary_line(grader: &Value) -> String {
    let current = &grader["current"];
    let agreement = current["agreement"]
        .as_f64()
        .map(|share| {
            format!(
                ", {} held-out agreement on {}",
                percent(share),
                current["heldOut"]
            )
        })
        .unwrap_or_default();
    let check = if grader["releaseCheck"]["enabled"] == true {
        ", release check on"
    } else {
        ""
    };
    let name = grader["name"].as_str().unwrap_or("");
    format!(
        "{name}  {}{agreement}{check}",
        grader["status"].as_str().unwrap_or("")
    )
}

// --- Verbs --------------------------------------------------------------------------------------

fn init(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let name = value(m, "name");
    let root = folder::root(m);
    let path = folder::scaffold(&root, "grader", name, None)?;
    let saved = folder::metadata(&root)?;
    let link = given(m, "project", &saved.project, "SIKARU_PROJECT")
        .map(|project| dashboard(ctx, &format!("p/{}/graders", seg(&project))));
    let next = format!("sikaru agent grader test {name}");
    finish(m, json!({"path": &path, "next": &next}), link, |_| {
        println!("Wrote {path}. Write its yes/no questions (a yes flags a run), then score it on recent runs:");
        println!("  {next}");
    })
}

fn test(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let body = test_body(m, value(m, "name"));
    let t = target(m, ctx)?;
    let (status, reply) = call(ctx, Method::POST, &graders(&t, "/test"), Some(&body))?;
    if untestable(status, &reply) {
        usage(message(&reply))
    }
    let mut report = ok((status, reply))?;
    while report["status"] == "pending" {
        let identifier = report["runId"]
            .as_str()
            .ok_or_else(|| invalid("The Grader test has no run id"))?;
        let path = graders(&t, &format!("/tests/{}", seg(identifier)));
        tokio::task::block_in_place(|| std::thread::sleep(std::time::Duration::from_secs(2)));
        report = fetch(ctx, &path)?;
    }
    let code = test_exit(&report);
    finish(
        m,
        report,
        Some(agent_page(ctx, &t.project, &t.id, "graders")),
        print_test,
    )?;
    if code != 0 {
        exit(code)
    }
    Ok(())
}

/// An invalid file; its path and line are in the gateway message.
fn untestable(status: u16, reply: &Value) -> bool {
    matches!(
        (status, reply["code"].as_str()),
        (422, Some("invalid_grader_file"))
    )
}

fn test_exit(report: &Value) -> i32 {
    match (report["status"].as_str(), report["wouldBlock"].as_bool()) {
        (Some("no_data"), _) => EXIT_NO_DATA,
        (Some("failed"), _) | (_, Some(true)) => EXIT_BLOCKED,
        _ => 0,
    }
}

/// The local Grader file as written, with the window to score; the gateway reads it as a push does.
fn test_body(m: &ArgMatches, name: &str) -> Value {
    if folder::require_name(name).is_err() {
        usage("Grader names are lowercase letters, digits and single hyphens")
    }
    let relative = format!("graders/{name}.toml");
    let text = std::fs::read_to_string(folder::root(m).join(&relative)).unwrap_or_else(|_| {
        usage(format!(
            "No {relative} here; start one with `sikaru agent grader init {name}`"
        ))
    });
    json!({"name": name, "file": text, "limit": m.get_one::<i64>("last")})
}

fn print_test(report: &Value) {
    if report["status"] == "no_data" {
        println!("No recent runs to score yet; run the agent, then test again.");
        return;
    }
    if report["status"] == "failed" {
        println!(
            "Could not test the Grader: {}",
            report["error"].as_str().unwrap_or("Test again.")
        );
        return;
    }
    println!(
        "{} of {} flagged",
        report["flagged"], report["conversations"]
    );
    let mut results: Vec<&Value> = report["results"].as_array().into_iter().flatten().collect();
    results.sort_by_key(|item| item["verdict"] != "flagged");
    for item in results {
        let run = item["runId"]
            .as_str()
            .or(item["conversationKey"].as_str())
            .unwrap_or("");
        let reason = item["reason"].as_str().unwrap_or("");
        println!("  {:<12} {run}  {reason}", verdict_word(&item["verdict"]));
    }
    if let (Some(rate), Some(ceiling)) = (report["flagRate"].as_f64(), report["ceiling"].as_f64()) {
        let side = if report["wouldBlock"] == true {
            "above"
        } else {
            "within"
        };
        println!(
            "Flag rate {} is {side} the release check ceiling {}",
            percent(rate),
            percent(ceiling)
        );
    }
}

enum Answer {
    Label(&'static str),
    Skip,
    Quit,
}

enum Step {
    Labeled(Value),
    Skipped,
    Quit,
}

fn label(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    if !std::io::stdin().is_terminal() {
        usage("grader label asks for each answer, so run it in a terminal, or label on the Grader's dashboard page")
    }
    let (t, grader) = chosen(m, ctx)?;
    let name = grader["name"].as_str().unwrap_or_default().to_owned();
    let queue = fetch(ctx, &grader_route(&t, &grader, "/label-queue"))?;
    let items = queue["items"].as_array().cloned().unwrap_or_default();
    if items.is_empty() {
        eprintln!(
            "Nothing to label right now: {name} has no unlabeled conversations it is unsure about."
        );
    }
    let route = grader_route(&t, &grader, "/labels");
    let (mut labeled, mut skipped, mut labels) = (0, 0, queue["labels"].clone());
    for (index, item) in items.iter().enumerate() {
        show(item, index, items.len());
        match ask_and_label(ctx, &route, &name, item)? {
            Step::Quit => break,
            Step::Skipped => skipped += 1,
            Step::Labeled(reply) => (labeled, labels) = (labeled + 1, reply["labels"].clone()),
        }
    }
    let summary = json!({"labeled": labeled, "skipped": skipped, "labels": labels});
    finish(m, summary, Some(grader_page(ctx, &t, &grader)), |summary| {
        println!(
            "Labeled {labeled}, skipped {skipped}; {name} has {} labels.",
            summary["labels"]["total"]
        );
    })
}

/// One answer for the shown conversation; a label is saved, then the Grader's own verdict revealed.
fn ask_and_label(
    ctx: &AppContext,
    route: &str,
    name: &str,
    item: &Value,
) -> Result<Step, CliError> {
    let verdict = match answer()? {
        Answer::Quit => return Ok(Step::Quit),
        Answer::Skip => return Ok(Step::Skipped),
        Answer::Label(verdict) => verdict,
    };
    let reason = prompt("Reason (optional): ")?.filter(|reason| !reason.is_empty());
    let body =
        json!({"conversationKey": item["conversationKey"], "verdict": verdict, "reason": reason});
    let reply = ok(call(ctx, Method::PUT, route, Some(&body))?)?;
    let said = reply["graderVerdict"]
        .as_str()
        .map_or("no verdict yet", |_| verdict_word(&reply["graderVerdict"]));
    eprintln!(
        "Saved {}. {name} said: {said}.",
        verdict_word(&reply["label"]["verdict"])
    );
    Ok(Step::Labeled(reply))
}

/// The conversation before any verdict: its last steps, one capped line each.
fn show(item: &Value, index: usize, total: usize) {
    let run = item["runId"]
        .as_str()
        .map(|run| format!(" (run {run})"))
        .unwrap_or_default();
    eprintln!("\nConversation {} of {total}{run}", index + 1);
    let steps = item["excerpt"].as_array().cloned().unwrap_or_default();
    for step in steps.iter().skip(steps.len().saturating_sub(EXCERPT_STEPS)) {
        let text = step["message"]
            .as_str()
            .unwrap_or("")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let capped: String = text.chars().take(STEP_CHARS).collect();
        let more = if capped.len() < text.len() { "..." } else { "" };
        eprintln!(
            "  {}: {capped}{more}",
            step["source"].as_str().unwrap_or("step")
        );
    }
}

fn answer() -> Result<Answer, CliError> {
    loop {
        let reply = prompt("[f] Flagged  [l] Looks Fine  [s] Skip  [q] Quit: ")?;
        match reply.map(|text| text.to_ascii_lowercase()).as_deref() {
            None | Some("q") => return Ok(Answer::Quit),
            Some("f") => return Ok(Answer::Label("flagged")),
            Some("l") => return Ok(Answer::Label("looks_fine")),
            Some("s") => return Ok(Answer::Skip),
            Some(_) => {}
        }
    }
}

/// One trimmed line from the terminal; `None` at end of input.
fn prompt(question: &str) -> Result<Option<String>, CliError> {
    eprint!("{question}");
    std::io::stderr().flush().map_err(invalid)?;
    let mut line = String::new();
    let read = std::io::stdin()
        .lock()
        .read_line(&mut line)
        .map_err(invalid)?;
    Ok((read > 0).then(|| line.trim().to_owned()))
}

fn list(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let t = target(m, ctx)?;
    let listed = fetch(ctx, &graders(&t, ""))?;
    finish(m, listed, None, |listed| {
        if listed["grading"]["status"] == "off" {
            println!(
                "{}",
                listed["grading"]["message"]
                    .as_str()
                    .unwrap_or("Grading is off for this agent.")
            );
        }
        let all = listed["graders"].as_array().cloned().unwrap_or_default();
        if all.is_empty() {
            println!("No Graders yet; start one with `sikaru agent grader init <name>`.");
        }
        for grader in &all {
            println!("{}", summary_line(grader));
        }
    })
}

fn get(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let (t, grader) = chosen(m, ctx)?;
    let detail = fetch(ctx, &grader_route(&t, &grader, ""))?;
    finish(m, detail, None, print_detail)
}

fn print_detail(grader: &Value) {
    println!("{}", summary_line(grader));
    if let Some(description) = grader["description"]
        .as_str()
        .filter(|text| !text.is_empty())
    {
        println!("  {description}");
    }
    let escalates = if grader["escalate"] == true {
        ", escalates uncertain runs to the LLM judge"
    } else {
        ""
    };
    println!("  Judge: {}{escalates}", judge_word(&grader["judge"]));
    let check = &grader["releaseCheck"];
    let ceiling = check["ceiling"]
        .as_f64()
        .map(|share| format!(", ceiling {}", percent(share)))
        .unwrap_or_default();
    let state = if check["enabled"] == true {
        "on"
    } else {
        "off"
    };
    println!("  Release check: {state}{ceiling}");
    let labels = &grader["labels"];
    println!(
        "  Labels: {} ({} Flagged, {} Looks Fine)",
        labels["total"], labels["flagged"], labels["looksFine"]
    );
    for criterion in grader["criteria"].as_array().into_iter().flatten() {
        println!("  - {}", criterion["description"].as_str().unwrap_or(""));
    }
}

fn samples(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let (t, grader) = chosen(m, ctx)?;
    let read = fetch(ctx, &grader_route(&t, &grader, "/samples"))?;
    finish(m, read, None, |read| {
        let all = read["samples"].as_array().cloned().unwrap_or_default();
        if all.is_empty() {
            println!("No verdicts yet.");
        }
        for sample in &all {
            println!(
                "{:<12} {}  {}  {}",
                verdict_word(&sample["verdict"]),
                sample["conversationKey"].as_str().unwrap_or(""),
                sample["runId"].as_str().unwrap_or("-"),
                sample["scoredAt"].as_str().unwrap_or("")
            );
        }
    })
}

fn rescore(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let (t, grader) = chosen(m, ctx)?;
    let body = m
        .get_one::<i64>("days")
        .map_or_else(|| json!({}), |days| json!({"days": days}));
    let mut started = ok(call(
        ctx,
        Method::POST,
        &grader_route(&t, &grader, "/run"),
        Some(&body),
    )?)?;
    if !json_mode(m) {
        eprintln!(
            "Scoring {} recent conversations. Waiting for the result...",
            started["conversations"]
        );
    }
    let identifier = started["runId"]
        .as_str()
        .ok_or_else(|| invalid("The Grader run has no id"))?
        .to_owned();
    let path = grader_route(&t, &grader, &format!("/runs/{}", seg(&identifier)));
    loop {
        started = fetch(ctx, &path)?;
        if !matches!(started["status"].as_str(), Some("queued" | "running")) {
            break;
        }
        tokio::task::block_in_place(|| std::thread::sleep(std::time::Duration::from_secs(2)));
    }
    let failed = started["status"] == "failed";
    let name = grader["name"].as_str().unwrap_or_default();
    finish(m, started, Some(grader_page(ctx, &t, &grader)), |started| {
        let estimate = started["estimatedUsd"]
            .as_str()
            .map(|cost| format!("estimated ${cost}"))
            .unwrap_or_else(|| "estimate unavailable".into());
        if failed {
            println!(
                "Could not re-score {name}: {}",
                started["error"].as_str().unwrap_or("Try again.")
            );
        } else {
            println!(
                "Scored {} conversations with {name}: {} flagged ({estimate}).",
                started["conversations"], started["flagged"]
            );
        }
    })?;
    if failed {
        exit(1)
    }
    Ok(())
}
