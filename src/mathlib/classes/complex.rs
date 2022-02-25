use num::Zero;
use std::fmt;

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Complexf32 {
    pub real: f32,
    pub imaginary: f32,
}

impl Complexf32 {
    pub fn new(a: f32, b: f32) -> Self {
        Self {
            real: a,
            imaginary: b,
        }
    }
}

impl fmt::Display for Complexf32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            write!(f, "0")
        } else if self.real.is_zero() {
            write!(f, "{}i", &self.imaginary)
        } else if self.imaginary.is_zero() {
            write!(f, "{}", &self.real)
        } else {
            write!(f, "{}{:+}i", &self.real, &self.imaginary)
        }
    }
}

impl From<f32> for Complexf32 {
    fn from(d: f32) -> Self {
        Self {
            real: d,
            imaginary: f32::zero(),
        }
    }
}

impl Zero for Complexf32 {
    fn zero() -> Complexf32 {
        Complexf32::new(f32::zero(), f32::zero())
    }

    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imaginary.is_zero()
    }
}
