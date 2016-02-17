use std::ops::{Add, AddAssign,
               Sub, SubAssign,
               Mul, MulAssign,
               Div, DivAssign,
               Neg,
               Index, IndexMut};

use traits::{Field, UncheckedIndex, UncheckedIndexMut, Zero};

pub trait Matrix: Add<Output=Self> + AddAssign
                + Sub<Output=Self> + SubAssign
                + Mul<<Self as Matrix>::Scalar, Output=Self> + MulAssign<<Self as Matrix>::Scalar>
                + Div<<Self as Matrix>::Scalar, Output=Self> + DivAssign<<Self as Matrix>::Scalar>
                + Neg<Output=Self>
                + Zero
                + Index<(usize, usize), Output=<Self as Matrix>::Scalar>
                + IndexMut<(usize, usize)>
                + UncheckedIndex<(usize, usize)>
                + UncheckedIndexMut<(usize, usize)>
                + Clone {
    type Scalar: Field + Mul<Self, Output=Self>;

    fn row_switch(self, usize, usize) -> Self;
    fn row_mul(self, usize, Self::Scalar) -> Self;
    fn row_add(self, usize, usize, Self::Scalar) -> Self;

    fn col_switch(self, usize, usize) -> Self;
    fn col_mul(self, usize, Self::Scalar) -> Self;
    fn col_add(self, usize, usize, Self::Scalar) -> Self;

    fn dimensions(&self) -> (usize, usize);

    fn row_switch_mut(&mut self, usize, usize);
    fn row_mul_mut(&mut self, usize, Self::Scalar);
    fn row_add_mut(&mut self, usize, usize, Self::Scalar);

    fn col_switch_mut(&mut self, usize, usize);
    fn col_mul_mut(&mut self, usize, Self::Scalar);
    fn col_add_mut(&mut self, usize, usize, Self::Scalar);
}

pub trait Transpose: Matrix where
        <Self as Matrix>::Scalar: Mul<Self::Transpose, Output=Self::Transpose> {
    type Transpose: Matrix<Scalar=Self::Scalar> + Transpose<Transpose=Self>;

    fn transpose(&self) -> Self::Transpose;
}

pub trait Square: Matrix
                + Transpose<Transpose=Self>
                + Mul<Output=Self>
                + MulAssign {

    fn identity() -> Self;

    fn determinant(&self) -> Self::Scalar { unimplemented!(); }
    fn order(&self) -> usize;

    fn transpose_mut(&mut self);
}
