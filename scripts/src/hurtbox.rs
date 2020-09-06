use crate::utils::*;
use gdnative::api::*;
use gdnative::prelude::*;

// Hurtbox "class".
#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Hurtbox {
    hit_effect_scene_load: Ref<PackedScene>,
    scene_tree: Ref<SceneTree>,
    main: Ref<Node>,
    #[property(default = true)]
    show_hit: bool,
}

// Hurtbox implementation.
#[gdnative::methods]
impl Hurtbox {
    // The "constructor" of the class.
    fn new(_owner: &Area2D) -> Self {
        Hurtbox {
            hit_effect_scene_load: PackedScene::new().into_shared(),
            scene_tree: SceneTree::new().into_shared(),
            main: Node::new().into_shared(),
            show_hit: true,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Area2D) {
        // Loading scene
        let effect_scene_load = load_scene("res://Effects/HitEffect.tscn");
        match effect_scene_load {
            Some(_scene) => self.hit_effect_scene_load = _scene,
            None => godot_print!("Could not load child scene. Check name."),
        }
    }

    #[export]
    fn _on_hurtbox_area_entered(&mut self, owner: &Area2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        self.show_hit = area.get("show_hit").to_bool();
        if self.show_hit {
            let effect = unsafe { self.hit_effect_scene_load.assume_safe() };
            let effect = effect
                .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
                .expect("should be able to instance scene");

            // Accessing scene tree
            let main = owner.get_tree();
            match main {
                Some(tree) => self.scene_tree = tree,
                None => godot_print!("Could not load scene tree."),
            }

            // Getting scene node
            let main = unsafe { self.scene_tree.assume_safe() };
            match main.current_scene() {
                Some(node) => self.main = node,
                None => godot_print!("Could not get scene node."),
            }

            // Adding Effect child node
            let main = unsafe { self.main.assume_safe() };
            main.add_child(effect, false);

            // Accessing to Effect node
            let effect = effect.to_variant();
            let effect = effect
                .try_to_object::<AnimatedSprite>()
                .expect("Should cast to AnimatedSprite");
            let effect = unsafe { effect.assume_safe() };

            // Moving position of Effect
            effect.set_global_position(owner.global_position());
        }
    }
}
