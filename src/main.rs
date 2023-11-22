mod map;

use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;

fn main() {
    println!("i like cats");

    map::red_layer::test_function();
    map::green_layer::test_function();
    map::blue_layer::test_function();
    map::test_function();

    let map_size = PixelBufferSize {
        size: UVec2::new(32, 32),       // amount of pixels
        pixel_size: UVec2::new(16, 16), // size of each pixel in the screen
    }; // Setup system

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, pixel_buffer_setup(map_size))
        .add_systems(Update, pixel_update)
        .run();
}

fn pixel_update(mut pb: QueryPixelBuffer) {
    // Set each pixel to a random color
    pb.frame().per_pixel(|_, _| Pixel::random());
}