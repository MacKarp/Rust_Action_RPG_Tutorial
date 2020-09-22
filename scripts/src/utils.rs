use gdnative::api::*;
use gdnative::prelude::*;

/// Returns the vector scaled to unit length. Equivalent to v / v.length().
#[allow(dead_code)]
#[inline]
pub fn normalized(vector_to_normalize: Vector2) -> Vector2 {
    let option = Vector2::try_normalize(vector_to_normalize);
    match option {
        None => Vector2::zero(),
        Some(vector2) => vector2,
    }
}

/// Returns the vector with a maximum length by limiting its length to `length`.
#[allow(dead_code)]
#[inline]
pub fn clamped(vector2_to_clamp: Vector2, length: f32) -> Vector2 {
    vector2_to_clamp.clamp_length(0.0, length)
}

#[allow(dead_code)]
#[inline]
// Scene loading helper function
pub fn load_scene(path: &str) -> Option<Ref<PackedScene, Shared>> {
    let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;
    let scene = unsafe { scene.assume_unique().into_shared() };
    scene.cast::<PackedScene>()
}
