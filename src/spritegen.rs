use std::collections::HashMap;

use anyhow::Context;
use tiny_skia::{Pixmap, PixmapPaint, Transform};

use crate::{config::Config, sources::SpriteSource};

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

pub async fn spritegen(config: &Config) -> anyhow::Result<HashMap<&str, Spritesheet>> {
    let reqwest = reqwest::Client::builder()
        .build()
        .context("failed to create reqwest client")?;

    let mut spritesheets: HashMap<&str, Spritesheet> = HashMap::new();

    for (spritesheet_key, spritesheet) in &config.spritesheets {
        let spritesheet_size = spritesheet.spritegen.spritesheet_size;

        let mut pixmaps: Vec<Pixmap> = Vec::new();
        let mut pixmap_index = 0;

        let mut current_spritesheet: Pixmap =
            Pixmap::new(spritesheet_size, spritesheet_size).unwrap();

        let mut current_x: i32 = 0;
        let mut current_y: i32 = 0;
        let mut highest_y_in_row: i32 = 0;

        let sprite_size = spritesheet_size / spritesheet.spritegen.sprites_per_row;

        let sprites = spritesheet.sprites.clone();
        let mut sorted_sprites: Vec<(&String, &crate::sources::SpriteSpecifier)> =
            sprites.iter().collect();
        sorted_sprites.sort_by(|lhs, rhs| lhs.0.cmp(rhs.0));

        let mut sprites_for_spritesheet: HashMap<String, Sprite> = HashMap::new();

        #[cfg(feature = "indev")]
        let mut visualizer_rng = rand::rng();

        for (sprite_key, specifier) in sorted_sprites {
            match specifier.fetch(reqwest.clone()).await {
                Ok(source) => {
                    let (width, height) = match &source {
                        SpriteSource::Raster(pixmap) => {
                            (pixmap.width() as f32, pixmap.height() as f32)
                        }
                        SpriteSource::Vector(tree) => {
                            let size = tree.size();
                            (size.width(), size.height())
                        }
                    };

                    let transform = Transform::from_scale(
                        sprite_size as f32 / width,
                        sprite_size as f32 / height,
                    )
                    .post_translate(current_x as f32, current_y as f32);

                    #[cfg(feature = "indev")]
                    {
                        use rand::Rng;
                        use tiny_skia::{Color, Paint, Rect};

                        let mut paint = Paint::default();
                        paint.set_color(Color::from_rgba8(
                            visualizer_rng.random_range(0..128),
                            visualizer_rng.random_range(0..128),
                            visualizer_rng.random_range(0..128),
                            128,
                        ));

                        current_spritesheet.fill_rect(
                            Rect::from_xywh(0.0, 0.0, width, height).unwrap(),
                            &paint,
                            transform,
                            None,
                        );
                    }

                    match source {
                        SpriteSource::Raster(pixmap) => {
                            current_spritesheet.draw_pixmap(
                                0,
                                0,
                                pixmap.as_ref(),
                                &PixmapPaint::default(),
                                transform,
                                None,
                            );
                        }
                        SpriteSource::Vector(tree) => {
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

        let mut index = 0;
        for pixmap in &pixmaps {
            let _ = pixmap.save_png(format!("test/{spritesheet_key}_{index}.png"));
            index += 1;
        }

        spritesheets.insert(spritesheet_key, Spritesheet {
            pixmaps,
            sprites: sprites_for_spritesheet,
        });
    }

    Ok(spritesheets)
}
