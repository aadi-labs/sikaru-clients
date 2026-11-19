//! Run an executor using only the attachment authority supplied over stdin.
use super::{config::Bootstrap, runtime, transport::HttpIdentity};
use anyhow::{ensure, Context, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};
use fern_cli_sdk::openapi::AppContext;
use serde_json::Value;
use std::io::Read;

const BOOTSTRAP_LIMIT: u64 = 16 * 1024;

pub fn command() -> Command {
    Command::new("serve-attachment")
        .hide(true)
        .arg(
            Arg::new("no-workspace-checkpoints")
                .long("no-workspace-checkpoints")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("restore-product-workspace")
                .long("restore-product-workspace")
                .action(ArgAction::SetTrue)
                .conflicts_with("no-workspace-checkpoints"),
        )
}

pub async fn execute(matches: &ArgMatches, context: &AppContext) -> Result<Value> {
    let mut input = Vec::new();
    std::io::stdin()
        .lock()
        .take(BOOTSTRAP_LIMIT + 1)
        .read_to_end(&mut input)
        .context("could not read executor bootstrap")?;
    ensure!(
        input.len() as u64 <= BOOTSTRAP_LIMIT,
        "executor bootstrap exceeds the input limit"
    );
    let bootstrap: Bootstrap =
        serde_json::from_slice(&input).context("invalid executor bootstrap")?;
    let options = runtime::RunOptions {
        skip_checkpoints: matches.get_flag("no-workspace-checkpoints"),
        restore_product_workspace: matches.get_flag("restore-product-workspace"),
        ..Default::default()
    };
    let http = HttpIdentity::from_config(context.http_config())?;
    runtime::serve_with_options(bootstrap, context.effective_base_url(), http, options).await
}
