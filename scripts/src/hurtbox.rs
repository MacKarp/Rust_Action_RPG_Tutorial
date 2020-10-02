use crate::utils::*;
use gdnative::api::*;
use gdnative::prelude::*;

// Hurtbox "class".
#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_signals)]
pub struct Hurtbox {
    hit_effect_scene_load: Ref<PackedScene>,
    scene_tree: Ref<SceneTree>,
    main: Ref<Node>,
    invincible: bool,
    timer: Ref<Node>,
    collision_shape: Ref<Node>,
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
            invincible: false,
            timer: Node::new().into_shared(),
            collision_shape: Node::new().into_shared(),
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "invincibility_started",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "invincibility_ended",
            args: &[],
        });
    }

    #[export]
    fn _ready(&mut self, owner: &Area2D) {
        // Loading scene
        let effect_scene_load = load_scene("res://Effects/HitEffect.tscn");
        match effect_scene_load {
            Some(_scene) => self.hit_effect_scene_load = _scene,
            None => godot_print!("Could not load child scene. Check name."),
        }

        //Accesing Timer node
        self.timer = owner.get_node("Timer").expect("Timer node should exist");
        //Accessing CollisionShape2D node
        self.collision_shape = owner
            .get_node("CollisionShape2D")
            .expect("CollisionShape2D node should exist");
    }

    #[export]
    fn set_invincible(&mut self, owner: &Area2D, value: bool) {
        self.invincible = value;
        if self.invincible {
            // can't emit signal when `&mut self` is in use, instead call function directly (is there a way to emmit signal in this case?)
            self._on_hurtbox_invincibility_started(owner);
            unsafe {
                owner
                    .get_parent()
                    .unwrap()
                    .assume_safe()
                    .call_deferred("_on_hurtbox_invincibility_started", &[])
            };
        } else {
            // can't emit signal when `&mut self` is in use, instead call function directly (is there a way to emmit signal in this case?)
            self._on_hurtbox_invincibility_ended(owner);
            unsafe {
                owner
                    .get_parent()
                    .unwrap()
                    .assume_safe()
                    .call_deferred("_on_hurtbox_invincibility_ended", &[])
            };
        }
    }

    #[export]
    fn start_invincibility(&mut self, owner: &Area2D, duration: Variant) {
        self.set_invincible(&owner, true);
        let duration = duration.to_f64();
        let timer = unsafe { self.timer.assume_safe() };
        let timer = timer.cast::<Timer>().unwrap();
        timer.start(duration);
    }

    #[export]
    fn create_hit_effect(&mut self, owner: &Area2D) {
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

    #[export]
    fn _on_timer_timeout(&mut self, owner: &Area2D) {
        self.set_invincible(&owner, false);
    }

    #[export]
    fn _on_hurtbox_invincibility_started(&self, _owner: &Area2D) {
        let collision_shape = unsafe { self.collision_shape.assume_safe() };
        let collision_shape = collision_shape.cast::<CollisionShape2D>().unwrap();
        collision_shape.set_deferred("disabled", true.to_variant());
    }

    #[export]
    fn _on_hurtbox_invincibility_ended(&mut self, _owner: &Area2D) {
        let collision_shape = unsafe { self.collision_shape.assume_safe() };
        let collision_shape = collision_shape.cast::<CollisionShape2D>().unwrap();
        collision_shape.set("disabled", false.to_variant());
    }
}
