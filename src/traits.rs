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
               + One {}

pub trait Matrix: Add<Output=Self> + AddAssign
                + Sub<Output=Self> + SubAssign
                + Mul<<Self as Matrix>::Scalar, Output=Self> + MulAssign<<Self as Matrix>::Scalar>
                + Div<<Self as Matrix>::Scalar, Output=Self> + DivAssign<<Self as Matrix>::Scalar>
                + Neg<Output=Self>
                + Zero {
    type Scalar: Field + Mul<Self, Output=Self>;

    fn dimensions(&self) -> (usize, usize);
}

pub trait Transpose: Matrix where
        <Self as Matrix>::Scalar: Mul<Self::Transpose, Output=Self::Transpose> {
    type Transpose: Matrix<Scalar=Self::Scalar>;

    fn transpose(&self) -> Self::Transpose;
}

pub trait Square: Matrix
                + Transpose<Transpose=Self>
                + Mul<Output=Self>
                + MulAssign {

    fn identity() -> Self;

    fn determinant(&self) -> Self::Scalar { unimplemented!(); }

    fn transpose_mut(&mut self);
}

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
