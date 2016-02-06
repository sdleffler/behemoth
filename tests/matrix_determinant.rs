#![feature(associated_consts)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(type_macros)]
#![feature(log_syntax)]

#[macro_use] extern crate behemoth;
use behemoth::{ApproxEq, Square};

behemoth! {
    matrices! {
        f64:
        {
            Mat1x1 => 1, 1
            Mat1x2 => 1, 2
            Mat1x3 => 1, 3
            Mat1x4 => 1, 4
            Mat2x1 => 2, 1
            Mat2x2 => 2, 2
            Mat2x3 => 2, 3
            Mat2x4 => 2, 4
            Mat3x1 => 3, 1
            Mat3x2 => 3, 2
            Mat3x3 => 3, 3
            Mat3x4 => 3, 4
            Mat4x1 => 4, 1
            Mat4x2 => 4, 2
            Mat4x3 => 4, 3
            Mat4x4 => 4, 4
        }
    }

    matrices! {
        f64:
        {
            Mat5x5 => 5, 5
            Mat5x8 => 5, 8
            Mat6x6 => 6, 6
            Mat8x5 => 8, 5
            Mat8x8 => 8, 8
        }
    }
}

#[test]
fn det_special_case() {
    assert_approx_eq!(Mat1x1([[5.]]).determinant(), 5.);
    assert_approx_eq!(Mat2x2([[2., 3.], [4., 1.]]).determinant(), -10.);
    assert_approx_eq!(Mat3x3([[7., 6., 5.], [4., 8., 4.], [10., 8., 2.]]).determinant(), -160.);
}

#[test]
fn det_general_case() {
    assert_approx_eq!(
        Mat6x6([[1., 5., 2., 5., 5., 3.],
                [2., 4., 3., 5., 2., 3.],
                [1., 1., 1., 1., 1., 1.],
                [1., 2., 3., 5., 5., 3.],
                [2., 5., 3., 2., 2., 5.],
                [5., 5., 2., 4., 4., 4.]]).determinant(),
        225.
    );
}
