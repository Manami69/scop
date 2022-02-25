use crate::mathlib::classes::{complex::Complexf32, matrix::Matrix, vector::Vector};
use crate::mathlib::types::number_type::Number;

use std::fmt;

use std::ops::{Add, Mul, Sub};

/// OPERATOR + / - / * OVERLOADING
///
/// *** Use references(&) to not consume the Vectors ***
///
/// * All operation can be made with or without references if you are finished with your old Vector.s

/// ADD
impl<K> Add<&Matrix<K>> for &Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn add(self, _rhs: &Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot add 2 matrixes with different shapes");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) + K::clone(x.1))
                .collect(),
            self.shape(),
        )
    }
}

impl<K> Add<Matrix<K>> for &Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn add(self, _rhs: Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot add 2 matrixes with different shapes");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) + K::clone(x.1))
                .collect(),
            self.shape(),
        )
    }
}

impl<K> Add<&Matrix<K>> for Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn add(self, _rhs: &Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot add 2 matrixes with different shapes");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) + K::clone(x.1))
                .collect(),
            self.shape(),
        )
    }
}

impl<K> Add<Matrix<K>> for Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn add(self, _rhs: Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot add 2 matrixes with different shapes");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) + K::clone(x.1))
                .collect(),
            self.shape(),
        )
    }
}

/// SUB
impl<K> Sub<&Matrix<K>> for &Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn sub(self, _rhs: &Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot sub 2 matrixes with different shapes");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) - K::clone(x.1))
                .collect(),
            self.shape(),
        )
    }
}

impl<K> Sub<Matrix<K>> for &Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn sub(self, _rhs: Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot sub 2 matrixes with different shapes");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) - K::clone(x.1))
                .collect(),
            self.shape(),
        )
    }
}

impl<K> Sub<&Matrix<K>> for Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn sub(self, _rhs: &Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot sub 2 matrixes with different shapes");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) - K::clone(x.1))
                .collect(),
            self.shape(),
        )
    }
}

impl<K> Sub<Matrix<K>> for Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn sub(self, _rhs: Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot sub 2 matrixes with different shapes");
        }
        self.from_vec(
            self.data
                .iter()
                .zip(_rhs.data.iter())
                .map(|x| K::clone(x.0) - K::clone(x.1))
                .collect(),
            self.shape(),
        )
    }
}

// SCALE

impl<K> Mul<K> for Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn mul(self, _rhs: K) -> Matrix<K> {
        self.from_vec(
            self.data.iter().map(|x| K::clone(x) * _rhs).collect(),
            self.shape(),
        )
    }
}

impl<K> Mul<K> for &Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn mul(self, _rhs: K) -> Matrix<K> {
        self.from_vec(
            self.data.iter().map(|x| K::clone(x) * _rhs).collect(),
            self.shape(),
        )
    }
}
impl Mul<f32> for Matrix<Complexf32> {
    type Output = Matrix<Complexf32>;
    fn mul(self, _rhs: f32) -> Matrix<Complexf32> {
        self.from_vec(
            self.data
                .iter()
                .map(|x| Complexf32::clone(x) * _rhs)
                .collect(),
            self.shape(),
        )
    }
}

impl Mul<f32> for &Matrix<Complexf32> {
    type Output = Matrix<Complexf32>;
    fn mul(self, _rhs: f32) -> Matrix<Complexf32> {
        self.from_vec(
            self.data
                .iter()
                .map(|x| Complexf32::clone(x) * _rhs)
                .collect(),
            self.shape(),
        )
    }
}

// MUL Matrix

impl<K> Mul<Matrix<K>> for Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn mul(self, _rhs: Matrix<K>) -> Matrix<K> {
        self.mul_mat(&_rhs)
    }
}

impl<K> Mul<Matrix<K>> for &Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn mul(self, _rhs: Matrix<K>) -> Matrix<K> {
        self.mul_mat(&_rhs)
    }
}

impl<K> Mul<&Matrix<K>> for Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn mul(self, _rhs: &Matrix<K>) -> Matrix<K> {
        self.mul_mat(_rhs)
    }
}

