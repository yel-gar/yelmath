use std::fmt::{write, Debug, Display, Formatter};

pub enum VectorErr {
    LengthErr {expected: usize, got: usize},
}

impl Debug for VectorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorErr::LengthErr { expected, got } => {
                write!(f, "Expected {} got {}", expected, got)
            }
        }
    }
}

impl Display for VectorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorErr::LengthErr { expected, got } => {
                write!(f, "Expected {} got {}", expected, got)
            }
        }
    }
}

impl std::error::Error for VectorErr {}

pub enum MatrixErr {
    NotInvertible
}

impl Debug for MatrixErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixErr::NotInvertible => {
                write!(f, "Matrix is not invertible")
            }
        }
    }
}

impl Display for MatrixErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixErr::NotInvertible => {
                write!(f, "Matrix is not invertible")
            }
        }
    }
}

impl std::error::Error for MatrixErr {}