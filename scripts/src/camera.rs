use gdnative::api::*;
use gdnative::prelude::*;

// Camera "class".
#[derive(NativeClass)]
#[inherit(Camera2D)]
pub struct Camera {
    top_left: Ref<Node>,
    bottom_right: Ref<Node>,
}
#[gdnative::methods]
impl Camera {
    pub fn new(_owner: &Camera2D) -> Self {
        Camera {
            top_left: Node::new().into_shared(),
            bottom_right: Node::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Camera2D) {
        self.top_left = owner
            .get_node("Limits/TopLeft")
            .expect("Limits/TopLeft should exist");

        let top_left = unsafe { self.top_left.assume_safe() };
        let top_left = top_left.cast::<Position2D>().unwrap();

        self.bottom_right = owner
            .get_node("Limits/BottomRight")
            .expect("Limits/BottomRight should exist");
        let bottom_right = unsafe { self.bottom_right.assume_safe() };
        let bottom_right = bottom_right.cast::<Position2D>().unwrap();

        //Setting camera limit
        owner.set("limit_top", top_left.position().y);
        owner.set("limit_left", top_left.position().x);
        owner.set("limit_bottom", bottom_right.position().y);
        owner.set("limit_right", bottom_right.position().x);
    }
}
