use anyhow::Result;
use deckclip_core::DeckClient;

use crate::cli::PanelAction;
use crate::output::OutputMode;

pub async fn run(
    client: &mut DeckClient,
    output: OutputMode,
    action: PanelAction,
) -> Result<()> {
    match action {
        PanelAction::Toggle => {
            let response = client.panel_toggle().await?;
            output.print_success("面板已切换");
            if let OutputMode::Json = output {
                output.print_response(&response);
            }
        }
    }
    Ok(())
}
