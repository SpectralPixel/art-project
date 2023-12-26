// the functions in layer.rs are meant to be kinda ambigous so that they support many different cellular automata.
// it makes testing of varous different automata a lot less messy and streamlined.

use crate::{ARRAY_LENGTH, MAP_DIMS};
use array_init::array_init;
use bevy::math::*;
use bevy_pixel_buffer::pixel::Pixel;

pub mod conway;

// when you are getting the sum of nearby values that will be used to determine the cell's next state,
// it is unclear whether is should be getting the values in the GoL way or the Lenia way, and if in the lenia way then what color channel???
pub enum SumFilter {
    White,          // if the pixel is perfectly white, return 1. else return 0
    NotBlack,       // if the pixel is not black, return 1. else return 0
    Color(Channel), // get only the value of a single color channel
    TotalValue,     // red, green and blue all added together to create a "total value" (a buzzword i just made up)
}

pub enum Channel {
    Red,
    Green,
    Blue
}

pub struct Neighbor {
    pub position: IVec2,
    pub weight: f32
}

pub fn _calculate_next_gen(_cur_gen: &[Pixel]) -> [Pixel; ARRAY_LENGTH] {

    // let calculated_red: [f64; ARRAY_LENGTH] = array_init(|_| 0.);
    // let calculated_green: [f64; ARRAY_LENGTH] = array_init(|_| 0.);
    // let calculated_blue: [f64; ARRAY_LENGTH] = array_init(|_| 0.);

    let placeholder: [Pixel; ARRAY_LENGTH] = array_init(|_| Pixel::WHITE);
    placeholder
}

pub fn calc_cell_sum(pos: &UVec2, pattern: &[Neighbor], cur_gen: &[Pixel]) -> f32 {
    let mut cell_sum = 0.; 

    for neighbor in pattern {
        let rel_pos = neighbor.position;
        let check_pos = IVec2 {
            x: pos.x as i32 + rel_pos.x,
            y: pos.y as i32 + rel_pos.y,
        };

        let cell_index = get_cell_index(check_pos);
        let cell_pixel = cur_gen[cell_index];
        let cell_value = get_cell_value(cell_pixel, SumFilter::White);
        cell_sum += cell_value * neighbor.weight;
    }

    cell_sum
}

pub fn get_cell_value(pixel: Pixel, filter: SumFilter) -> f32 {
    let cell_color = pixel.as_color();
    match filter {
        SumFilter::White => {
            if pixel == Pixel::WHITE {
                1.
            } else {
                0.
            }
        },
        SumFilter::NotBlack => {
            if pixel != Pixel::BLACK {
                1.
            } else {
                0.
            }
        },
        SumFilter::Color(channel) => {
            match channel {
                Channel::Red => cell_color.r(),
                Channel::Green => cell_color.g(),
                Channel::Blue => cell_color.b(),
            }
        },
        SumFilter::TotalValue => {
            let total_value = cell_color.r() + cell_color.g() + cell_color.b();
            total_value
        },
    }
}

pub fn get_cell_index(pos: IVec2) -> usize {
    let inbounds_pos = ensure_inbounds(pos);
    let index: usize = flatten_pos(inbounds_pos);
    
    index
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