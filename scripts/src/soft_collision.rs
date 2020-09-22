use crate::utils::*;
use gdnative::api::*;
use gdnative::prelude::*;

// SoftCollision "class".
#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct SoftCollision {}
#[gdnative::methods]
impl SoftCollision {
    pub fn new(_owner: &Area2D) -> Self {
        SoftCollision {}
    }

    #[export]
    fn is_colliding(&self, owner: &Area2D) -> bool {
        let area = owner.get_overlapping_areas();
        !area.is_empty()
    }

    #[export]
    fn get_push_vector(&self, owner: &Area2D) -> Vector2 {
        let areas = owner.get_overlapping_areas();
        let mut push_vector = Vector2::zero();
        if self.is_colliding(owner) {
            let area = unsafe { areas.get_ref(0) };
            let area = area.try_to_object::<Node2D>().unwrap();
            let area = unsafe { area.assume_safe() };
            push_vector = area.global_position().direction_to(owner.global_position());
            push_vector = normalized(push_vector);
            return push_vector;
        }
        push_vector
    }
}
