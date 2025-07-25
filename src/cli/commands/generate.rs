use anyhow::{Context, anyhow};
use clap::Args;
use springroll::config::Config;
use springroll::spritegen::spritegen;

#[derive(Debug, Args)]
pub struct GenerateCommand {}

impl GenerateCommand {
    pub async fn run(self) -> anyhow::Result<()> {
        let config = &Config::read().await.context("failed to get config")?;
        let spritesheets = spritegen(&config)
            .await
            .context("failed to generate spritesheets")?;

        for (key, spritesheet_config) in &config.spritesheets {
            let spritesheet = spritesheets
                .get(key.as_str())
                .ok_or_else(|| anyhow!("spritegen somehow missing for {key}"))?;

            for output in &spritesheet_config.outputs {
                output
                    .output(key, &spritesheet)
                    .await
                    .context("failed to output")?;
            }
        }

        Ok(())
    }
}
