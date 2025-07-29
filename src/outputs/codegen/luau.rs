use std::{fmt::Display, path::PathBuf};

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::fs;

use super::create_disclaimer_comment;
use crate::{Sprite, Spritesheet, match_casings, util::casings::Casing};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct LuauCodegenOutput {
    pub path: PathBuf,
    pub include_prelude_types: bool,
    pub new_luau_solver: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_true"))]
    pub freeze_tables: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_snake"))]
    pub type_casing: Casing,
    #[cfg_attr(feature = "serde", serde(default = "default_pascal"))]
    pub field_casing: Casing,
}

#[cfg(feature = "serde")]
fn default_true() -> bool {
    true
}

#[cfg(feature = "serde")]
fn default_snake() -> Casing {
    Casing::Snake
}

#[cfg(feature = "serde")]
fn default_pascal() -> Casing {
    Casing::Pascal
}

lazy_static! {
    static ref LUAU_IDENTIFIER_REGEX: Regex = Regex::new("^([_a-zA-Z][_a-zA-Z0-9]?)+").unwrap();
}

fn is_luau_keyword(str: &str) -> bool {
    match str {
        "and" => true,
        "break" => true,
        "continue" => true,
        "do" => true,
        "else" => true,
        "elseif" => true,
        "end" => true,
        "export" => true,
        "false" => true,
        "for" => true,
        "function" => true,
        "if" => true,
        "in" => true,
        "local" => true,
        "nil" => true,
        "not" => true,
        "or" => true,
        "repeat" => true,
        "return" => true,
        "self" => true,
        "then" => true,
        "type" => true,
        "typeof" => true,
        "until" => true,
        "while" => true,
        _ => false,
    }
}

/// Checks if a string is a valid Luau identifier, which is longer than 0
/// characters, use alphanumeric characters and underscores, and not start with
/// a number.
fn is_luau_ident(str: &str) -> bool {
    !str.is_empty() && !is_luau_keyword(str) && LUAU_IDENTIFIER_REGEX.is_match(str)
}

fn wrap_luau_ident(str: &str) -> String {
    if is_luau_ident(str) {
        str.to_string()
    } else {
        format!("[\"{str}\"]")
    }
}

impl LuauCodegenOutput {
    pub async fn output(&self, name: &str, spritesheet: &Spritesheet) -> Result<()> {
        fs::write(&self.path, self.codegen(name, spritesheet))
            .context("failed to save luau codegen output")?;

        Ok(())
    }

    pub fn codegen(&self, name: &str, spritesheet: &Spritesheet) -> String {
        let mut code = String::new();
        code.push_str("--!strict");
        code.push('\n');
        code.push_str(&create_disclaimer_comment("--"));
        code.push('\n');

        code.push('\n');
        if self.include_prelude_types {
            code.push_str(&self.prelude_types());
            code.push('\n');
        }

        code.push_str(&format!("local {name} = "));

        let mut body = String::new();
        body.push('{');
        body.push('\n');

        for (key, sprite) in &spritesheet.sprites {
            body.push('\t');
            body.push_str(wrap_luau_ident(key).as_str());
            body.push_str(" = ");
            body.push_str(&self.wrap_sprite(sprite));
            body.push(',');
            body.push('\n');
        }

        body.push('}');
        code.push_str(&self.wrap_freeze(body));

        code.push('\n');
        code.push('\n');
        code.push_str(&format!("return {name}"));
        code.push('\n');

        code
    }

    pub fn prelude_types(&self) -> String {
        let readonly_modifier = match self.new_luau_solver {
            true => "read ",
            false => "",
        };

        let image = self.ident_spritesheet();
        let x = self.ident_x();
        let y = self.ident_y();
        let width = self.ident_width();
        let height = self.ident_height();
        let sprite = self.ident_sprite();

        format!(
            "\
                export type {sprite} = {{\
                \n\t{readonly_modifier} {image}: string,\
                \n\t{readonly_modifier} {x}: number,\
                \n\t{readonly_modifier} {y}: number,\
                \n\t{readonly_modifier} {width}: number,\
                \n\t{readonly_modifier} {height}: number,\
                \n}}\
                \n\
                \nlocal function {sprite}(x: {sprite}): {sprite}\
                \n\treturn {}\
                \nend\
                \n\
            ",
            self.wrap_freeze("x")
        )
    }

    pub fn wrap_sprite(&self, sprite: &Sprite) -> String {
        // typa shit pirate software would defend but okay
        let inner = format!(
            "{{ {} = {}, {} = {}, {} = {}, {} = {}, {} = {} }}",
            self.ident_spritesheet(),
            "\"todo lololol\"",
            self.ident_x(),
            sprite.x,
            self.ident_y(),
            sprite.y,
            self.ident_width(),
            sprite.width,
            self.ident_height(),
            sprite.height,
        );

        if self.include_prelude_types {
            format!("{}({inner})", self.ident_sprite())
        } else {
            self.wrap_freeze(inner)
        }
    }

    pub fn wrap_freeze(&self, inner: impl Display) -> String {
        if self.freeze_tables {
            format!("table.freeze({inner})")
        } else {
            inner.to_string()
        }
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
