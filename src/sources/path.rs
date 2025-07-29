use std::{fs, path::PathBuf};

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use tiny_skia::Pixmap;
use usvg::{Options, Tree};

use crate::sources::SpriteSource;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct PathSource {
    path: PathBuf,
}

impl PathSource {
    pub async fn fetch(&self) -> Result<SpriteSource> {
        match self.path.extension() {
            None => bail!("file has no extension"),
            Some(extension) => {
                let extension = extension
                    .to_str()
                    .context("file extension is not valid UTF-8")?;

                match extension {
                    "png" => Ok(SpriteSource::Pixmap(
                        Pixmap::load_png(&self.path).context("failed to load png file")?,
                    )),
                    "svg" => Ok(SpriteSource::Tree(
                        Tree::from_str(
                            fs::read_to_string(&self.path)
                                .context("failed to read svg file")?
                                .as_str(),
                            &Options::default(),
                        )
                        .context("failed to load svg file")?,
                    )),
                    _ => bail!("unsupported file extension: {}", extension),
                }
            }
        }
    }
}
