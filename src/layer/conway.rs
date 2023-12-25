use crate::{ARRAY_LENGTH, MAP_DIMS};
use array_init::array_init;
use bevy::math::*;
use bevy_pixel_buffer::pixel::Pixel;

use super::Weight;

const PATTERN: [Weight; 8] = [
    Weight { position: IVec2 { x: -1, y:  1 }, multiplier: 1. },
    Weight { position: IVec2 { x:  0, y:  1 }, multiplier: 1. },
    Weight { position: IVec2 { x:  1, y:  1 }, multiplier: 1. },
    Weight { position: IVec2 { x: -1, y:  0 }, multiplier: 1. },
    Weight { position: IVec2 { x:  1, y:  0 }, multiplier: 1. },
    Weight { position: IVec2 { x: -1, y: -1 }, multiplier: 1. },
    Weight { position: IVec2 { x:  0, y: -1 }, multiplier: 1. },
    Weight { position: IVec2 { x:  1, y: -1 }, multiplier: 1. }
];

pub fn calculate_next_gen(cur_gen: &[Pixel]) -> [Pixel; ARRAY_LENGTH] {
    let mut calculated_gen: [Pixel; ARRAY_LENGTH] = array_init(|_| Pixel::WHITE);

    for cell_index in 0..cur_gen.len() {

        let cur_cell_value = cur_gen[cell_index];
        let cell_pos = UVec2 {
            x: cell_index as u32 % MAP_DIMS.size.x,
            y: (cell_index as f64 / MAP_DIMS.size.x as f64).floor() as u32,
        };

        let cell_sum = super::calc_cell_sum(&cell_pos, &PATTERN, &cur_gen);

        let calculated_cell = apply_rules(cell_sum as u8, cur_cell_value);

        calculated_gen[cell_index] = calculated_cell;
    }

    calculated_gen
}

fn apply_rules(nearby_cell_count: u8, cur_cell: Pixel) -> Pixel {
    if nearby_cell_count == 3 || (cur_cell == Pixel::WHITE && nearby_cell_count == 2) {
        Pixel::WHITE
    } else {
        Pixel::BLACK
    }
}