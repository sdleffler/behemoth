#![feature(associated_consts)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(type_macros)]
#![feature(unsize)]

#[macro_use]
extern crate behemoth;

use behemoth::{ApproxEq};

use behemoth::matrix::decomp::lu;

behemoth! {
    matrices! {
        f64: {
            Mat3x3 => 3, 3
        }
    }
}

#[test]
fn lup_3x3() {
    let a = Mat3x3::new([
        [2.,  1.,  3.],
        [5., 2.5,  4.],
        [1.,  2., 0.5]]);

    let (p, l, u) = lu(a);
    assert_approx_eq!(a, p * l * u);
}
