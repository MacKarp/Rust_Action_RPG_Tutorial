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
    fn _ready(&self, owner: TRef<KinematicBody2D>) {
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
    }
}
