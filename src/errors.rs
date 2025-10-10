use std::fmt::{write, Display, Formatter};

pub enum VectorErr {
    LengthErr {expected: usize, got: usize},
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

pub enum MatrixErr {
    NotInvertible
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