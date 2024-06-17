use std::cmp::{max, min};
use crate::textures::cobble_stone::LightDirection;

pub fn update_color_based_on_light_direction(light_direction: LightDirection, base_color: [u8;3], x_pos: usize, y_pos: usize) -> [u8;3] {
    let mut color = base_color;
    for i in 0..base_color.len() {
        let x_pos = x_pos as u8;
        let y_pos = y_pos as u8;
        match light_direction {
            LightDirection::Left => {
                color[i] = max(0, min(255, base_color[i] - (x_pos * 10)));
            }
            LightDirection::Right => {
                color[i] = max(0, min(255, base_color[i] + (x_pos * 10)));
            }
            LightDirection::Front => {
                color[i] = max(0, min(255, base_color[i] - (y_pos * 10)));
            }
            LightDirection::Back => {
                color[i] = max(0, min(255, base_color[i] + (y_pos * 10)));
            }
            LightDirection::LeftFront => {
                color[i] = max(0, min(255, base_color[i] - (x_pos * 10) - (y_pos * 5)));
            }
            LightDirection::RightFront => {
                color[i] = max(0, min(255, base_color[i] + (x_pos * 10) - (y_pos * 5)));
            }
            LightDirection::LeftBack => {
                color[i] = max(0, min(255, base_color[i] - (x_pos * 10) + (y_pos * 5)));
            }
            LightDirection::RightBack => {
                color[i] = max(0, min(255, base_color[i] + (x_pos * 10) + (y_pos * 5)));
            }
        }
    }
    color
}