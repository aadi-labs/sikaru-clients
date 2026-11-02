//! `sikaru credits`: an account utility beside `login` and `config`. `add` creates a top-up offer
//! for the project's organization and prints its pay URL; an agent wallet pays that URL. `status`
//! reads an offer. The CLI never sees payment credentials. `--output json` prints the gateway's
//! offer as is.
use super::inference::{blocking, endpoint, json_output, output_arg, print_json};
use clap::{Arg, ArgMatches, Command};
use fern_cli_sdk::{
    error::CliError,
    openapi::{AppContext, OpenApiBinding},
    sdk_executor::{SdkError, SdkRequestExecutor},
};
use reqwest::Method;
use serde_json::{json, Value};

pub fn install(app: fern_cli_sdk::app::CliApp) -> fern_cli_sdk::app::CliApp {
    app.command(
        command(),
        OpenApiBinding::handler(|m, ctx| blocking(run(m, ctx))),
    )
}

fn command() -> Command {
    Command::new("credits")
        .about("Add credits to your organization from an agent wallet")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Create a top-up offer and print the URL an agent wallet pays")
                .after_help("Example:\n  sikaru credits add --amount 20")
                .arg(
                    Arg::new("amount")
                        .long("amount")
                        .required(true)
                        .help("US dollars in whole cents, for example 20 or 20.50"),
                )
                .arg(project())
                .arg(output_arg(["text", "json"], "text")),
        )
        .subcommand(
            Command::new("status")
                .about("Show a top-up offer's status")
                .arg(Arg::new("id").required(true).help("Top-up id from `credits add`"))
                .arg(project())
                .arg(output_arg(["text", "json"], "text")),
        )
}

fn project() -> Arg {
    Arg::new("project")
        .long("project")
        .short('p')
        .env("SIKARU_PROJECT")
        .required(true)
        .help("Project whose organization receives the credits")
}

async fn run(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let (verb, m) = m.subcommand().expect("clap requires a subcommand");
    let offers = format!("v1/projects/{}/credit-top-ups", arg(m, "project"));
    let offer = if verb == "add" {
        let body = json!({"amountUsd": arg(m, "amount")});
        send(ctx, endpoint(ctx, Method::POST, &offers).json(&body)).await?
    } else {
        let path = format!("{offers}/{}", arg(m, "id"));
        send(ctx, endpoint(ctx, Method::GET, &path)).await?
    };
    if json_output(m) {
        return print_json(&offer);
    }
    print_offer(&offer, verb == "add");
    Ok(())
}

fn arg<'a>(m: &'a ArgMatches, name: &str) -> &'a str {
    m.get_one::<String>(name)
        .expect("clap validates required arguments")
}

/// Sends through the CLI's own transport after the sign-in check, so nothing leaves signed out.
async fn send(ctx: &AppContext, request: reqwest::RequestBuilder) -> Result<Value, CliError> {
    super::user_auth::ensure_signed_in()?;
    let request = request
        .header("Accept", "application/json")
        .build()
        .map_err(|error| CliError::Validation(error.to_string()))?;
    let response = ctx
        .build_sdk_executor()
        .execute(request)
        .await
        .map_err(SdkError::into_cli_error)?;
    let status = response.status().as_u16();
    let text = response.text().await.unwrap_or_default();
    let body: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
    if (200..300).contains(&status) {
        return Ok(body);
    }
    // Gateway refusals carry a readable `detail` and a stable `code`.
    let message = body["detail"].as_str().unwrap_or(&text);
    let code = body["code"].as_str().unwrap_or("request_failed");
    Err(CliError::api(status, message, code))
}

fn print_offer(offer: &Value, created: bool) {
    let field = |name: &str| offer[name].as_str().unwrap_or("").to_string();
    println!("Top-up {}: ${} ({})", field("id"), field("amountUsd"), field("status"));
    if !created {
        return;
    }
    println!("Pay URL: {}", field("payUrl"));
    println!("Expires: {}", expiry(offer["expiresAt"].as_i64().unwrap_or(0)));
    println!(
        "Pay this URL with an agent wallet that supports HTTP 402 machine payments; the credits land as soon as it is paid."
    );
}

/// Minutes from now, so the line needs no date formatting dependency.
fn expiry(at: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as i64);
    let minutes = (at - now).max(0) / 60;
    format!("in {minutes} minutes (unix {at})")
}
