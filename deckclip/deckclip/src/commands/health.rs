use anyhow::Result;
use deckclip_core::DeckClient;

use crate::output::OutputMode;

pub async fn run(client: &mut DeckClient, output: OutputMode) -> Result<()> {
    let response = client.health().await?;
    output.print_success("ok — Deck App 连接正常");
    if let OutputMode::Json = output {
        output.print_response(&response);
    }
    Ok(())
}
