#[path = "../../../../cli/cli/sikaru/commands.rs"]
mod commands;

use fern_cli_sdk::{app::CliApp, openapi::OpenApiBinding};

#[path = "authoring.rs"]
mod authoring;

#[path = "user_auth.rs"]
mod user_auth;

fn main() {
    let app = CliApp::new("sikaru-authoring")
        .binding(OpenApiBinding::new().commands(commands::description()));
    authoring::install(user_auth::install(app)).run()
}
