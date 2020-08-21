extern crate gdnative;
use gdnative::prelude::*;

mod grass;
mod grass_effect;
mod player;
mod utils;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<grass::Grass>();
    handle.add_class::<grass_effect::GrassEffect>();
}

// Macro that create the entry-points of the dynamic library.
godot_init!(init);
