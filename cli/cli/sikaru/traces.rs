//! `sikaru traces`: upload a trace file. What arrives from other systems is traces; Sikaru keeps
//! them as spans and reads each run back as a trajectory (`sikaru trajectories`). The upload's
//! format comes from the file's shape: OTLP `resourceSpans`, ATIF trajectories, OpenInference spans, or a
//! Langfuse, LangSmith or Braintrust export. An export is sent in batches that keep each trace
//! (for Langfuse, each agent run with its sub-agent runs) whole, because the service rebuilds a
//! trace only from the rows in one request.
use super::agent::{emit, invalid, project, value};
use clap::{Arg, ArgMatches, Command};
use fern_cli_sdk::{
    app::CliApp,
    error::CliError,
    openapi::{AppContext, OpenApiBinding},
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use sha2::{Digest, Sha256};
use sikaru_sdk::api::*;
use std::collections::HashMap;

/// Upper bound on one export batch; a single larger trace is still sent whole, on its own.
const MAX_BATCH_BYTES: usize = 4 * 1024 * 1024;

pub fn install(app: CliApp) -> CliApp {
    app.command_under(
        &["traces"],
        upload_command(),
        OpenApiBinding::handler(upload),
    )
}

fn upload_command() -> Command {
    Command::new("upload")
        .about("Upload a JSON or JSONL trace file: OTLP, ATIF, OpenInference spans, or a Langfuse, LangSmith or Braintrust export")
        .arg(project())
        .arg(Arg::new("file").required(true).help("Trace file"))
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Otlp,
    Atif,
    Langfuse,
    LangSmith,
    Braintrust,
    Spans,
}

fn kind(record: &Value) -> Option<Kind> {
    let has = |key| record.get(key).is_some();
    if has("resourceSpans") {
        Some(Kind::Otlp)
    } else if has("traceId") && has("type") {
        Some(Kind::Langfuse)
    } else if has("run_type") && has("trace_id") {
        Some(Kind::LangSmith)
    } else if has("root_span_id") && has("span_id") {
        Some(Kind::Braintrust)
    } else if ["steps", "schema_version", "trajectory", "trajectories"]
        .into_iter()
        .any(has)
    {
        Some(Kind::Atif)
    } else if has("trace_id") && has("span_id") {
        Some(Kind::Spans)
    } else {
        None
    }
}

/// One JSON document (an array is its items), else one JSON value per non-blank line.
fn records(text: &str) -> Result<Vec<Value>, CliError> {
    match serde_json::from_str::<Value>(text) {
        Ok(Value::Array(items)) => Ok(items),
        Ok(value) => Ok(vec![value]),
        Err(_) => text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line).map_err(invalid))
            .collect(),
    }
}

/// Every record must be the same recognized format.
fn file_kind(records: &[Value]) -> Result<Kind, CliError> {
    let kinds: Vec<Option<Kind>> = records.iter().map(kind).collect();
    match kinds.first() {
        Some(Some(first)) if kinds.iter().all(|k| *k == Some(*first)) => Ok(*first),
        Some(Some(_)) => Err(invalid("The trace file mixes formats; upload one format per file")),
        _ => Err(invalid(
            "Unrecognized trace file: expected OTLP resourceSpans, ATIF trajectories, spans with trace_id and span_id, or a Langfuse, LangSmith or Braintrust export",
        )),
    }
}

fn convert<T: DeserializeOwned>(
    values: impl IntoIterator<Item = Value>,
) -> Result<Vec<T>, CliError> {
    values
        .into_iter()
        .map(|value| serde_json::from_value(value).map_err(invalid))
        .collect()
}

fn resource_spans(record: Value) -> Vec<Value> {
    match record.get("resourceSpans") {
        Some(Value::Array(spans)) => spans.clone(),
        _ => vec![],
    }
}

/// A record wrapping `trajectory` or `trajectories` contributes those; otherwise it is one.
fn trajectories(record: Value) -> Vec<Value> {
    if let Some(one) = record.get("trajectory") {
        return vec![one.clone()];
    }
    if let Some(Value::Array(many)) = record.get("trajectories") {
        return many.clone();
    }
    vec![record]
}

fn run_field<'a>(row: &'a Value, key: &str) -> Option<&'a str> {
    row.get("metadata")?.get(key)?.as_str()
}

fn text_field(row: &Value, key: &str) -> Option<String> {
    match row.get(key)? {
        Value::String(text) => Some(text.clone()),
        Value::Null => None,
        other => Some(other.to_string()),
    }
}

/// Langfuse rows keyed by top-level agent run, following `parent_agent_run_id` to a run in the
/// file; a row outside any run is keyed on its own.
fn langfuse_keys(rows: &[Value]) -> Vec<String> {
    let mut parents: HashMap<String, Option<String>> = HashMap::new();
    for row in rows {
        if let Some(run) = run_field(row, "agent_run_id") {
            let parent = run_field(row, "parent_agent_run_id").map(str::to_owned);
            parents.entry(run.to_owned()).or_insert(parent);
        }
    }
    let root = |run: &str| {
        let mut run = run.to_owned();
        for _ in 0..parents.len() {
            match parents.get(&run) {
                Some(Some(parent)) if parents.contains_key(parent) => run = parent.clone(),
                _ => break,
            }
        }
        run
    };
    rows.iter()
        .enumerate()
        .map(|(position, row)| match run_field(row, "agent_run_id") {
            Some(run) => format!("run:{}", root(run)),
            None => format!("row:{position}"),
        })
        .collect()
}

