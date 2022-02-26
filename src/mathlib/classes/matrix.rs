use crate::mathlib::classes::vector::Vector;
use crate::mathlib::types::number_type::Number;

use std::fmt;

/// Returns a Matrix
///
/// ### values
///
/// * `data` : vec with all elements of the Matrix
/// * `n` : number of column
/// * `m` : number of rows
#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
pub struct Matrix<K>
where
    K: Number,
{
    pub data: Vec<K>,
    m: usize,
    n: usize,
}

impl<K> Matrix<K>
where
    K: Number,
{
    /// SHAPE
    ///
    /// return [m, n]
    /// m as rows
    /// n as column
    pub fn shape(&self) -> [usize; 2] {
        [self.m, self.n]
    }
    pub fn get_data(&self) -> Vec<K> {
        self.data.clone()
    }
    pub fn from_vec(&self, vec: Vec<K>, shape: [usize; 2]) -> Self {
        Self {
            data: vec,
            m: shape[0],
            n: shape[1],
        }
    }
    pub fn is_square(&self) -> bool {
        self.m == self.n
    }
    pub fn get_val(&self, row: usize, col: usize) -> K {
        self.data[self.n * row + col]
    }
}

impl<K> Matrix<K>
where
    K: Number,
{
    pub fn empty_matrix(&self, rows: usize, col: usize) -> Self {
        Self {
            data: vec![K::zero(); rows * col],
            m: rows,
            n: col,
        }
    }
}

/// CONSTRUCTOR FROM

impl<K, const N: usize, const M: usize> From<[[K; N]; M]> for Matrix<K>
where
    K: Number,
{
    fn from(d: [[K; N]; M]) -> Self {
        let mut vec = Vec::<K>::new();
        for item in d.into_iter().enumerate() {
            let mut next = Vec::<K>::from(item.1);
            vec.append(&mut next);
        }
        Self {
            data: vec,
            m: M,
            n: N,
        }
    }
}
impl<K> From<(Vector<K>, usize)> for Matrix<K>
where
    K: Number,
{
    fn from(d: (Vector<K>, usize)) -> Self {
        if d.0.size() % d.1 != 0 {
            panic!("The vector you're trying to convert has wrong dimension");
        }
        Self {
            data: d.0.get_data(),
            m: d.0.size() / d.1,
            n: d.1,
        }
    }
}
impl<K> From<(&Vector<K>, usize)> for Matrix<K>
where
    K: Number,
{
    fn from(d: (&Vector<K>, usize)) -> Self {
        if d.0.size() % d.1 != 0 {
            panic!("The vector you're trying to convert has wrong dimension");
        }
        Self {
            data: d.0.get_data(),
            m: d.0.size() / d.1,
            n: d.1,
        }
    }
}

impl<K> From<Vector<K>> for Matrix<K>
where
    K: Number,
{
    fn from(d: Vector<K>) -> Self {
        Self {
            data: d.get_data(),
            m: d.size(),
            n: 1,
        }
    }
}

impl<K> From<&Vector<K>> for Matrix<K>
where
    K: Number,
{
    fn from(d: &Vector<K>) -> Self {
        Self {
            data: d.get_data(),
            m: d.size(),
            n: 1,
        }
    }
}

impl<K> From<(Vec<K>, [usize; 2])> for Matrix<K>
where
    K: Number,
{
    fn from(d: (Vec<K>, [usize; 2])) -> Self {
        Self {
            data: d.0.clone(),
            m: d.1[0],
            n: d.1[1],
        }
    }
}

///
/// DISPLAY
///
impl<K> fmt::Display for Matrix<K>
where
    K: Number,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_display = String::new();
        to_display.push('[');
        to_display.push('[');
        for (pos, e) in self.data.iter().enumerate() {
            to_display += &format!("{}", e);
            if (pos + 1) % self.n == 0 {
                to_display += "]\n[";
            } else {
                to_display += ", "
            }
        }
        to_display.pop();
        to_display.pop();
        to_display.push(']');
        write!(f, "{}", to_display)
    }
}
