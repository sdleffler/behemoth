#![allow(unused_features)]
#![feature(log_syntax)]
#![feature(trace_macros)]
#![feature(type_macros)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(associated_consts)]
#![feature(unsize)]

#[cfg(any(feature = "matrix_rand", feature = "vector_rand"))]
extern crate rand;

#[macro_use]
mod macros;
#[macro_use]
pub mod matrix;
#[macro_use]
pub mod vector;

mod traits;

#[cfg(feature = "as_mathematica")]
pub mod mathematica;

pub mod scalar;

#[cfg(feature = "as_mathematica")]
pub use mathematica::*;

pub use traits::*;
pub use matrix::traits::*;
pub use vector::traits::*;

macro_rules! _behemoth_in_wrapper_check {
    () => (!! format!("Behemoth macros must be used inside the behemoth wrapper macro! Invoked on line {} in {}", line!(), file!()));
}

#[macro_export]
macro_rules! behemoth {
    ($($stuff:tt)*) => (
        macro_rules! _behemoth_in_wrapper_check {
            () => ();
        }

        mod __behemoth {
            #[allow(unused_imports)] use std::slice::Iter;

            #[allow(unused_imports)] use std::ops::Add;
            #[allow(unused_imports)] use std::ops::AddAssign;
            #[allow(unused_imports)] use std::ops::Sub;
            #[allow(unused_imports)] use std::ops::SubAssign;
            #[allow(unused_imports)] use std::ops::Mul;
            #[allow(unused_imports)] use std::ops::MulAssign;
            #[allow(unused_imports)] use std::ops::Div;
            #[allow(unused_imports)] use std::ops::DivAssign;
            #[allow(unused_imports)] use std::ops::Neg;
            #[allow(unused_imports)] use std::ops::Deref;
            #[allow(unused_imports)] use std::ops::DerefMut;
            #[allow(unused_imports)] use std::ops::Index;
            #[allow(unused_imports)] use std::ops::IndexMut;

            #[allow(unused_imports)] use $crate::One;
            #[allow(unused_imports)] use $crate::Zero;

            #[allow(unused_imports)] use $crate::Field;

            #[allow(unused_imports)] use $crate::Matrix;
            #[allow(unused_imports)] use $crate::Square;
            #[allow(unused_imports)] use $crate::Transpose;

            #[allow(unused_imports)] use $crate::Vector;
            #[allow(unused_imports)] use $crate::Cross;
            #[allow(unused_imports)] use $crate::Dot;
            #[allow(unused_imports)] use $crate::Norm;

            #[allow(unused_imports)] use $crate::ApproxEq;

            #[allow(unused_imports)] use $crate::{UncheckedIndex, UncheckedIndexMut};

            #[allow(unused_imports)]
            #[cfg(feature = "as_mathematica")]
            use $crate::AsMathematica;

            use std::fmt;
            use std::marker::{PhantomData, Unsize};

            #[derive(Clone, Copy)]
            pub struct DenseMatrix<T, U, V>(T, PhantomData<(U, V)>) where
                    T: Copy + Unsize<[U]>, U: Copy + Unsize<[V]>, V: Copy;

            impl<T, U, V> From<T> for DenseMatrix<T, U, V>  where
                    T: Copy + Unsize<[U]>, U: Copy + Unsize<[V]>, V: Copy {
                #[inline]
                fn from(data: T) -> Self {
                    DenseMatrix(data, PhantomData)
                }
            }

            impl<T, U, V> fmt::Debug for DenseMatrix<T, U, V> where
                    T: fmt::Debug + Copy + Unsize<[U]>, U: Copy + Unsize<[V]>, V: Copy {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "DenseMatrix {:?}", self.0)
                }
            }

            impl<T, U, V> DenseMatrix<T, U, V> where
                    T: Copy + Unsize<[U]>, U: Copy + Unsize<[V]>, V: Copy {
                #[inline]
                pub fn new(data: T) -> DenseMatrix<T, U, V> {
                    DenseMatrix(data, PhantomData)
                }
            }

            impl<T, U, V> Index<(usize, usize)> for DenseMatrix<T, U, V> where
                    T: Copy + Unsize<[U]>, U: Copy + Unsize<[V]>, V: Copy {
                type Output = V;

                #[inline]
                fn index(&self, idx: (usize, usize)) -> &V {
                    let sliced: &[U] = &self.0;
                    let row: &[V] = &sliced[idx.0];
                    &row[idx.1]
                }
            }

            impl<T, U, V> IndexMut<(usize, usize)> for DenseMatrix<T, U, V> where
                    T: Copy + Unsize<[U]>, U: Copy + Unsize<[V]>, V: Copy {
                #[inline]
                fn index_mut(&mut self, idx: (usize, usize)) -> &mut V {
                    let sliced: &mut [U] = &mut self.0;
                    let row: &mut [V] = &mut sliced[idx.0];
                    &mut row[idx.1]
                }
            }

            impl<T, U, V> UncheckedIndex<(usize, usize)> for DenseMatrix<T, U, V> where
                    T: Copy + Unsize<[U]>, U: Copy + Unsize<[V]>, V: Copy {
                #[inline]
                unsafe fn index_unchecked(&self, idx: (usize, usize)) -> &V {
                    let sliced: &[U] = &self.0;
                    let row: &[V] = sliced.get_unchecked(idx.0);
                    row.get_unchecked(idx.1)
                }
            }

            impl<T, U, V> UncheckedIndexMut<(usize, usize)> for DenseMatrix<T, U, V> where
                    T: Copy + Unsize<[U]>, U: Copy + Unsize<[V]>, V: Copy {
                #[inline]
                unsafe fn index_unchecked_mut(&mut self, idx: (usize, usize)) -> &mut V {
                    let sliced: &mut [U] = &mut self.0;
                    let row: &mut [V] = sliced.get_unchecked_mut(idx.0);
                    row.get_unchecked_mut(idx.1)
                }
            }

            as_items! {
                $($stuff)*
            }
        }
        pub use self::__behemoth::*;

        macro_rules! _behemoth_in_wrapper_check {
            () => (!! format!("Behemoth macros must be used inside the behemoth wrapper macro! Invoked on line {} in {}", line!(), file!()));
        }
    );
}
