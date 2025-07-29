use anyhow::{Result, bail};
use serde::Deserialize;

use crate::Spritesheet;

#[cfg(feature = "_output_codegen")]
pub mod codegen;
#[cfg(feature = "output_dirs")]
pub mod dir;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
#[non_exhaustive]
pub enum OutputSpecifier {
    #[cfg(feature = "output_codegen_luau")]
    Luau(self::codegen::luau::LuauCodegenOutput),
    #[cfg(feature = "output_codegen_ts")]
    #[cfg_attr(feature = "serde", serde(rename = "typescript", alias = "ts"))]
    TypeScript(self::codegen::typescript::TypeScriptCodegenOutput),
    #[cfg(feature = "output_codegen_dts")]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "typescript_declarations", alias = "dts")
    )]
    TypeScriptDeclarations(
        self::codegen::typescript_declarations::TypeScriptDeclarationsCodegenOutput,
    ),
}

impl OutputSpecifier {
    pub async fn output(&self, name: &str, spritesheet: &Spritesheet) -> Result<()> {
        match self {
            #[cfg(feature = "output_codegen_luau")]
            OutputSpecifier::Luau(luau) => luau.output(name, spritesheet).await,
            #[cfg(feature = "output_codegen_ts")]
            OutputSpecifier::TypeScript(ts) => ts.output(name, spritesheet).await,
            #[cfg(feature = "output_codegen_dts")]
            OutputSpecifier::TypeScriptDeclarations(dts) => dts.output(name, spritesheet).await,
            #[allow(unreachable_patterns)]
            _ => bail!("not yet supported"),
        }
    }
}
