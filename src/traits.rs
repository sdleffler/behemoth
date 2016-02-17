use std::ops::{Add,
               Sub,
               Mul,
               Div,
               Neg,
               Index, IndexMut};


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

pub trait UncheckedIndex<T>: Index<T> {
    #[inline]
    unsafe fn index_unchecked<'a>(&'a self, T) -> &'a Self::Output;
}

pub trait UncheckedIndexMut<T>: UncheckedIndex<T> + IndexMut<T> {
    #[inline]
    unsafe fn index_unchecked_mut<'a>(&'a mut self, T) -> &'a mut Self::Output;
}
