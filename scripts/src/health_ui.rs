use gdnative::api::*;
use gdnative::prelude::*;

// HealthUI "class".
#[derive(NativeClass)]
#[inherit(Control)]
pub struct HealthUI {
    //#[property(default = 1)]
    hearts: i64,
    max_hearts: i64,
    heart_ui_full: Ref<Node>,
    heart_ui_empty: Ref<Node>,
}
#[gdnative::methods]
impl HealthUI {
    pub fn new(_owner: &Control) -> Self {
        HealthUI {
            hearts: 4,
            max_hearts: 4,
            heart_ui_full: Node::new().into_shared(),
            heart_ui_empty: Node::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Control>) {
        self.heart_ui_full = owner
            .get_node("HeartUIFull")
            .expect("Label node should exist");
        self.heart_ui_empty = owner
            .get_node("HeartUIEmpty")
            .expect("Label node should exist");

        // Access `PlayerStats` singleton
        let player_stats = owner
            .get_node("../../PlayerStats")
            .expect("PlayerStats node Should Exist");
        let player_stats = unsafe { player_stats.assume_safe() };

        self.set_max_hearts(&owner, player_stats.get("max_health").to_i64());
        self.set_hearts(
            &owner,
            unsafe { player_stats.call("get_health", &[]) }.to_i64(),
        );

        player_stats
            .connect(
                "health_changed",
                owner,
                "set_hearts",
                VariantArray::new_shared(),
                1,
            )
            .unwrap();

        player_stats
            .connect(
                "max_health_changed",
                owner,
                "set_max_hearts",
                VariantArray::new_shared(),
                1,
            )
            .unwrap();
    }

    #[export]
    fn set_hearts(&mut self, _owner: &Control, value: i64) {
        self.hearts = num::clamp(value, 0, self.max_hearts);
        let health_ui_full = unsafe { self.heart_ui_full.assume_safe() };
        let health_ui_full = health_ui_full
            .cast::<TextureRect>()
            .expect("Node should cast to TextureRect");
        health_ui_full.set_size(Vector2::new(self.hearts as f32 * 15.0, 11.0), false);
    }

    #[export]
    fn set_max_hearts(&mut self, owner: &Control, value: i64) {
        self.max_hearts = value.max(1);
        self.set_hearts(owner, self.hearts.min(self.max_hearts));
        let health_ui_empty = unsafe { self.heart_ui_empty.assume_safe() };
        let health_ui_empty = health_ui_empty
            .cast::<TextureRect>()
            .expect("Node should cast to TextureRect");
        health_ui_empty.set_size(Vector2::new(self.max_hearts as f32 * 15.0, 11.0), false);
    }
}
