use crate::utils::*;
use gdnative::api::*;
use gdnative::prelude::*;

// Player "class".
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    velocity: Vector2,
    state: PlayerState,
    input_vector: Vector2,
    roll_vector: Vector2,
}

const ACCELERATION: f32 = 500.0;
const MAX_SPEED: f32 = 80.0;
const ROLL_SPEED: f32 = 120.0;
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
            input_vector: Vector2::zero(),
            roll_vector: Vector2::new(-1.0, 0.0),
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
        /*
        // Access to AnimationPlayer node
        let animation_player = owner
            .get_node("AnimationPlayer")
            .expect("AnimationPlayer node Should Exist");
        let animation_player = unsafe { animation_player.assume_safe() };
        let animation_player = animation_player
            .cast::<AnimationPlayer>()
            .expect("Node should cast to AnimationPlayer");
        */

        // Access to AnimationTree node
        let animation_tree = owner
            .get_node("AnimationTree")
            .expect("AnimationTree node Should Exist");
        let animation_tree = unsafe { animation_tree.assume_safe() };
        let animation_tree = animation_tree
            .cast::<AnimationTree>()
            .expect("Node should cast to AnimationTree");

        animation_tree.set_active(true);

        // Access to Animation State AnimationNodePlayerStatePlayback inside AnimationTree node
        let animation_state = animation_tree.get("parameters/playback");
        let animation_state = animation_state
            .try_to_object::<AnimationNodeStateMachinePlayback>()
            .expect("Should cast to AnimationNodeStateMachinePlayback");
        let animation_state = unsafe { animation_state.assume_safe() };

        match self.state {
            PlayerState::MOVE => self.move_state(owner, delta, animation_tree, animation_state),
            PlayerState::ROLL => self.roll_state(owner, delta, animation_state),
            PlayerState::ATTACK => self.attack_state(owner, delta, animation_state),
        }
    }

    fn move_state(
        &mut self,
        owner: &KinematicBody2D,
        delta: f64,
        animation_tree: TRef<AnimationTree>,
        animation_state: TRef<AnimationNodeStateMachinePlayback>,
    ) {
        let input = Input::godot_singleton();
        self.input_vector = Vector2::zero();
        self.input_vector.x = Input::get_action_strength(input, "ui_right") as f32
            - Input::get_action_strength(input, "ui_left") as f32;
        self.input_vector.y = Input::get_action_strength(input, "ui_down") as f32
            - Input::get_action_strength(input, "ui_up") as f32;

        self.input_vector = normalized(self.input_vector);

        if self.input_vector != Vector2::zero() {
            self.roll_vector = self.input_vector;
            animation_tree.set("parameters/Idle/blend_position", self.input_vector);
            animation_tree.set("parameters/Run/blend_position", self.input_vector);
            animation_tree.set("parameters/Attack/blend_position", self.input_vector);
            animation_tree.set("parameters/Roll/blend_position", self.input_vector);

            animation_state.travel("Run");
            self.velocity = move_towards(
                self.velocity,
                self.input_vector * MAX_SPEED,
                ACCELERATION * delta as f32,
            );
        } else {
            animation_state.travel("Idle");

            self.velocity = move_towards(self.velocity, Vector2::zero(), FRICTION * delta as f32);
        }

        self.player_move(owner);

        if Input::is_action_just_pressed(input, "roll") {
            self.state = PlayerState::ROLL;
        }

        if Input::is_action_just_pressed(input, "attack") {
            self.state = PlayerState::ATTACK;
        }
    }

    fn roll_state(
        &mut self,
        owner: &KinematicBody2D,
        _delta: f64,
        animation_state: TRef<AnimationNodeStateMachinePlayback>,
    ) {
        self.velocity = self.roll_vector * ROLL_SPEED;
        animation_state.travel("Roll");
        self.player_move(owner);
    }

    fn attack_state(
        &mut self,
        _owner: &KinematicBody2D,
        _delta: f64,
        animation_state: TRef<AnimationNodeStateMachinePlayback>,
    ) {
        self.velocity = Vector2::zero();
        animation_state.travel("Attack");
    }

    // `move` is keyword so had to use `player_move` name instead
    fn player_move(&mut self, owner: &KinematicBody2D) {
        self.velocity = KinematicBody2D::move_and_slide(
            owner,
            self.velocity,
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );
    }

    // Need to add script function "_ready" and then rename function in inspector to `attack_animation_finished`
    #[export]
    fn attack_animation_finished(&mut self, _owner: &KinematicBody2D) {
        self.state = PlayerState::MOVE;
    }

    // Need to add script function "_ready" and then rename function in inspector to `roll_animation_finished`
    #[export]
    fn roll_animation_finished(&mut self, _owner: &KinematicBody2D) {
        self.velocity *= 0.8;
        self.state = PlayerState::MOVE;
    }
}
