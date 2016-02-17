#![feature(associated_consts)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(type_macros)]
#![feature(unsize)]

#![feature(test)]

extern crate test;
use test::Bencher;

#[macro_use] extern crate behemoth;

use behemoth::matrix::decomp::lu;

behemoth! {
    matrices! {
        f64: {
            Mat2x2 => 2, 2
            Mat3x3 => 3, 3
            Mat4x4 => 4, 4
            Mat5x5 => 5, 5
        }
    }
}

#[bench]
fn bench_lu_2x2(b: &mut Bencher) {
    let mat = Mat2x2::new([
            [8., 2.],
            [4., 3.],
        ]);

    b.iter(|| {
        lu(test::black_box(mat))
    });
}

#[bench]
fn bench_lu_3x3(b: &mut Bencher) {
    let mat = Mat3x3::new([
            [8., 2., 6.],
            [4., 3., 5.],
            [9., 1., 7.],
        ]);

    b.iter(|| {
        lu(test::black_box(mat))
    });

    test::black_box(mat);
}

#[bench]
fn bench_lu_4x4(b: &mut Bencher) {
    let mat = Mat4x4::new([
            [8.,  2.,  6., 11.],
            [4.,  3.,  5., -5.],
            [9.,  1.,  7., 0.5],
            [6., 10., -3.,  2.],
        ]);

    b.iter(|| {
        lu(test::black_box(mat))
    });

    test::black_box(mat);
}

#[bench]
fn bench_lu_5x5(b: &mut Bencher) {
    let mat = Mat5x5::new([
            [8.,  2.,  6., 11., 3.],
            [4.,  3.,  5., -5., 0.],
            [9.,  1.,  7., 0.5, 5.],
            [6., 10., -3.,  2., 8.],
            [2., -2., -1., 0.5, 3.],
        ]);

    b.iter(|| {
        lu(test::black_box(mat))
    });

    test::black_box(mat);
}
