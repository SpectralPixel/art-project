use crate::{
    ARRAY_LENGTH,
    MAP_DIMS,
    layer::utils::*
};
use array_init::array_init;
use bevy::math::*;
use bevy_pixel_buffer::pixel::Pixel;
use std::sync::OnceLock;

fn pattern() -> &'static [Neighbor; 8] {
    static PATTERN: OnceLock<[Neighbor; 8]> = OnceLock::new();
    PATTERN.get_or_init(|| {
        let pattern: [Neighbor; 8] = [
            Neighbor::new_full((-1, 1)),
            Neighbor::new_full(( 0, 1)),
            Neighbor::new_full(( 1, 1)),
            Neighbor::new_full((-1, 0)),
            Neighbor::new_full(( 1, 0)),
            Neighbor::new_full((-1,-1)),
            Neighbor::new_full(( 0,-1)),
            Neighbor::new_full(( 1,-1))
        ];
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