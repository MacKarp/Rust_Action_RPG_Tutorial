use gdnative::api::*;
use gdnative::prelude::*;

// Effect "class".
#[derive(NativeClass)]
#[inherit(AnimatedSprite)]
pub struct Effect {}

// GrassEffect implementation.
#[gdnative::methods]
impl Effect {
    // The "constructor" of the class.
    fn new(_owner: &AnimatedSprite) -> Self {
        Effect {}
    }

    #[export]
    fn _ready(&self, owner: TRef<AnimatedSprite>) {
        // Connecting to signal
        owner
            .connect(
                "animation_finished",
                owner,
                "on_animated_sprite_animation_finished",
                VariantArray::new_shared(),
                1,
            )
            .unwrap();

        owner.play("Animate", false);
    }

    // Accepting signal
    #[export]
    fn on_animated_sprite_animation_finished(&self, owner: &AnimatedSprite) {
        owner.queue_free();
    }
}
