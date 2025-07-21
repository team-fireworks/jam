use serde::Deserialize;

fn default_luicide_stroke_width() -> u16 {
    2
}

fn default_luicide_size() -> u16 {
    24
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct LuicideSource {
    #[serde(rename = "luicide")]
    icon: String,
    #[serde(default = "default_luicide_stroke_width")]
    stroke_width: u16,
    #[serde(default = "default_luicide_size")]
    size: u16,
}

impl LuicideSource {
    pub async fn fetch(&self) {}
}
