use gdnative::api::*;
use gdnative::prelude::*;

// PlayerHurtSound "class".
#[derive(NativeClass)]
#[inherit(AudioStreamPlayer)]
pub struct PlayerHurtSound {}

#[gdnative::methods]
impl PlayerHurtSound {
    fn new(_owner: &AudioStreamPlayer) -> Self {
        PlayerHurtSound {}
    }

    #[export]
    fn _ready(&self, owner: TRef<AudioStreamPlayer>) {
        // Connecting to signal
        owner
            .connect(
                "finished",
                owner,
                "queue_free",
                VariantArray::new_shared(),
                1,
            )
            .unwrap();
    }
}
