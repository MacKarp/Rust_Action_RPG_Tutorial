use crate::utils::*;
use gdnative::api::*;
use gdnative::prelude::*;

// Grass "class".
#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Grass {
    effect_scene_load: Ref<PackedScene>,
}

// Grass implementation.
#[gdnative::methods]
impl Grass {
    // The "constructor" of the class.
    fn new(_owner: &Node2D) -> Self {
        Grass {
            effect_scene_load: PackedScene::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        // Loading effect scene
        let effect_scene_load = load_scene("res://Effects/GrassEffect.tscn");
        match effect_scene_load {
            Some(_scene) => self.effect_scene_load = _scene,
            None => godot_print!("Could not load child scene. Check name."),
        }
    }
    #[export]
    fn _physics_process(&mut self, _owner: &Node2D, _delta: f64) {}

    fn create_grass_effect(&mut self, owner: &Node2D) {
        let grass_effect = unsafe { self.effect_scene_load.assume_safe() };
        let grass_effect = grass_effect
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .expect("should be able to instance scene");

        let parent = owner.get_parent().unwrap();
        let parent = unsafe { parent.assume_safe() };
        parent.add_child(grass_effect, false);

        // Accessing to GrassEffect node
        let grass_effect = grass_effect.to_variant();
        let grass_effect = grass_effect
            .try_to_object::<Node2D>()
            .expect("Should cast to Node2D");
        let grass_effect = unsafe { grass_effect.assume_safe() };

        // Moving position of GrassEffect
        grass_effect.set_global_position(owner.global_position());
    }

    // Accepting signal
    #[export]
    fn _on_hurtbox_area_entered(&mut self, owner: &Node2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        area.set("show_hit", false.to_variant());
        self.create_grass_effect(owner);
        // Deleting Grass node
        owner.queue_free();
    }
}
