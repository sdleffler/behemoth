#![feature(associated_consts)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(type_macros)]

#![feature(test)]

extern crate test;
use test::Bencher;

#[macro_use] extern crate behemoth;

behemoth! {
    matrices! {
        f64:
        {
            Mat3x3 => 3, 3
        }
    }
}

#[bench]
fn bench_mul_square(b: &mut Bencher) {
    let mut mat = Mat3x3::new([
            [8., 2., 6.],
            [4., 3., 5.],
            [9., 1., 7.],
        ]);

    b.iter(|| {
        mat = mat * mat;
    });

    test::black_box(mat);
}
