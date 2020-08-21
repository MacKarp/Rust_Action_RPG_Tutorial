use gdnative::api::*;
use gdnative::prelude::*;

// GrassEffect "class".
#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GrassEffect {}

// GrassEffect implementation.
#[gdnative::methods]
impl GrassEffect {
    // The "constructor" of the class.
    fn new(_owner: &Node2D) -> Self {
        GrassEffect {}
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        // Access to AnimatedSprite node
        let animated_sprite = owner
            .get_node("AnimatedSprite")
            .expect("AnimatedSprite node Should Exist");
        let animated_sprite = unsafe { animated_sprite.assume_safe() };
        let animated_sprite = animated_sprite
            .cast::<AnimatedSprite>()
            .expect("Node should cast to AnimatedSprite");

        animated_sprite.set_frame(0);
        animated_sprite.play("Animate", false);
    }

    // Accepting signal
    #[export]
    fn on_animated_sprite_animation_finished(&self, owner: &Node2D) {
        owner.queue_free();
    }
}
