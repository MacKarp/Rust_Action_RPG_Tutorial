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
    stats: Ref<Node>,
    hurtbox: Ref<Node>,
    player_hurt_sound_load: Ref<PackedScene>,
    blink_animation_player: Ref<Node>,
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
            roll_vector: Vector2::new(0.0, 1.0),
            stats: Node::new().into_shared(),
            hurtbox: Node::new().into_shared(),
            player_hurt_sound_load: PackedScene::new().into_shared(),
            blink_animation_player: Node::new().into_shared(),
        }
    }

    // Called when the node is "ready", i.e. when both the node and its children have entered the scene tree.
    // If the node has children, their _ready() callbacks get triggered first, and the parent node will receive the ready notification afterwards.
    #[export]
    fn _ready(&mut self, owner: TRef<KinematicBody2D>) {
        // Access to `HitboxPivot/SwordHitbox` node
        let sword_hitbox = owner
            .get_node("HitboxPivot/SwordHitbox")
            .expect("SwordHitbox node Should Exist");
        let sword_hitbox = unsafe { sword_hitbox.assume_safe() };
        let sword_hitbox = sword_hitbox
            .cast::<Area2D>()
            .expect("Node should cast to Area2D");

        // Set `knockback_vector` variable in HitboxPivot/SwordHitbox node
        sword_hitbox.set("knockback_vector", self.roll_vector);

        // Access `PlayerStats` singleton
        self.stats = owner
            .get_node("../../../PlayerStats")
            .expect("PlayerStats node Should Exist");

        let stats = unsafe { self.stats.assume_safe() };
        // Connecting to signal
        stats
            .connect(
                "no_health",
                owner,
                "queue_free",
                VariantArray::new_shared(),
                1,
            )
            .unwrap();

        // Access `Hurtbox` node
        self.hurtbox = owner
            .get_node("Hurtbox")
            .expect("Hurtbox node Should Exist");

        // Loading scene
        let player_hurt_sound_load = load_scene("res://Player/PlayerHurtSound.tscn");
        match player_hurt_sound_load {
            Some(_scene) => self.player_hurt_sound_load = _scene,
            None => godot_print!("Could not load child scene. Check name."),
        }

        // Access `BlinkAnimationPlayer` node
        self.blink_animation_player = owner
            .get_node("BlinkAnimationPlayer")
            .expect("BlinkAnimationPlayer node Should Exist");
    }

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
            PlayerState::ROLL => self.roll_state(owner, animation_state),
            PlayerState::ATTACK => self.attack_state(owner, animation_state),
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

            // Access to HitboxPivot/SwordHitbox node
            let sword_hitbox = owner
                .get_node("HitboxPivot/SwordHitbox")
                .expect("SwordHitbox node Should Exist");
            let sword_hitbox = unsafe { sword_hitbox.assume_safe() };
            let sword_hitbox = sword_hitbox
                .cast::<Area2D>()
                .expect("Node should cast to Area2D");

            // Set `knockback_vector` variable in HitboxPivot/SwordHitbox node
            sword_hitbox.set("knockback_vector", self.input_vector);

            animation_tree.set("parameters/Idle/blend_position", self.input_vector);
            animation_tree.set("parameters/Run/blend_position", self.input_vector);
            animation_tree.set("parameters/Attack/blend_position", self.input_vector);
            animation_tree.set("parameters/Roll/blend_position", self.input_vector);

            animation_state.travel("Run");
            self.velocity = self
                .velocity
                .move_towards(self.input_vector * MAX_SPEED, ACCELERATION * delta as f32);
        } else {
            animation_state.travel("Idle");

            self.velocity = self
                .velocity
                .move_towards(Vector2::zero(), FRICTION * delta as f32);
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
        animation_state: TRef<AnimationNodeStateMachinePlayback>,
    ) {
        self.velocity = self.roll_vector * ROLL_SPEED;
        animation_state.travel("Roll");
        self.player_move(owner);
    }

    fn attack_state(
        &mut self,
        _owner: &KinematicBody2D,
        animation_state: TRef<AnimationNodeStateMachinePlayback>,
    ) {
        self.velocity = Vector2::zero();
        animation_state.travel("Attack");
    }

    // `move` is keyword so had to use `player_move` name instead
    fn player_move(&mut self, owner: &KinematicBody2D) {
        self.velocity = owner.move_and_slide(
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

    #[export]
    fn _on_hurtbox_area_entered(&self, owner: &KinematicBody2D, area: Ref<Area2D>) {
        let stats = unsafe { self.stats.assume_safe() };
        let area = unsafe { area.assume_safe() };

        // Update `health` variable in `Stats` node
        let health = unsafe {
            (stats.call("get_health", &[]).to_i64() - area.call("get_hitbox_damage", &[]).to_i64())
                .to_variant()
        };

        unsafe {
            stats.call("set_health", &[health]);
        }

        let hurtbox = unsafe { self.hurtbox.assume_safe() };
        unsafe { hurtbox.call("start_invincibility", &[(0.6).to_variant()]) };
        unsafe { hurtbox.call("create_hit_effect", &[]) };

        let player_hurt_sound = unsafe { self.player_hurt_sound_load.assume_safe() };
        let player_hurt_sound = player_hurt_sound
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .expect("should be able to instance scene");
        unsafe {
            owner
                .get_tree()
                .unwrap()
                .assume_safe()
                .current_scene()
                .unwrap()
                .assume_safe()
                .add_child(player_hurt_sound, false)
        };
    }

    #[export]
    fn _on_hurtbox_invincibility_started(&self, _owner: &KinematicBody2D) {
        let blink_animation_player = unsafe { self.blink_animation_player.assume_safe() };
        let blink_animation_player = blink_animation_player.cast::<AnimationPlayer>().unwrap();
        blink_animation_player.play("Start", -1.0, 1.0, false)
    }

    #[export]
    fn _on_hurtbox_invincibility_ended(&self, _owner: &KinematicBody2D) {
        let blink_animation_player = unsafe { self.blink_animation_player.assume_safe() };
        let blink_animation_player = blink_animation_player.cast::<AnimationPlayer>().unwrap();
        blink_animation_player.play("Stop", -1.0, 1.0, false)
    }
}
