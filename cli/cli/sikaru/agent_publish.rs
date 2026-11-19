//! Compare-and-set folder publication, with preview and independent reach acknowledgement. A
//! publish follows its release checks until they settle, reading the publication rather than
//! publishing again: a held release prints what each Grader flagged and the Publish Anyway
//! command, and exits with `EXIT_BLOCKED`, the common release-check outcome code.
use super::{
    agent::{emit, invalid, value},
    agent_folder as folder,
    agent_grader::{self as grader, Reply},
};
use clap::ArgMatches;
use fern_cli_sdk::{error::CliError, openapi::AppContext};
use reqwest::Method;
use serde_json::{json, Value};
use sikaru_sdk::api::*;
use std::io::{IsTerminal, Write};
use std::time::Duration;

const POLL: Duration = Duration::from_secs(2);
pub const PUBLISH_EXITS: &str = "Exit codes:
  0  published, previewed, or (with --no-wait) queued
  1  a release check held the release, or the API refused the request
  2  invalid input or sign-in needed";
/// The refusals of a release its Grader release check held; Publish Anyway can release it.
const HELD: [&str; 2] = ["release_check_blocked", "release_check_unverified"];

pub fn save(
    m: &ArgMatches,
    client: &ApiClient,
    agent: &str,
    base: i64,
    files: &folder::Files,
) -> Result<AgentDocument, CliError> {
    let request = SaveDocument {
        base_revision: base,
        document: None,
        files: Some(
            files
                .iter()
                .map(|(k, v)| (k.clone(), Some(v.clone())))
                .collect(),
        ),
    };
    crate::sdk::block_on(
        client
            .agent_documents
            .save(value(m, "project"), agent, &request, None),
    )
}
pub fn publish(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    if let Some(reason) = folder::optional(m, "anyway") {
        return publish_anyway(m, ctx, reason);
    }
    let client = super::agent_transport::client(ctx);
    let agent = folder::slug(m)?;
    let (revision, live) = publication_revision(m, &client, &agent)?;
    let review = crate::sdk::block_on(client.agent_documents.review(
        value(m, "project"),
        &agent,
        &ReviewDocument {
            revision,
            expected_live_version_id: live,
            ..Default::default()
        },
        None,
    ))?;
    if let Some(next) = reach_next(m, &agent, &review) {
        return emit(m, &json!({"status":"pending","review":review,"next":next}));
    }
    if !confirmed(m, &review)? {
        return emit(
            m,
            &json!({"status":"preview","review":review,"next":preview_next(m, &agent, &review)}),
        );
    }
    let request = PublishDocument {
        revision: review.revision,
        expected_live_version_id: review.live_version_id.clone(),
        expected_access_digest: folder::optional(m, "access-digest").map(str::to_owned),
        acknowledge_widening: Some(m.get_flag("acknowledge-widening")),
        acknowledge_removals: Some(m.get_flag("acknowledge-removals")),
        ..Default::default()
    };
    // Raw, so the receipt keeps its publication id and release-check fields until the SDK has them.
    let follow = Follow {
        m,
        ctx,
        agent: &agent,
        path: document_route(m, &agent, "publish"),
        body: serde_json::to_value(&request).map_err(invalid)?,
    };
    follow.settle(follow.send()?)
}

fn document_route(m: &ArgMatches, agent: &str, rest: &str) -> String {
    let (project, agent) = (grader::seg(value(m, "project")), grader::seg(agent));
    format!("v1/projects/{project}/managed-agents/{agent}/document/{rest}")
}

/// Releases a publication its release check held, with the owner's reason.
fn publish_anyway(m: &ArgMatches, ctx: &AppContext, reason: &str) -> Result<(), CliError> {
    let agent = folder::slug(m)?;
    let publication = folder::optional(m, "publication").expect("clap requires --publication");
    let route = format!("publications/{}/publish-anyway", grader::seg(publication));
    let reply = grader::call(
        ctx,
        Method::POST,
        &document_route(m, &agent, &route),
        Some(&json!({"reason": reason})),
    )?;
    released(m, ctx, &agent, grader::ok(reply)?)
}

