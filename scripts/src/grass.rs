use gdnative::api::*;
use gdnative::prelude::*;

// Grass "class".
#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Grass {
    grass_effect_scene_load: Ref<PackedScene>,
    scene_tree: Ref<SceneTree>,
    world: Ref<Node>,
}

// Grass implementation.
#[gdnative::methods]
impl Grass {
    // The "constructor" of the class.
    fn new(_owner: &Node2D) -> Self {
        Grass {
            grass_effect_scene_load: PackedScene::new().into_shared(),
            scene_tree: SceneTree::new().into_shared(),
            world: Node::new().into_shared(),
        }
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Node2D, _delta: f64) {}

    fn create_grass_effect(&mut self, owner: &Node2D) {
        // Loading scene
        let grass_effect_scene_load = self.load_scene("res://Effects/GrassEffect.tscn");
        match grass_effect_scene_load {
            Some(_scene) => self.grass_effect_scene_load = _scene,
            None => godot_print!("Could not load child scene. Check name."),
        }
        let grass_effect = unsafe { self.grass_effect_scene_load.assume_safe() };
        let grass_effect = grass_effect
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .expect("should be able to instance scene");

        // Accessing scene tree
        let world = owner.get_tree();
        match world {
            Some(tree) => self.scene_tree = tree,
            None => godot_print!("Could not load scene tree."),
        }

        // Getting scene node
        let world = unsafe { self.scene_tree.assume_safe() };
        match world.current_scene() {
            Some(node) => self.world = node,
            None => godot_print!("Could not get scene node."),
        }

        // Adding GrassEffect child node
        let world = unsafe { self.world.assume_safe() };
        world.add_child(grass_effect, false);

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
    fn _on_hurtbox_area_entered(&mut self, owner: &Node2D, _area: Ref<Area2D>) {
        self.create_grass_effect(owner);
        // Deleting Grass node
        owner.queue_free();
    }

    // Scene loading helper function
    fn load_scene(&self, path: &str) -> Option<Ref<PackedScene, Shared>> {
        let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;

        let scene = unsafe { scene.assume_unique().into_shared() };

        scene.cast::<PackedScene>()
    }
}
