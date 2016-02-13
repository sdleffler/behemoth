use std::ops::{Add, AddAssign,
               Sub, SubAssign,
               Mul, MulAssign,
               Div, DivAssign,
               Neg};


/// These two traits are intended simply for holdover until std::num is
/// stabilized.
pub trait Zero: Sized + Add<Self, Output=Self> {
	const ZERO: Self;
}

/// These two traits are intended simply for holdover until std::num is
/// stabilized.
pub trait One: Sized + Mul<Self, Output=Self> {
	const ONE: Self;
}


pub trait Field: Add<Output=Self>
               + Sub<Output=Self>
               + Mul<Output=Self>
               + Div<Output=Self>
               + Neg<Output=Self>
               + Zero
               + One
               + PartialOrd
               + PartialEq
               + Copy {}

pub trait Vector: Add<Output=Self> + AddAssign
                + Sub<Output=Self> + SubAssign
                + Mul<<Self as Vector>::Scalar, Output=Self> + MulAssign<<Self as Vector>::Scalar>
                + Div<<Self as Vector>::Scalar, Output=Self> + DivAssign<<Self as Vector>::Scalar>
                + Neg<Output=Self>
                + Zero where
          <Self as Vector>::Scalar: Field + Mul<Self, Output=Self> {
    type Scalar: Field;
}

pub trait Cross<Rhs=Self> {
    type Perpendicular;

    fn cross(self, Rhs) -> Self::Perpendicular;
}

pub trait Dot: Vector {
    fn dot(self, Self) -> Self::Scalar;
}

pub trait Metric {
    type Distance;

    fn distance(self, Self) -> Self::Distance;
}

pub trait Norm: Vector {
    type Length; // FIXME: Need some way to represent the real numbers (Length must always be real)

    fn length(self) -> Self::Length;
}

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(&self, other: &Rhs) -> bool;
    fn approx_ne(&self, other: &Rhs) -> bool { !self.approx_eq(other) }
}

impl ApproxEq for f32 {
    fn approx_eq(&self, other: &Self) -> bool {
        use std::f32;
        (self - other).abs() < f32::EPSILON
    }
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &Self) -> bool {
        use std::f64;
        (self - other).abs() < f64::EPSILON
    }
}
