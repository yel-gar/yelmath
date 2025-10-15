use num_traits::{One, Signed, ToPrimitive};

pub trait Scalar: Signed + ToPrimitive + PartialOrd + Default + Copy + One {}

impl Scalar for f32 {}
impl Scalar for f64 {}
impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for i128 {}
