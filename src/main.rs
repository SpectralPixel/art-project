mod map;

// this is how to do imports *the cool, brogrammer way*
use bevy::{
    prelude::*, // for example, this actually means bevy::prelude::*
    time::prelude::Fixed, // and this actually means bevy::time::prelude::Fixed
    window::WindowMode, // basically it means you have to type in "bevy::" 3 times less, but also makes everything 100% more confusing
};
use bevy_pixel_buffer::prelude::*;

const UPDATE_RATE: f64 = 0.25;
const MAP_SIZE: PixelBufferSize = PixelBufferSize {
    size: UVec2::new(160, 90),    // amount of pixels
    pixel_size: UVec2::new(9, 9), // size of each pixel in the screen
};

fn main() {
    println!("i like cats");

    map::test_function();
    map::red_layer::test_function();
    map::green_layer::test_function();
    map::blue_layer::test_function();

    // All window setting in here
    // docs for "Window": https://docs.rs/bevy/latest/bevy/window/struct.Window.html
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
        .add_systems(FixedUpdate, update_simulation)
        // set FixedUpdate rate
        .insert_resource(Time::<Fixed>::from_seconds(UPDATE_RATE))
        .run();
}

fn update_simulation(mut pb: QueryPixelBuffer) {

    // SEPARATE PIXEL DATA INTO IT'S OWN ARRAY
    // INITIALIZE ARRAYS FOR EVERY COLOR
    // CALCULATE ARRAY OF COLORS WITH PIXEL DATA
    // SET ALL SCREEN TO COMBINATION OF PIXEL DATAS




    // THIS IS PLACEHOLDER CODE
    // Set each pixel to a random color
    pb.frame().per_pixel(|_, _| Pixel::random());
}