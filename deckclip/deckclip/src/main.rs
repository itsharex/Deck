mod cli;
mod commands;
mod output;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use cli::{Cli, Commands};
use deckclip_core::{Config, DeckClient};
use output::OutputMode;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .with_target(false)
        .init();

    let output = if cli.json {
        OutputMode::Json
    } else {
        OutputMode::Text
    };

    let config = Config::default();
    let mut client = DeckClient::new(config);

    let result = run(cli.command, &mut client, output).await;

    if let Err(e) = result {
        output.print_error(&e);
        std::process::exit(1);
    }
}

async fn run(command: Commands, client: &mut DeckClient, output: OutputMode) -> Result<()> {
    match command {
        Commands::Health => commands::health::run(client, output).await,
        Commands::Write(args) => commands::write::run(client, output, args).await,
        Commands::Read => commands::read::run(client, output).await,
        Commands::Paste(args) => commands::paste::run(client, output, args).await,
        Commands::Panel { action } => commands::panel::run(client, output, action).await,
        Commands::Ai(sub) => commands::ai::run(client, output, sub).await,
        Commands::Completion { shell } => {
            commands::completion::run(shell);
            Ok(())
        }
        Commands::Version => {
            commands::version::run(output);
            Ok(())
        }
    }
}
