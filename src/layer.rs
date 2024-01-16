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
use rand::{thread_rng, Rng};

pub mod utils;
pub mod conway;
pub mod boscos;
pub mod majority;
pub mod marine;

pub const SURVIVAL_VALUE: f32 = 0.95;
pub const FADE_FACTOR: f32 = 2.1;
const LEAKAGE: i32 = 20; // lower value = higher chance of leak

pub fn calculate_next_gen(cur_gen: &[Pixel]) -> [Pixel; ARRAY_LENGTH] {

    let calculated_red: [f32; ARRAY_LENGTH] = boscos::calculate_next_gen(cur_gen);
    let calculated_green: [f32; ARRAY_LENGTH] = marine::calculate_next_gen(cur_gen);
    let calculated_blue: [f32; ARRAY_LENGTH] = majority::calculate_next_gen(cur_gen);

    let mut calculated_gen: [Pixel; ARRAY_LENGTH] = array_init(|_| Pixel::WHITE);
    for i in 0..calculated_gen.len() {
        // calculated_gen[i] = Pixel::from([
        //     0.,//calculated_red[i],
        //     calculated_green[i],
        //     0.//calculated_blue[i]
        // ]);

        let rand_r = thread_rng().gen_range(0..LEAKAGE);
        let rand_g = thread_rng().gen_range(0..LEAKAGE);
        let rand_b = thread_rng().gen_range(0..LEAKAGE);
        calculated_gen[i] = Pixel::from([
            if rand_r != 0 { calculated_red[i] }
            else {
                if rand_r % 2 == 0 {
                    if calculated_green[i] >= SURVIVAL_VALUE { calculated_green[i] }
                    else { calculated_red[i] }
                }
                else {
                    if calculated_blue[i] >= SURVIVAL_VALUE { calculated_blue[i] }
                    else { calculated_red[i] }
                }   
            },

            if rand_g != 0 { calculated_green[i] }
            else {
                if rand_g % 2 == 0 {
                    if calculated_red[i] >= SURVIVAL_VALUE { calculated_red[i] }
                    else { calculated_green[i] }
                }
                else {
                    if calculated_blue[i] >= SURVIVAL_VALUE { calculated_blue[i] }
                    else { calculated_green[i] }
                }    
            },

            if rand_b != 0 { calculated_blue[i] }
            else {
                if rand_b % 2 == 0 {
                    if calculated_red[i] >= SURVIVAL_VALUE { calculated_red[i] }
                    else { calculated_blue[i] }
                }
                else {
                    if calculated_green[i] >= SURVIVAL_VALUE { calculated_green[i] }
                    else { calculated_blue[i] }
                }    
            },
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
        CellMode::Channel(channel, threshold) => {
            let cell_color = round_color_fix(cell_pixel.as_color());
            match channel {
                ColorChannel::Red => {
                    let r = cell_color.r();
                    if r >= *threshold {
                        1.
                    } else {
                        0.
                    }
                },
                ColorChannel::Green => {
                    let g = cell_color.g();
                    if g >= *threshold {
                        1.
                    } else {
                        0.
                    }
                },
                ColorChannel::Blue => {
                    let b = cell_color.b();
                    if b >= *threshold {
                        1.
                    } else {
                        0.
                    }
                },
            }
        },
        CellMode::ChannelValue(channel) => {
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
pub fn round_color_fix(mut color: Color) -> Color {
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

pub fn fade_cell(mut cell: f32) -> f32 {
    if cell < SURVIVAL_VALUE {
        cell /= FADE_FACTOR;
        cell -= 0.03;
    }
    cell
}