/// A publish followed until its release checks settle: released, held, or (`--no-wait`) queued.
struct Follow<'a> {
    m: &'a ArgMatches,
    ctx: &'a AppContext,
    agent: &'a str,
    path: String,
    body: Value,
}

/// Where a pending publication's release check settled.
enum Settled {
    /// Its version is live.
    Live,
    /// The check cleared; publishing the same reviewed body again releases it.
    Passed,
    /// The check blocked it or couldn't verify it: code, message, publication id and results.
    Held(Value),
}

impl Follow<'_> {
    fn send(&self) -> Result<Reply, CliError> {
        grader::call(self.ctx, Method::POST, &self.path, Some(&self.body))
    }

    fn settle(&self, mut reply: Reply) -> Result<(), CliError> {
        let mut waited = false;
        loop {
            if is_held(&reply) {
                return self.held(&held_view(&reply.1));
            }
            let receipt = grader::ok(reply)?;
            if receipt["publicationStatus"] != "pending" || self.m.get_flag("no-wait") {
                return released(self.m, self.ctx, self.agent, receipt);
            }
            announce_wait(&mut waited);
            match self.after_check(&receipt)? {
                Some(next) => reply = next,
                None => return released(self.m, self.ctx, self.agent, completed(receipt)),
            }
        }
    }

    /// The publish's next reply once its check settles; `None` once its version is live. A held
    /// release exits here.
    fn after_check(&self, receipt: &Value) -> Result<Option<Reply>, CliError> {
        match self.watch(receipt)? {
            Settled::Held(view) => self.held(&view).map(|()| None),
            Settled::Passed => self.publish_passed(receipt),
            Settled::Live => Ok(None),
        }
    }

    /// Reads the receipt's publication until its check settles. Reading changes nothing, so a draft
    /// saved meanwhile (a teammate's edit, a dashboard Grader change) never ends the wait.
    fn watch(&self, receipt: &Value) -> Result<Settled, CliError> {
        let publication = receipt["publicationId"].as_str().unwrap_or_default();
        let route = format!("publications/{}", grader::seg(publication));
        let path = document_route(self.m, self.agent, &route);
        loop {
            tokio::task::block_in_place(|| std::thread::sleep(POLL));
            let state = grader::ok(grader::call(self.ctx, Method::GET, &path, None)?)?;
            match state["status"].as_str() {
                Some("pending") => continue,
                Some("released") => return Ok(Settled::Live),
                Some("passed") => return Ok(Settled::Passed),
                Some("held") => return Ok(Settled::Held(state)),
                _ => {
                    return Err(CliError::Other(anyhow::anyhow!(
                    "The publication ended without releasing; review the agent and publish again"
                )))
                }
            }
        }
    }

    /// The one publish of a passed check that releases it; `None` once its version is live anyway.
    fn publish_passed(&self, receipt: &Value) -> Result<Option<Reply>, CliError> {
        let reply = self.send()?;
        if reply.0 != 409 || is_held(&reply) {
            return Ok(Some(reply));
        }
        if self.live(receipt)? {
            return Ok(None); // released between the read and the publish
        }
        eprintln!("The release checks passed, but the draft or live version changed since the review; review and publish again.");
        Err(grader::ok(reply).expect_err("a 409 reply is a refusal"))
    }

    /// Whether the receipt's version is now the agent's live version.
    fn live(&self, receipt: &Value) -> Result<bool, CliError> {
        let agent = grader::managed_agent(self.ctx, value(self.m, "project"), self.agent)?;
        Ok(agent["activeHarnessVersionId"] == receipt["harnessVersionId"])
    }

    /// What each Grader flagged and the exact Publish Anyway command; exits `EXIT_BLOCKED`.
    fn held(&self, view: &Value) -> Result<(), CliError> {
        let publication = view["publicationId"].as_str().unwrap_or_default();
        let project = value(self.m, "project");
        let next = format!(
            "sikaru agent publish {} -p {} --publication {} --anyway \"<reason>\"",
            folder::shell_word(self.agent),
            folder::shell_word(project),
            folder::shell_word(publication)
        );
        let message = view["message"].as_str().unwrap_or_default();
        eprintln!("{message}");
        for result in view["results"].as_array().into_iter().flatten() {
            eprintln!("  {}", check_line(result));
        }
        eprintln!("Fix the agent and publish again, or release it anyway with a reason:\n  {next}");
        // The agent's editor follows this publication from the link, showing the hold and Publish Anyway.
        let id = grader::agent_id(self.ctx, project, self.agent).unwrap_or_default();
        let editor = format!(
            "configure?view=settings&publication={}",
            grader::seg(publication)
        );
        let link = (!id.is_empty()).then(|| grader::agent_page(self.ctx, project, &id, &editor));
        emit(
            self.m,
            &json!({"status": "blocked", "code": view["code"], "message": message,
            "publicationId": publication, "results": view["results"], "next": next, "dashboardUrl": link}),
        )?;
        grader::exit(grader::EXIT_BLOCKED)
    }
}

