use crate::types::Scalar;
use crate::vectors::{Vector, Vector3D, Vector4D};

pub trait Matrix<T: Scalar>: Sized {
    type VEC: Vector<T>;
    fn zero() -> Self;
    fn dimensions(&self) -> (usize, usize); // EXCLUSIVE
    fn identity() -> Self;
    fn from_func(f: impl Fn(usize, usize) -> T) -> Self;
    fn get_val(&self, i: usize, j: usize) -> T;
    fn determinant(&self) -> T;
    fn minor(&self, i: usize, j: usize) -> T;
    fn transposed(&self) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn cross(&self, other: &Self) -> Self;
    fn inverse(&self) -> Option<Self>;
    fn mul_vec(&self, v: &Self::VEC) -> Self::VEC;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix3x3<T: Scalar> {
    _data: [T; 9],
}

impl<T: Scalar> Matrix3x3<T> {
    pub fn new(data: [[T; 3]; 3]) -> Self {
        Self::from_func(|i, j| data[i][j])
    }
}

impl<T: Scalar> Matrix<T> for Matrix3x3<T> {
    type VEC = Vector3D<T>;

    fn zero() -> Self {
        Self {
            _data: [T::default(); 9],
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        (3, 3)
    }

    fn identity() -> Self {
        Self::from_func(|i, j| {
            if i == j {
                T::from_i8(1).unwrap() // TODO well this shouldn't panic but let's try
            } else {
                T::default()
            }
        })
    }

    fn from_func(f: impl Fn(usize, usize) -> T) -> Self {
        let mut _data: [T; 9] = Default::default();
        for i in 0..3 {
            for j in 0..3 {
                _data[i * 3 + j] = f(i, j);
            }
        }
        Self { _data }
    }

    fn get_val(&self, i: usize, j: usize) -> T {
        self._data[i * 3 + j]
    }

    fn determinant(&self) -> T {
        // very cool code yes
        let [a11, a12, a13, a21, a22, a23, a31, a32, a33] = self._data;
        a11 * a22 * a33 + a12 * a23 * a31 + a13 * a21 * a32
            - a13 * a22 * a31
            - a12 * a21 * a33
            - a11 * a23 * a32
    }

    fn minor(&self, i: usize, j: usize) -> T {
        let mut values = Vec::<T>::with_capacity(4);
        for x in 0..3 {
            for y in 0..3 {
                if x != i && y != j {
                    values.push(self._data[x * 3 + y]);
                }
            }
        }
        if values.len() != 4 {
            panic!("Invalid matrix dimensions");
        }

        values[0] * values[3] - values[1] * values[2]
    }

    fn transposed(&self) -> Self {
        Self::from_func(|i, j| self.get_val(j, i))
    }

    fn add(&self, other: &Self) -> Self {
        Self::from_func(|i, j| self.get_val(i, j) + other.get_val(i, j))
    }

    fn sub(&self, other: &Self) -> Self {
        Self::from_func(|i, j| self.get_val(i, j) - other.get_val(i, j))
    }

    fn cross(&self, other: &Self) -> Self {
        Self::from_func(|i, j| {
            let mut val = T::default();
            for k in 0..3 {
                val = val + self.get_val(i, k) * other.get_val(k, j)
            }
            val
        })
    }

    fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.to_f32().unwrap_or(0.).abs() < f32::EPSILON {
            // TODO shouldn't panic either but let's try
            return None;
        }

        Some(Matrix3x3::from_func(|i, j| {
            if (i + j) % 2 == 0 {
                self.minor(j, i) / det
            } else {
                -self.minor(j, i) / det
            }
        }))
    }

    fn mul_vec(&self, v: &Self::VEC) -> Self::VEC {
        let [a11, a12, a13, a21, a22, a23, a31, a32, a33] = self._data;
        Vector3D::new(
            v.x * a11 + v.y * a12 + v.z * a13,
            v.x * a21 + v.y * a22 + v.z * a23,
            v.x * a31 + v.y * a32 + v.z * a33,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix4x4<T: Scalar> {
    _data: [T; 16],
}

impl<T: Scalar> Matrix4x4<T> {
    pub fn new(data: [[T; 4]; 4]) -> Self {
        Self::from_func(|i, j| data[i][j])
    }
}

impl<T: Scalar> Matrix<T> for Matrix4x4<T> {
    type VEC = Vector4D<T>;

    fn zero() -> Self {
        Self {
            _data: [T::default(); 16],
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        (4, 4)
    }

    fn identity() -> Self {
        Self::from_func(|i, j| {
            if i == j {
                T::from_i32(1).unwrap() // TODO
            } else {
                T::default()
            }
        })
    }

    fn from_func(f: impl Fn(usize, usize) -> T) -> Self {
        let mut _data: [T; 16] = Default::default();
        for i in 0..4 {
            for j in 0..4 {
                _data[i * 4 + j] = f(i, j);
            }
        }
        Self { _data }
    }

    fn get_val(&self, i: usize, j: usize) -> T {
        self._data[i * 4 + j]
    }

    fn determinant(&self) -> T {
        let mut det_sum = T::default();
        for x in 0..4 {
            if x % 2 == 0 {
                det_sum = det_sum + self.get_val(0, x) * self.minor(0, x);
            } else {
                det_sum = det_sum - self.get_val(0, x) * self.minor(0, x);
            }
        }

        det_sum
    }

    fn minor(&self, i: usize, j: usize) -> T {
        let mut values = Vec::<T>::with_capacity(9);
        for x in 0..4 {
            for y in 0..4 {
                if x != i && y != j {
                    values.push(self._data[x * 4 + y]);
                }
            }
        }
        if values.len() != 9 {
            panic!("Invalid matrix dimensions");
        }

        Matrix3x3::from_func(|i1, j1| values[i1 * 3 + j1]).determinant()
    }

    fn transposed(&self) -> Self {
        Self::from_func(|i, j| self.get_val(j, i))
    }

    fn add(&self, other: &Self) -> Self {
        Self::from_func(|i, j| self.get_val(i, j) + other.get_val(i, j))
    }

    fn sub(&self, other: &Self) -> Self {
        Self::from_func(|i, j| self.get_val(i, j) - other.get_val(i, j))
    }

    fn cross(&self, other: &Self) -> Self {
        Self::from_func(|i, j| {
            let mut val = T::default();
            for k in 0..4 {
                val = val + self.get_val(i, k) * other.get_val(k, j)
            }
            val
        })
    }

    fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.to_f32().unwrap().abs() < f32::EPSILON {
            // TODO
            return None;
        }

        Some(Matrix4x4::from_func(|i, j| {
            if (i + j) % 2 == 0 {
                self.minor(j, i) / det
            } else {
                -self.minor(j, i) / det
            }
        }))
    }

    fn mul_vec(&self, v: &Self::VEC) -> Self::VEC {
        let vec_data = [v.x, v.y, v.z, v.w];
        let mut output_data = [T::default(); 4];
        for i in 0..4 {
            for j in 0..4 {
                output_data[i] = output_data[i] + vec_data[i] * self.get_val(i, j);
            }
        }

        let [x, y, z, w] = output_data;
        Vector4D::new(x, y, z, w)
    }
}
