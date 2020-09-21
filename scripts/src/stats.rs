use gdnative::api::*;
use gdnative::prelude::*;

// Stats "class".
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct Stats {
    #[property(default = 1)]
    max_health: i64,
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

        builder.add_signal(Signal {
            name: "health_changed",
            args: &[SignalArgument {
                name: "value",
                default: Variant::from_i64(1),
                export_info: ExportInfo::new(VariantType::I64),
                usage: PropertyUsage::DEFAULT,
            }],
        });

        builder.add_signal(Signal {
            name: "max_health_changed",
            args: &[SignalArgument {
                name: "value",
                default: Variant::from_i64(1),
                export_info: ExportInfo::new(VariantType::I64),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        self.set_health(owner, self.max_health);
    }

    #[export]
    fn set_health(&mut self, owner: &Node, value: i64) {
        self.health = value;
        owner.emit_signal("health_changed", &[self.health.to_variant()]);
        if self.health <= 0 {
            owner.emit_signal("no_health", &[]);
        }
    }

    #[export]
    fn set_max_health(&mut self, owner: &Node, value: i64) {
        self.max_health = value;
        self.set_health(owner, self.health.min(self.max_health));
        owner.emit_signal("max_health_changed", &[self.health.to_variant()]);
    }

    #[export]
    fn get_health(&self, _owner: &Node) -> i64 {
        self.health
    }
}
