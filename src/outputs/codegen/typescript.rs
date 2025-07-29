use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

use super::create_disclaimer_comment;
use crate::{Spritesheet, match_casings, util::casings::Casing};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct TypeScriptCodegenOutput {
    pub path: PathBuf,
    pub export: TypeScriptExport,
    pub include_prelude_types: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_camel"))]
    pub type_casing: Casing,
    #[cfg_attr(feature = "serde", serde(default = "default_pascal"))]
    pub field_casing: Casing,
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum TypeScriptExport {
    #[default]
    /// many of `export const ...`
    Consts,
    /// `export namespace ...`
    NamedNamespace,
    /// `export default namespace ...`
    DefaultNamespace,
    /// `namespace ...; export = ...`
    ExportNamespace,
}

#[cfg(feature = "serde")]
fn default_camel() -> Casing {
    Casing::Camel
}

#[cfg(feature = "serde")]
fn default_pascal() -> Casing {
    Casing::Pascal
}

impl TypeScriptCodegenOutput {
    pub async fn output(&self, name: &str, spritesheet: &Spritesheet) -> Result<()> {
        fs::write(&self.path, self.codegen(name, spritesheet))
            .context("failed to save dts codegen output")?;

        Ok(())
    }

    pub fn codegen(&self, name: &str, spritesheet: &Spritesheet) -> String {
        let mut code = String::new();
        code.push_str(&create_disclaimer_comment("//"));
        code.push('\n');
        code.push('\n');

        let mut indent = String::new();

        let mut footer = String::new();

        match self.export.clone() {
            TypeScriptExport::Consts => (),
            namespace => {
                footer.push('}');
                footer.push('\n');

                code.push_str(match namespace {
                    TypeScriptExport::DefaultNamespace => "export default ",
                    TypeScriptExport::ExportNamespace => "",
                    TypeScriptExport::NamedNamespace => {
                        footer.push_str("export = ");
                        footer.push_str(name);
                        footer.push(';');

                        "export "
                    }
                    _ => unreachable!(),
                });

                code.push_str("namespace ");
                code.push_str(name);
                code.push_str(" {");
                code.push('\n');

                indent.push('\t');
            }
        }

        if self.include_prelude_types {
            code.push_str(&indent);
            code.push_str(
                &self
                    .prelude_types()
                    .replace("\n", format!("\n{indent}").as_str()),
            );
            code.push('\n');
            code.push('\n');
        }

        let sprite_type = if self.include_prelude_types {
            ": Sprite"
        } else {
            ""
        };

        macro_rules! push_field {
            ($field:expr, $value:expr) => {
                code.push_str(indent.as_str());
                code.push('\t');
                code.push_str($field);
                code.push_str(": ");
                code.push_str($value);
                code.push(',');
                code.push('\n');
            };
        }

        for (key, sprite) in &spritesheet.sprites {
            code.push_str(indent.as_str());
            code.push_str("export const ");
            code.push_str(key);
            code.push_str(sprite_type);
            code.push_str(" = {");
            code.push('\n');

            push_field!(self.ident_spritesheet(), "\"todo lolol\"");
            push_field!(self.ident_x(), format!("{}", sprite.x).as_str());
            push_field!(self.ident_y(), format!("{}", sprite.y).as_str());
            push_field!(self.ident_width(), format!("{}", sprite.width).as_str());
            push_field!(self.ident_height(), format!("{}", sprite.height).as_str());

            code.push_str(indent.as_str());
            code.push('}');
            code.push(';');
            code.push('\n');
        }

        code.push_str(footer.as_str());

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
