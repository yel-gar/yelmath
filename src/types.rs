use std::ops::{Add, Div, Mul, Neg, Sub};

// aka number alias
pub trait Scalar:
Add<Output = Self> +
Sub<Output = Self> +
Mul<Output = Self> +
Neg<Output = Self> +
Div<Output = Self> +
Copy + Default + PartialEq
{}

impl Scalar for f32 {}
impl Scalar for f64 {}
impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for i128 {}
