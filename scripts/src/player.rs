use gdnative::prelude::*;


/// Player "class".
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player{
    velocity: Vector2,
}

// Player implementation.
#[gdnative::methods]
impl Player {
    /// The "constructor" of the class.
    fn new(_owner: &KinematicBody2D) -> Self {
        Player{
            velocity: Vector2::new(0.0,0.0),
        }
    }

    // Called when the node is "ready", i.e. when both the node and its children have entered the scene tree.
    // If the node has children, their _ready() callbacks get triggered first, and the parent node will receive the ready notification afterwards.
    #[export]
    fn _ready(&self, _owner: &KinematicBody2D) {
        // godot_print!("Hello world!");
    }

    // Called during the physics processing step of the main loop.
    // Physics processing means that the frame rate is synced to the physics, i.e. the delta variable should be constant.
    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        // godot_print!("Hello world!");
        let input = Input::godot_singleton();
        
        /*if Input::is_action_pressed(input, "ui_right"){
            // godot_print!("Your pressed the right arrow key");
            self.velocity.x = 4.0;
        }
        else if Input::is_action_pressed(input, "ui_left"){
            // godot_print!("Your pressed the left arrow key");
            self.velocity.x = -4.0;
        }
        else if Input::is_action_pressed(input, "ui_up"){
            // godot_print!("Your pressed the up arrow key");
            self.velocity.y = -4.0;
        }
        else if Input::is_action_pressed(input, "ui_down"){
            // godot_print!("Your pressed the down arrow key");
            self.velocity.y = 4.0;
        }
        else{
            self.velocity.x = 0.0;
            self.velocity.y = 0.0;
        }
        */

        let mut input_vector = Vector2::new(0.0, 0.0);
        input_vector.x = Input::get_action_strength(input, "ui_right") as f32 - Input::get_action_strength(input, "ui_left") as f32;
        input_vector.y = Input::get_action_strength(input, "ui_down") as f32 - Input::get_action_strength(input, "ui_up") as f32;

        if input_vector != Vector2::new(0.0, 0.0){
            self.velocity = input_vector;
        }else{
            self.velocity = Vector2::new(0.0, 0.0);
        }
        let _move_and_collide = KinematicBody2D::move_and_collide(owner, self.velocity,false, false, false);
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

// Macro that create the entry-points of the dynamic library.
godot_init!(init);