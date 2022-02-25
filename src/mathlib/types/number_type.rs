use std::ops::{Add, Div, Mul, Sub};

pub trait Number<Rhs = Self, Output = Self>:
    Clone
    + Add<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Mul<f32, Output = Output>
    + Div<f32, Output = Output>
    + Sub<Rhs, Output = Output>
    + Copy
    + num::Zero
    + std::fmt::Display
{
}
impl<K, Rhs, Output> Number<Rhs, Output> for K where
    K: Copy
        + Clone
        + Add<Rhs, Output = Output>
        + Mul<f32, Output = Output>
        + Div<f32, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + num::Zero
        + std::fmt::Display
{
}
