//! Local folder I/O and server-owned compilation. No contract compiler lives in the CLI.
use super::agent::{emit, invalid, value};
use clap::{Arg, ArgAction, ArgMatches, Command};
use fern_cli_sdk::{error::CliError, openapi::AppContext};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sikaru_sdk::api::*;
use std::{
    collections::BTreeMap,
    fs,
    io::{IsTerminal, Read},
    path::{Path, PathBuf},
};

const METADATA: &str = ".sikaru-agent.json";
pub type Files = BTreeMap<String, String>;
#[derive(Default, Serialize, Deserialize)]
pub struct Metadata {
    pub project: String,
    pub agent: String,
    #[serde(rename = "baseRevision")]
    pub revision: i64,
    #[serde(rename = "baseLiveVersionId")]
    pub live: Option<String>,
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default, rename = "baseHashes")]
    pub hashes: BTreeMap<String, String>,
}

pub fn options(command: Command) -> Command {
    command
        .arg(
            Arg::new("folder")
                .long("folder")
                .help("Agent folder (defaults to current directory)"),
        )
        .arg(
            Arg::new("folder-output")
                .long("output")
                .short('o')
                .value_parser(["json"])
                .help("Print JSON"),
        )
}
pub fn root(m: &ArgMatches) -> PathBuf {
    if let Some(folder) = optional(m, "folder") {
        return PathBuf::from(folder);
    }
    optional(m, "file")
        .map(PathBuf::from)
        .and_then(|p| p.parent().map(Path::to_owned))
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| PathBuf::from("."))
}
pub fn optional<'a>(m: &'a ArgMatches, name: &str) -> Option<&'a str> {
    m.try_get_one::<String>(name)
        .ok()
        .flatten()
        .map(String::as_str)
}
pub fn metadata(root: &Path) -> Result<Metadata, CliError> {
    let path = root.join(METADATA);
    if !path.exists() {
        return Ok(Metadata::default());
    }
    safe_file(&path)?;
    serde_json::from_str(&read_text(&path)?).map_err(invalid)
}
pub fn slug(m: &ArgMatches) -> Result<String, CliError> {
    if let Some(agent) = optional(m, "agent").filter(|name| *name != ".") {
        return Ok(agent.to_owned());
    }
    let root = root(m);
    let saved = metadata(&root)?;
    if !saved.agent.is_empty() {
        return Ok(saved.agent);
    }
    let canonical = root.canonicalize().map_err(invalid)?;
    let name = canonical.file_name().and_then(|n| n.to_str()).unwrap_or("");
    require_name(name)?;
    Ok(name.to_owned())
}
pub fn require_name(name: &str) -> Result<(), CliError> {
    if name.is_empty()
        || name.split('-').any(|p| {
            p.is_empty()
                || !p
                    .bytes()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
    {
        return Err(invalid(
            "Use lowercase letters, digits and single hyphens for names",
        ));
    }
    Ok(())
}
fn ignored(name: &str) -> bool {
    name.starts_with('.') || name.ends_with(".tfvars")
}
fn safe_file(path: &Path) -> Result<(), CliError> {
    if fs::symlink_metadata(path)
        .map_err(invalid)?
        .file_type()
        .is_symlink()
    {
        return Err(invalid(format!("Refusing symlink: {}", path.display())));
    }
    Ok(())
}
fn read_text(path: &Path) -> Result<String, CliError> {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    if name.starts_with(".env") || name.ends_with(".tfvars") {
        return Err(invalid("Secret configuration files are not agent inputs"));
    }
    safe_file(path)?;
    let mut text = String::new();
    fs::File::open(path)
        .map_err(invalid)?
        .take(1_000_001)
        .read_to_string(&mut text)
        .map_err(invalid)?;
    if text.len() > 1_000_000 || text.contains('\0') {
        return Err(invalid("Folder files must be text, at most 1 MB"));
    }
    Ok(text)
}
pub fn read(m: &ArgMatches) -> Result<Files, CliError> {
    let root = root(m);
    safe_file(&root)?;
    reject_legacy(&root)?;
    let source = document_path(m);
    let mut files = Files::from([("agent.md".into(), read_text(&source)?)]);
    for dir in ["subagents", "schedules", "channels", "skills", "graders"] {
        if root.join(dir).exists() {
            walk(&root, &root.join(dir), &mut files)?;
        }
    }
    Ok(files)
}
fn document_path(m: &ArgMatches) -> PathBuf {
    if optional(m, "file").is_some()
        && m.value_source("file") == Some(clap::parser::ValueSource::CommandLine)
    {
        PathBuf::from(optional(m, "file").unwrap())
    } else {
        root(m).join("agent.md")
    }
}
fn pull_destination(m: &ArgMatches, name: &str) -> Result<PathBuf, CliError> {
    if name != "agent.md" {
        return destination(&root(m), name);
    }
    let path = document_path(m);
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
    if ignored(&file_name) {
        return Err(invalid(
            "Cannot write an agent document to a hidden or configuration file",
        ));
    }
    if path.exists() || path.is_symlink() {
        safe_file(&path)?;
    }
    Ok(path)
}
fn reject_legacy(root: &Path) -> Result<(), CliError> {
    let old_file = ["sikaru.json", "customer-definition.json"]
        .iter()
        .any(|name| root.join(name).exists());
    if old_file || legacy_agents(&root.join("agents"))? {
        return Err(invalid(
            "Legacy folder declarations are unsupported; start with sikaru agent init",
        ));
    }
    Ok(())
}
fn legacy_agents(path: &Path) -> Result<bool, CliError> {
    if !path.is_dir() {
        return Ok(false);
    }
    safe_file(path)?;
    for entry in fs::read_dir(path).map_err(invalid)? {
        let entry = entry.map_err(invalid)?;
        if entry.path().join("instructions.md").exists() {
            return Ok(true);
        }
    }
    Ok(false)
}
fn walk(root: &Path, dir: &Path, files: &mut Files) -> Result<(), CliError> {
    safe_file(dir)?;
    for entry in fs::read_dir(dir).map_err(invalid)? {
        let entry = entry.map_err(invalid)?;
        if ignored(&entry.file_name().to_string_lossy()) {
            continue;
        }
        collect(root, &entry.path(), files)?;
    }
    Ok(())
}
fn collect(root: &Path, path: &Path, files: &mut Files) -> Result<(), CliError> {
    safe_file(path)?;
    if path.is_dir() {
        return walk(root, path, files);
    }
    let relative = path
        .strip_prefix(root)
        .map_err(invalid)?
        .to_str()
        .ok_or_else(|| invalid("Folder paths must be UTF-8"))?;
    files.insert(relative.replace('\\', "/"), read_text(path)?);
    if files.len() > 256 || files.values().map(String::len).sum::<usize>() > 1_000_000 {
        return Err(invalid("Agent folder exceeds 256 files or 1 MB"));
    }
    Ok(())
}
pub fn input(files: &Files) -> Result<DocumentInput, CliError> {
    serde_json::from_value(json!({"files": files})).map_err(invalid)
}
pub fn compile(m: &ArgMatches, client: &ApiClient, files: &Files) -> Result<Value, CliError> {
    let checked = crate::sdk::block_on(client.agent_documents.validate_text(
        value(m, "project"),
        &input(files)?,
        None,
    ))?;
    serde_json::to_value(checked).map_err(invalid)
}
pub fn save_metadata(
    m: &ArgMatches,
    agent: &str,
    revision: i64,
    live: Option<String>,
    files: &Files,
) -> Result<(), CliError> {
    let path = root(m).join(METADATA);
    if path.exists() {
        safe_file(&path)?;
    }
    let saved = Metadata {
        project: value(m, "project").into(),
        agent: agent.into(),
        revision,
        live,
        paths: files.keys().cloned().collect(),
        hashes: files
            .iter()
            .map(|(name, text)| (name.clone(), content_hash(text)))
            .collect(),
    };
    fs::write(path, serde_json::to_vec_pretty(&saved).map_err(invalid)?).map_err(invalid)
}
fn destination(root: &Path, name: &str) -> Result<PathBuf, CliError> {
    let path = Path::new(name);
    let known = name == "agent.md"
        || ["subagents/", "schedules/", "channels/", "skills/", "graders/"]
            .iter()
            .any(|prefix| name.starts_with(prefix));
    if !known {
        return Err(invalid("Unsupported folder path returned by server"));
    }
    if path
        .components()
        .any(|c| !matches!(c, std::path::Component::Normal(_)))
        || name.contains('\\')
    {
        return Err(invalid("Unsafe folder path returned by server"));
    }
    let mut checked = root.to_owned();
    for part in path.components() {
        checked.push(part);
        if checked.exists() || checked.is_symlink() {
            safe_file(&checked)?;
        }
    }
    Ok(checked)
}
fn content_hash(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}
fn pull_conflict(
    path: &Path,
    name: &str,
    incoming: Option<&String>,
    old: &Metadata,
) -> Result<bool, CliError> {
    if !path.exists() {
        return Ok(false);
    }
    let local = read_text(path)?;
    if incoming.is_some_and(|text| *text == local) {
        return Ok(false);
    }
    Ok(old.hashes.get(name) != Some(&content_hash(&local)))
}
fn pull_plan(
    m: &ArgMatches,
    files: &Files,
    old: &Metadata,
) -> Result<Vec<(String, PathBuf)>, CliError> {
    let names: std::collections::BTreeSet<_> =
        old.paths.iter().chain(files.keys()).cloned().collect();
    let mut plan = Vec::new();
    let mut conflicts = Vec::new();
    for name in names {
        let path = pull_destination(m, &name)?;
        if !m.get_flag("force") && pull_conflict(&path, &name, files.get(&name), old)? {
            conflicts.push(name.clone());
        }
        plan.push((name, path));
    }
    if !conflicts.is_empty() {
        return Err(invalid(format!("Pull would replace local edits or unverified files: {}. Preserve them, or explicitly use --force to replace them.",conflicts.join(", "))));
    }
    Ok(plan)
}
pub fn materialize(m: &ArgMatches, files: &Files) -> Result<(), CliError> {
    let root = root(m);
    if root.exists() {
        safe_file(&root)?;
    }
    let old = metadata(&root)?;
    let plan = pull_plan(m, files, &old)?;
    for (name, path) in plan {
        match files.get(&name) {
            Some(text) => {
                fs::create_dir_all(path.parent().unwrap()).map_err(invalid)?;
                fs::write(path, text.as_bytes()).map_err(invalid)?;
            }
            None if path.is_file() => fs::remove_file(path).map_err(invalid)?,
            None => {}
        }
    }
    Ok(())
}
pub fn init_command() -> Command {
    Command::new("init")
        .about("Create a folder containing only agent.md")
        .arg(Arg::new("name").required(true))
        .arg(
            Arg::new("folder-output")
                .long("output")
                .short('o')
                .value_parser(["json"]),
        )
}
pub fn init(m: &ArgMatches, _: &AppContext) -> Result<(), CliError> {
    let name = value(m, "name");
    require_name(name)?;
    let root = Path::new(name);
    if root.exists() {
        safe_file(root)?;
    }
    fs::create_dir_all(root).map_err(invalid)?;
    write_new(
        &root.join("agent.md"),
        &format!("# {name}\n\nDescribe what this agent should do.\n"),
    )?;
    emit(
        m,
        &json!({"folder":name,"files":["agent.md"],"next":format!("cd {name} && sikaru agent publish --project PROJECT")}),
    )
}
fn write_new(path: &Path, text: &str) -> Result<(), CliError> {
    use std::io::Write;
    if path.exists() {
        safe_file(path)?;
        return Ok(());
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(invalid)?;
    file.write_all(text.as_bytes()).map_err(invalid)
}
pub fn add_command() -> Command {
    options(
        Command::new("add")
            .about("Add a local subagent, schedule, channel, skill or Grader")
            .arg(Arg::new("item").required(true).value_name("KIND/NAME"))
            .arg(super::agent::project().required(false))
            .arg(
                Arg::new("room")
                    .long("room")
                    .help("Slack channel ID; omitted scaffolds a DM"),
            )
            .arg(
                Arg::new("non-interactive")
                    .long("non-interactive")
                    .action(ArgAction::SetTrue),
            ),
    )
}
fn template(kind: &str, name: &str, room: Option<&str>) -> Result<(String, String), CliError> {
    let result = match kind {
        "subagent" => (format!("subagents/{name}.md"), format!("# {name}\n\nResearch the requested topic and report the findings.\n")),
        "schedule" => (format!("schedules/{name}.md"), "---\nevery: 1d\n---\nPrepare a brief update.\n".into()),
        "channel" => (format!("channels/{name}.md"), format!("---\nslack:\n  {}\n---\n", room.map(|r| format!("room: {r}")).unwrap_or_else(|| "dm: true".into()))),
        "skill" => (format!("skills/{name}/SKILL.md"), format!("---\nname: {name}\ndescription: Describe when to use this skill.\n---\nExplain the steps to follow.\n")),
        "grader" => (format!("graders/{name}.toml"), "description = \"Describe what this Grader catches.\"\n\n# Yes/no questions about a conversation; a yes flags it.\n[[criteria]]\nquestion = \"Does the agent ...?\"\n".into()),
        _ => return Err(invalid("Kinds: subagent, schedule, channel, skill, grader")),
    };
    Ok(result)
}
/// Writes a starter `kind/name` file into the folder unless one exists; its folder-relative path.
pub fn scaffold(
    root: &Path,
    kind: &str,
    name: &str,
    room: Option<&str>,
) -> Result<String, CliError> {
    require_name(name)?;
    let (relative, text) = template(kind, name, room)?;
    safe_file(root)?;
    let path = destination(root, &relative)?;
    fs::create_dir_all(path.parent().unwrap()).map_err(invalid)?;
    write_new(&path, &text)?;
    Ok(relative)
}
pub fn add(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let (kind, name) = value(m, "item")
        .split_once('/')
        .ok_or_else(|| invalid("Use KIND/NAME"))?;
    let relative = scaffold(&root(m), kind, name, optional(m, "room"))?;
    let mut result = json!({"path": relative});
    if kind == "channel" {
        let setup = setup_channel(m, ctx)?;
        result["next"] = setup["next"].clone();
        result["authorizationUrl"] = setup["authorizationUrl"].clone();
    }
    emit(m, &result)
}
fn setup_channel(m: &ArgMatches, ctx: &AppContext) -> Result<Value, CliError> {
    let Some(project) = optional(m, "project") else {
        return Ok(json!({"next":"sikaru login"}));
    };
    let client = super::agent_transport::client(ctx);
    let installed = crate::sdk::block_on(client.channels.installations(project, None));
    if installed
        .as_ref()
        .is_ok_and(|items| items.iter().any(|i| i.status == "active"))
    {
        return Ok(json!({"next":null}));
    }
    let next = format!(
        "sikaru agent add {} -p {} --folder {}",
        shell_word(value(m, "item")),
        shell_word(project),
        shell_word(&root(m).to_string_lossy())
    );
    if m.get_flag("non-interactive") || !std::io::stdin().is_terminal() {
        return Ok(
            json!({"next":format!("sikaru agent channels start --project-id {} --json '{{}}'",shell_word(project))}),
        );
    }
    match crate::sdk::block_on(client.channels.start(project, &StartOAuth::default(), None)) {
        Ok(auth) => {
            open_authorization(&auth.authorization_url);
            Ok(json!({"next":next,"authorizationUrl":auth.authorization_url}))
        }
        Err(_) => Ok(json!({"next":"sikaru login"})),
    }
}
pub fn shell_word(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}
fn open_authorization(url: &str) {
    #[cfg(target_os = "macos")]
    let opener = "open";
    #[cfg(not(target_os = "macos"))]
    let opener = "xdg-open";
    let _ = std::process::Command::new(opener).arg(url).spawn();
}
pub fn info_command() -> Command {
    options(
        Command::new("info")
            .about("Show the server-compiled local folder")
            .arg(super::agent::project())
            .arg(
                Arg::new("compute")
                    .long("compute")
                    .value_parser(["managed", "local"])
                    .default_value("managed"),
            ),
    )
}
pub fn info(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let mut checked = compile(m, &super::agent_transport::client(ctx), &read(m)?)?;
    if optional(m, "compute") == Some("local") {
        checked["executionNote"] = json!("Async subagents run inline on local compute.");
    }
    emit(m, &checked)
}
pub fn is_folder_run(m: &ArgMatches) -> bool {
    optional(m, "folder").is_some() || root(m).join("agent.md").is_file()
}
pub fn draft_request(
    m: &ArgMatches,
    client: &ApiClient,
    mut request: SessionInput,
) -> Result<SessionInput, CliError> {
    if !is_folder_run(m) {
        return Ok(request);
    }
    let files = read(m)?;
    let checked = compile(m, client, &files)?;
    if checked["publishable"] != true {
        return Err(invalid(checked));
    }
    let mut body = serde_json::to_value(&request).map_err(invalid)?;
    body["environment"] = json!("draft");
    body["draft_files"] = json!(files);
    body["draft_schedule"] = json!(optional(m, "schedule"));
    body["acknowledge_widening"] = json!(m.get_flag("acknowledge-widening"));
    body["expected_access_digest"] = json!(optional(m, "access-digest"));
    request = serde_json::from_value(body).map_err(invalid)?;
    Ok(request)
}
pub fn schedule_prompt(m: &ArgMatches, client: &ApiClient) -> Result<Option<String>, CliError> {
    let Some(name) = optional(m, "schedule") else {
        return Ok(None);
    };
    if m.try_get_one::<bool>("dry-run").ok().flatten() == Some(&true) {
        return Ok(Some(String::new()));
    }
    let checked = compile(m, client, &read(m)?)?;
    checked["folder"]["schedules"][name]["prompt"]
        .as_str()
        .map(|s| Some(s.to_owned()))
        .ok_or_else(|| invalid(format!("No declared schedule named {name}")))
}
