use self::{
    fluent::FluentSource, font_awesome::FontAwesomeSource, luicide::LuicideSource,
    material_symbols::MaterialSymbolsSource, path::PathSource,
};
use serde::Deserialize;

pub mod fluent;
pub mod font_awesome;
pub mod luicide;
pub mod material_symbols;
pub mod path;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum SpriteSource {
    Path(PathSource),
    FontAwesome(FontAwesomeSource),
    MaterialSymbols(MaterialSymbolsSource),
    Luicide(LuicideSource),
    Fluent(FluentSource),
}
