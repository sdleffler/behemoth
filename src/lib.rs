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

macro_rules! _behemoth_in_wrapper_check {
    () => (!! format!("Behemoth macros must be used inside the behemoth wrapper macro! Invoked on line {} in {}", line!(), file!()));
}

#[macro_export]
macro_rules! behemoth {
    ($($stuff:tt)*) => (
        macro_rules! _behemoth_in_wrapper_check {
            () => ();
        }

        lazy_single_use! {
            _use_Add => ( use std::ops::Add; );
            _use_AddAssign => ( use std::ops::AddAssign; );
            _use_Sub => ( use std::ops::Sub; );
            _use_SubAssign => ( use std::ops::SubAssign; );
            _use_Mul => ( use std::ops::Mul; );
            _use_MulAssign => ( use std::ops::MulAssign; );
            _use_Div => ( use std::ops::Div; );
            _use_DivAssign => ( use std::ops::DivAssign; );
            _use_Neg => ( use std::ops::Neg; );
            _use_Deref => ( use std::ops::Deref; );
            _use_DerefMut => ( use std::ops::DerefMut; );
            _use_Index => ( use std::ops::Index; );
            _use_IndexMut => ( use std::ops::IndexMut; );

            _use_One => ( use $crate::traits::One; );
            _use_Zero => ( use $crate::traits::Zero; );

            _use_Field => ( use $crate::traits::Field; );

            _use_Matrix => ( use $crate::traits::Matrix; );
            _use_Square => ( use $crate::traits::Square; );

            _use_Vector => ( use $crate::traits::Vector; );
            _use_Cross => ( use $crate::traits::Cross; );
            _use_InnerProduct => ( use $crate::traits::InnerProduct; );
            _use_Metric => ( use $crate::traits::Metric; );
            _use_Norm => ( use $crate::traits::Norm; );
        }

        mod __behemoth {
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
