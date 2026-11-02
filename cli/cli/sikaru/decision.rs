//! Native decision command: System One answers choice, yes-or-no and score questions
//! about a text or JSON state. It prints a table for people; `--output json` prints the
//! raw edge response for scripts and agents.
use clap::{Arg, ArgAction, ArgMatches, Command};
use fern_cli_sdk::{
    app::CliApp,
    error::CliError,
    openapi::{AppContext, OpenApiBinding},
};
use reqwest::Method;
use serde_json::{json, Map, Value};

use super::inference::{
    argument_or_stdin, blocking, endpoint, execute, invalid, json_output, output_arg, print_json,
    transport_error,
};

pub fn install(app: CliApp) -> CliApp {
    app.command_under(
        &["decision"],
        infer_command(),
        OpenApiBinding::handler(|m, ctx| blocking(infer(m, ctx))),
    )
}

fn infer_command() -> Command {
    Command::new("infer")
        .about("Answer choice, yes-or-no and score questions about a state")
        .after_help(
            "Examples:\n  sikaru decision infer \"I was charged twice\" --noul refund=\"Does the customer want a refund?\"\n  sikaru decision infer \"I was charged twice\" --questions questions.json\n  cat ticket.json | sikaru decision infer --json --questions questions.json",
        )
        .arg(Arg::new("state").help("Text to decide about. If omitted, reads from stdin."))
        .arg(
            Arg::new("json")
                .long("json")
                .action(ArgAction::SetTrue)
                .help("Parse the state as JSON"),
        )
        .arg(
            Arg::new("questions")
                .long("questions")
                .value_name("FILE")
                .help("JSON file of named questions: choice, noul or score"),
        )
        .arg(
            Arg::new("noul")
                .long("noul")
                .value_name("NAME=QUESTION")
                .action(ArgAction::Append)
                .help("Add a yes-or-no question; repeatable"),
        )
        .arg(output_arg(["table", "json"], "table"))
}

async fn infer(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let questions = questions(m)?;
    let body = json!({"state": state(m)?, "questions": questions});
    let request = endpoint(ctx, Method::POST, "v1/systemone")
        .header("Accept", "application/json")
        .json(&body);
    let decision: Value = execute(ctx, request)
        .await?
        .json()
        .await
        .map_err(transport_error)?;
    if json_output(m) {
        return print_json(&decision);
    }
    print!("{}", render(&rows(&questions, &decision["answers"])));
    eprintln!("{}", usage_line(&decision["usage"]));
    Ok(())
}

fn state(m: &ArgMatches) -> Result<Value, CliError> {
    let text = argument_or_stdin(m, "state")?;
    if !m.get_flag("json") {
        return Ok(Value::String(text));
    }
    serde_json::from_str(&text)
        .map_err(|error| invalid(format!("state is not valid JSON: {error}")))
}

/// Questions from `--questions`, plus one noul question per `--noul NAME=QUESTION`.
fn questions(m: &ArgMatches) -> Result<Map<String, Value>, CliError> {
    let mut questions = match m.get_one::<String>("questions") {
        Some(path) => questions_file(path)?,
        None => Map::new(),
    };
    for entry in m.get_many::<String>("noul").into_iter().flatten() {
        let (name, question) = entry
            .split_once('=')
            .filter(|(name, question)| !name.is_empty() && !question.is_empty())
            .ok_or_else(|| invalid(format!("--noul expects NAME=QUESTION, got {entry:?}")))?;
        questions.insert(
            name.to_string(),
            json!({"type": "noul", "instructions": question}),
        );
    }
    if questions.is_empty() {
        return Err(invalid(
            "no questions provided (use --questions FILE or --noul NAME=QUESTION)",
        ));
    }
    Ok(questions)
}

fn questions_file(path: &str) -> Result<Map<String, Value>, CliError> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| invalid(format!("reading {path} failed: {error}")))?;
    match serde_json::from_str(&text) {
        Ok(Value::Object(questions)) => Ok(questions),
        Ok(_) => Err(invalid(format!(
            "{path} must be a JSON object of named questions"
        ))),
        Err(error) => Err(invalid(format!("{path} is not valid JSON: {error}"))),
    }
}

/// One row per asked question, sorted by name; a question the model left unanswered says so.
fn rows(questions: &Map<String, Value>, answers: &Value) -> Vec<[String; 3]> {
    let mut names: Vec<&String> = questions.keys().collect();
    names.sort();
    let mut rows = vec![["question", "answer", "confidence"].map(str::to_string)];
    rows.extend(names.into_iter().map(|name| {
        let (answer, confidence) = cells(&answers[name.as_str()]);
        [name.clone(), answer, confidence]
    }));
    rows
}

fn cells(answer: &Value) -> (String, String) {
    let confidence = answer["confidence"]
        .as_f64()
        .map_or_else(|| "-".to_string(), |value| format!("{value:.2}"));
    let answer = match answer["type"].as_str() {
        Some("choice") => answer["choice"].as_str().unwrap_or("?").to_string(),
        Some("noul") => answer["noul"]
            .as_f64()
            .map_or_else(|| "?".to_string(), |yes| format!("yes {yes:.2}")),
        Some("score") => score(answer),
        _ => "no answer".to_string(),
    };
    (answer, confidence)
}

/// The score with the legend label of its nearest level: 1.60 -> `1.60 (high)`.
fn score(answer: &Value) -> String {
    let Some(score) = answer["score"].as_f64() else {
        return "?".to_string();
    };
    let level = (score.round() as i64).to_string();
    match answer["legend"][level.as_str()].as_str() {
        Some(label) => format!("{score:.2} ({label})"),
        None => format!("{score:.2}"),
    }
}

fn render(rows: &[[String; 3]]) -> String {
    let widths: Vec<usize> = (0..3)
        .map(|column| {
            rows.iter()
                .map(|row| row[column].chars().count())
                .max()
                .unwrap_or(0)
        })
        .collect();
    rows.iter()
        .map(|row| {
            let cells: Vec<String> = row
                .iter()
                .zip(&widths)
                .map(|(cell, &width)| format!("{cell:<width$}"))
                .collect();
            format!("{}\n", cells.join("  ").trim_end())
        })
        .collect()
}

fn usage_line(usage: &Value) -> String {
    let count = |key: &str| {
        usage[key]
            .as_i64()
            .map_or_else(|| "?".to_string(), |n| n.to_string())
    };
    format!(
        "usage: {} input + {} output tokens",
        count("input_tokens"),
        count("output_tokens")
    )
}
