use anyhow::{Result, bail};
use serde::Deserialize;

use crate::spritegen::Spritesheet;

use self::codegen::luau::LuauCodegenOutput;

pub mod codegen;
pub mod dir;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum OutputSpecifier {
    Luau(LuauCodegenOutput),
}

impl OutputSpecifier {
    pub async fn output(&self, name: &str, spritesheet: &Spritesheet) -> Result<()> {
        match self {
            OutputSpecifier::Luau(luau) => luau.output(name, spritesheet).await,
            _ => bail!("not yet implemented"),
        }
    }
}
