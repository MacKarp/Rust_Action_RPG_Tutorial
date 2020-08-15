use gdnative::prelude::Vector2;

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

/// Moves the vector toward to by the fixed delta amount.
#[allow(dead_code)]
#[inline]
pub fn move_towards(start_vector: Vector2, to: Vector2, delta: f32) -> Vector2 {
    let vd = to - start_vector;
    let len = vd.length();
    if len <= delta || approx::abs_diff_eq!(0.0, len) {
        to
    } else {
        Vector2::lerp(&start_vector, to, delta / len)
    }
}

/// Returns the vector with a maximum length by limiting its length to `length`.
#[allow(dead_code)]
#[inline]
pub fn clamped(vector2_to_clamp: Vector2, length: f32) -> Vector2 {
    vector2_to_clamp.clamp_length(0.0, length)
}
