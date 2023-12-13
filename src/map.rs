use bevy::math::*;
use bevy_pixel_buffer::pixel::Pixel;
use array_init::array_init;
use crate::{ARRAY_LENGTH, MAP_DIMS};

pub mod red_layer;
pub mod green_layer;
pub mod blue_layer;

pub fn test_function() {
    println!("map works!");
}

// pub fn calculate_next_gen(cur_gen: &[Pixel]) -> Vec<Pixel> {

//     let mut calc_red: [f64; ARRAY_LENGTH] = array_init(|_| 0.);
//     let mut calc_green: [f64; ARRAY_LENGTH] = array_init(|_| 0.);
//     let mut calc_clue: [f64; ARRAY_LENGTH] = array_init(|_| 0.);

    

//     vec![Pixel::WHITE]

// }

pub fn calculate_next_gen_conway(cur_gen: &[Pixel]) -> [Pixel; ARRAY_LENGTH] {

    let mut calc_conway: [Pixel; ARRAY_LENGTH] = array_init(|_| Pixel::BLACK);
    let mut live_cells = 0;

    for cell_index in 0..cur_gen.len() {

        //println!();

        let cell = cur_gen[cell_index];
        let cell_pos = UVec2 {
           x: cell_index as u32 % MAP_DIMS.size.x,
           y: (cell_index as f64 / MAP_DIMS.size.x as f64).floor() as u32,
        };
        //println!("i: {} - pos: {}, {}", cell_index, cell_pos.x, cell_pos.y);

        if cell_pos.x == 0 || cell_pos.y == 0 || cell_pos.x == MAP_DIMS.size.x - 1 || cell_pos.y == MAP_DIMS.size.y - 1 {
            calc_conway[cell_index] = Pixel::RED;
            continue;
        }

        let nearby_cell_count = get_nearby_cell_count(&cell_pos, &cur_gen);
        // if nearby_cell_count > 3 {
        //     println!("neighbors: {}", nearby_cell_count);
        // }

        let calc_cell = apply_rules_conway(nearby_cell_count, cell);

        if calc_cell == Pixel::WHITE {
            live_cells += 1;
        }

        calc_conway[cell_index] = calc_cell;
    }

    println!("{} cells alive!", live_cells);

    calc_conway

}

fn get_nearby_cell_count(pos: &UVec2, cur_gen: &[Pixel]) -> u8 {

    let mut nearby_cell_count: u8 = 0;

    // directions that will be added to the position to check for nearby live cells
    let check_directions: [IVec2; 8] = [
        IVec2 {x: -1, y:  1}, IVec2 {x: 0, y:  1}, IVec2 {x:  1,  y: 1},
        IVec2 {x: -1, y:  0}, /*  CUR CELL POS  */ IVec2 {x: -1, y:  1},
        IVec2 {x: -1, y: -1}, IVec2 {x: 0, y: -1}, IVec2 {x: -1, y: -1},
    ];

    for dir in check_directions {
        let check_pos = IVec2 {
            x: pos.x as i32 + dir.x,
            y: pos.y as i32 + dir.y
        };

        //println!("{} + {} = {}", &pos, &dir, &check_pos);

        nearby_cell_count += get_cell_value_conway(check_pos, &cur_gen);
    }

    nearby_cell_count
}

fn get_cell_value_conway(pos: IVec2, cur_gen: &[Pixel]) -> u8 {

    // ensure that the position is inbounds and turn it into a UVec2 rather than a IVec32
    let pos = ensure_inbounds(&pos);

    //println!("{}", &pos);
    
    let flattened_pos: usize = (pos.x + pos.y * MAP_DIMS.size.x) as usize;
    if cur_gen[flattened_pos] == Pixel::WHITE {
        1
    }
    else {
        0
    }
}

fn ensure_inbounds(pos: &IVec2) -> UVec2 {

    let mut pos = IVec2 { x: pos.x, y: pos.y };
    //let prev_pos = IVec2 { x: pos.x, y: pos.y };

    pos.x = if pos.x < 0 {
        (MAP_DIMS.size.x - 1) as i32
    } else {
        pos.x % MAP_DIMS.size.x as i32
    };

    pos.y = if pos.y < 0 {
        (MAP_DIMS.size.y - 1) as i32
    } else {
        pos.y % MAP_DIMS.size.y as i32
    };

    // // print inbounds checks
    // if pos != prev_pos {
    //     println!("{}, {} -> {}, {}", prev_pos.x, prev_pos.y, pos.x, pos.y);
    // }

    UVec2 {
        x: pos.x as u32,
        y: pos.y as u32
    }
}

fn apply_rules_conway(nearby_cell_count: u8, cur_cell: Pixel) -> Pixel {

    // let state = if cur_cell == Pixel::WHITE {
    //     String::from("live")
    // } else {
    //     String::from("dead")
    // };

    if 
        nearby_cell_count == 3 ||
        (cur_cell == Pixel::WHITE && nearby_cell_count == 2)
    {
        //println!("{} cell near {} -> alive", state, nearby_cell_count);
        Pixel::WHITE 
    }
    else {
        //println!("{} cell near {} -> dead", state, nearby_cell_count);
        Pixel::BLACK
    }
}