#![feature(associated_consts)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(type_macros)]
#![feature(unsize)]

#![feature(test)]

extern crate test;
use test::Bencher;

#[macro_use] extern crate behemoth;

behemoth! {
    matrices! {
        f64:
        {
            Mat2x2 => 2, 2
            Mat3x3 => 3, 3
            Mat4x4 => 4, 4
            Mat5x5 => 5, 5
        }
    }
}

use behemoth::Square;

#[bench]
fn bench_det_2x2(b: &mut Bencher) {
    let mat = Mat2x2::new([
            [ 3., 4.],
            [-1., 2.]
        ]);

    b.iter(|| {
        test::black_box(mat).determinant()
    });
}

#[bench]
fn bench_det_3x3(b: &mut Bencher) {
    let mat = Mat3x3::new([
            [ 3.,  4., -1.],
            [ 5., -2.,  3.],
            [ 7., -5.,  3.],
        ]);

    b.iter(|| {
        test::black_box(mat).determinant()
    });
}

#[bench]
fn bench_det_4x4(b: &mut Bencher) {
    let mat = Mat4x4::new([
            [ 5., -2.,  5., -4.],
            [ 1., -4., -1., -3.],
            [ 3.,  5.,  4.,  3.],
            [-4., -5.,  2.,  3.],
        ]);

    b.iter(|| {
        test::black_box(mat).determinant()
    });
}

#[bench]
fn bench_det_5x5(b: &mut Bencher) {
    let mat = Mat5x5::new([
            [-4.,  3.,  5.,  2.,  2.],
            [ 2.,  3., -2., -1.,  5.],
            [-5., -5., -5., -5.,  1.],
            [-5., -4., -3., -3.,  1.],
            [ 4., -5.,  4., -5., -3.],
        ]);

    b.iter(|| {
        test::black_box(mat).determinant()
    });
}
