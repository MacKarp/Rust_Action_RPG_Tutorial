use gdnative::prelude::*;

mod bat;
mod grass;
mod grass_effect;
mod hitbox;
mod player;
mod stats;
mod sword_hitbox;
mod utils;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<grass::Grass>();
    handle.add_class::<grass_effect::GrassEffect>();
    handle.add_class::<bat::Bat>();
    handle.add_class::<stats::Stats>();
    handle.add_class::<hitbox::Hitbox>();
    handle.add_class::<sword_hitbox::SwordHitbox>();
}

// Macro that create the entry-points of the dynamic library.
godot_init!(init);
