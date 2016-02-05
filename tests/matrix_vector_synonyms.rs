#![feature(associated_consts)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(type_macros)]

#![feature(log_syntax)]

#[macro_use] extern crate behemoth;
use behemoth::Square;

behemoth! {
    matrices! {
        f64:
        {
            Mat3 => 3, 3
        }

        Vec3 => 3
    }

    euclidean_space! {
        Vec3: f64 { x, y, z }
    }
}

#[test]
fn matrix_vector_synonym_mul() {
    let x = Vec3::new(1., 1., 1.);
    let i = Mat3::identity();
    assert_eq!(i * x, x);
}
