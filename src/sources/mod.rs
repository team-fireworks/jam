use self::{
    fluent::FluentSource, font_awesome::FontAwesomeSource, luicide::LuicideSource,
    material_symbols::MaterialSymbolsSource, path::PathSource,
};
use anyhow::bail;
use serde::Deserialize;
use tiny_skia::Pixmap;
use usvg::Tree;

pub mod fluent;
pub mod font_awesome;
pub mod luicide;
pub mod material_symbols;
pub mod path;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
#[non_exhaustive]
pub enum SpriteSpecifier {
    Path(PathSource),
    FontAwesome(FontAwesomeSource),
    MaterialSymbols(MaterialSymbolsSource),
    Luicide(LuicideSource),
    Fluent(FluentSource),
}

#[derive(Debug, Clone)]
pub enum SpriteSource {
    Raster(Pixmap),
    Vector(Tree),
}

impl SpriteSpecifier {
    pub async fn fetch(&self, reqwest: reqwest::Client) -> anyhow::Result<SpriteSource> {
        Ok(match self {
            Self::Path(path) => path.fetch().await?,
            Self::MaterialSymbols(material) => material.fetch(reqwest.clone()).await?,
            _ => bail!("not yet implemented"),
        })
    }
}
