use serde::Deserialize;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum FontAwesomeStyle {
    #[default]
    Solid,
    Regular,
    Light,
    Thin,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum FontAwesomePack {
    Brand,
    #[default]
    Classic,
    Duotone,
    Sharp,
    SharpDuotone,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct FontAwesomeSource {
    #[cfg_attr(feature = "serde", serde(rename = "font_awesome"))]
    icon: String,
    #[serde(default)]
    style: FontAwesomeStyle,
    #[serde(default)]
    pack: FontAwesomePack,
}

impl FontAwesomeSource {
    pub async fn fetch(&self) {}
}
