#![feature(associated_consts)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(type_macros)]
#![feature(log_syntax)]
#![feature(unsize)]

#[macro_use] extern crate behemoth;

behemoth! {
    matrices! {
        f64: {
            //Mat2x2 => 2, 2
            Mat2x3 => 2, 3
            Mat3x2 => 3, 2
        }
    }
}

fn main() {
    let a = Mat2x3::new([
        [ 1.,  2., 1.5],
        [2.5, -1.,  2.],
        ]);
    let b = Mat3x2::new([
        [-2., 3.],
        [0.5, 1.],
        [ 4., 0.],
        ]);

    println!("{:?}", a * b);
}
