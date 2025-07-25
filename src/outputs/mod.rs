use anyhow::Result;
use serde::Deserialize;

use crate::{
    outputs::codegen::typescript_declarations::TypeScriptDeclarationsCodegenOutput,
    spritegen::Spritesheet,
};

use self::codegen::luau::LuauCodegenOutput;

pub mod codegen;
pub mod dir;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum OutputSpecifier {
    Luau(LuauCodegenOutput),
    #[serde(alias = "dts")]
    TypeScriptDeclarations(TypeScriptDeclarationsCodegenOutput),
}

impl OutputSpecifier {
    pub async fn output(&self, name: &str, spritesheet: &Spritesheet) -> Result<()> {
        match self {
            OutputSpecifier::Luau(luau) => luau.output(name, spritesheet).await,
            OutputSpecifier::TypeScriptDeclarations(dts) => dts.output(name, spritesheet).await,
        }
    }
}
