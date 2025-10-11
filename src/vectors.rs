use crate::errors::VectorErr;
use crate::types::Scalar;

// vector trait for both 2D and 3D
pub trait Vector<T: Scalar> : Sized {
    fn zero() -> Self;
    fn from_slice(data: &[T]) -> Result<Self, VectorErr>;
    fn from_vec(data: &Vec<T>) -> Result<Self, VectorErr>;
    fn invert(&self) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn dot(&self, other: &Self) -> T;
    fn scale(&self, a: T) -> Self;
    fn magnitude_f32(&self) -> f32;
    fn magnitude_f64(&self) -> f64;
}

#[derive(PartialEq, Clone, Debug)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Scalar> Vector<T> for Vector2D<T> {
    fn zero() -> Self {
        Self {x: T::default(), y: T::default()}
    }

    fn from_slice(data: &[T]) -> Result<Self, VectorErr>
    where
        Self: Sized
    {
        if data.len() != 2 {
            return Err(VectorErr::LengthErr {expected: 2, got: data.len()});
        }

        Ok(Self {x: data[0], y: data[1]})
    }

    fn from_vec(data: &Vec<T>) -> Result<Self, VectorErr>
    where
        Self: Sized
    {
        Self::from_slice(data.as_slice())
    }

    fn invert(&self) -> Self {
        Self {x: -self.x, y: -self.y}
    }

    fn add(&self, other: &Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }

    fn sub(&self, other: &Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    fn scale(&self, a: T) -> Self {
        Self {x: self.x * a, y: self.y * a}
    }

    fn magnitude_f32(&self) -> f32 {
        (self.x * self.x + self.y * self.y).to_f32().unwrap().sqrt()  // TODO handle this
    }

    fn magnitude_f64(&self) -> f64 {
        (self.x * self.x + self.y * self.y).to_f64().unwrap().sqrt()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn cross(&self, other: &Self) -> Self {
        todo!()
    }
}

impl<T: Scalar> Vector<T> for Vector3D<T>
{
    fn zero() -> Self {
        Self { x: T::default(), y: T::default(), z: T::default() }
    }

    fn from_slice(data: &[T]) -> Result<Self, VectorErr>
    where
        Self: Sized
    {
        if data.len() != 3 {
            return Err(VectorErr::LengthErr {expected: 3, got: data.len()});
        }

        Ok(Self {x: data[0], y: data[1], z: data[2]})
    }

    fn from_vec(data: &Vec<T>) -> Result<Self, VectorErr>
    where
        Self: Sized
    {
        Self::from_slice(data.as_slice())
    }

    fn invert(&self) -> Self {
        Self {x: -self.x, y: -self.y, z: -self.z}
    }

    fn add(&self, other: &Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }

    fn sub(&self, other: &Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn scale(&self, a: T) -> Self {
        Self {x: self.x * a, y: self.y * a, z: self.z * a}
    }

    fn magnitude_f32(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).to_f32().unwrap().sqrt()  // TODO
    }

    fn magnitude_f64(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).to_f64().unwrap().sqrt()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Vector4D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4D<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: Scalar> Vector<T> for Vector4D<T>
{
    fn zero() -> Self {
        Self { x: T::default(), y: T::default(), z: T::default(), w: T::default() }
    }

    fn from_slice(data: &[T]) -> Result<Self, VectorErr>
    where
        Self: Sized
    {
        if data.len() != 4 {
            return Err(VectorErr::LengthErr {expected: 4, got: data.len()});
        }

        Ok(Self {x: data[0], y: data[1], z: data[2], w: data[3]})
    }

    fn from_vec(data: &Vec<T>) -> Result<Self, VectorErr>
    where
        Self: Sized
    {
        Self::from_slice(data.as_slice())
    }

    fn invert(&self) -> Self {
        Self {x: -self.x, y: -self.y, z: -self.z, w: -self.w}
    }

    fn add(&self, other: &Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: self.w + other.w}
    }

    fn sub(&self, other: &Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: self.w - other.w}
    }

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn scale(&self, a: T) -> Self {
        Self {x: self.x * a, y: self.y * a, z: self.z * a, w: self.w * a}
    }

    fn magnitude_f32(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).to_f32().unwrap().sqrt()  // TODO
    }

    fn magnitude_f64(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).to_f64().unwrap().sqrt()
    }
}