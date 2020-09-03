use gdnative::api::*;
use gdnative::prelude::*;

// Hitbox "class".
#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Hitbox {
    #[property(default = 1)]
    pub damage: i64,
}
#[gdnative::methods]
impl Hitbox {
    pub fn new(_owner: &Area2D) -> Self {
        Hitbox { damage: 1 }
    }
}