fn announce_wait(waited: &mut bool) {
    if !std::mem::replace(waited, true) {
        eprintln!("Release checks are running; waiting for them (Ctrl-C leaves them running).");
    }
}

fn is_held((status, body): &Reply) -> bool {
    *status == 409
        && body["code"]
            .as_str()
            .is_some_and(|code| HELD.contains(&code))
}

/// A held publish's refusal in the publication read's terms.
fn held_view(body: &Value) -> Value {
    let detail = &body["detail"];
    json!({"code": body["code"], "message": grader::message(body),
        "publicationId": detail["publicationId"], "results": detail["results"]})
}

fn completed(mut receipt: Value) -> Value {
    receipt["publicationStatus"] = json!("completed");
    receipt
}

/// One Grader's check result: its outcome and the cases flagged only for the new version.
fn check_line(result: &Value) -> String {
    let id = result["graderId"].as_str().unwrap_or("");
    let name = result["name"].as_str().unwrap_or(id);
    let outcome = match result["outcome"].as_str() {
        Some("couldnt_verify") => "couldn't verify",
        Some(other) => other,
        None => "",
    };
    let flagged: Vec<&str> = result["flagged"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    let more = result["flaggedCount"]
        .as_u64()
        .map_or(0, |count| count.saturating_sub(flagged.len() as u64));
    let cases = match (flagged.is_empty(), more) {
        (true, _) => "none".to_owned(),
        (false, 0) => flagged.join(", "),
        (false, more) => format!("{} and {more} more", flagged.join(", ")),
    };
    format!("{name}: {outcome}; flagged only for this version: {cases}")
}

/// The receipt with a link to the agent's releases; a link that cannot be resolved is left out
/// rather than failing a publish that already happened.
fn released(
    m: &ArgMatches,
    ctx: &AppContext,
    agent: &str,
    mut receipt: Value,
) -> Result<(), CliError> {
    let project = value(m, "project");
    let link = grader::agent_id(ctx, project, agent)
        .map(|id| grader::agent_page(ctx, project, &id, "releases"));
    if let Some(link) = &link {
        receipt["dashboardUrl"] = json!(link);
    }
    emit(m, &receipt)?;
    let table = matches!(folder::optional(m, "format"), None | Some("table"));
    if let (Some(link), true) = (
        link,
        table && folder::optional(m, "folder-output").is_none() && std::io::stdout().is_terminal(),
    ) {
        println!("Open in dashboard: {link}");
    }
    Ok(())
}
fn publication_revision(
    m: &ArgMatches,
    client: &ApiClient,
    agent: &str,
) -> Result<(i64, Option<String>), CliError> {
    if let Some(revision) = m.get_one::<i64>("revision") {
        return Ok((
            *revision,
            folder::optional(m, "live-version").map(str::to_owned),
        ));
    }
    let files = folder::read(m)?;
    let draft = current_or_create(m, client, agent, &files)?;
    let metadata = folder::metadata(&folder::root(m))?;
    let saved = if same_files(&draft, &files) {
        draft
    } else {
        check_identity(m, agent, &metadata)?;
        save(m, client, agent, metadata.revision, &files)?
    };
    let current =
        crate::sdk::block_on(client.managed_agents.get(value(m, "project"), agent, None))?;
    let live = current.managed_agent.active_harness_version_id;
    folder::save_metadata(m, agent, saved.revision, live.clone(), &files)?;
    Ok((saved.revision, live))
}
fn same_files(draft: &AgentDocument, files: &folder::Files) -> bool {
    let stored = draft
        .files
        .clone()
        .unwrap_or_else(|| [("agent.md".into(), draft.document.clone())].into());
    stored.into_iter().collect::<folder::Files>() == *files
}
pub fn check_identity(
    m: &ArgMatches,
    agent: &str,
    saved: &folder::Metadata,
) -> Result<(), CliError> {
    if saved.revision != 0 && (saved.agent != agent || saved.project != value(m, "project")) {
        return Err(invalid("Folder metadata belongs to another agent/project; pull that agent into a separate folder"));
    }
    Ok(())
}
fn current_or_create(
    m: &ArgMatches,
    client: &ApiClient,
    agent: &str,
    files: &folder::Files,
) -> Result<AgentDocument, CliError> {
    match crate::sdk::block_on(client.agent_documents.get(value(m, "project"), agent, None)) {
        Ok(draft) => return Ok(draft),
        Err(CliError::Api { code: 404, .. }) => {}
        Err(error) => return Err(error),
    }
    let request: CreateManagedAgentRequest = serde_json::from_value(json!({"agentSlug":agent,
        "source":{"sourceKind":"agent_document","files":files}}))
    .map_err(invalid)?;
    crate::sdk::block_on(client.managed_agents.create_managed_agent(
        value(m, "project"),
        &request,
        None,
    ))?;
    crate::sdk::block_on(client.agent_documents.get(value(m, "project"), agent, None))
}
fn preview_next(m: &ArgMatches, agent: &str, review: &DocumentReview) -> String {
    let live = review
        .live_version_id
        .as_ref()
        .map(|id| format!(" --live-version {}", folder::shell_word(id)))
        .unwrap_or_default();
    format!(
        "sikaru agent publish {} -p {} --revision {}{} --yes",
        folder::shell_word(agent),
        folder::shell_word(value(m, "project")),
        review.revision,
        live
    )
}
fn reach_next(m: &ArgMatches, agent: &str, review: &DocumentReview) -> Option<String> {
    let delta = serde_json::to_value(&review.access_delta).ok()?;
    let widening = ["added", "changed", "capabilities"]
        .iter()
        .any(|key| nonempty(&delta[key]));
    let removal = nonempty(&delta["removed"]);
    if !widening && !removal {
        return None;
    }
    if reach_accepted(m, review, widening, removal) {
        return None;
    }
    Some(format!(
        "sikaru agent publish {agent} -p {} --revision {} --access-digest {}{}{}{} --yes",
        value(m, "project"),
        review.revision,
        review.access_digest,
        review
            .live_version_id
            .as_ref()
            .map(|id| format!(" --live-version {id}"))
            .unwrap_or_default(),
        if widening {
            " --acknowledge-widening"
        } else {
            ""
        },
        if removal {
            " --acknowledge-removals"
        } else {
            ""
        }
    ))
}
fn reach_accepted(m: &ArgMatches, review: &DocumentReview, widening: bool, removal: bool) -> bool {
    folder::optional(m, "access-digest") == Some(review.access_digest.as_str())
        && (!widening || m.get_flag("acknowledge-widening"))
        && (!removal || m.get_flag("acknowledge-removals"))
}
fn nonempty(value: &Value) -> bool {
    value.as_array().is_some_and(|items| !items.is_empty())
}
fn confirmed(m: &ArgMatches, review: &DocumentReview) -> Result<bool, CliError> {
    if m.get_flag("yes") {
        return Ok(true);
    }
    eprintln!("{}", serde_json::to_string_pretty(review).map_err(invalid)?);
    if !std::io::stdin().is_terminal() {
        return Ok(false);
    }
    eprint!("Publish this plan? [y/N] ");
    std::io::stderr().flush().map_err(invalid)?;
    let mut reply = String::new();
    std::io::stdin().read_line(&mut reply).map_err(invalid)?;
    Ok(matches!(
        reply.trim().to_ascii_lowercase().as_str(),
        "y" | "yes"
    ))
}
