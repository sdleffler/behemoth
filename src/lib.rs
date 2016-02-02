#![feature(log_syntax)]
#![feature(trace_macros)]

#![feature(type_macros)]

#![feature(augmented_assignments)]
#![feature(op_assign_traits)]

#![feature(associated_consts)]

#[macro_use]
pub mod matrix;
pub mod scalar;
pub mod traits;
#[macro_use]
pub mod vector;

mod tests;

use std::ops::{Add, Mul};

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

macro_rules! impl_add_mul_identities {
    ($({ zero: $zero:expr, one: $one:expr, { $($t:ty)* } })*) => (
    	$(
    		$(
    			impl Zero for $t {
    				const ZERO: $t = $zero;
    			}

    			impl One for $t {
    				const ONE: $t = $one;
    			}
    		)*
    	)*
    )
}

impl_add_mul_identities! {
	{ zero: 0, one: 1, { u8 u16 u32 u64 usize i8 i16 i32 i64 isize } }
	{ zero: 0.0, one: 1.0, { f32 f64 } }
}
