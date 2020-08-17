use crate::utils::{move_towards, normalized};
use gdnative::api::*;
use gdnative::prelude::*;

// Player "class".
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    velocity: Vector2,
    state: PlayerState,
}

const ACCELERATION: f32 = 500.0;
const MAX_SPEED: f32 = 80.0;
const FRICTION: f32 = 500.0;

enum PlayerState {
    MOVE,
    ROLL,
    ATTACK,
}

// Player implementation.
#[gdnative::methods]
impl Player {
    // The "constructor" of the class.
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::zero(),
            state: PlayerState::MOVE,
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

        /*
        // Access to AnimationPlayer node
        let animation_player = owner
            .get_node("AnimationPlayer")
            .expect("Node Should Exist");
        let animation_player = unsafe { animation_player.assume_safe() };
        let animation_player = animation_player
            .cast::<AnimationPlayer>()
            .expect("Node should cast to AnimationPlayer");
        */

        // Access to AnimationTree node
        let animation_tree = owner.get_node("AnimationTree").expect("Node Should Exist");
        let animation_tree = unsafe { animation_tree.assume_safe() };
        let animation_tree = animation_tree
            .cast::<AnimationTree>()
            .expect("Node should cast to AnimationTree");

        animation_tree.set_active(true);

        // Access to Animation State AnimationNodePlayerStatePlayback inside AnimationTree node
        let animation_state = animation_tree.get("parameters/playback");
        let animation_state = animation_state
            .try_to_object::<AnimationNodeStateMachinePlayback>()
            .expect("cast should be valid");
        let animation_state = unsafe { animation_state.assume_safe() };

        match self.state {
            PlayerState::MOVE => {
                self.move_state(&owner, delta, input_vector, animation_tree, animation_state)
            }
            PlayerState::ROLL => self.roll_state(),
            PlayerState::ATTACK => self.attack_state(owner, animation_state),
        }
    }

    fn move_state(
        &mut self,
        owner: &KinematicBody2D,
        delta: f64,
        input_vector: Vector2,
        animation_tree: TRef<AnimationTree>,
        animation_state: TRef<AnimationNodeStateMachinePlayback>,
    ) {
        if input_vector != Vector2::zero() {
            animation_tree.set("parameters/Idle/blend_position", input_vector);
            animation_tree.set("parameters/Run/blend_position", input_vector);
            animation_tree.set("parameters/Attack/blend_position", input_vector);

            animation_state.travel("Run");
            self.velocity = move_towards(
                self.velocity,
                input_vector * MAX_SPEED,
                ACCELERATION * delta as f32,
            );
        } else {
            animation_state.travel("Idle");

            self.velocity = move_towards(self.velocity, Vector2::zero(), FRICTION * delta as f32);
        }

        self.velocity = KinematicBody2D::move_and_slide(
            owner,
            self.velocity,
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );

        let input = Input::godot_singleton();
        if Input::is_action_just_pressed(input, "attack") {
            self.state = PlayerState::ATTACK;
        }
    }
    fn attack_state(
        &mut self,
        owner: &KinematicBody2D,
        animation_state: TRef<AnimationNodeStateMachinePlayback>,
    ) {
        self.velocity = Vector2::zero();
        animation_state.travel("Attack");

        // Can't find a way to run function at end of animation...
        let animation_player = owner
            .get_node("AnimationPlayer")
            .expect("Node Should Exist");
        let animation_player = unsafe { animation_player.assume_safe() };
        let animation_player = animation_player
            .cast::<AnimationPlayer>()
            .expect("Node should cast to AnimationPlayer");

        // So i check if current animation is still playing
        if (animation_player.current_animation_length()
            - animation_player.current_animation_position())
        .abs()
            < f64::EPSILON
        {
            self.state = PlayerState::MOVE;
        }
    }

    fn roll_state(&mut self) {
        self.state = PlayerState::ROLL;
    }
}
