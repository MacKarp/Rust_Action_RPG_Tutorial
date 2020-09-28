use gdnative::api::*;
use gdnative::prelude::*;
use rand::Rng;

// WanderController "class".
#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct WanderController {
    start_position: Vector2,
    target_position: Vector2,
    #[property(default = 32)]
    wander_range: i32,
    timer: Ref<Node>,
}

#[gdnative::methods]
impl WanderController {
    fn new(_owner: &Node2D) -> Self {
        WanderController {
            start_position: Vector2::zero(),
            target_position: Vector2::zero(),
            wander_range: 32,
            timer: Node::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        self.start_position = owner.global_position();
        self.update_target_position();
        self.timer = owner.get_node("Timer").expect("Timer node should exist");
    }

    #[export]
    fn get_time_left(&self, _owner: &Node2D) -> f64 {
        let timer = unsafe { self.timer.assume_safe() };
        let timer = timer.cast::<Timer>().unwrap();
        timer.time_left()
    }

    fn update_target_position(&mut self) {
        /*let target_vector = Vector2::new(
            RandomNumberGenerator::new()
                .randf_range(-self.wander_range as f64, self.wander_range as f64)
                as f32,
            RandomNumberGenerator::new()
                .randf_range(-self.wander_range as f64, self.wander_range as f64)
                as f32,
        );*/
        let mut rng = rand::thread_rng();
        let target_vector = Vector2::new(
            rng.gen_range(-self.wander_range as f64, self.wander_range as f64) as f32,
            rng.gen_range(-self.wander_range as f64, self.wander_range as f64) as f32,
        );

        self.target_position = self.start_position + target_vector;
    }

    #[export]
    fn start_wander_timer(&self, _owner: &Node2D, duration: f64) {
        let timer = unsafe { self.timer.assume_safe() };
        let timer = timer.cast::<Timer>().unwrap();
        timer.start(duration);
    }

    #[export]
    fn _on_timer_timeout(&mut self, _owner: &Node2D) {
        self.update_target_position();
    }

    #[export]
    fn get_target_position(&self, _owner: &Node2D) -> Vector2 {
        self.target_position
    }
}
