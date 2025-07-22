use anyhow::Context;
use fs_err::tokio as fs;
use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::sources::SpriteSpecifier;

pub const FILE_NAME: &str = "Springroll.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub spritesheets: HashMap<String, SpritesheetSpecifier>,
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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct SpritesheetSpecifier {
    pub codegen: Codegen,
    pub imagegen: Imagegen,
    pub sprites: HashMap<String, SpriteSpecifier>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Default)]
#[serde(default)]
pub struct Imagegen {
    pub output_dir: PathBuf,
    pub spritesheet_size: u32,
    pub sprites_per_row: u32,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Default)]
#[serde(default)]
pub struct Codegen {
    pub output_path: PathBuf,

    #[serde(default)]
    pub reference_imagegen_output_as: ImagegenOutputReference,
    #[serde(default)]
    pub roblox_imagegen_output_path: Option<Vec<String>>,

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

#[derive(Debug, Deserialize, Default, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CodegenStyle {
    Flat,
    #[default]
    Nested,
}

#[derive(Debug, Deserialize, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ImagegenOutputReference {
    SpritesheetFileName,
    #[default]
    RelativeToSpritesheet,
    AbsoluteToSpritesheet,
}
