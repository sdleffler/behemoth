#![feature(associated_consts)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(type_macros)]
#![feature(unsize)]

#![feature(log_syntax)]

#[macro_use] extern crate behemoth;
use behemoth::{ApproxEq, Square};

behemoth! {
    matrices! {
        f64:
        {
            Mat2x2 => 2, 2
            Mat2x3 => 2, 3
            Mat3x2 => 3, 2
            Mat3x3 => 3, 3
        }

        {
            Vec2 => 2
            Vec3 => 3
        }
    }

    euclidean! {
        Vec2: f64 { x, y }
        Vec3: f64 { x, y, z }
    }
}

#[test]
fn mat3x3_mul_vec3() {
    let x = Vec3::new(1., 1., 1.);
    let i = Mat3x3::identity();
    assert_approx_eq!(i * x, x);
}

#[test]
fn mat3x2_mul_vec2() {
    let x = Vec2::new(1., 2.);
    let a = Mat3x2::new([
            [1., 0.],
            [0., 1.],
            [2., 2.],
        ]);
    let b = Vec3::new(1., 2., 6.);
    assert_approx_eq!(a * x, b);
}

#[test]
fn mat2x3_mul_vec3() {
    let x = Vec3::new(1., 3., 2.);
    let a = Mat2x3::new([
            [1., 2., 1.],
            [4., 3., 1.],
        ]);
    let b = Vec2::new(9., 15.);
    assert_approx_eq!(a * x, b);
}
