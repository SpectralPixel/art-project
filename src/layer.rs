// the functions in layer.rs are meant to be kinda ambigous so that they support many different cellular automata.
// it makes testing of varous different automata a lot less messy and streamlined.

use crate::{
    ARRAY_LENGTH,
    MAP_DIMS,
    layer::utils::*
};
use bevy::{
    math::*,
    render::color::Color
};
use array_init::array_init;
use bevy_pixel_buffer::pixel::Pixel;

pub mod utils;
pub mod conway;
pub mod boscos;
pub mod majority;

pub fn calculate_next_gen(cur_gen: &[Pixel]) -> [Pixel; ARRAY_LENGTH] {

    let calculated_red: [f32; ARRAY_LENGTH] = boscos::calculate_next_gen(cur_gen);
    let calculated_green: [f32; ARRAY_LENGTH] = conway::calculate_next_gen(cur_gen);
    let calculated_blue: [f32; ARRAY_LENGTH] = majority::calculate_next_gen(cur_gen);

    let mut calculated_gen: [Pixel; ARRAY_LENGTH] = array_init(|_| Pixel::WHITE);
    for i in 0..calculated_gen.len() {
        calculated_gen[i] = Pixel::from([
            calculated_red[i],
            calculated_green[i],
            calculated_blue[i]
        ]);
    }
    calculated_gen
}

pub fn calc_cell_sum(pos: &UVec2, pattern: &[Neighbor], cur_gen: &[Pixel], filter: CellMode) -> f32 {
    let mut cell_sum = 0.; 

    for neighbor in pattern {
        let rel_pos = neighbor.position();
        let check_pos = IVec2 {
            x: pos.x as i32 + rel_pos.x,
            y: pos.y as i32 + rel_pos.y,
        };

        let cell_index = get_cell_index(check_pos);
        let cell_pixel = cur_gen[cell_index];
        let cell_value = get_cell_value(cell_pixel, &filter);
        cell_sum += cell_value * neighbor.weight();
    }

    cell_sum
}

pub fn get_cell_value(cell_pixel: Pixel, filter: &CellMode) -> f32 {
    match filter {
        CellMode::Color(pixel) => {
            if cell_pixel == *pixel {
                1.
            } else {
                0.
            }
        },
        CellMode::NotColor(pixel) => {
            if cell_pixel != *pixel {
                1.
            } else {
                0.
            }
        },
        CellMode::Channel(channel) => {
            let cell_color = round_color_fix(cell_pixel.as_color());
            match channel {
                ColorChannel::Red => cell_color.r(),
                ColorChannel::Green => cell_color.g(),
                ColorChannel::Blue => cell_color.b(),
            }
        },
        CellMode::TotalValue => {
            let cell_color = round_color_fix(cell_pixel.as_color());
            let total_value = cell_color.r() + cell_color.g() + cell_color.b();
            total_value
        },
    }
}

// when converting from a pixel to a color, the color ends up with a slight precision error.
// this aims to fix that by rounding all channels during a conversion.
fn round_color_fix(mut color: Color) -> Color {
    let r = color.r();
    let g = color.g();
    let b = color.b();
    if r > 0.99 || r < 0.01 {
        color.set_r(r.round());
    }
    if g > 0.99 || g < 0.01 {
        color.set_g(g.round());
    }
    if b > 0.99 || b < 0.01 {
        color.set_b(b.round());
    }
    color
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