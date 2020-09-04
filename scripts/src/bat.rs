use crate::utils::*;
use gdnative::api::*;
use gdnative::prelude::*;

// Bat "class".
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Bat {
    knockback: Vector2,
    effect_scene_load: Ref<PackedScene>,
}

#[gdnative::methods]
impl Bat {
    fn new(_owner: &KinematicBody2D) -> Self {
        Bat {
            knockback: Vector2::zero(),
            effect_scene_load: PackedScene::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<KinematicBody2D>) {
        // Loading scene
        let effect_scene_load = self.load_scene("res://Effects/EnemyDeathEffect.tscn");
        match effect_scene_load {
            Some(_scene) => self.effect_scene_load = _scene,
            None => godot_print!("Could not load child scene. Check name."),
        }

        // Access to `Stats` node
        let stats = owner.get_node("Stats").expect("Stats node should exist");
        let stats = unsafe { stats.assume_safe() };
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
    fn _on_hurtbox_area_entered(&mut self, owner: &KinematicBody2D, area: Ref<Area2D>) {
        // Access to `Stats` node
        let stats = owner.get_node("Stats").expect("Stats node Should Exist");
        let stats = unsafe { stats.assume_safe() };
        let stats = stats.cast::<Node>().expect("Node should cast to Node");
        let area = unsafe { area.assume_safe() };

        // Update `health` variable in `Stats` node
        let health = (stats.get("health").to_i64()
            - unsafe { area.call("get_hitbox_damage", &[]).to_i64() })
        .to_variant();
        //       let health = (stats.get("health").to_i64() - 1).to_variant();

        unsafe {
            stats.call("set_health", &[health]);
        }

        self.knockback = area.get("knockback_vector").to_vector2() * 120.0;
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

    // Scene loading helper function
    fn load_scene(&self, path: &str) -> Option<Ref<PackedScene, Shared>> {
        let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;

        let scene = unsafe { scene.assume_unique().into_shared() };

        scene.cast::<PackedScene>()
    }
}
