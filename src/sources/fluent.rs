use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum FluentStyle {
    #[default]
    Filled,
    Outlined,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct FluentSource {
    #[serde(rename = "fluent")]
    icon: String,
    #[serde(default)]
    style: FluentStyle,
}

impl FluentSource {
    pub async fn fetch(&self) {}
}
