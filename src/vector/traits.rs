use std::ops::{Add, AddAssign,
               Sub, SubAssign,
               Mul, MulAssign,
               Div, DivAssign,
               Neg};

use traits::{Field, Zero};

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

pub trait Norm: Vector {
    type Length; // FIXME: Need some way to represent the real numbers (Length must always be real)

    fn length(self) -> Self::Length;
}
