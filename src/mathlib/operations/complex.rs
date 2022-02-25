use crate::mathlib::classes::complex::Complexf32;
use num::Zero;
use std::ops::{Add, Div, Mul, Sub};

impl Add<Complexf32> for Complexf32 {
    type Output = Complexf32;
    fn add(self, _rhs: Complexf32) -> Complexf32 {
        Complexf32::new(self.real + _rhs.real, self.imaginary + _rhs.imaginary)
    }
}

impl<T> Add<T> for Complexf32
where
    T: Into<f32>,
{
    type Output = Complexf32;
    fn add(self, _rhs: T) -> Complexf32 {
        Complexf32::new(self.real + _rhs.into(), self.imaginary)
    }
}
impl Sub<Complexf32> for Complexf32 {
    type Output = Complexf32;
    fn sub(self, _rhs: Complexf32) -> Complexf32 {
        Complexf32::new(self.real - _rhs.real, self.imaginary - _rhs.imaginary)
    }
}

impl<T> Sub<T> for Complexf32
where
    T: Into<f32>,
{
    type Output = Complexf32;
    fn sub(self, _rhs: T) -> Complexf32 {
        Complexf32::new(self.real - _rhs.into(), self.imaginary)
    }
}

// https://wims.univ-cotedazur.fr/wims/wims.cgi?session=3P6507DCBB.1&+lang=fr&+module=H6%2Falgebra%2Fdocintrocomplexf32.fr&+cmd=reply&+job=read&+doc=1&+block=produit
impl Mul<Complexf32> for Complexf32 {
    type Output = Complexf32;
    fn mul(self, _rhs: Complexf32) -> Complexf32 {
        let a: f32 = self.real;
        let b: f32 = self.imaginary;
        let c: f32 = _rhs.real;
        let d: f32 = _rhs.imaginary;
        Complexf32::new((a * c) - (b * d), (a * d) + (b * c))
    }
}

impl<T> Mul<T> for Complexf32
where
    T: Into<f32>,
{
    type Output = Complexf32;
    fn mul(self, _rhs: T) -> Complexf32 {
        let num: f32 = _rhs.into();
        Complexf32::new(self.real * num, self.imaginary * num)
    }
}

// https://wims.univ-cotedazur.fr/wims/wims.cgi?session=3P6507DCBB.7&+lang=fr&+module=H6%2Falgebra%2Fdocintrocomplexf32.fr&+cmd=reply&+job=read&+doc=1&+block=quotient
impl Div<Complexf32> for Complexf32 {
    type Output = Complexf32;
    fn div(self, _rhs: Complexf32) -> Complexf32 {
        if _rhs.is_zero() {
            panic!("Can't divide by zero")
        }
        let a: f32 = self.real;
        let b: f32 = self.imaginary;
        let c: f32 = _rhs.real;
        let d: f32 = _rhs.imaginary;
        Complexf32::new(
            ((a * c) + (b * d)) / ((c * c) + (d * d)),
            ((b * c) - (a * d)) / ((c * c) + (d * d)),
        )
    }
}

impl<T> Div<T> for Complexf32
where
    T: Into<f32>,
{
    type Output = Complexf32;
    fn div(self, _rhs: T) -> Complexf32 {
        let num: f32 = _rhs.into();
        if num.is_zero() {
            panic!("Can't divide by zero")
        }

        Complexf32::new(self.real / num, self.imaginary / num)
    }
}
