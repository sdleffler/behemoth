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

macro_rules! impl_add_mul_identities {
    ($({ zero: $zero:expr, one: $one:expr, { $($t:ty)* } })*) => (
    	$(
    		$(
    			impl traits::Zero for $t {
    				const ZERO: $t = $zero;
    			}

    			impl traits::One for $t {
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