impl<K> Mul<&Matrix<K>> for &Matrix<K>
where
    K: Number,
{
    type Output = Matrix<K>;
    fn mul(self, _rhs: &Matrix<K>) -> Matrix<K> {
        self.mul_mat(_rhs)
    }
}

// MUL VECTOR

impl<K> Mul<Vector<K>> for Matrix<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn mul(self, _rhs: Vector<K>) -> Vector<K> {
        self.mul_vec(&_rhs)
    }
}

impl<K> Mul<Vector<K>> for &Matrix<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn mul(self, _rhs: Vector<K>) -> Vector<K> {
        self.mul_vec(&_rhs)
    }
}

impl<K> Mul<&Vector<K>> for Matrix<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn mul(self, _rhs: &Vector<K>) -> Vector<K> {
        self.mul_vec(_rhs)
    }
}

impl<K> Mul<&Vector<K>> for &Matrix<K>
where
    K: Number,
{
    type Output = Vector<K>;
    fn mul(self, _rhs: &Vector<K>) -> Vector<K> {
        self.mul_vec(_rhs)
    }
}

impl<K> Matrix<K>
where
    K: Number,
{
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        let mn = self.shape();
        let size = vec.size();
        if mn[1] != size {
            panic!("For matrix and vector multiplication, the number of col in the matrix must be equal to the vector dimension.")
        }
        let mut new = Vec::<K>::new();
        for row in 0..mn[0] {
            let mut num = K::zero();

            for col in 0..mn[1] {
                num = num + self.get_val(row, col) * vec.data[col];
            }
            new.push(num);
        }
        Vector::from(new)
    }

    /// Matrix mul
    ///
    /// If A is an m × n matrix and B is an n × p matrix,
    /// the matrix product C = AB (denoted without multiplication signs or dots)
    /// is defined to be the m × p matrix
    /// https://en.wikipedia.org/wiki/Matrix_multiplication#Definition
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        let mn = self.shape();
        let np = mat.shape();
        if mn[1] != np[0] {
            panic!("For matrix multiplication, the number of columns in the first matrix must be equal to the number of rows in the second matrix.")
        }
        let mut new = Vec::<K>::new();
        for i in 0..mn[0] * np[1] {
            let row = i / np[1];
            let col = i % np[1];
            let mut num = K::zero();
            for j in 0..mn[1] {
                num = num + self.get_val(row, j) * mat.get_val(j, col);
            }
            new.push(num);
        }
        Matrix::from((new, [mn[0], np[1]]))
    }

    /// TRACE
    ///
    /// the trace of a square matrix A, denoted tr(A),
    /// is defined to be the sum of elements on the main diagonal
    /// (from the upper left to the lower right) of A.
    /// The trace is only defined for a square matrix (n × n).
    pub fn trace(&self) -> K {
        if !self.is_square() {
            panic!("The trace is only defined for a square matrix (n × n).");
        }
        let mn = self.shape();
        let mut num = K::zero();
        for i in 0..mn[0] {
            num = num + self.get_val(i, i);
        }
        num
    }

    /// TRANSPOSE
    ///
    /// the transpose of a matrix is an operator which flips a matrix over its diagonal.
    pub fn transpose(&self) -> Matrix<K> {
        let nm: [usize; 2] = self.shape();
        let mut data = vec![K::zero(); nm[0] * nm[1]];

        for rowtocol in 0..nm[0] {
            for coltorow in 0..nm[1] {
                data[coltorow * nm[0] + rowtocol] = self.get_val(rowtocol, coltorow);
            }
        }
        Matrix::from((data, [nm[1], nm[0]]))
    }

    /// ROW REDUCED ECHELON FORM
    /// fr - Matrice échelonnée
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut new = self.data.clone();
        let nm = self.shape();
        let mut row = 0;
        let mut pivot: K;
        for cur_pivot_place in 0..nm[1] {
            if row >= nm[0] {
                break;
            }

            // Step 1 - Swap if needed
            if new[row * nm[1] + cur_pivot_place].is_zero() {
                for r in row..nm[0] {
                    if !new[r * nm[1] + cur_pivot_place].is_zero() {
                        for i in 0..nm[1] {
                            new.swap(row * nm[1] + i, r * nm[1] + i);
                        }
                        break;
                    }
                }
            }
            pivot = new[row * nm[1] + cur_pivot_place];
            if !pivot.is_zero() {
                // Step 2 - Scale the row to make the pivot 1 (by divising)
                for i in 0..nm[1] {
                    new[row * nm[1] + i] = new[row * nm[1] + i] / pivot
                }

                // Step 3 - get zeros above and below the pivot (substract by the first row multiplied by the number we need to get to zero and do it to all the row)

                for r in 0..nm[0] {
                    if r == row {
                        continue;
                    }
                    let to_make_zero = new[r * nm[1] + cur_pivot_place];
                    for col in 0..nm[1] {
                        new[r * nm[1] + col] =
                            new[r * nm[1] + col] - (new[row * nm[1] + col] * to_make_zero);
                    }
                }
                row += 1;
            }
        }
        Matrix::from((new, nm))
    }

    /// DETERMINANT
    pub fn determinant(&self) -> K {
        if !self.is_square() {
            panic!("can't calculate determinant of a non square Matrix");
        }
        let nm = self.shape();

        if nm[0] > 4 {
            panic!("determinant of n*n matrixes is only supported for n <= 4");
        }
        // 1x1 Matrix
        if nm[0] == 1 {
            self.data[0]
        } else if nm[0] == 2 {
            // 2x2 Matrix det(A) is the aera of the parallelogram formed by the image of the basis vectors
            self.data[0] * self.data[3] - self.data[1] * self.data[2]
        } else if nm[0] == 3 {
            // 3x3 Matrix
            let add: [[usize; 3]; 3] = [[0, 4, 8], [1, 5, 6], [2, 3, 7]];
            let sub: [[usize; 3]; 3] = [[1, 3, 8], [0, 5, 7], [2, 4, 6]];
            let to_add = add.into_iter().fold(K::zero(), |mut acc, num| {
                acc = acc + self.data[num[0]] * self.data[num[1]] * self.data[num[2]];
                acc
            });
            let to_sub = sub.into_iter().fold(K::zero(), |mut acc, num| {
                acc = acc + self.data[num[0]] * self.data[num[1]] * self.data[num[2]];
                acc
            });
            to_add - to_sub
        } else {
            // 4x4 Matrix
            let mut num = K::zero();
            for i in 0..4 {
                let mut vec: Vec<K> = Vec::new();
                self.data.iter().enumerate().for_each(|(j, &x)| {
                    if !(j % 4 == i || j / 4 == 0) {
                        vec.push(x);
                    }
                });
                if i % 2 == 0 {
                    num = num + self.data[i] * Matrix::<K>::from((vec, [3, 3])).determinant();
                } else {
                    num = num - self.data[i] * Matrix::<K>::from((vec, [3, 3])).determinant();
                }
            }
            num
        }
    }

    /// IDENTITY
    ///
    /// return identity matrix Vec dataset of size n*n
    fn identity(&self, n: usize) -> Vec<K>
    where
        K: From<f32>,
    {
        let mut id = vec![K::zero(); n * n];
        for i in 0..n {
            id[i * n + i] = (1.).into();
        }
        id
    }

    /// INVERSE
    ///
    /// for a matrix A, the inverse of that matrix is A-1.
    ///
    /// If the Matrix determinant is zero, the matrix is singular and an error is returned;
    pub fn inverse(&self) -> Result<Matrix<K>, SingularMatrix>
    where
        K: From<f32>,
    {
        if !self.is_square() {
            panic!("Cannot compute non square matrix inverse")
        }
        let nm = self.shape();
        if self.determinant().is_zero() {
            return Err(SingularMatrix);
        }
        let mut last = self.data.clone();
        let mut next = self.identity(nm[0]);
        let mut pivot: K;
        for i in 0..nm[0] {
            // Step 1 - Swap if needed
            if last[i * nm[1] + i].is_zero() {
                for r in i..nm[0] {
                    if !last[r * nm[1] + i].is_zero() {
                        for j in 0..nm[1] {
                            last.swap(i * nm[1] + j, r * nm[1] + j);
                            next.swap(i * nm[1] + j, r * nm[1] + j);
                        }
                        break;
                    }
                }
            }
            pivot = last[i * nm[1] + i];
            // Step 2 - Scale the row to make the pivot 1 (by divising)
            for j in 0..nm[1] {
                last[i * nm[1] + j] = last[i * nm[1] + j] / pivot;
                next[i * nm[1] + j] = next[i * nm[1] + j] / pivot;
            }
            // Step 3 - get zeros above and below the pivot
            for r in 0..nm[0] {
                if r == i {
                    continue;
                }
                let to_make_zero = last[r * nm[1] + i];
                for col in 0..nm[1] {
                    last[r * nm[1] + col] =
                        last[r * nm[1] + col] - (last[i * nm[1] + col] * to_make_zero);
                    next[r * nm[1] + col] =
                        next[r * nm[1] + col] - (next[i * nm[1] + col] * to_make_zero);
                }
            }
        }
        Ok(Matrix::from((next, nm)))
    }

    /// RANK
    ///
    /// The rank is the column space dimension of the matrix A.
    pub fn rank(&self) -> usize {
        let rref = self.row_echelon();
        let nm = rref.shape();
        let mut count: usize = 0;
        for row in 0..nm[0] {
            for col in 0..nm[1] {
                if !rref.get_val(row, col).is_zero() {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
}

#[derive(Debug, Clone)]
pub struct SingularMatrix;

impl fmt::Display for SingularMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot find inverse of a singular matrix.")
    }
}

/// FUNCTION FOR GRAPHIC PROJECTS
impl Matrix<f32> {
    /// MAT4
    ///
    /// create an identity matrix of shape 4*4
    pub fn mat4() -> Matrix<f32> {
        Matrix::from([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }

    /// PROJECTION MATRIX
    pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
		let tanf = (fov * 0.5).tan();
        Matrix::from([
            [1. / (ratio * tanf), 0., 0., 0.],
            [0., 1. / tanf, 0., 0.],
            [
                0.,
                0.,
                (far + near) / (far - near) * -1.,
                -1.,
            ],
            [0., 0., (2.0 * far * near) / (far - near) * -1., 1.],
        ])
    }

    /// scale the actual matrix by scalar
    pub fn scale(&self, scl: f32) -> Matrix<f32> {
        self * Matrix::from([
            [scl, 0., 0., 0.],
            [0., scl, 0., 0.],
            [0., 0., scl, 0.],
            [0., 0., 0., 1.],
        ])
    }

    /// translate the actual matrix by (Tx, Ty, Tz)
    pub fn translate(&self, tx: f32, ty: f32, tz: f32) -> Matrix<f32> {
        self * Matrix::from([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [tx, ty, tz, 1.],
        ])
    }

    /// Rotate the actual matrix of angle theta in radian on the Axis (x, y , z , 1)
    ///
    /// https://learnopengl.com/Getting-started/Transformations
    pub fn rotate(&self, angle: f32, axis: Vector<f32>) -> Matrix<f32> {
        if axis.size() != 3 {
            panic!("ROTATION MATRIX ON {} SIZED VECTOR", axis.size());
        }
        let (rx, ry, rz) = (axis.data[0], axis.data[1], axis.data[2]);
        let cos = angle.cos();
        let sin = angle.sin();
        let cos1 = 1. - cos;
        self * Matrix::from([
            [
                cos + (rx * rx) * cos1,
                rx * ry * cos1 - rz * sin,
                rx * rz * cos1 + ry * sin,
                0.,
            ],
            [
                ry * rx * cos1 + rz * sin,
                cos + (ry * ry) * cos1,
                ry * rz * cos1 - rx * sin,
                0.,
            ],
            [
                rz * rx * cos1 - ry * sin,
                rz * ry * cos1 + rx * sin,
                cos + (rz * rz) * cos1,
                0.,
            ],
            [0., 0., 0., 1.],
        ])
    }

	pub fn as_ptr(&self) -> *const gl::types::GLfloat {
		self.data.as_ptr() as *const gl::types::GLfloat
	}
}
