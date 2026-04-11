mod cli;
mod commands;
mod i18n;
mod output;

use anyhow::Result;
use clap::{CommandFactory, FromArgMatches};
use tracing_subscriber::EnvFilter;

use cli::{Cli, Commands};
use deckclip_core::{Config, DeckClient};
use output::OutputMode;

#[tokio::main]
async fn main() {
    // Initialize i18n before anything else (so -h shows the correct language)
    i18n::init();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .with_target(false)
        .init();

    // Build & localize clap command, then parse
    let cmd = localize_command(Cli::command());
    let matches = cmd.get_matches();
    let cli = Cli::from_arg_matches(&matches).expect("Failed to parse CLI args");

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

/// Override clap help text with translations for the current locale.
fn localize_command(cmd: clap::Command) -> clap::Command {
    use i18n::t;

    cmd.about(t("cli.about"))
        .long_about(t("cli.long_about"))
        .mut_arg("json", |a| a.help(t("arg.json")))
        .mut_subcommand("health", |s| s.about(t("cmd.health")))
        .mut_subcommand("write", |s| {
            s.about(t("cmd.write"))
                .mut_arg("text", |a| a.help(t("arg.write.text")))
                .mut_arg("tag", |a| a.help(t("arg.write.tag")))
                .mut_arg("tag_id", |a| a.help(t("arg.write.tag_id")))
                .mut_arg("raw", |a| a.help(t("arg.write.raw")))
        })
        .mut_subcommand("read", |s| s.about(t("cmd.read")))
        .mut_subcommand("paste", |s| {
            s.about(t("cmd.paste"))
                .mut_arg("index", |a| a.help(t("arg.paste.index")))
                .mut_arg("plain", |a| a.help(t("arg.paste.plain")))
                .mut_arg("target", |a| a.help(t("arg.paste.target")))
        })
        .mut_subcommand("panel", |s| {
            s.about(t("cmd.panel"))
                .mut_subcommand("toggle", |ss| ss.about(t("cmd.panel.toggle")))
        })
        .mut_subcommand("ai", |s| {
            s.about(t("cmd.ai"))
                .mut_subcommand("run", |ss| {
                    ss.about(t("cmd.ai.run"))
                        .mut_arg("prompt", |a| a.help(t("arg.ai.prompt")))
                        .mut_arg("text", |a| a.help(t("arg.ai.text")))
                        .mut_arg("save", |a| a.help(t("arg.ai.save")))
                        .mut_arg("tag_id", |a| a.help(t("arg.ai.tag_id")))
                })
                .mut_subcommand("search", |ss| {
                    ss.about(t("cmd.ai.search"))
                        .mut_arg("query", |a| a.help(t("arg.ai.query")))
                        .mut_arg("mode", |a| a.help(t("arg.ai.mode")))
                        .mut_arg("limit", |a| a.help(t("arg.ai.limit")))
                })
                .mut_subcommand("transform", |ss| {
                    ss.about(t("cmd.ai.transform"))
                        .mut_arg("prompt", |a| a.help(t("arg.ai.prompt")))
                        .mut_arg("text", |a| a.help(t("arg.ai.transform_text")))
                        .mut_arg("plugin", |a| a.help(t("arg.ai.plugin")))
                })
        })
        .mut_subcommand("completion", |s| {
            s.about(t("cmd.completion"))
                .mut_arg("shell", |a| a.help(t("arg.completion.shell")))
        })
        .mut_subcommand("version", |s| s.about(t("cmd.version")))
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
