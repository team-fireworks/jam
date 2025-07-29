use anyhow::{Context, Ok, Result};
use serde::Deserialize;
use usvg::{Options, Tree};

use crate::sources::SpriteSource;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct LuicideSource {
    #[cfg_attr(feature = "serde", serde(rename = "luicide"))]
    icon: String,
    // #[serde(default = "default_luicide_stroke_width")]
    // stroke_width: u16,
}

// fn default_luicide_stroke_width() -> u16 {
//     2
// }

impl LuicideSource {
    pub async fn fetch(&self, reqwest: reqwest::Client) -> Result<SpriteSource> {
        let url = format!(
            "https://raw.githubusercontent.com/lucide-icons/lucide/refs/heads/master/icons/{}.svg",
            self.icon
        );

        let raw_icon = reqwest
            .get(url)
            .header("cache-control", "public, max-age=3600")
            .send()
            .await
            .context("failed to fetch icon")?
            .text()
            .await
            .context("failed to parse fetched as text")?;

        let svg = Tree::from_str(&raw_icon, &Options::default()).context("failed to parse svg")?;
        // for node in svg.clip_paths() {}

        Ok(SpriteSource::Tree(svg))
    }
}
