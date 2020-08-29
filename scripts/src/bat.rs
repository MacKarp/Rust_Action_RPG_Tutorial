use crate::utils::*;
use gdnative::api::*;
use gdnative::prelude::*;

// Bat "class".
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Bat {
    knockback: Vector2,
}

#[gdnative::methods]
impl Bat {
    fn new(_owner: &KinematicBody2D) -> Self {
        Bat {
            knockback: Vector2::zero(),
        }
    }
    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f64) {
        self.knockback = move_towards(self.knockback, Vector2::zero(), 200.0 * delta as f32);
        self.knockback = owner.move_and_slide(
            self.knockback,
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );
    }

    // Accepting signal
    #[export]
    fn _on_hurtbox_area_entered(&mut self, _owner: &KinematicBody2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        self.knockback = area.get("knockback_vector").to_vector2() * 120.0;
    }
}
