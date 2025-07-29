pub mod outputs;
pub mod sources;

mod util;

use crate::{
    outputs::OutputSpecifier,
    sources::{SpriteSource, SpriteSpecifier},
};
use serde::Deserialize;
use std::collections::HashMap;
use tiny_skia::{Pixmap, PixmapPaint, Transform};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct SpritesheetSpecifier {
    pub spritegen: Spritegen,
    pub outputs: Vec<OutputSpecifier>,
    pub sprites: HashMap<String, SpriteSpecifier>,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Spritegen {
    pub spritesheet_size: u32,
    pub sprites_per_row: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Config {
    pub spritesheets: HashMap<String, SpritesheetSpecifier>,
}

pub struct Sprite {
    pub pixmap_index: i32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct Spritesheet {
    pub pixmaps: Vec<Pixmap>,
    pub sprites: HashMap<String, Sprite>,
}

pub async fn spritegen(
    spritesheet: &SpritesheetSpecifier,
    reqwest: reqwest::Client,
) -> anyhow::Result<Spritesheet> {
    let spritesheet_size = spritesheet.spritegen.spritesheet_size;

    let mut pixmaps: Vec<Pixmap> = Vec::new();
    let mut pixmap_index = 0;

    let mut current_spritesheet: Pixmap = Pixmap::new(spritesheet_size, spritesheet_size).unwrap();

    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;
    let mut highest_y_in_row: i32 = 0;

    let sprite_size = spritesheet_size / spritesheet.spritegen.sprites_per_row;

    let sprites = spritesheet.sprites.clone();
    let mut sorted_sprites: Vec<(&String, &crate::sources::SpriteSpecifier)> =
        sprites.iter().collect();
    sorted_sprites.sort_by(|lhs, rhs| lhs.0.cmp(rhs.0));

    let mut sprites_for_spritesheet: HashMap<String, Sprite> = HashMap::new();

    for (sprite_key, specifier) in sorted_sprites {
        match specifier.fetch(reqwest.clone()).await {
            Ok(source) => {
                let (width, height) = match &source {
                    SpriteSource::Pixmap(pixmap) => (pixmap.width() as f32, pixmap.height() as f32),
                    #[cfg(feature = "svg")]
                    SpriteSource::Tree(tree) => {
                        let size = tree.size();
                        (size.width(), size.height())
                    }
                };

                let transform =
                    Transform::from_scale(sprite_size as f32 / width, sprite_size as f32 / height)
                        .post_translate(current_x as f32, current_y as f32);

                match source {
                    SpriteSource::Pixmap(pixmap) => {
                        current_spritesheet.draw_pixmap(
                            0,
                            0,
                            pixmap.as_ref(),
                            &PixmapPaint::default(),
                            transform,
                            None,
                        );
                    }
                    #[cfg(feature = "svg")]
                    SpriteSource::Tree(tree) => {
                        resvg::render(&tree, transform, &mut current_spritesheet.as_mut());
                    }
                };

                sprites_for_spritesheet.insert(sprite_key.clone(), Sprite {
                    pixmap_index,
                    x: current_x as u32,
                    y: current_y as u32,
                    width: width as u32,
                    height: height as u32,
                });

                let spritesheet_size_i32: i32 = spritesheet_size.try_into().unwrap();
                let sprite_size_i32 = sprite_size as i32;

                current_x += sprite_size_i32;

                if highest_y_in_row < height as i32 {
                    highest_y_in_row = height as i32;
                };

                if current_x >= spritesheet_size_i32 {
                    current_x = 0;
                    current_y += highest_y_in_row;
                    highest_y_in_row = 0;

                    if current_y >= spritesheet_size_i32 {
                        current_x = 0;
                        current_y = 0;
                        pixmap_index += 1;
                        pixmaps.push(current_spritesheet.clone());
                        current_spritesheet =
                            Pixmap::new(spritesheet_size, spritesheet_size).unwrap();
                    }
                }
            }
            Err(e) => {
                println!("failed to fetch sprite {}: {}", sprite_key, e);
                continue;
            }
        }
    }

    if current_x > 0 || current_y > 0 || pixmaps.is_empty() {
        pixmaps.push(current_spritesheet);
    }

    Ok(Spritesheet {
        pixmaps,
        sprites: sprites_for_spritesheet,
    })
}
