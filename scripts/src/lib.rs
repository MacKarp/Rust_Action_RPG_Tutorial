use gdnative::prelude::*;

mod bat;
mod effect;
mod grass;
mod health_ui;
mod hitbox;
mod hurtbox;
mod player;
mod player_detecion_zone;
mod soft_collision;
mod stats;
mod sword_hitbox;
mod utils;
mod wander_controller;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<grass::Grass>();
    handle.add_class::<effect::Effect>();
    handle.add_class::<bat::Bat>();
    handle.add_class::<stats::Stats>();
    handle.add_class::<hitbox::Hitbox>();
    handle.add_class::<sword_hitbox::SwordHitbox>();
    handle.add_class::<hurtbox::Hurtbox>();
    handle.add_class::<player_detecion_zone::PlayerDetecionZone>();
    handle.add_class::<health_ui::HealthUI>();
    handle.add_class::<soft_collision::SoftCollision>();
    handle.add_class::<wander_controller::WanderController>();
}

// Macro that create the entry-points of the dynamic library.
godot_init!(init);
