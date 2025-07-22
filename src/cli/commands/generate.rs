use clap::Args;
use springroll::config::Config;
use springroll::spritegen::spritegen;

#[derive(Debug, Args)]
pub struct GenerateCommand {}

impl GenerateCommand {
    pub async fn run(self) -> anyhow::Result<()> {
        spritegen(&Config::read().await?).await?;
        Ok(())
    }
}
