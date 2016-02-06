#![feature(type_macros)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(associated_consts)]

#[macro_use] extern crate behemoth;

behemoth! {
    vector_space! {
        ScalarVec {
            (f64)

            Scalar = f64;
        }
    }

    euclidean! {
        Vec2: f64 { x, y }
        Vec3: f64 { x, y, z }
    }
}

#[test]
fn tuplevec_arithmetic() {
    let x = ScalarVec(2.);
    let y = ScalarVec(3.);
    assert_eq!(x + y, ScalarVec(5.));
    assert_eq!(x - y, ScalarVec(-1.));
    assert_eq!(x * 4., ScalarVec(8.));
}
