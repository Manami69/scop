use crate::mathlib::classes::matrix;
use crate::mathlib::types::number_type::Number;
use matrix::Matrix;
use num::Zero;
use std::fmt;
/// VECTOR CLASS

#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
pub struct Vector<K>
where
    K: Number,
{
    pub data: Vec<K>,
}

impl<K> Vector<K>
where
    K: Number,
{
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn from_vec(&self, vec: Vec<K>) -> Self {
        Self { data: vec }
    }
    pub fn get_data(&self) -> Vec<K> {
        self.data.clone()
    }

    pub fn is_zero(&self) -> bool
    where
        K: Zero,
    {
        for d in &self.data {
            if !d.is_zero() {
                return false;
            }
        }
        true
    }
}

/// CONSTRUCTOR FROM :
/// * An Array of K
/// * a simple Matrix
/// * a simple Vector
/// * a Vector Array
impl<K, const N: usize> From<[K; N]> for Vector<K>
where
    K: Number,
{
    fn from(d: [K; N]) -> Self {
        Self {
            data: Vec::<K>::from(d),
        }
    }
}

impl<K> From<Matrix<K>> for Vector<K>
where
    K: Number,
{
    fn from(d: Matrix<K>) -> Self {
        Self { data: d.get_data() }
    }
}

impl<K> From<&Matrix<K>> for Vector<K>
where
    K: Number,
{
    fn from(d: &Matrix<K>) -> Self {
        Self { data: d.get_data() }
    }
}

impl<K> From<&Vector<K>> for Vector<K>
where
    K: Number,
{
    fn from(d: &Vector<K>) -> Self {
        Self { data: d.get_data() }
    }
}

impl<K> From<usize> for Vector<K>
where
    K: Number,
{
    fn from(d: usize) -> Self {
        Self {
            data: vec![K::zero(); d],
        }
    }
}

impl<K> From<Vec<K>> for Vector<K>
where
    K: Number,
{
    fn from(d: Vec<K>) -> Self {
        Self { data: d }
    }
}
/// Display Vector
impl<K> fmt::Display for Vector<K>
where
    K: Number,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_display = self
            .data
            .iter()
            .fold(String::new(), |acc, num| acc + &format!("[{}]\n", &num));
        to_display.pop();
        write!(f, "{}", to_display)
    }
}

impl Vector<f32> {
	pub fn vec3(x: f32, y: f32, z: f32) -> Vector<f32> {
		Vector::from([x, y, z])
	}
}