use crate::{
    ARRAY_LENGTH,
    MAP_DIMS,
    layer::utils::*
};
use array_init::array_init;
use bevy::math::*;
use bevy_pixel_buffer::pixel::Pixel;
use std::sync::OnceLock;
use super::{SURVIVAL_VALUE, FADE_FACTOR};

// diameter = radius * 2 + 1
// area of square = diameter ^ 2
// remove center cell = area - 1
const KERNEL_RADIUS: i32 = 4;
const KERNEL_CELL_COUNT: usize = ((KERNEL_RADIUS * 2 + 1 as i32).pow(2) - 1) as usize;

fn pattern() -> &'static [Neighbor; KERNEL_CELL_COUNT] {
    static PATTERN: OnceLock<[Neighbor; KERNEL_CELL_COUNT]> = OnceLock::new();
    PATTERN.get_or_init(|| {
        let mut pattern: [Neighbor; KERNEL_CELL_COUNT] = array_init(|_| Neighbor::DUMMY);

        let mut i = 0;
        for x in -KERNEL_RADIUS..KERNEL_RADIUS + 1 {
        for y in -KERNEL_RADIUS..KERNEL_RADIUS + 1 {
            if x == 0 && y == 0 {
                continue;
            }
            pattern[i] = Neighbor::new_full((x, y));
            i += 1;
        }}
        
        pattern
    })
}

pub fn calculate_next_gen(cur_gen: &[Pixel]) -> [f32; ARRAY_LENGTH] {
    let mut calculated_gen: [f32; ARRAY_LENGTH] = array_init(|_| 0.);

    for cell_index in 0..cur_gen.len() {

        let cur_cell_value = cur_gen[cell_index];

        let cell_pos = UVec2 {
            x: cell_index as u32 % MAP_DIMS.size.x,
            y: (cell_index as f64 / MAP_DIMS.size.x as f64).floor() as u32,
        };

        let cell_sum = super::calc_cell_sum(&cell_pos, pattern(), &cur_gen, CellMode::Channel(ColorChannel::Blue, SURVIVAL_VALUE));

        let calculated_cell = apply_rules(cell_sum, super::round_color_fix(cur_cell_value.as_color()).b());

        let faded_cell = super::fade_cell(calculated_cell);

        calculated_gen[cell_index] = faded_cell;
    }

    calculated_gen
}

fn apply_rules(value: f32, cur_cell: f32) -> f32 {
    if (cur_cell >= SURVIVAL_VALUE && (value >= 40. && value <= 80.)) || (cur_cell < SURVIVAL_VALUE && (value >= 41. && value <= 81.)) {
        1.
    } else {
        // thread_rng().gen_range(0..10) as f32 / 100.
        cur_cell / FADE_FACTOR
        // 0.
    }
}