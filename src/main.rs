mod map;

use bevy::prelude::*;
use bevy::time::prelude::Fixed;
use bevy_pixel_buffer::prelude::*;

const UPDATE_RATE: f32 = 0.25;
const MAP_SIZE: PixelBufferSize = PixelBufferSize {
    size: UVec2::new(32, 32),       // amount of pixels
    pixel_size: UVec2::new(16, 16), // size of each pixel in the screen
};

fn main() {
    println!("i like cats");

    map::red_layer::test_function();
    map::green_layer::test_function();
    map::blue_layer::test_function();
    map::test_function();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, pixel_buffer_setup(MAP_SIZE))
        // FixedUpdate runs a set amount of times every seconds, and is independent from screen updates
        .add_systems(FixedUpdate, update_screen)
        // set FixedUpdate rate
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .run();
}

fn update_screen(mut pb: QueryPixelBuffer) {
    // Set each pixel to a random color
    pb.frame().per_pixel(|_, _| Pixel::random());
}