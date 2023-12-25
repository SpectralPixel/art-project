use crate::{ARRAY_LENGTH, MAP_DIMS};
use array_init::array_init;
use bevy::math::*;
use bevy_pixel_buffer::pixel::Pixel;

pub mod blue;
pub mod green;
pub mod red;
pub mod conway;

pub fn test_function() {
    println!("map works!");
}

// pub fn calculate_next_gen(cur_gen: &[Pixel]) -> [Pixel; ARRAY_LENGTH] {

//     let mut calc_red: [f64; ARRAY_LENGTH] = array_init(|_| 0.);
//     let mut calc_green: [f64; ARRAY_LENGTH] = array_init(|_| 0.);
//     let mut calc_clue: [f64; ARRAY_LENGTH] = array_init(|_| 0.);

//     vec![Pixel::WHITE]

// }

pub fn calculate_next_gen_conway(cur_gen: &[Pixel]) -> [Pixel; ARRAY_LENGTH] {
    let mut calculated_conway: [Pixel; ARRAY_LENGTH] = array_init(|_| Pixel::BLACK);

    for cell_index in 0..cur_gen.len() {

        let cell = cur_gen[cell_index];
        let cell_pos = UVec2 {
            x: cell_index as u32 % MAP_DIMS.size.x,
            y: (cell_index as f64 / MAP_DIMS.size.x as f64).floor() as u32,
        };

        let nearby_cell_count = get_nearby_cell_count(&cell_pos, &cur_gen);

        let calculated_cell = apply_rules_conway(nearby_cell_count, cell);

        calculated_conway[cell_index] = calculated_cell;
    }

    calculated_conway
}

fn get_nearby_cell_count(pos: &UVec2, cur_gen: &[Pixel]) -> u8 {
    let mut nearby_cell_count: u8 = 0;

    // directions that will be added to the position to check for nearby live cells
    let check_directions: [IVec2; 8] = [
        IVec2 { x: -1, y: 1 },
        IVec2 { x: 0, y: 1 },
        IVec2 { x: 1, y: 1 },
        IVec2 { x: -1, y: 0 },
        IVec2 { x: 1, y: 0 },
        IVec2 { x: -1, y: -1 },
        IVec2 { x: 0, y: -1 },
        IVec2 { x: 1, y: -1 },
    ];

    for dir in check_directions {
        let check_pos = IVec2 {
            x: pos.x as i32 + dir.x,
            y: pos.y as i32 + dir.y,
        };

        nearby_cell_count += get_cell_value_conway(check_pos, &cur_gen);
    }

    nearby_cell_count
}

fn get_cell_value_conway(pos: IVec2, cur_gen: &[Pixel]) -> u8 {
    let inbounds_pos = ensure_inbounds(pos);

    let flattened_pos: usize = flatten_pos(inbounds_pos);
    if cur_gen[flattened_pos] == Pixel::WHITE {
        1
    } else {
        0
    }
}

fn flatten_pos(pos: UVec2) -> usize {
    (pos.x + pos.y * MAP_DIMS.size.x) as usize
}

fn ensure_inbounds(pos: IVec2) -> UVec2 {
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

fn apply_rules_conway(nearby_cell_count: u8, cur_cell: Pixel) -> Pixel {

    if nearby_cell_count == 3 || (cur_cell == Pixel::WHITE && nearby_cell_count == 2) {
        Pixel::WHITE
    } else {
        Pixel::BLACK
    }
}