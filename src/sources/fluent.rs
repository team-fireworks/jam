use serde::Deserialize;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum FluentStyle {
    #[default]
    Filled,
    Outlined,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct FluentSource {
    #[cfg_attr(feature = "serde", serde(rename = "fluent"))]
    icon: String,
    #[serde(default)]
    style: FluentStyle,
}

impl FluentSource {
    pub async fn fetch(&self) {}
}
