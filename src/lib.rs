#![feature(log_syntax)]
#![feature(trace_macros)]
#![feature(type_macros)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(associated_consts)]

#[cfg(any(feature = "matrix_rand", feature = "vector_rand"))]
extern crate rand;

#[macro_use] mod macros;

#[macro_use] pub mod matrix;
#[macro_use] pub mod vector;

pub mod scalar;

mod traits;
pub use traits::*;

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
            #[allow(unused_imports)] use $crate::Metric;
            #[allow(unused_imports)] use $crate::Norm;

            #[allow(unused_imports)] use $crate::ApproxEq;

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
