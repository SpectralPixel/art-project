mod map;

// this is how to do imports *the cool, brogrammer way*
use bevy::{
    prelude::*, // for example, this actually means bevy::prelude::*
    time::prelude::Fixed, // and this actually means bevy::time::prelude::Fixed
    window::WindowMode, // basically it means you have to type in "bevy::" 3 times less, but also makes everything 100% more confusing
};
use bevy_pixel_buffer::prelude::*;


// How often the screen is updated and calculations are run
const UPDATE_RATE: f64 = 1.;

// Map dimensions
const MAP_DIMS: PixelBufferSize = PixelBufferSize {
    size: UVec2::new(40, 22),       // amount of pixels               160, 90
    pixel_size: UVec2::new(36, 36), // size of each pixel onscreen      9, 9
};

// How large flattened arrays storing the map data should be
const ARRAY_LENGTH: usize = (MAP_DIMS.size.x * MAP_DIMS.size.y) as usize;


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
            setup_simulation
        )
        .add_systems(
            FixedUpdate, // FixedUpdate runs a set amount of times every seconds, and is independent from screen updates
            update_simulation
        )
        .insert_resource(
            Time::<Fixed>::from_seconds(UPDATE_RATE) // set FixedUpdate rate
        )
        .run();
}

fn setup_simulation(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    PixelBufferBuilder::new()
        .with_size(MAP_DIMS)
        .spawn(&mut commands, &mut images)

        // initialize map with random pixel data
        .edit_frame(|frame| {
            frame.per_pixel(|_, _| {
                if rand::random::<f32>() > 0.9 {
                    Pixel::WHITE
                } else {
                    Pixel::TRANSPARENT
                }
            })
        });
}

fn update_simulation(mut pb: QueryPixelBuffer) {

    println!("----------");

    let frame = pb.frame();
    let cur_gen: &[Pixel] = frame.raw();

    let next_gen = map::calculate_next_gen_conway(cur_gen);

    /*
    let mut next_gen: [Pixel; ARRAY_LENGTH] = array_init::array_init(|_| {
        if rand::random::<f32>() > 0.9 {
            Pixel::RED
        } else {
            Pixel::TRANSPARENT
        }
    }); // PLACEHOLDER
     */

    // SET THE SCREEN TO THE NEXT GENERATION
    pb.frame().per_pixel_par(|pos, _| {
       let index = (pos.x + pos.y * MAP_DIMS.size.x) as usize;
       next_gen[index]
    });
    //frame.per_pixel(|_, _| Pixel::WHITE);

    // THIS IS PLACEHOLDER CODE
    // Set each pixel to a random color
    //pb.frame().per_pixel(|_, _| Pixel::random());
}