use common::texture_manager::{TextureManager, TextureResource, create_texture_image};
use crate::textures::cobble_stone::TextureError::ImageError;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum TextureError {
    #[error("Error loading texture: {0}")]
    ImageError(String),
}
pub struct CobbleStone {
    pub texture: TextureResource,
}

pub fn create_cobble_stone_texture(texture_manager: &mut TextureManager) -> anyhow::Result<()> {
    let width = 8;
    let height = 16;
    let data = [
        [50, 50, 50, 255],
        [200, 200, 200, 255],
        [20, 20, 20, 255],
        [220, 220, 220, 255],
    ];
    let texture_image = create_texture_image(width, height, data);
    let file_path = "/assets/cobble_stone.png";

    texture_image
        .save(file_path)
        .map_err(|e|
            ImageError(e.to_string())
        )?;

    texture_manager
        .load_texture("test_texture", file_path);

    Ok(())
}

