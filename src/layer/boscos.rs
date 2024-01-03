use crate::{
    ARRAY_LENGTH,
    MAP_DIMS,
    layer::utils::*
};
use array_init::array_init;
use bevy::math::*;
use bevy_pixel_buffer::pixel::Pixel;
use std::sync::OnceLock;

// diameter = radius * 2 + 1
// area of square = diameter ^ 2
// remove center cell = area - 1
const KERNEL_RADIUS: i32 = 5;
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

pub fn calculate_next_gen(cur_gen: &[Pixel]) -> [Pixel; ARRAY_LENGTH] {
    let mut calculated_gen: [Pixel; ARRAY_LENGTH] = array_init(|_| Pixel::BLACK);

    for cell_index in 0..cur_gen.len() {

        let cur_cell_value = cur_gen[cell_index];

        let cell_pos = UVec2 {
            x: cell_index as u32 % MAP_DIMS.size.x,
            y: (cell_index as f64 / MAP_DIMS.size.x as f64).floor() as u32,
        };

        let cell_sum = super::calc_cell_sum(&cell_pos, pattern(), &cur_gen, CellMode::Color(Pixel::WHITE));

        let calculated_cell = apply_rules(cell_sum as u16, cur_cell_value);

        calculated_gen[cell_index] = calculated_cell;
    }

    calculated_gen
}

fn apply_rules(nearby_cell_count: u16, cur_cell: Pixel) -> Pixel {
    if (cur_cell == Pixel::WHITE && (nearby_cell_count >= 33 && nearby_cell_count <= 57)) || (cur_cell != Pixel::WHITE && (nearby_cell_count >= 34 && nearby_cell_count <= 45)) {
        Pixel::WHITE
    } else {
        Pixel::BLACK
    }
}