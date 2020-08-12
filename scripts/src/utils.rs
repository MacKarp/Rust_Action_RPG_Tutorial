use gdnative::prelude::*;

const CMP_EPSILON: f32 = 0.00001;

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
    if len <= delta || len < CMP_EPSILON {
        to
    } else {
        start_vector + vd / len * delta
    }
}

// Returns the vector with a maximum length by limiting its length to `length`.
#[allow(dead_code)]
#[inline]
pub fn clamped(vector_to_clamp: Vector2, length: f32) -> Vector2 {
    vector_to_clamp.clamp_length(0.0, length)
}
