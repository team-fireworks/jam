use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum FontAwesomeStyle {
    #[default]
    Solid,
    Regular,
    Light,
    Thin,
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum FontAwesomePack {
    Brand,
    #[default]
    Classic,
    Duotone,
    Sharp,
    SharpDuotone,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct FontAwesomeSource {
    #[serde(rename = "font_awesome")]
    icon: String,
    #[serde(default)]
    style: FontAwesomeStyle,
    #[serde(default)]
    pack: FontAwesomePack,
}

impl FontAwesomeSource {
    pub async fn fetch(&self) {}
}
