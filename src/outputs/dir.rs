use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

use crate::Spritesheet;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DirOutput {
    pub path: PathBuf,
    #[cfg_attr(feature = "serde", serde(default = "default_file_extension"))]
    pub file_extension: FileExtension,
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum FileExtension {
    #[default]
    Png,
}

impl FileExtension {
    pub fn as_extension(&self) -> &str {
        match self {
            Self::Png => "png",
        }
    }
}

fn default_file_extension() -> FileExtension {
    FileExtension::Png
}

impl DirOutput {
    // TODO: replace with actual ast parsing
    pub async fn output(&self, _: &str, spritesheet: &Spritesheet) -> Result<()> {
        for (file_name, pixmap) in spritesheet.pixmaps.iter() {
            let path = self.path.clone().join(format!(
                "{file_name}.{}",
                self.file_extension.as_extension()
            ));

            let png_encoded = pixmap
                .encode_png()
                .context("failed to encode spritesheet as png")?;

            let contents = match self.file_extension {
                FileExtension::Png => png_encoded,
            };

            fs::write(path, contents).context("failed to save spritesheet as file")?;
        }

        Ok(())
    }
}
