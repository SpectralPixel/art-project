mod map;

use bevy::prelude::*;

fn main() {
    println!("i like cats");

    map::red_layer::test_function();
    map::green_layer::test_function();
    map::blue_layer::test_function();
    map::test_function();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, hello_world)
        .run();
}

fn hello_world() {
    println!("THIS CODE RUNS EVERY FRAME YAYYYYYYYY!");
}