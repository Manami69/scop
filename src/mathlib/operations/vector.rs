use crate::mathlib::classes::{complex::Complexf32, vector::Vector};
use crate::mathlib::types::{number_type::Number, trait_absolute::Absolute};
use num::Zero;
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

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
                .map(|x| *x.0 + *x.1)
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
                .map(|x| *x.0 + *x.1)
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
                .map(|x| *x.0 + *x.1)
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
                .map(|x| *x.0 + *x.1)
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
                .map(|x| *x.0 - *x.1)
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
                .map(|x| *x.0 - *x.1)
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
                .map(|x| *x.0 - *x.1)
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
                .map(|x| *x.0 - *x.1)
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
        self.from_vec(self.data.iter().map(|x| *x * _rhs).collect())
    }
}

impl<K> Mul<K> for &Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn mul(self, _rhs: K) -> Vector<K> {
        self.from_vec(self.data.iter().map(|x| *x * _rhs).collect())
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

impl<K> Div<K> for Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn div(self, _rhs: K) -> Vector<K> {
        self.from_vec(self.data.iter().map(|x| *x / _rhs).collect())
    }
}

impl<K> Div<K> for &Vector<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn div(self, _rhs: K) -> Vector<K> {
        self.from_vec(self.data.iter().map(|x| *x / _rhs).collect())
    }
}

impl<K> Vector<K>
where
    K: Number,
{
    /// LINEAR COMBINATION
    /// fr => combinaison linéaire
    ///
    /// a linear combination is an expression constructed from a set of terms
    /// by multiplying each term by a constant and adding the results   
    pub fn linear_combination<const N: usize>(u: &[Vector<K>; N], coefs: &[K; N]) -> Vector<K> {
        u.iter()
            .zip(coefs.iter())
            .fold(Vector::<K>::from(u[0].size()), |mut sum, plus| {
                sum = sum + plus.0 * *plus.1;
                sum
            })
    }
    /// DOT PRODUCT
    /// fr - produit scalaire
    ///
    /// The function must compute and return the scalar 〈u|v〉 = u·v, called the dot product,
    /// or inner product, of u and v.
    ///
    /// If both vectors have different dimensions, the behaviour is undefined.
    pub fn dot(&self, v: &Vector<K>) -> K {
        if self.size() != v.size() {
            panic!("cannot compute 2 Vectors of different dimension.")
        }
        self.data
            .iter()
            .zip(v.data.iter())
            .fold(K::zero(), |mut acc, num| {
                acc = acc + *num.0 * *num.1;
                acc
            })
    }

    /// CROSS PRODUCT
    /// fr - Produit vectoriel
    ///
    /// Given two linearly independent vectors a and b, the cross product,
    /// a × b (read "a cross b"), is a vector that is perpendicular to both a and b,
    /// and thus normal to the plane containing them.
    ///
    /// If both vector aren't of dimension 3, the behaviour is undefined
    pub fn cross_product(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
        if u.size() != 3 || v.size() != 3 {
            panic!("cross product can only be done with 3 dimension vectors");
        }
        let a: K = u.data[1] * v.data[2] - v.data[1] * u.data[2];
        let b: K = u.data[2] * v.data[0] - v.data[2] * u.data[0];
        let c: K = u.data[0] * v.data[1] - v.data[0] * u.data[1];

        Vector::from([a, b, c])
    }

	pub fn cross_2d(u: &Vector<f32>, v: &Vector<f32>) -> f32 {
		if u.size() != 2 || v.size() != 2 {
            panic!("cross product can only be done with 3 dimension vectors");
        }
		u.data[0] * v.data[1] - u.data[1] * v.data[0]
	}
}

impl<V> Vector<V>
where
    V: Number + Absolute,
{
    /// Taxicab Norm
    pub fn norm_1(&self) -> f32 {
        self.data.iter().fold(f32::zero(), |mut acc, num| {
            acc += num.abs();
            acc
        })
    }
    /// Euclidian Norm
    ///
    /// return the Magnitude (length of a vector)
    pub fn norm(&self) -> f32 {
        self.data
            .iter()
            .fold(f32::zero(), |mut acc, num| {
                acc += num.abs() * num.abs();
                acc
            })
            .sqrt()
    }
    /// Supremum Norm
    pub fn norm_inf(&self) -> f32 {
        self.data
            .iter()
            .max_by(|last, next| {
                if last.abs() >= next.abs() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .unwrap()
            .abs()
    }
    pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> K
    where
        K: Number + Absolute + Div<f32, Output = K>,
    {
        u.dot(v) / (u.norm() * v.norm())
    }
}

impl Vector<f32> {
    /// Normalize
    ///
    /// Change the vector's magnitude/norm (length of the vector) to 1
    pub fn normalize(&self) -> Vector<f32> {
        self / self.norm()
    }
}
