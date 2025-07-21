use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

pub const FILE_NAME: &str = "jam.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub version: u16,
    pub spritesheets: HashMap<String, Spritesheet>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Spritesheet {
    pub codegen: Codegen,
    pub imagegen: Imagegen,
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
    pub json: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CodegenStyle {
    Flat,
    #[default]
    Nested,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    File {
        file: PathBuf,
    },
    MaterialSymbols {
        material_symbols: String,
        #[serde(default)]
        style: MaterialSymbolsStyle,
        #[serde(default)]
        weight: i8,
        #[serde(default)]
        grade: i8,
        #[serde(default = "default_material_symbols_optical_size")]
        optical_size: u8,
    },
    FontAwesome {
        font_awesome: String,
        #[serde(default)]
        style: FontAwesomeStyle,
        #[serde(default)]
        pack: FontAwesomePack,
    },
    Fluent {
        fluent: String,
        #[serde(default)]
        style: FluentStyle,
    },
    Luicide {
        luicide: String,
        #[serde(default = "default_luicide_stroke_width")]
        stroke_width: u16,
        #[serde(default = "default_luicide_size")]
        size: u16,
    },
    #[default]
    Unknown,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MaterialSymbolsStyle {
    #[default]
    Filled,
    Outlined,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FontAwesomeStyle {
    #[default]
    Solid,
    Regular,
    Light,
    Thin,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FontAwesomePack {
    Brand,
    #[default]
    Classic,
    Duotone,
    Sharp,
    SharpDuotone,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FluentStyle {
    #[default]
    Filled,
    Outlined,
}

fn default_luicide_stroke_width() -> u16 {
    2
}

fn default_luicide_size() -> u16 {
    24
}

fn default_material_symbols_optical_size() -> u8 {
    48
}
