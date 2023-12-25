use crate::{ARRAY_LENGTH, MAP_DIMS};
use array_init::array_init;
use bevy::math::*;
use bevy_pixel_buffer::pixel::Pixel;

pub mod conway;

pub struct Weight {
    position: IVec2,
    multiplier: f64
}

pub fn _calculate_next_gen(_cur_gen: &[Pixel]) -> [Pixel; ARRAY_LENGTH] {

    // let calculated_red: [f64; ARRAY_LENGTH] = array_init(|_| 0.);
    // let calculated_green: [f64; ARRAY_LENGTH] = array_init(|_| 0.);
    // let calculated_blue: [f64; ARRAY_LENGTH] = array_init(|_| 0.);

    let placeholder: [Pixel; ARRAY_LENGTH] = array_init(|_| Pixel::WHITE);
    placeholder
}

pub fn calc_cell_sum(pos: &UVec2, pattern: &[Weight], cur_gen: &[Pixel]) -> f64 {
    let mut cell_value = 0.; 

    for weight in pattern {
        let rel_pos = weight.position;
        let check_pos = IVec2 {
            x: pos.x as i32 + rel_pos.x,
            y: pos.y as i32 + rel_pos.y,
        };

        cell_value += get_cell_value(check_pos, &cur_gen) * weight.multiplier;
    }

    cell_value
}

pub fn get_cell_value(pos: IVec2, cur_gen: &[Pixel]) -> f64 {
    let inbounds_pos = ensure_inbounds(pos);

    let flattened_pos: usize = flatten_pos(inbounds_pos);
    if cur_gen[flattened_pos] == Pixel::WHITE {
        1.
    } else {
        0.
    }
}

pub fn flatten_pos(pos: UVec2) -> usize {
    (pos.x + pos.y * MAP_DIMS.size.x) as usize
}

pub fn ensure_inbounds(pos: IVec2) -> UVec2 {
    let mut pos = IVec2 { x: pos.x, y: pos.y };

    pos.x = if pos.x < 0 {
        pos.x + MAP_DIMS.size.x as i32
    } else {
        pos.x % MAP_DIMS.size.x as i32
    };

    pos.y = if pos.y < 0 {
        pos.y + MAP_DIMS.size.y as i32
    } else {
        pos.y % MAP_DIMS.size.y as i32
    };

    UVec2 {
        x: pos.x as u32,
        y: pos.y as u32,
    }
}