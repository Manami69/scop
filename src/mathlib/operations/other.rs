use std::ops::{Add, Mul, Sub};

pub fn radian(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.
}

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: Clone + Add<V, Output = V> + Mul<f32, Output = V> + Sub<V, Output = V>,
{
    if t <= 0.5 {
        u.clone() + (v - u) * t
    } else {
        v.clone() - (v - u) * (1.0 - t)
    }
}
