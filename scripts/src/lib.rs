extern crate gdnative;
use gdnative::prelude::*;

mod player;
mod utils;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
}

// Macro that create the entry-points of the dynamic library.
godot_init!(init);
