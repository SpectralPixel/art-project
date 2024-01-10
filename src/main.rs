// this is how to do imports *the cool, brogrammer way*
use bevy::{
    prelude::*,           // for example, this actually means bevy::prelude::*
    time::prelude::Fixed, // and this actually means bevy::time::prelude::Fixed
    window::WindowMode,   // basically it means you have to type in "bevy::" 3 times less, but also makes everything 100% more confusing
};
use bevy_pixel_buffer::prelude::*;
use std::time::SystemTime;

mod layer;

const UPDATE_RATE: f64 = 0.1; // IMPORTANT!!!!!!!!! use space to progress time for now

// Map dimensions
const MAP_DIMS: PixelBufferSize = PixelBufferSize {
    size: UVec2::new(240, 135),       // amount of pixels               160, 90
    pixel_size: UVec2::new(6, 6), // size of each pixel onscreen      9, 9
};

const ARRAY_LENGTH: usize = (MAP_DIMS.size.x * MAP_DIMS.size.y) as usize; // How large flattened arrays storing the map data should be

fn main() {
    println!("i like cats");

    // All window setting in here
    // docs for "Window": https://docs.rs/bevy/latest/bevy/window/struct.Window.html
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: String::from("TERRIBLE Cellular Automata by Makki & Emil"),
            mode: WindowMode::Fullscreen,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_systems(Startup, setup_simulation)
        .add_systems(
            PostStartup, // FixedUpdate runs a set amount of times every seconds, and is independent from screen updates
            update_simulation,
        )
        .add_systems(
            Update, // (chenge this line to fixedupdate instead of update in final product) FixedUpdate runs a set amount of times every seconds, and is independent from screen updates
            check_for_keys, // FOR MANUAL: Update - check_for_keys | FOR AUTOMATIC: FixedUpdate - update_simulation
        )
        .add_systems(
            Update,
            bevy::window::close_on_esc, // close window when esc is pressed
        )
        .insert_resource(
            Time::<Fixed>::from_seconds(UPDATE_RATE), // set FixedUpdate rate
        )
        .run();
}

fn setup_simulation(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    PixelBufferBuilder::new()
        .with_size(MAP_DIMS)
        .spawn(&mut commands, &mut images)
        // initialize map with random pixel data
        .edit_frame(|frame| {
            frame.per_pixel(|_, _| {
                let mut new_color = Color::BLACK;
                if rand::random::<f32>() > 0.5 {
                    new_color = *new_color.set_r(1.);
                } 
                if rand::random::<f32>() > 0.35 {
                    new_color = *new_color.set_g(1.);
                }
                if rand::random::<f32>() > 0.5 {
                    new_color = *new_color.set_b(1.);
                }
                Pixel::from(new_color)
            })
        });
}

fn check_for_keys(pb: QueryPixelBuffer, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        update_simulation(pb);
    }
}

fn update_simulation(mut pb: QueryPixelBuffer) {
    let step_start_timestamp = SystemTime::now();

    let mut frame = pb.frame();
    let cur_gen: &[Pixel] = &frame.raw();

    let next_gen = layer::calculate_next_gen(cur_gen);

    // SET THE SCREEN TO THE NEXT GENERATION
    frame.per_pixel_par(|pos, _| {
        let index = layer::flatten_pos(pos);
        next_gen[index]
    });

    println!(
        "Time to calculate: {}",
        SystemTime::now() // gets the current system time
            .duration_since(step_start_timestamp) // gets the difference between the current time and the time at the start of the calculation, returns Result<Type, Error>
            .unwrap_or_default() // gets the Type out of the returned Result<Type, Error>, and if there is an error it just turns it into the default (which in this case is 0)
            .as_secs_f32() // cast from Duration into f32
    )
}
