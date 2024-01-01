use bevy::math::*;
use bevy_pixel_buffer::pixel::Pixel;

// when you are getting the sum of nearby values that will be used to determine the cell's next state,
// it is unclear whether is should be getting the values in the GoL way or the Lenia way, and if in the lenia way then what color channel???
pub enum CellMode {
    Color(Pixel),          // if the pixel is perfectly white, return 1. else return 0
    NotColor(Pixel),       // if the pixel is not black, return 1. else return 0
    Channel(ColorChannel),     // get only the value of a single color channel
    TotalValue,                // red, green and blue all added together to create a "total value" (a buzzword i just made up)
}

#[derive(Eq, PartialEq, Hash)]
pub enum ColorChannel {
    Red,
    Green,
    Blue,
}

pub struct Neighbor {
    position: IVec2,
    weight: f32,
}

impl Neighbor {
    pub fn new(pos: (i32, i32), weight: f32) -> Self {
        Neighbor {
            position: IVec2 { x: pos.0, y: pos.1 }, 
            weight: weight
        }
    }

    pub fn new_full(pos: (i32, i32)) -> Self {
        Neighbor {
            position: IVec2 { x: pos.0, y: pos.1 },
            weight: 1.
        }
    }

    pub fn position(&self) -> IVec2 {
        self.position
    }
    
    pub fn weight(&self) -> f32 {
        self.weight
    }
}