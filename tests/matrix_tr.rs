#![feature(associated_consts)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(type_macros)]
#![feature(log_syntax)]

#[macro_use] extern crate behemoth;
use behemoth::{ApproxEq, Transpose};

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
            Mat8x5 => 8, 5
            Mat8x8 => 8, 8
        }
    }
}

#[test]
fn transpose_special_case() {
    assert_approx_eq!(Mat1x1([[1.]]).transpose(), Mat1x1([[1.]]));
    assert_approx_eq!(Mat1x2([[1., 2.]]).transpose(), Mat2x1([[1.], [2.]]));
    assert_approx_eq!(Mat1x3([[1., 2., 3.]]).transpose(), Mat3x1([[1.], [2.], [3.]]));
    assert_approx_eq!(Mat1x4([[1., 2., 3., 4.]]).transpose(), Mat4x1([[1.], [2.], [3.], [4.]]));
    assert_approx_eq!(Mat2x1([[1.], [2.]]).transpose(), Mat1x2([[1., 2.]]));
    assert_approx_eq!(Mat2x2([[1., 2.], [3., 4.]]).transpose(), Mat2x2([[1., 3.], [2., 4.]]));
    assert_approx_eq!(Mat2x3([[1., 2., 3.], [4., 5., 6.]]).transpose(),
               Mat3x2([[1., 4.], [2., 5.], [3., 6.]]));
    assert_approx_eq!(Mat2x4([[1., 2., 3., 4.], [5., 6., 7., 8.]]).transpose(),
               Mat4x2([[1., 5.], [2., 6.], [3., 7.], [4., 8.]]));
    assert_approx_eq!(Mat3x1([[1.], [2.], [3.]]).transpose(), Mat1x3([[1., 2., 3.]]));
    assert_approx_eq!(Mat3x2([[1., 2.], [3., 4.], [5., 6.]]).transpose(),
               Mat2x3([[1., 3., 5.], [2., 4., 6.]]));
    assert_approx_eq!(Mat3x3([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]).transpose(),
               Mat3x3([[1., 4., 7.], [2., 5., 8.], [3., 6., 9.]]));
    assert_approx_eq!(Mat3x4([[1., 2., 3., 4.], [5., 6., 7., 8.], [9., 10., 11., 12.]]).transpose(),
               Mat4x3([[1., 5., 9.], [2., 6., 10.], [3., 7., 11.], [4., 8., 12.]]));
    assert_approx_eq!(Mat4x1([[1.], [2.], [3.], [4.]]).transpose(), Mat1x4([[1., 2., 3., 4.]]));
    assert_approx_eq!(Mat4x2([[1., 2.], [3., 4.], [5., 6.], [7., 8.]]).transpose(),
               Mat2x4([[1., 3., 5., 7.], [2., 4., 6., 8.]]));
    assert_approx_eq!(Mat4x3([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.], [10., 11., 12.]]).transpose(),
               Mat3x4([[1., 4., 7., 10.], [2., 5., 8., 11.], [3., 6., 9., 12.]]));
    assert_approx_eq!(Mat4x4([[ 1.,  2.,  3.,  4.],
                       [ 5.,  6.,  7.,  8.],
                       [ 9., 10., 11., 12.],
                       [13., 14., 15., 16.]]).transpose(),
               Mat4x4([[ 1.,  5.,  9., 13.],
                       [ 2.,  6., 10., 14.],
                       [ 3.,  7., 11., 15.],
                       [ 4.,  8., 12., 16.]]));
}

#[test]
fn transpose_general_case() {
    assert_approx_eq!(
        Mat5x8([[ 1.,  2.,  3.,  4.,  5.,  6.,  7.,  8.],
                [ 9., 10., 11., 12., 13., 14., 15., 16.],
                [17., 18., 19., 20., 21., 22., 23., 24.],
                [25., 26., 27., 28., 29., 30., 31., 32.],
                [33., 34., 35., 36., 37., 38., 39., 40.]]).transpose(),
        Mat8x5([[ 1.,  9., 17., 25., 33.],
                [ 2., 10., 18., 26., 34.],
                [ 3., 11., 19., 27., 35.],
                [ 4., 12., 20., 28., 36.],
                [ 5., 13., 21., 29., 37.],
                [ 6., 14., 22., 30., 38.],
                [ 7., 15., 23., 31., 39.],
                [ 8., 16., 24., 32., 40.]])
    );
}
