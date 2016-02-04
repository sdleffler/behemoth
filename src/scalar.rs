use traits::{Field, One, Zero};

use std::ops::{Add, Sub, Mul, Div, Neg};

impl<T> Field for T where T: Add<Output=T>
                           + Sub<Output=T>
                           + Mul<Output=T>
                           + Div<Output=T>
                           + Neg<Output=T>
                           + Zero
                           + One {}

macro_rules! impl_add_mul_identities {
    ($({ zero: $zero:expr, one: $one:expr, { $($t:ty)* } })*) => (
        $(
            $(
                impl $crate::traits::Zero for $t {
                    const ZERO: $t = $zero;
                }

                impl $crate::traits::One for $t {
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
