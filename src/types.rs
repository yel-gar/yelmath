use num_traits::{FromPrimitive, Signed, ToPrimitive};

pub trait Scalar: Signed + ToPrimitive + FromPrimitive + Default + Copy {}

impl Scalar for f32 {}
impl Scalar for f64 {}
impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for i128 {}
