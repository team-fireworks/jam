use crate::{config::Config, sources::SpriteSource};

pub async fn spritegen(config: &Config) -> anyhow::Result<()> {
    println!("{}", format!("{:#?}", config));
    for spritesheet in &config.spritesheets {
        for (_, source) in &spritesheet.1.sprites {
            if let SpriteSource::MaterialSymbols(material) = source {
                material.fetch(reqwest::Client::new()).await?;
                return Ok(());
            }
        }
    }

    Ok(())
}
