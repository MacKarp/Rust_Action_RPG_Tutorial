use crate::hitbox::*;
use gdnative::api::*;
use gdnative::prelude::*;

// SwordHitbox "class".
#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct SwordHitbox {
    #[property]
    knockback_vector: Vector2,
    hitbox: Hitbox,
    #[property(default = true)]
    show_hit: bool,
}

#[gdnative::methods]
impl SwordHitbox {
    fn new(owner: &Area2D) -> Self {
        SwordHitbox {
            knockback_vector: Vector2::zero(),
            hitbox: Hitbox::new(&owner),
            show_hit: true,
        }
    }

    #[export]
    fn get_hitbox_damage(&self, _owner: &Area2D) -> i64 {
        self.hitbox.damage
    }
}
