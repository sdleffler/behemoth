#![allow(unused_features)]

#![feature(log_syntax)]
#![feature(trace_macros)]
#![feature(type_macros)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(associated_consts)]
#![feature(test)]

#[cfg(test)]
extern crate test;

#[cfg(any(feature = "matrix_rand", feature = "vector_rand"))]
extern crate rand;

#[macro_use]
mod macros;

#[macro_use]
pub mod matrix;
pub mod scalar;
pub mod traits;
#[macro_use]
pub mod vector;

mod tests;

macro_rules! _behemoth_in_wrapper_check {
    () => (!! "Behemoth macros must be used inside the behemoth! {} wrapper macro!");
}

macro_rules! behemoth {
    ($($stuff:tt)*) => (
        macro_rules! _behemoth_in_wrapper_check {
            () => ();
        }

        lazy_single_instantiate! {
            _behemoth_use:
            (Add) => { use std::ops::Add; }
            (AddAssign) => { use std::ops::AddAssign; }
            (Sub) => { use std::ops::Sub; }
            (SubAssign) => { use std::ops::SubAssign; }
            (Mul) => { use std::ops::Mul; }
            (MulAssign) => { use std::ops::MulAssign; }
            (Div) => { use std::ops::Div; }
            (DivAssign) => { use std::ops::DivAssign; }
            (Neg) => { use std::ops::Neg; }
            (Deref) => { use std::ops::Deref; }
            (DerefMut) => { use std::ops::DerefMut; }
            (Index) => { use std::ops::Index; }
            (IndexMut) => { use std::ops::IndexMut; }

            (One) => { use $crate::traits::One; }
            (Zero) => { use $crate::traits::Zero; }

            (Field) => { use $crate::traits::Field; }

            (Matrix) => { use $crate::traits::Matrix; }
            (Square) => { use $crate::traits::Square; }

            (Vector) => { use $crate::traits::Vector; }
            (Cross) => { use $crate::traits::Cross; }
            (InnerProduct) => { use $crate::traits::InnerProduct; }
            (Metric) => { use $crate::traits::Metric; }
            (Norm) => { use $crate::traits::Norm; }
        }

        mod __behemoth {
            as_items! {
                $($stuff)*
            }
        }
        pub use __behemoth::*;

        macro_rules! _behemoth_in_wrapper_check {
            () => (!! "Behemoth macros must be used inside the behemoth! {} wrapper macro!");
        }
    );
}
