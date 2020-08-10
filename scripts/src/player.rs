use crate::utils::*;
use gdnative::prelude::*;

// Player "class".
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    velocity: Vector2,
}

const ACCELERATION: f32 = 10.0;
const MAX_SPEED: f32 = 100.0;
const FRICTION: f32 = 10.0;

// Player implementation.
#[gdnative::methods]
impl Player {
    // The "constructor" of the class.
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::zero(),
        }
    }

    // Called when the node is "ready", i.e. when both the node and its children have entered the scene tree.
    // If the node has children, their _ready() callbacks get triggered first, and the parent node will receive the ready notification afterwards.
    #[export]
    fn _ready(&self, _owner: &KinematicBody2D) {}

    // Called during the physics processing step of the main loop.
    // Physics processing means that the frame rate is synced to the physics, i.e. the delta variable should be constant.
    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f64) {
        let input = Input::godot_singleton();

        let mut input_vector = Vector2::zero();
        input_vector.x = Input::get_action_strength(input, "ui_right") as f32
            - Input::get_action_strength(input, "ui_left") as f32;
        input_vector.y = Input::get_action_strength(input, "ui_down") as f32
            - Input::get_action_strength(input, "ui_up") as f32;

        input_vector = normalized(input_vector);

        if input_vector != Vector2::zero() {
            self.velocity += input_vector * ACCELERATION * delta as f32;
            self.velocity = clamped(self.velocity, MAX_SPEED * delta as f32);
        } else {
            self.velocity = move_towards(self.velocity, Vector2::zero(), FRICTION * delta as f32);
        }

        let _move_and_collide =
            KinematicBody2D::move_and_collide(owner, self.velocity, false, false, false);
    }
}
