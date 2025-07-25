use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::Deserialize;
use tokio::fs;

use super::create_disclaimer_comment;
use crate::{match_casings, spritegen::Spritesheet, util::casings::Casing};

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct TypeScriptDeclarationsCodegenOutput {
    pub path: PathBuf,
    pub include_prelude_types: bool,
    #[serde(default = "default_camel")]
    pub type_casing: Casing,
    #[serde(default = "default_pascal")]
    pub field_casing: Casing,
}

fn default_camel() -> Casing {
    Casing::Camel
}

fn default_pascal() -> Casing {
    Casing::Pascal
}

impl TypeScriptDeclarationsCodegenOutput {
    pub async fn output(&self, name: &str, spritesheet: &Spritesheet) -> Result<()> {
        fs::write(&self.path, self.codegen(name, spritesheet))
            .await
            .context("failed to save dts codegen output")?;

        Ok(())
    }

    pub fn codegen(&self, name: &str, spritesheet: &Spritesheet) -> String {
        let mut code = String::new();
        code.push_str(&create_disclaimer_comment("//"));
        code.push('\n');
        code.push('\n');

        code.push_str("declare namespace ");
        code.push_str(name);
        code.push_str(" {");
        code.push('\n');

        if self.include_prelude_types {
            code.push('\t');
            code.push_str(self.prelude_types().replace("\n", "\n\t").as_str());
            code.push('\n');
        }

        let sprite_type = self.sprite_type();
        let ident_sprite = self.ident_sprite();
        let indented_sprite_type = sprite_type.replace("\n", "\n\t");
        let real_sprite_type = if self.include_prelude_types {
            ident_sprite
        } else {
            indented_sprite_type.as_str()
        };

        for key in spritesheet.sprites.keys() {
            code.push('\t');
            code.push_str("export const ");
            code.push_str(key.as_str());

            code.push_str(": ");
            code.push_str(real_sprite_type);
            code.push(';');
            code.push('\n');
        }

        code.push_str("}");
        code.push('\n');
        code.push('\n');
        code.push_str("export = ");
        code.push_str(name);
        code.push(';');
        code.push('\n');
        code.push_str("export as namespace ");
        code.push_str(name);
        code.push(';');
        code.push('\n');

        code
    }

    pub fn prelude_types(&self) -> String {
        format!(
            "export interface {} {}",
            self.ident_sprite(),
            self.sprite_type()
        )
    }

    pub fn sprite_type(&self) -> String {
        let image = self.ident_spritesheet();
        let x = self.ident_x();
        let y = self.ident_y();
        let width = self.ident_width();
        let height = self.ident_height();

        format!(
            "{{\
                \n\treadonly {image}: string,\
                \n\treadonly {x}: number,\
                \n\treadonly {y}: number,\
                \n\treadonly {width}: number,\
                \n\treadonly {height}: number,\
            \n}}"
        )
    }

    pub fn ident_sprite(&self) -> &'static str {
        match_casings!(self.type_casing => Sprite)
    }

    pub fn ident_spritesheet(&self) -> &'static str {
        match_casings!(self.field_casing => spritesheet)
    }

    pub fn ident_x(&self) -> &'static str {
        match_casings!(self.field_casing => x)
    }

    pub fn ident_y(&self) -> &'static str {
        match_casings!(self.field_casing => y)
    }

    pub fn ident_width(&self) -> &'static str {
        match_casings!(self.field_casing => width)
    }

    pub fn ident_height(&self) -> &'static str {
        match_casings!(self.field_casing => height)
    }
}
