use anyhow::Context;
use fs_err::tokio as fs;
use std::collections::HashMap;

use serde::Deserialize;

use crate::{outputs::OutputSpecifier, sources::SpriteSpecifier};

pub const FILE_NAME: &str = "Springroll.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub spritesheets: HashMap<String, SpritesheetSpecifier>,
}

impl Config {
    pub async fn read() -> anyhow::Result<Config> {
        let config = fs::read_to_string(FILE_NAME)
            .await
            .context("failed to read config file")?;

        let config: Config = toml::from_str(&config)?;

        Ok(config)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpritesheetSpecifier {
    pub spritegen: Spritegen,
    pub outputs: Vec<OutputSpecifier>,
    pub sprites: HashMap<String, SpriteSpecifier>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Spritegen {
    pub spritesheet_size: u32,
    pub sprites_per_row: u32,
}
