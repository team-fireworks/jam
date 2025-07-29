use anyhow::bail;
use serde::Deserialize;
use tiny_skia::Pixmap;
use usvg::Tree;

#[cfg(feature = "source_fluent")]
pub mod fluent;
#[cfg(feature = "source_font_awesome")]
pub mod font_awesome;
#[cfg(feature = "source_luicide")]
pub mod luicide;
#[cfg(feature = "source_material_symbols")]
pub mod material_symbols;
#[cfg(feature = "source_path")]
pub mod path;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged, rename_all = "snake_case"))]
#[non_exhaustive]
pub enum SpriteSpecifier {
    #[cfg(feature = "source_fluent")]
    Fluent(self::fluent::FluentSource),
    #[cfg(feature = "source_font_awesome")]
    FontAwesome(self::font_awesome::FontAwesomeSource),
    #[cfg(feature = "source_luicide")]
    Luicide(self::luicide::LuicideSource),
    #[cfg(feature = "source_material_symbols")]
    MaterialSymbols(self::material_symbols::MaterialSymbolsSource),
    #[cfg(feature = "source_path")]
    Path(self::path::PathSource),
}

#[derive(Debug, Clone)]
pub enum SpriteSource {
    Pixmap(Pixmap),
    #[cfg(feature = "svg")]
    Tree(Tree),
}

impl SpriteSpecifier {
    pub async fn fetch(&self, reqwest: reqwest::Client) -> anyhow::Result<SpriteSource> {
        Ok(match self {
            #[cfg(feature = "source_luicide")]
            Self::Luicide(luicide) => luicide.fetch(reqwest.clone()).await?,
            #[cfg(feature = "source_material_symbols")]
            Self::MaterialSymbols(material) => material.fetch(reqwest.clone()).await?,
            #[cfg(feature = "source_path")]
            Self::Path(path) => path.fetch().await?,
            #[allow(unreachable_patterns)]
            _ => bail!("not yet supported"),
        })
    }
}
