// FIXME: Needs ApproxEq and assert_approx_eq! macros.

#[cfg(test)]
mod matrix_special_cased {
    matrices! {
        f64:
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

    #[test]
    fn mul_special_cased() {
        assert_eq!(
            Mat1x1([[1.]]) *
            Mat1x1([[2.]]),
            Mat1x1([[2.]])
        );
        assert_eq!(
            Mat1x2([[1., 2.]]) *
            Mat2x1([[3.], [4.]]),
            Mat1x1([[11.]])
        );
        assert_eq!(
            Mat1x3([[1., 2., 3.]]) *
            Mat3x1([[4.], [5.], [6.]]),
            Mat1x1([[32.]])
        );
        assert_eq!(
            Mat1x4([[1., 2., 3., 4.]]) *
            Mat4x1([[5.], [6.], [7.], [8.]]),
            Mat1x1([[70.]])
        );

        assert_eq!(
            Mat2x1([[7.], [1.]]) *
            Mat1x2([[5., 3.]]),
            Mat2x2([[35., 21.], [5., 3.]]));
        assert_eq!(
            Mat2x2([[2., 6.], [10., 9.]]) *
            Mat2x2([[8., 1.], [5., 9.]]),
            Mat2x2([[46., 56.], [125., 91.]]));
        assert_eq!(
            Mat2x3([[2., 5., 2.], [9., 3., 7.]]) *
            Mat3x2([[3., 6.], [2., 8.], [5., 10.]]),
            Mat2x2([[26., 72.], [68., 148.]]));
        assert_eq!(
            Mat2x4([[9., 5., 5., 7.], [7., 2., 8., 6.]]) *
            Mat4x2([[4., 7.], [1., 8.], [8., 5.], [10., 3.]]),
            Mat2x2([[151., 149.], [154., 123.]])
        );

        assert_eq!(
            Mat3x1([[4.], [2.], [1.]]) *
            Mat1x3([[1., 8., 9.]]),
            Mat3x3([[4., 32., 36.], [2., 16., 18.], [1., 8., 9.]])
        );
        assert_eq!(
            Mat3x2([[6., 5.], [7., 2.], [6., 10.]]) *
            Mat2x3([[2., 9., 4.], [4., 8., 5.]]),
            Mat3x3([[32., 94., 49.], [22., 79., 38.], [52., 134., 74.]])
        );
        assert_eq!(
            Mat3x3([[3., 3., 8.], [4., 9., 10.], [2., 1., 10.]]) *
            Mat3x3([[7., 4., 2.], [7., 5., 5.], [8., 1., 8.]]),
            Mat3x3([[106., 35., 85.], [171., 71., 133.], [101., 23., 89.]])
        );
        assert_eq!(
            Mat3x4([[5., 8., 2., 1.], [8., 2., 9., 10.], [8., 10., 3., 9.]]) *
            Mat4x3([[7., 1., 3.], [7., 9., 6.], [5., 6., 2.], [3., 9., 5.]]),
            Mat3x3([[104., 98., 72.], [145., 170., 104.], [168., 197., 135.]])
        );

        assert_eq!(
            Mat4x1([[9.], [5.], [8.], [1.]]) *
            Mat1x4([[7., 6., 6., 5.]]),
            Mat4x4([[63., 54., 54., 45.],
                    [35., 30., 30., 25.],
                    [56., 48., 48., 40.],
                    [ 7.,  6.,  6.,  5.]])
        );
        assert_eq!(
            Mat4x2([[4., 9.], [5., 8.], [2., 1.], [3., 10.]]) *
            Mat2x4([[7., 7., 8., 8.], [10., 6., 3., 6.]]),
            Mat4x4([[118., 82., 59., 86.],
                    [115., 83., 64., 88.],
                    [ 24., 20., 19., 22.],
                    [121., 81., 54., 84.]])
        );
        assert_eq!(
            Mat4x3([[4., 3., 3.], [8., 10., 4.], [2., 10., 9.], [2., 1., 3.]]) *
            Mat3x4([[10., 9., 7., 2.], [10., 3., 7., 1.], [1., 5., 9., 4.]]),
            Mat4x4([[ 73.,  60.,  76., 23.],
                    [184., 122., 162., 42.],
                    [129.,  93., 165., 50.],
                    [ 33.,  36.,  48., 17.]])
        );
        assert_eq!(
            Mat4x4([[1.,  7.,  6., 4.],
                    [8.,  7., 10., 2.],
                    [9.,  9.,  8., 1.],
                    [9., 10.,  6., 3.]]) *
            Mat4x4([[5., 7., 10., 10.],
                    [5., 3.,  9.,  2.],
                    [6., 9., 10.,  6.],
                    [2., 1.,  1.,  3.]]),
            Mat4x4([[ 84.,  86., 137.,  72.],
                    [139., 169., 245., 160.],
                    [140., 163., 252., 159.],
                    [137., 150., 243., 155.]])
        );
    }

    #[test]
    fn det_special_cased() {
        assert_eq!(Mat1x1([[5.]]).determinant(), 5.);
        assert_eq!(Mat2x2([[2., 3.], [4., 1.]]).determinant(), -10.);
        assert_eq!(Mat3x3([[7., 6., 5.], [4., 8., 4.], [10., 8., 2.]]).determinant(), -160.);
    }

    #[test]
    fn transpose_special_cased() {
        assert_eq!(Mat1x1([[1.]]).transpose(), Mat1x1([[1.]]));
        assert_eq!(Mat1x2([[1., 2.]]).transpose(), Mat2x1([[1.], [2.]]));
        assert_eq!(Mat1x3([[1., 2., 3.]]).transpose(), Mat3x1([[1.], [2.], [3.]]));
        assert_eq!(Mat1x4([[1., 2., 3., 4.]]).transpose(), Mat4x1([[1.], [2.], [3.], [4.]]));
        assert_eq!(Mat2x1([[1.], [2.]]).transpose(), Mat1x2([[1., 2.]]));
        assert_eq!(Mat2x2([[1., 2.], [3., 4.]]).transpose(), Mat2x2([[1., 3.], [2., 4.]]));
        assert_eq!(Mat2x3([[1., 2., 3.], [4., 5., 6.]]).transpose(),
                   Mat3x2([[1., 4.], [2., 5.], [3., 6.]]));
        assert_eq!(Mat2x4([[1., 2., 3., 4.], [5., 6., 7., 8.]]).transpose(),
                   Mat4x2([[1., 5.], [2., 6.], [3., 7.], [4., 8.]]));
        assert_eq!(Mat3x1([[1.], [2.], [3.]]).transpose(), Mat1x3([[1., 2., 3.]]));
        assert_eq!(Mat3x2([[1., 2.], [3., 4.], [5., 6.]]).transpose(),
                   Mat2x3([[1., 3., 5.], [2., 4., 6.]]));
        assert_eq!(Mat3x3([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]).transpose(),
                   Mat3x3([[1., 4., 7.], [2., 5., 8.], [3., 6., 9.]]));
        assert_eq!(Mat3x4([[1., 2., 3., 4.], [5., 6., 7., 8.], [9., 10., 11., 12.]]).transpose(),
                   Mat4x3([[1., 5., 9.], [2., 6., 10.], [3., 7., 11.], [4., 8., 12.]]));
        assert_eq!(Mat4x1([[1.], [2.], [3.], [4.]]).transpose(), Mat1x4([[1., 2., 3., 4.]]));
        assert_eq!(Mat4x2([[1., 2.], [3., 4.], [5., 6.], [7., 8.]]).transpose(),
                   Mat2x4([[1., 3., 5., 7.], [2., 4., 6., 8.]]));
        assert_eq!(Mat4x3([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.], [10., 11., 12.]]).transpose(),
                   Mat3x4([[1., 4., 7., 10.], [2., 5., 8., 11.], [3., 6., 9., 12.]]));
        assert_eq!(Mat4x4([[ 1.,  2.,  3.,  4.],
                           [ 5.,  6.,  7.,  8.],
                           [ 9., 10., 11., 12.],
                           [13., 14., 15., 16.]]).transpose(),
                   Mat4x4([[ 1.,  5.,  9., 13.],
                           [ 2.,  6., 10., 14.],
                           [ 3.,  7., 11., 15.],
                           [ 4.,  8., 12., 16.]]));
    }
}

