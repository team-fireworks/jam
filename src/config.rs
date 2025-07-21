use anyhow::Context;
use fs_err::tokio as fs;
use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::sources::SpriteSource;

pub const FILE_NAME: &str = "jam.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub spritesheets: HashMap<String, Spritesheet>,
}

impl Config {
    pub async fn read() -> anyhow::Result<Config> {
        let config = fs::read_to_string(FILE_NAME)
            .await
            .context("Failed to read config file")?;

        let config: Config = toml::from_str(&config)?;

        Ok(config)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Spritesheet {
    pub codegen: Codegen,
    pub imagegen: Imagegen,
    pub sprites: HashMap<String, SpriteSource>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Imagegen {
    pub output_dir: PathBuf,
    pub size_xy: u16,
    pub sprites_per_row: u16,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Codegen {
    pub output_path: PathBuf,
    #[serde(default)]
    pub style: CodegenStyle,
    #[serde(default)]
    pub strip_extensions: bool,

    #[serde(default)]
    pub luau: bool,
    #[serde(default)]
    pub typescript: bool,
    #[serde(default)]
    pub typescript_definitions: bool,
    #[serde(default)]
    pub json: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CodegenStyle {
    Flat,
    #[default]
    Nested,
}
