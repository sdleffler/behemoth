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

#[macro_use] mod macros;

#[macro_use] pub mod matrix;
#[macro_use] pub mod vector;

pub mod scalar;

mod traits;
pub use traits::*;

pub struct BlackHole<T>(T); // Swallow anything whole.

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

            _use_BlackHole => ( use $crate::BlackHole; );

            _use_One => ( use $crate::One; );
            _use_Zero => ( use $crate::Zero; );

            _use_Field => ( use $crate::Field; );

            _use_Matrix => ( use $crate::Matrix; );
            _use_Square => ( use $crate::Square; );
            _use_Transpose => (use $crate::Transpose; );

            _use_Vector => ( use $crate::Vector; );
            _use_Cross => ( use $crate::Cross; );
            _use_InnerProduct => ( use $crate::InnerProduct; );
            _use_Metric => ( use $crate::Metric; );
            _use_Norm => ( use $crate::Norm; );
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
