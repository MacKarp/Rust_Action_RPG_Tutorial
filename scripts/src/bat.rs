use crate::utils::*;
use gdnative::api::*;
use gdnative::prelude::*;

// Bat "class".
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Bat {
    #[property(default = 300.0)]
    acceleration: f32,
    #[property(default = 50.0)]
    max_speed: f32,
    #[property(default = 200.0)]
    friction: f32,
    velocity: Vector2,
    knockback: Vector2,
    effect_scene_load: Ref<PackedScene>,
    state: BatState,
    stats: Ref<Node>,
    player_detecion_zone: Ref<Node>,
    sprite: Ref<Node>,
    hurtbox: Ref<Node>,
}

enum BatState {
    IDLE,
    WANDER,
    CHASE,
}

#[gdnative::methods]
impl Bat {
    fn new(_owner: &KinematicBody2D) -> Self {
        Bat {
            acceleration: 300.0,
            max_speed: 50.0,
            friction: 200.0,
            velocity: Vector2::zero(),
            knockback: Vector2::zero(),
            effect_scene_load: PackedScene::new().into_shared(),
            state: BatState::CHASE,
            stats: Node::new().into_shared(),
            player_detecion_zone: Node::new().into_shared(),
            sprite: Node::new().into_shared(),
            hurtbox: Node::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<KinematicBody2D>) {
        // Loading scene
        let effect_scene_load = load_scene("res://Effects/EnemyDeathEffect.tscn");
        match effect_scene_load {
            Some(_scene) => self.effect_scene_load = _scene,
            None => godot_print!("Could not load child scene. Check name."),
        }

        // Access to `Stats` node
        self.stats = owner.get_node("Stats").expect("Stats node should exist");
        let stats = unsafe { self.stats.assume_safe() };
        let stats = stats.cast::<Node>().expect("Stats should cast to Node");

        // Connecting to signal
        stats
            .connect(
                "no_health",
                owner,
                "_on_stats_no_health",
                VariantArray::new_shared(),
                1,
            )
            .unwrap();

        // Set `max_health` and `health` variable in `Stats` node
        stats.set("max_health", 2);
        stats.set("health", 2);

        // Access to `PlayerDetectionZone` node
        self.player_detecion_zone = owner
            .get_node("PlayerDetectionZone")
            .expect("Stats node should exist");

        // Access to `PlayerDetectionZone` node
        self.sprite = owner
            .get_node("AnimatedSprite")
            .expect("AnimatedSprite node should exist");

        // Access to `Hurtbox` node
        self.hurtbox = owner
            .get_node("Hurtbox")
            .expect("Hurtbox node should exist");
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f64) {
        self.knockback = move_towards(
            self.knockback,
            Vector2::zero(),
            self.friction * delta as f32,
        );
        self.knockback = owner.move_and_slide(
            self.knockback,
            Vector2::zero(),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );
        match self.state {
            BatState::IDLE => {
                self.velocity =
                    move_towards(self.velocity, Vector2::zero(), self.friction * delta as f32);
                self.seek_player(owner);
            }
            BatState::WANDER => godot_print!("WANDER"),
            BatState::CHASE => {
                let player = unsafe { self.player_detecion_zone.assume_safe() };
                let player = player.get("player").try_to_object::<Node>().unwrap();
                let player = unsafe { player.assume_safe() };

                if player.name() == GodotString::from_str("Player") {
                    let player = player.cast::<Node2D>().expect("Node should cast to Node2D");
                    let direction = normalized(player.global_position() - owner.global_position());
                    self.velocity = move_towards(
                        self.velocity,
                        direction * self.max_speed,
                        self.acceleration * delta as f32,
                    );
                } else {
                    self.state = BatState::IDLE;
                }
                let sprite = unsafe { self.sprite.assume_safe() };
                let sprite = sprite
                    .cast::<AnimatedSprite>()
                    .expect("Node should cast to AnimatedSprite");
                sprite.set_flip_h(self.velocity.x < 0.0);
            }
        }

        self.velocity = owner.move_and_slide(
            self.velocity,
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
        let stats = unsafe { self.stats.assume_safe() };
        let stats = stats.cast::<Node>().expect("Node should cast to Node");
        let area = unsafe { area.assume_safe() };
        //area.set("show_hit", true.to_variant());

        // Update `health` variable in `Stats` node
        let health = (stats.get("health").to_i64()
            - unsafe { area.call("get_hitbox_damage", &[]).to_i64() })
        .to_variant();

        unsafe {
            stats.call("set_health", &[health]);
        }

        self.knockback = area.get("knockback_vector").to_vector2() * 120.0;

        let hurtbox = unsafe { self.hurtbox.assume_safe() };
        unsafe { hurtbox.call("create_hit_effect", &[]) };
    }

    // Accepting signal
    #[export]
    fn _on_stats_no_health(&self, owner: &KinematicBody2D) {
        //Deleting Bat node
        owner.queue_free();
        let enemy_death_effect = unsafe { self.effect_scene_load.assume_safe() };
        let enemy_death_effect = enemy_death_effect
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .expect("should be able to instance scene");
        let parent = owner.get_parent().unwrap();
        let parent = unsafe { parent.assume_safe() };
        parent.add_child(enemy_death_effect, false);

        // Accessing to GrassEffect node
        let enemy_death_effect = enemy_death_effect.to_variant();
        let enemy_death_effect = enemy_death_effect
            .try_to_object::<Node2D>()
            .expect("Should cast to Node2D");
        let enemy_death_effect = unsafe { enemy_death_effect.assume_safe() };

        // Moving position of GrassEffect
        enemy_death_effect.set_global_position(owner.global_position());
    }

    fn seek_player(&mut self, _owner: &KinematicBody2D) {
        let player_detecion_zone = unsafe { self.player_detecion_zone.assume_safe() };
        if unsafe { player_detecion_zone.call("can_see_player", &[]).to_bool() } {
            self.state = BatState::CHASE;
        }
    }
}
