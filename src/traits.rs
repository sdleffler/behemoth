use ::{One, Zero};

use std::ops::{Add, AddAssign,
               Sub, SubAssign,
               Mul, MulAssign,
               Div, DivAssign,
               Neg};

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
    type Transpose: Matrix;

    fn dimensions(&self) -> (usize, usize);

    fn transpose(&self) -> Self::Transpose;
}

pub trait Square: Matrix<Transpose=Self>
                + Mul<Output=Self>
                + MulAssign {

    fn identity() -> Self;

    fn determinant(&self) -> Self::Scalar { unimplemented!(); }

    fn transpose_mut(&mut self);
}

pub trait Vector: Add<Output=Self>
                + Sub<Output=Self>
                + Mul<<Self as Vector>::Scalar, Output=Self>
                + Div<<Self as Vector>::Scalar, Output=Self>
                + Neg<Output=Self>
                + Zero where
          <Self as Vector>::Scalar: Mul<Self, Output=Self> {
    type Scalar: Field;
}

pub trait Cross<Rhs=Self> {
    type Perpendicular;

    fn cross(self, Rhs) -> Self::Perpendicular;
}

pub trait InnerProduct: Vector {
    fn dot(self, Self) -> Self::Scalar;
}

pub trait Metric {
    type Distance;

    fn distance(self, Self) -> Self::Distance;
}

pub trait Norm {
    type Length;

    fn length(self) -> Self::Length;
}
