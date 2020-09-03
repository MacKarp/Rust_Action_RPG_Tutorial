use gdnative::api::*;
use gdnative::prelude::*;

// Stats "class".
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct Stats {
    #[property(default = 1)]
    max_health: i64,
    #[property(default = 1)]
    health: i64,
}

#[gdnative::methods]
impl Stats {
    fn new(_owner: &Node) -> Self {
        Stats {
            max_health: 1,
            health: 1,
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "no_health",
            args: &[],
        });
    }

    #[export]
    fn set_health(&mut self, owner: &Node, value: Variant) {
        self.health = value.to_i64();
        if self.health <= 0 {
            owner.emit_signal("no_health", &[]);
        }
    }
}
