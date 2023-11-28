mod map;

use bevy::prelude::*;
use bevy::time::prelude::Fixed;
use bevy::window::WindowMode;
use bevy_pixel_buffer::prelude::*;

const UPDATE_RATE: f64 = 0.25;
const MAP_SIZE: PixelBufferSize = PixelBufferSize {
    size: UVec2::new(160, 90),       // amount of pixels
    pixel_size: UVec2::new(5, 5), // size of each pixel in the screen
};

fn main() {
    println!("i like cats");

    map::red_layer::test_function();
    map::green_layer::test_function();
    map::blue_layer::test_function();
    map::test_function();

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: String::from("Cellular Automata by Makki & Emil"),
            mode: WindowMode::Fullscreen,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_systems(
            Startup, 
            PixelBufferBuilder::new()
                .with_size(MAP_SIZE)
                .with_fill(FillKind::Window)
                .setup()
        )
        // FixedUpdate runs a set amount of times every seconds, and is independent from screen updates
        .add_systems(FixedUpdate, update_screen)
        // set FixedUpdate rate
        .insert_resource(Time::<Fixed>::from_seconds(UPDATE_RATE))
        .run();
}

fn update_screen(mut pb: QueryPixelBuffer) {
    // Set each pixel to a random color
    pb.frame().per_pixel(|_, _| Pixel::random());
    //pb.frame().try_into()
}