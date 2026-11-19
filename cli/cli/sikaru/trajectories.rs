//! `sikaru trajectories`: read what Sikaru kept for a run as its trajectory: steps, tool calls,
//! metrics and reward. Uploads stay under `sikaru traces`, since what arrives from other systems
//! is traces.
use super::agent::{emit, invalid, project, value};
use clap::{Arg, ArgMatches, Command};
use fern_cli_sdk::{
    app::CliApp,
    error::CliError,
    openapi::{AppContext, OpenApiBinding},
};

pub fn install(app: CliApp) -> CliApp {
    app.command(command(), OpenApiBinding::handler(dispatch))
}

fn command() -> Command {
    Command::new("trajectories")
        .about("Read runs as trajectories: steps, tool calls, metrics and reward")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(get_command())
}

fn get_command() -> Command {
    Command::new("get")
        .about("Show a run's trajectory")
        .arg(project())
        .arg(Arg::new("run").required(true).help("Run id"))
}

fn dispatch(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    match m.subcommand() {
        Some(("get", sub)) => get(sub, ctx),
        _ => Err(invalid("Choose a trajectories command; see `sikaru trajectories --help`")),
    }
}

fn get(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let trajectory = crate::sdk::block_on(client.runs.get_trajectory(
        value(m, "project"),
        value(m, "run"),
        None,
    ))?;
    emit(m, &trajectory)
}
