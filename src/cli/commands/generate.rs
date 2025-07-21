use clap::Args;
use jam::config::Config;
use jam::spritegen::spritegen;

#[derive(Debug, Args)]
pub struct GenerateCommand {}

impl GenerateCommand {
    pub async fn run(self) -> anyhow::Result<()> {
        spritegen(&Config::read().await?).await?;
        Ok(())
    }
}
