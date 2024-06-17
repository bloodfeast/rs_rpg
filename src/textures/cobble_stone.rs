use std::path::Path;
use common::texture_manager::{TextureManager, TextureResource, create_texture_tile, concat_image_tiles, concat_image_rows};
use crate::textures::cobble_stone::TextureError::ImageError;
use thiserror::Error;
use crate::textures::utils::update_color_based_on_light_direction;

#[derive(Debug, Error)]
pub enum TextureError {
    #[error("Error loading texture: {0}")]
    ImageError(String),
}

pub struct CobbleStone {
    pub texture: TextureResource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightDirection {
    Left,
    Right,
    Front,
    Back,
    LeftFront,
    RightFront,
    LeftBack,
    RightBack,
}

fn create_cobble_stone_texture_tile(
    x_pos: usize,
    y_pos: usize,
    base_color: [u8; 3],
    light_direction: LightDirection
) -> anyhow::Result<String> {

    let color = update_color_based_on_light_direction(light_direction, base_color, x_pos, y_pos);
    let alpha = if x_pos == 0 || x_pos == 7 || y_pos == 0 || y_pos == 3 {
        100
    } else {
        255
    };

    let width = 2;
    let height = 2;
    let data = [
        [color[0], color[1], color[2], alpha],
        [color[0], color[1], color[2], alpha],
        [color[0], color[1], color[2], alpha],
        [color[0], color[1], color[2], alpha],
    ];

    let texture_image = create_texture_tile(width, height, data);
    let file_path = format!("/assets/cobble_stone-x{x_pos}-y{y_pos}.png");

    texture_image
        .save(&file_path)
        .map_err(|e|
            ImageError(e.to_string())
        )?;

    Ok(file_path)
}

pub fn create_cobble_stone_brick(
    texture_manager: &mut TextureManager,
    base_color: [u8; 3],
    light_direction: LightDirection
) -> anyhow::Result<()> {
    let mut texture_rows = Vec::new();

    for y_pos in 0..4 {
        let mut paths_in_row = Vec::new();

        for x_pos in 0..8 {
            let file_path = create_cobble_stone_texture_tile(x_pos, y_pos, base_color, light_direction)?;
            paths_in_row.push(file_path);
        }

        texture_rows.push(
            concat_image_tiles(
                paths_in_row
                    .iter()
                    .map(|x| Path::new(x))
                    .collect::<Vec<&Path>>()
                    .as_slice()
            )
        );

        paths_in_row
            .iter()
            .for_each(|x|
                std::fs::remove_file(x)
                    .expect("Error removing file (create_cobble_stone_brick)")
            );
    }

    let full_img = concat_image_rows(texture_rows);
    let file_path = format!("/assets/cobble_stone_{:?}.png", light_direction);

     full_img
        .save(&file_path)
        .map_err(|e|
            ImageError(e.to_string())
        )?;

    texture_manager
        .load_texture(format!("cobble_stone_{:?}", light_direction).as_str(), file_path.as_str());

    Ok(())
}