/// The trace each export row belongs to; rows sharing a key travel in one request.
fn trace_keys(kind: Kind, rows: &[Value]) -> Vec<String> {
    let field = match kind {
        Kind::Langfuse => return langfuse_keys(rows),
        Kind::LangSmith => "trace_id",
        _ => "root_span_id",
    };
    rows.iter()
        .enumerate()
        .map(|(position, row)| text_field(row, field).unwrap_or_else(|| format!("row:{position}")))
        .collect()
}

/// Rows grouped by key, groups in the order their first row appears.
fn grouped(rows: Vec<Value>, keys: Vec<String>) -> Vec<Vec<Value>> {
    let mut index: HashMap<String, usize> = HashMap::new();
    let mut groups: Vec<Vec<Value>> = Vec::new();
    for (row, key) in rows.into_iter().zip(keys) {
        let slot = *index.entry(key).or_insert_with(|| {
            groups.push(Vec::new());
            groups.len() - 1
        });
        groups[slot].push(row);
    }
    groups
}

/// Whole traces packed into batches of at most `MAX_BATCH_BYTES`, in order.
fn export_batches(kind: Kind, rows: Vec<Value>) -> Vec<Vec<Value>> {
    let keys = trace_keys(kind, &rows);
    let mut batches = Vec::new();
    let (mut current, mut size) = (Vec::new(), 0);
    for group in grouped(rows, keys) {
        let bytes: usize = group.iter().map(|row| row.to_string().len()).sum();
        if !current.is_empty() && size + bytes > MAX_BATCH_BYTES {
            batches.push(std::mem::take(&mut current));
            size = 0;
        }
        size += bytes;
        current.extend(group);
    }
    if !current.is_empty() {
        batches.push(current);
    }
    batches
}

fn export_format(kind: Kind) -> Option<TraceStreamRequestFormat> {
    match kind {
        Kind::Langfuse => Some(TraceStreamRequestFormat::LangfuseObservationsV1),
        Kind::LangSmith => Some(TraceStreamRequestFormat::LangsmithRunsV1),
        Kind::Braintrust => Some(TraceStreamRequestFormat::BraintrustSpansV1),
        _ => None,
    }
}

fn empty_request(project: &str, format: TraceStreamRequestFormat) -> TraceStreamRequest {
    TraceStreamRequest {
        dataset: None,
        format,
        metadata: TraceMetadata {
            project_id: project.to_owned(),
            account_id: project.to_owned(),
            source: "cli".to_owned(),
            ..Default::default()
        },
        records: None,
        resource_spans: None,
        spans: None,
        trajectories: None,
        trajectory: None,
    }
}

/// The trace stream requests for a file: one, or one per batch for a trace export.
fn requests(project: &str, records: Vec<Value>) -> Result<Vec<TraceStreamRequest>, CliError> {
    let kind = file_kind(&records)?;
    if let Some(format) = export_format(kind) {
        return export_batches(kind, records)
            .into_iter()
            .map(|batch| {
                let mut request = empty_request(project, format.clone());
                request.records = Some(convert(batch)?);
                Ok(request)
            })
            .collect();
    }
    let mut request = empty_request(project, TraceStreamRequestFormat::OpeninferenceV1);
    match kind {
        Kind::Otlp => {
            request.format = TraceStreamRequestFormat::OpentelemetryOtlpV1;
            request.resource_spans = Some(convert(records.into_iter().flat_map(resource_spans))?);
        }
        Kind::Atif => {
            request.format = TraceStreamRequestFormat::HarborAtifV1;
            request.trajectories = Some(convert(records.into_iter().flat_map(trajectories))?);
        }
        _ => request.spans = Some(convert(records)?),
    }
    Ok(vec![request])
}

/// Content-derived, so retrying an identical batch is acknowledged without storing it twice.
fn idempotency_key(request: &TraceStreamRequest) -> Result<String, CliError> {
    let body = serde_json::to_vec(request).map_err(invalid)?;
    Ok(format!("cli-{:x}", Sha256::digest(body)))
}

fn upload(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let text = std::fs::read_to_string(value(m, "file")).map_err(invalid)?;
    let requests = requests(value(m, "project"), records(&text)?)?;
    let client = crate::sdk::client(ctx);
    let mut accepted = Vec::with_capacity(requests.len());
    for request in &requests {
        let options = sikaru_sdk::RequestOptions::new()
            .additional_header("Idempotency-Key", idempotency_key(request)?);
        accepted.push(crate::sdk::block_on(
            client
                .trace_streams
                .stream_openinference_spans(request, Some(options)),
        )?);
    }
    match accepted.as_slice() {
        [one] => emit(m, one),
        _ => emit(m, &accepted),
    }
}
