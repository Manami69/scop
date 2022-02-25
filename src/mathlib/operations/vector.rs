use crate::mathlib::classes::{complex::Complexf32, vector::Vector};
use crate::mathlib::types::number_type::Number;
use std::ops::{Add, Mul, Sub};

/// OPERATOR + / - / * OVERLOADING
///
/// *** Use references(&) to not consume the Vectors ***
///
/// * All operation can be made with or without references if you are finished with your old Vector.s

/// ADD
impl<K> Add<&Vector<K>> for &Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn add(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) + K::clone(x.1))
                .collect(),
        )
    }
}

impl<K> Add<Vector<K>> for &Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn add(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) + K::clone(x.1))
                .collect(),
        )
    }
}

impl<K> Add<&Vector<K>> for Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn add(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) + K::clone(x.1))
                .collect(),
        )
    }
}

impl<K> Add<Vector<K>> for Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn add(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) + K::clone(x.1))
                .collect(),
        )
    }
}

/// SUB
impl<K> Sub<&Vector<K>> for &Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn sub(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) - K::clone(x.1))
                .collect(),
        )
    }
}

impl<K> Sub<Vector<K>> for &Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn sub(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) - K::clone(x.1))
                .collect(),
        )
    }
}

impl<K> Sub<&Vector<K>> for Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn sub(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) - K::clone(x.1))
                .collect(),
        )
    }
}

impl<K> Sub<Vector<K>> for Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn sub(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) - K::clone(x.1))
                .collect(),
        )
    }
}

// SCALE

impl<K> Mul<K> for Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn mul(self, _rhs: K) -> Vector<K> {
        self.from_vec(self.data.iter().map(|x| K::clone(x) * _rhs).collect())
    }
}

impl<K> Mul<K> for &Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn mul(self, _rhs: K) -> Vector<K> {
        self.from_vec(self.data.iter().map(|x| K::clone(x) * _rhs).collect())
    }
}

impl Mul<f32> for Vector<Complexf32> {
    type Output = Vector<Complexf32>;
    fn mul(self, _rhs: f32) -> Vector<Complexf32> {
        self.from_vec(self.data.iter().map(|x| *x * _rhs).collect())
    }
}
impl Mul<f32> for &Vector<Complexf32> {
    type Output = Vector<Complexf32>;
    fn mul(self, _rhs: f32) -> Vector<Complexf32> {
        self.from_vec(self.data.iter().map(|x| *x * _rhs).collect())
    }
}

impl<K> Mul<&K> for Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn mul(self, _rhs: &K) -> Vector<K> {
        self.from_vec(
            self.data
                .iter()
                .map(|x| K::clone(x) * K::clone(_rhs))
                .collect(),
        )
    }
}

impl<K> Mul<&K> for &Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn mul(self, _rhs: &K) -> Vector<K> {
        self.from_vec(
            self.data
                .iter()
                .map(|x| K::clone(x) * K::clone(_rhs))
                .collect(),
        )
    }
}