#[cfg(test)]
mod matrix_general_case {
    matrices! {
        f64:
        Mat5x5 => 5, 5
        Mat5x8 => 5, 8
        Mat6x6 => 6, 6
        Mat8x5 => 8, 5
        Mat8x8 => 8, 8
    }

    #[test]
    fn mul_general_case() {
        assert_eq!(
            Mat5x8([[ 6., 4., 10.,  8., 10., 9.,  5.,  9.],
                    [10., 5.,  8.,  9., 10., 6.,  4.,  8.],
                    [10., 6.,  2., 10.,  7., 2., 10., 10.],
                    [ 6., 1.,  6.,  8., 10., 9.,  9.,  4.],
                    [ 1., 9.,  9.,  3.,  7., 8.,  3.,  4.]]) *
            Mat8x5([[3., 9., 10., 5., 8.],
                    [1., 6.,  8., 9., 4.],
                    [9., 1.,  9., 5., 2.],
                    [6., 9.,  5., 9., 9.],
                    [4., 5.,  6., 1., 5.],
                    [7., 7.,  9., 7., 4.],
                    [6., 8.,  2., 3., 4.],
                    [5., 5.,  7., 3., 8.]]),
            Mat5x5([[338., 358., 436., 303., 334.],
                    [307., 373., 435., 304., 351.],
                    [266., 397., 366., 285., 361.],
                    [298., 343., 349., 253., 290.],
                    [233., 234., 326., 242., 200.]])
        );
    }

    #[test]
    fn det_general_case() {
        assert_eq!(
            Mat6x6([[1., 5., 2., 5., 5., 3.],
                    [2., 4., 3., 5., 2., 3.],
                    [1., 1., 1., 1., 1., 1.],
                    [1., 2., 3., 5., 5., 3.],
                    [2., 5., 3., 2., 2., 5.],
                    [5., 5., 2., 4., 4., 4.]]).determinant(),
            225.
        );
    }
}
