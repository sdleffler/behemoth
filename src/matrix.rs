#[macro_export]
macro_rules! _matrix_mul_impl {
    ([1, 1], [1, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0]]])
    );
    ([1, 1], [1, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1]]])
    );
    ([1, 1], [1, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1], $lhs[0][0]*$rhs[0][2]]])
    );
    ([1, 1], [1, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1],
                         $lhs[0][0]*$rhs[0][2], $lhs[0][0]*$rhs[0][3]]])
    );
    ([2, 1], [1, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0]],
                        [$lhs[1][0]*$rhs[0][0]]])
    );
    ([2, 1], [1, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1]],
                        [$lhs[1][0]*$rhs[0][0], $lhs[1][0]*$rhs[0][1]]])
    );
    ([2, 1], [1, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1], $lhs[0][0]*$rhs[0][2]],
                        [$lhs[1][0]*$rhs[0][0], $lhs[1][0]*$rhs[0][1], $lhs[1][0]*$rhs[0][2]]])
    );
    ([2, 1], [1, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1], $lhs[0][0]*$rhs[0][2], $lhs[0][0]*$rhs[0][3]],
                        [$lhs[1][0]*$rhs[0][0], $lhs[1][0]*$rhs[0][1], $lhs[1][0]*$rhs[0][2], $lhs[1][0]*$rhs[0][3]]])
    );
    ([3, 1], [1, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0]],
                        [$lhs[1][0]*$rhs[0][0]],
                        [$lhs[2][0]*$rhs[0][0]]])
    );
    ([3, 1], [1, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1]],
                        [$lhs[1][0]*$rhs[0][0], $lhs[1][0]*$rhs[0][1]],
                        [$lhs[2][0]*$rhs[0][0], $lhs[2][0]*$rhs[0][1]]])
    );
    ([3, 1], [1, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1], $lhs[0][0]*$rhs[0][2]],
                        [$lhs[1][0]*$rhs[0][0], $lhs[1][0]*$rhs[0][1], $lhs[1][0]*$rhs[0][2]],
                        [$lhs[2][0]*$rhs[0][0], $lhs[2][0]*$rhs[0][1], $lhs[2][0]*$rhs[0][2]]])
    );
    ([3, 1], [1, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1], $lhs[0][0]*$rhs[0][2], $lhs[0][0]*$rhs[0][3]],
                        [$lhs[1][0]*$rhs[0][0], $lhs[1][0]*$rhs[0][1], $lhs[1][0]*$rhs[0][2], $lhs[1][0]*$rhs[0][3]],
                        [$lhs[2][0]*$rhs[0][0], $lhs[2][0]*$rhs[0][1], $lhs[2][0]*$rhs[0][2], $lhs[2][0]*$rhs[0][3]]])
    );
    ([4, 1], [1, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0]],
                        [$lhs[1][0]*$rhs[0][0]],
                        [$lhs[2][0]*$rhs[0][0]],
                        [$lhs[3][0]*$rhs[0][0]]])
    );
    ([4, 1], [1, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1]],
                        [$lhs[1][0]*$rhs[0][0], $lhs[1][0]*$rhs[0][1]],
                        [$lhs[2][0]*$rhs[0][0], $lhs[2][0]*$rhs[0][1]],
                        [$lhs[3][0]*$rhs[0][0], $lhs[3][0]*$rhs[0][1]]])
    );
    ([4, 1], [1, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1], $lhs[0][0]*$rhs[0][2]],
                        [$lhs[1][0]*$rhs[0][0], $lhs[1][0]*$rhs[0][1], $lhs[1][0]*$rhs[0][2]],
                        [$lhs[2][0]*$rhs[0][0], $lhs[2][0]*$rhs[0][1], $lhs[2][0]*$rhs[0][2]],
                        [$lhs[3][0]*$rhs[0][0], $lhs[3][0]*$rhs[0][1], $lhs[3][0]*$rhs[0][2]]])
    );
    ([4, 1], [1, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0], $lhs[0][0]*$rhs[0][1], $lhs[0][0]*$rhs[0][2], $lhs[0][0]*$rhs[0][3]],
                        [$lhs[1][0]*$rhs[0][0], $lhs[1][0]*$rhs[0][1], $lhs[1][0]*$rhs[0][2], $lhs[1][0]*$rhs[0][3]],
                        [$lhs[2][0]*$rhs[0][0], $lhs[2][0]*$rhs[0][1], $lhs[2][0]*$rhs[0][2], $lhs[2][0]*$rhs[0][3]],
                        [$lhs[3][0]*$rhs[0][0], $lhs[3][0]*$rhs[0][1], $lhs[3][0]*$rhs[0][2], $lhs[3][0]*$rhs[0][3]]])
    );

    ([1, 2], [2, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0]]])
    );
    ([1, 2], [2, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1]]])
    );
    ([1, 2], [2, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2]]])
    );
    ([1, 2], [2, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3]]])
    );
    ([2, 2], [2, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0]]])
    );
    ([2, 2], [2, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1]]])
    );
    ([2, 2], [2, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2]]])
    );
    ([2, 2], [2, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2],
                         $lhs[1][0]*$rhs[0][3] + $lhs[1][1]*$rhs[1][3]]])
    );
    ([3, 2], [2, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0]]])
    );
    ([3, 2], [2, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1]]])
    );
    ([3, 2], [2, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2]]])
    );
    ([3, 2], [2, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2],
                         $lhs[1][0]*$rhs[0][3] + $lhs[1][1]*$rhs[1][3]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2],
                         $lhs[2][0]*$rhs[0][3] + $lhs[2][1]*$rhs[1][3]]])
    );
    ([4, 2], [2, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0]]])
    );
    ([4, 2], [2, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0],
                         $lhs[3][0]*$rhs[0][1] + $lhs[3][1]*$rhs[1][1]]])
    );
    ([4, 2], [2, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0],
                         $lhs[3][0]*$rhs[0][1] + $lhs[3][1]*$rhs[1][1],
                         $lhs[3][0]*$rhs[0][2] + $lhs[3][1]*$rhs[1][2]]])
    );
    ([4, 2], [2, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2],
                         $lhs[1][0]*$rhs[0][3] + $lhs[1][1]*$rhs[1][3]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2],
                         $lhs[2][0]*$rhs[0][3] + $lhs[2][1]*$rhs[1][3]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0],
                         $lhs[3][0]*$rhs[0][1] + $lhs[3][1]*$rhs[1][1],
                         $lhs[3][0]*$rhs[0][2] + $lhs[3][1]*$rhs[1][2],
                         $lhs[3][0]*$rhs[0][3] + $lhs[3][1]*$rhs[1][3]]])
    );

    ([1, 3], [3, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0]]])
    );
    ([1, 3], [3, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1]]])
    );
    ([1, 3], [3, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0]+ $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1]+ $lhs[0][2]*$rhs[2][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2]+ $lhs[0][2]*$rhs[2][2]]])
    );
    ([1, 3], [3, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3] + $lhs[0][2]*$rhs[2][3]]])
    );
    ([2, 3], [3, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0]]])
    );
    ([2, 3], [3, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1]]])
    );
    ([2, 3], [3, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0]+ $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1]+ $lhs[0][2]*$rhs[2][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2]+ $lhs[0][2]*$rhs[2][2]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0]+ $lhs[1][2]*$rhs[2][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1]+ $lhs[1][2]*$rhs[2][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2]+ $lhs[1][2]*$rhs[2][2]]])
    );
    ([2, 3], [3, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3] + $lhs[0][2]*$rhs[2][3]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2] + $lhs[1][2]*$rhs[2][2],
                         $lhs[1][0]*$rhs[0][3] + $lhs[1][1]*$rhs[1][3] + $lhs[1][2]*$rhs[2][3]]])
    );
    ([3, 3], [3, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0]]])
    );
    ([3, 3], [3, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1]]])
    );
    ([3, 3], [3, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2] + $lhs[1][2]*$rhs[2][2]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2] + $lhs[2][2]*$rhs[2][2]]])
    );
    ([3, 3], [3, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3] + $lhs[0][2]*$rhs[2][3]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2] + $lhs[1][2]*$rhs[2][2],
                         $lhs[1][0]*$rhs[0][3] + $lhs[1][1]*$rhs[1][3] + $lhs[1][2]*$rhs[2][3]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2] + $lhs[2][2]*$rhs[2][2],
                         $lhs[2][0]*$rhs[0][3] + $lhs[2][1]*$rhs[1][3] + $lhs[2][2]*$rhs[2][3]]])
    );
    ([4, 3], [3, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0] + $lhs[3][2]*$rhs[2][0]]])
    );
    ([4, 3], [3, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0] + $lhs[3][2]*$rhs[2][0],
                         $lhs[3][0]*$rhs[0][1] + $lhs[3][1]*$rhs[1][1] + $lhs[3][2]*$rhs[2][1]]])
    );
    ([4, 3], [3, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2] + $lhs[1][2]*$rhs[2][2]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2] + $lhs[2][2]*$rhs[2][2]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0] + $lhs[3][2]*$rhs[2][0],
                         $lhs[3][0]*$rhs[0][1] + $lhs[3][1]*$rhs[1][1] + $lhs[3][2]*$rhs[2][1],
                         $lhs[3][0]*$rhs[0][2] + $lhs[3][1]*$rhs[1][2] + $lhs[3][2]*$rhs[2][2]]])
    );
    ([4, 3], [3, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3] + $lhs[0][2]*$rhs[2][3]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2] + $lhs[1][2]*$rhs[2][2],
                         $lhs[1][0]*$rhs[0][3] + $lhs[1][1]*$rhs[1][3] + $lhs[1][2]*$rhs[2][3]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2] + $lhs[2][2]*$rhs[2][2],
                         $lhs[2][0]*$rhs[0][3] + $lhs[2][1]*$rhs[1][3] + $lhs[2][2]*$rhs[2][3]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0] + $lhs[3][2]*$rhs[2][0],
                         $lhs[3][0]*$rhs[0][1] + $lhs[3][1]*$rhs[1][1] + $lhs[3][2]*$rhs[2][1],
                         $lhs[3][0]*$rhs[0][2] + $lhs[3][1]*$rhs[1][2] + $lhs[3][2]*$rhs[2][2],
                         $lhs[3][0]*$rhs[0][3] + $lhs[3][1]*$rhs[1][3] + $lhs[3][2]*$rhs[2][3]]])
    );

    ([1, 4], [4, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0]]])
    );
    ([1, 4], [4, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1]]])
    );
    ([1, 4], [4, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2] +
                            $lhs[0][3]*$rhs[3][2]]])
    );
    ([1, 4], [4, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2] +
                            $lhs[0][3]*$rhs[3][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3] + $lhs[0][2]*$rhs[2][3] +
                            $lhs[0][3]*$rhs[3][3]]])
    );
    ([2, 4], [4, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0]]])
    );
    ([2, 4], [4, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1] +
                            $lhs[1][3]*$rhs[3][1]]])
    );
    ([2, 4], [4, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2] +
                            $lhs[0][3]*$rhs[3][2]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1] +
                            $lhs[1][3]*$rhs[3][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2] + $lhs[1][2]*$rhs[2][2] +
                            $lhs[1][3]*$rhs[3][2]]])
    );
    ([2, 4], [4, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2] +
                            $lhs[0][3]*$rhs[3][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3] + $lhs[0][2]*$rhs[2][3] +
                            $lhs[0][3]*$rhs[3][3]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1] +
                            $lhs[1][3]*$rhs[3][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2] + $lhs[1][2]*$rhs[2][2] +
                            $lhs[1][3]*$rhs[3][2],
                         $lhs[1][0]*$rhs[0][3] + $lhs[1][1]*$rhs[1][3] + $lhs[1][2]*$rhs[2][3] +
                            $lhs[1][3]*$rhs[3][3]]])
    );
    ([3, 4], [4, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0] +
                            $lhs[2][3]*$rhs[3][0]]])
    );
    ([3, 4], [4, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1] +
                            $lhs[1][3]*$rhs[3][1]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0] +
                            $lhs[2][3]*$rhs[3][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1] +
                            $lhs[2][3]*$rhs[3][1]]])
    );
    ([3, 4], [4, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2] +
                            $lhs[0][3]*$rhs[3][2]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1] +
                            $lhs[1][3]*$rhs[3][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2] + $lhs[1][2]*$rhs[2][2] +
                            $lhs[1][3]*$rhs[3][2]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0] +
                            $lhs[2][3]*$rhs[3][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1] +
                            $lhs[2][3]*$rhs[3][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2] + $lhs[2][2]*$rhs[2][2] +
                            $lhs[2][3]*$rhs[3][2]]])
    );
    ([3, 4], [4, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2] +
                            $lhs[0][3]*$rhs[3][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3] + $lhs[0][2]*$rhs[2][3] +
                            $lhs[0][3]*$rhs[3][3]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1] +
                            $lhs[1][3]*$rhs[3][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2] + $lhs[1][2]*$rhs[2][2] +
                            $lhs[1][3]*$rhs[3][2],
                         $lhs[1][0]*$rhs[0][3] + $lhs[1][1]*$rhs[1][3] + $lhs[1][2]*$rhs[2][3] +
                            $lhs[1][3]*$rhs[3][3]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0] +
                            $lhs[2][3]*$rhs[3][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1] +
                            $lhs[2][3]*$rhs[3][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2] + $lhs[2][2]*$rhs[2][2] +
                            $lhs[2][3]*$rhs[3][2],
                         $lhs[2][0]*$rhs[0][3] + $lhs[2][1]*$rhs[1][3] + $lhs[2][2]*$rhs[2][3] +
                            $lhs[2][3]*$rhs[3][3]]])
    );
    ([4, 4], [4, 1] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0] +
                            $lhs[2][3]*$rhs[3][0]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0] + $lhs[3][2]*$rhs[2][0] +
                            $lhs[3][3]*$rhs[3][0]]])
    );
    ([4, 4], [4, 2] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1] +
                            $lhs[1][3]*$rhs[3][1]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0] +
                            $lhs[2][3]*$rhs[3][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1] +
                            $lhs[2][3]*$rhs[3][1]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0] + $lhs[3][2]*$rhs[2][0] +
                            $lhs[3][3]*$rhs[3][0],
                         $lhs[3][0]*$rhs[0][1] + $lhs[3][1]*$rhs[1][1] + $lhs[3][2]*$rhs[2][1] +
                            $lhs[3][3]*$rhs[3][1]]])
    );
    ([4, 4], [4, 3] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2] +
                            $lhs[0][3]*$rhs[3][2]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1] +
                            $lhs[1][3]*$rhs[3][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2] + $lhs[1][2]*$rhs[2][2] +
                            $lhs[1][3]*$rhs[3][2]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0] +
                            $lhs[2][3]*$rhs[3][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1] +
                            $lhs[2][3]*$rhs[3][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2] + $lhs[2][2]*$rhs[2][2] +
                            $lhs[2][3]*$rhs[3][2]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0] + $lhs[3][2]*$rhs[2][0] +
                            $lhs[3][3]*$rhs[3][0],
                         $lhs[3][0]*$rhs[0][1] + $lhs[3][1]*$rhs[1][1] + $lhs[3][2]*$rhs[2][1] +
                            $lhs[3][3]*$rhs[3][1],
                         $lhs[3][0]*$rhs[0][2] + $lhs[3][1]*$rhs[1][2] + $lhs[3][2]*$rhs[2][2] +
                            $lhs[3][3]*$rhs[3][2]]])
    );
    ([4, 4], [4, 4] $lhs:ident, $rhs:ident, $scalar:ty) => (
        ([[$lhs[0][0]*$rhs[0][0] + $lhs[0][1]*$rhs[1][0] + $lhs[0][2]*$rhs[2][0] +
                            $lhs[0][3]*$rhs[3][0],
                         $lhs[0][0]*$rhs[0][1] + $lhs[0][1]*$rhs[1][1] + $lhs[0][2]*$rhs[2][1] +
                            $lhs[0][3]*$rhs[3][1],
                         $lhs[0][0]*$rhs[0][2] + $lhs[0][1]*$rhs[1][2] + $lhs[0][2]*$rhs[2][2] +
                            $lhs[0][3]*$rhs[3][2],
                         $lhs[0][0]*$rhs[0][3] + $lhs[0][1]*$rhs[1][3] + $lhs[0][2]*$rhs[2][3] +
                            $lhs[0][3]*$rhs[3][3]],
                        [$lhs[1][0]*$rhs[0][0] + $lhs[1][1]*$rhs[1][0] + $lhs[1][2]*$rhs[2][0] +
                            $lhs[1][3]*$rhs[3][0],
                         $lhs[1][0]*$rhs[0][1] + $lhs[1][1]*$rhs[1][1] + $lhs[1][2]*$rhs[2][1] +
                            $lhs[1][3]*$rhs[3][1],
                         $lhs[1][0]*$rhs[0][2] + $lhs[1][1]*$rhs[1][2] + $lhs[1][2]*$rhs[2][2] +
                            $lhs[1][3]*$rhs[3][2],
                         $lhs[1][0]*$rhs[0][3] + $lhs[1][1]*$rhs[1][3] + $lhs[1][2]*$rhs[2][3] +
                            $lhs[1][3]*$rhs[3][3]],
                        [$lhs[2][0]*$rhs[0][0] + $lhs[2][1]*$rhs[1][0] + $lhs[2][2]*$rhs[2][0] +
                            $lhs[2][3]*$rhs[3][0],
                         $lhs[2][0]*$rhs[0][1] + $lhs[2][1]*$rhs[1][1] + $lhs[2][2]*$rhs[2][1] +
                            $lhs[2][3]*$rhs[3][1],
                         $lhs[2][0]*$rhs[0][2] + $lhs[2][1]*$rhs[1][2] + $lhs[2][2]*$rhs[2][2] +
                            $lhs[2][3]*$rhs[3][2],
                         $lhs[2][0]*$rhs[0][3] + $lhs[2][1]*$rhs[1][3] + $lhs[2][2]*$rhs[2][3] +
                            $lhs[2][3]*$rhs[3][3]],
                        [$lhs[3][0]*$rhs[0][0] + $lhs[3][1]*$rhs[1][0] + $lhs[3][2]*$rhs[2][0] +
                            $lhs[3][3]*$rhs[3][0],
                         $lhs[3][0]*$rhs[0][1] + $lhs[3][1]*$rhs[1][1] + $lhs[3][2]*$rhs[2][1] +
                            $lhs[3][3]*$rhs[3][1],
                         $lhs[3][0]*$rhs[0][2] + $lhs[3][1]*$rhs[1][2] + $lhs[3][2]*$rhs[2][2] +
                            $lhs[3][3]*$rhs[3][2],
                         $lhs[3][0]*$rhs[0][3] + $lhs[3][1]*$rhs[1][3] + $lhs[3][2]*$rhs[2][3] +
                            $lhs[3][3]*$rhs[3][3]]])
    );

    ([$lrows:tt, $lcols:tt], [$rrows:tt, $rcols:tt] $lhs:ident, $rhs:ident, $scalar:ty) => (
        {
            let mut out = as_expr!([[<$scalar as Zero>::ZERO; $rcols]; $lrows]);

            for (i, row) in out.iter_mut().enumerate() {
                for (j, elem) in row.iter_mut().enumerate() {
                    for (l_ik, r_kj) in $lhs[i].iter()
                                               .zip($rhs.iter().map(|&row| row[j])) {
                        *elem += l_ik * r_kj;
                    }
                }
            }

            out
        }
    );
}

#[macro_export]
macro_rules! _matrices_auto_mul_impls {
    (@outer $scalar:ty: [$($lty:ty => $lrows:tt $lcols:tt)*]
        [$cty:ty => $crows:tt $ccols:tt $($rty:ty => $rrows:tt $rcols:tt)*]) => (
        _matrices_auto_mul_impls! {
            @inner
            $scalar:
            ($cty => $crows $ccols)
            []
            [$($lty => $lrows $lcols)* $cty => $crows $ccols $($rty => $rrows $rcols)*]
        }
        _matrices_auto_mul_impls! {
            @outer
            $scalar:
            [$($lty => $lrows $lcols)* $cty => $crows $ccols]
            [$($rty => $rrows $rcols)*]
        }
    );
    (@outer $scalar:ty: $ignore:tt []) => ();
    (@inner $scalar:ty: ($bty:ty => $brows:tt $bcols:tt)
        [$($done:tt)*]
        [$cty:ty => $crows:tt $ccols:tt $($rest:tt)*]) => (
        is_eq! {
            if ($crows) == ($bcols) {
                impl Mul<$cty> for $bty {
                    type Output = _matrix!($brows, $ccols);

                    #[inline]
                    #[cfg(not(feature = "no_special_cases"))]
                    fn mul(self, rhs: $cty) -> _matrix!($brows, $ccols) {
                        _matrix!($brows, $ccols)(_matrix_mul_impl!([$brows, $bcols], [$crows, $ccols] self, rhs, $scalar))
                    }

                    #[inline]
                    #[cfg(feature = "no_special_cases")]
                    fn mul(self, rhs: $cty) -> _matrix!($brows, $ccols) {
                        let mut out = <_matrix!($brows, $ccols) as Zero>::ZERO;

                        for (i, row) in out.iter_mut().enumerate() {
                            for (j, elem) in row.iter_mut().enumerate() {
                                for (l_ik, r_kj) in self[i].iter()
                                                           .zip(rhs.iter().map(|&row| row[j])) {
                                    *elem += l_ik * r_kj;
                                }
                            }
                        }

                        out
                    }
                }
            } else {}
        }
        _matrices_auto_mul_impls! {
            @inner
            $scalar:
            ($bty => $brows $bcols)
            [$($done)* $cty => $crows $ccols]
            [$($rest)*]
        }
    );
    (@inner $scalar:ty: $ignore:tt $empty:tt []) => ();

    ($scalar:ty: $($bty:ty => $brows:tt $bcols:tt)+) => (
        _matrices_auto_mul_impls! {
            @outer
            $scalar:
            []
            [$($bty => $brows $bcols)+]
        }
    );
}

#[macro_export]
macro_rules! _matrix_det_impl {
    (1, $s:ident, $scalar:ty) => (
        $s[0][0]
    );
    (2, $s:ident, $scalar:ty) => (
        $s[0][0] * $s[1][1] - $s[0][1] * $s[1][0]
    );
    (3, $s:ident, $scalar:ty) => (
        $s[0][0]*$s[1][1]*$s[2][2] + $s[0][1]*$s[1][2]*$s[2][0] + $s[0][2]*$s[1][0]*$s[2][1] -
        $s[0][2]*$s[1][1]*$s[2][0] - $s[0][1]*$s[1][0]*$s[2][2] - $s[0][0]*$s[1][2]*$s[2][1]
    );
    ($order:tt, $s:ident, $scalar:ty) => (
        {
            fn det(data: &Vec<Vec<$scalar>>) -> $scalar {
                if data.len() == 2 {
                    data[0][0] * data[1][1] - data[0][1] * data[1][0]
                } else {
                    let (top, body) = data.split_first().unwrap();
                    let mut minor: Vec<Vec<$scalar>>
                        = body.iter().map(|row| row[1..].to_vec()).collect();
                    let mut sum = top[0] * det(&minor);
                    for i in 1..top.len() {
                        for j in 0..body.len() {
                            minor[j][i-1] = body[j][i-1];
                        }
                        sum += top[i] * det(&minor) * (-((i as isize) & 1) | 1) as f64;
                    }
                    sum
                }
            }
            det(&$s.iter().map(|row| row.to_vec()).collect())
        }
    );
}

#[macro_export]
macro_rules! _matrix_id_impl {
    (1, $matrix:ident, $scalar:ty) => (
        $matrix([[<$scalar as One>::ONE]])
    );
    (2, $matrix:ident, $scalar:ty) => (
        $matrix([[<$scalar as One>::ONE, <$scalar as Zero>::ZERO],
                 [<$scalar as Zero>::ZERO, <$scalar as One>::ONE]])
    );
    (3, $matrix:ident, $scalar:ty) => (
        $matrix([[<$scalar as One>::ONE,
                  <$scalar as Zero>::ZERO,
                  <$scalar as Zero>::ZERO],
                 [<$scalar as Zero>::ZERO,
                  <$scalar as One>::ONE,
                  <$scalar as Zero>::ZERO],
                 [<$scalar as Zero>::ZERO,
                  <$scalar as Zero>::ZERO,
                  <$scalar as One>::ONE]])
    );
    (4, $matrix:ident, $scalar:ty) => (
        $matrix([[<$scalar as One>::ONE,
                  <$scalar as Zero>::ZERO,
                  <$scalar as Zero>::ZERO,
                  <$scalar as Zero>::ZERO],
                 [<$scalar as Zero>::ZERO,
                  <$scalar as One>::ONE,
                  <$scalar as Zero>::ZERO,
                  <$scalar as Zero>::ZERO],
                 [<$scalar as Zero>::ZERO,
                  <$scalar as Zero>::ZERO,
                  <$scalar as One>::ONE,
                  <$scalar as Zero>::ZERO],
                 [<$scalar as Zero>::ZERO,
                  <$scalar as Zero>::ZERO,
                  <$scalar as Zero>::ZERO,
                  <$scalar as One>::ONE]])
    );
    ($order:tt, $matrix:ident, $scalar:ty) => (
        {
            let mut id = <$matrix as Zero>::ZERO;
            for i in 0..$matrix::ROWS {
                id[i][i] = <$scalar as One>::ONE;
            }
            id
        }
    );
}

#[macro_export]
macro_rules! _matrix_transpose_impl {
    (1, 1, $s:ident) => (
        _matrix!(1, 1)([[$s[0][0]]])
    );
    (1, 2, $s:ident) => (
        _matrix!(2, 1)([[$s[0][0]],
                        [$s[0][1]]])
    );
    (1, 3, $s:ident) => (
        _matrix!(3, 1)([[$s[0][0]],
                        [$s[0][1]],
                        [$s[0][2]]])
    );
    (1, 4, $s:ident) => (
        _matrix!(4, 1)([[$s[0][0]],
                        [$s[0][1]],
                        [$s[0][2]],
                        [$s[0][3]]])
    );
    (2, 1, $s:ident) => (
        _matrix!(1, 2)([[$s[0][0], $s[1][0]]])
    );
    (2, 2, $s:ident) => (
        _matrix!(2, 2)(
            [[$s[0][0], $s[1][0]],
             [$s[0][1], $s[1][1]]]
        )
    );
    (2, 3, $s:ident) => (
        _matrix!(3, 2)(
            [[$s[0][0], $s[1][0]],
             [$s[0][1], $s[1][1]],
             [$s[0][2], $s[1][2]]]
        )
    );
    (2, 4, $s:ident) => (
        _matrix!(4, 2)(
            [[$s[0][0], $s[1][0]],
             [$s[0][1], $s[1][1]],
             [$s[0][2], $s[1][2]],
             [$s[0][3], $s[1][3]]]
        )
    );
    (3, 1, $s:ident) => (
        _matrix!(1, 3)(
            [[$s[0][0], $s[1][0], $s[2][0]]]
        )
    );
    (3, 2, $s:ident) => (
        _matrix!(2, 3)(
            [[$s[0][0], $s[1][0], $s[2][0]],
             [$s[0][1], $s[1][1], $s[2][1]]]
        )
    );
    (3, 3, $s:ident) => (
        _matrix!(3, 3)(
            [[$s[0][0], $s[1][0], $s[2][0]],
             [$s[0][1], $s[1][1], $s[2][1]],
             [$s[0][2], $s[1][2], $s[2][2]]]
        )
    );
    (3, 4, $s:ident) => (
        _matrix!(4, 3)(
            [[$s[0][0], $s[1][0], $s[2][0]],
             [$s[0][1], $s[1][1], $s[2][1]],
             [$s[0][2], $s[1][2], $s[2][2]],
             [$s[0][3], $s[1][3], $s[2][3]]]
        )
    );
    (4, 1, $s:ident) => (
        _matrix!(1, 4)(
            [[$s[0][0], $s[1][0], $s[2][0], $s[3][0]]]
        )
    );
    (4, 2, $s:ident) => (
        _matrix!(2, 4)(
            [[$s[0][0], $s[1][0], $s[2][0], $s[3][0]],
             [$s[0][1], $s[1][1], $s[2][1], $s[3][1]]]
        )
    );
    (4, 3, $s:ident) => (
        _matrix!(3, 4)(
            [[$s[0][0], $s[1][0], $s[2][0], $s[3][0]],
             [$s[0][1], $s[1][1], $s[2][1], $s[3][1]],
             [$s[0][2], $s[1][2], $s[2][2], $s[3][2]]]
        )
    );
    (4, 4, $s:ident) => (
        _matrix!(4, 4)(
            [[$s[0][0], $s[1][0], $s[2][0], $s[3][0]],
             [$s[0][1], $s[1][1], $s[2][1], $s[3][1]],
             [$s[0][2], $s[1][2], $s[2][2], $s[3][2]],
             [$s[0][3], $s[1][3], $s[2][3], $s[3][3]]]
        )
    );
    ($rows:tt, $cols:tt, $s:ident) => (
        {
            let mut out = <_matrix!($cols, $rows) as Zero>::ZERO;
            for (i, row) in $s.iter().enumerate() {
                for (j, &elem) in row.iter().enumerate() {
                    out[j][i] = elem;
                }
            }
            out
        }
    );
}

#[macro_export]
macro_rules! _matrix_synonym_space_impl {
    (
        @outer
        $scalar:ty:
        {$($lsyn:path => $ldims:tt)+}
        $rsyns:tt
        $mats:tt
    ) => (
        $(
            _matrix_synonym_space_impl! {
                @middle
                $scalar:
                $lsyn => $ldims
                $rsyns
                $mats
            }
        )+
    );
    (@middle $scalar:ty: $lsyn:path => $ldims:tt {$($rsyn:path => $rdims:tt)+} $mats:tt) => (
        as_items! {
            macro_rules! _matrix_synonym_impl {
                () => (
                    impl From<$lsyn> for _matrix!(1, $ldims) {
                        #[inline]
                        fn from(vec: $lsyn) -> Self {
                            _matrix!(1, $ldims)([vec.into()])
                        }
                    }

                    impl From<_matrix!(1, $ldims)> for $lsyn {
                        #[inline]
                        fn from(mat: _matrix!(1, $ldims)) -> Self {
                            <$lsyn as From<[[$scalar; $ldims]; 1]>>::from(mat.0)
                        }
                    }
                );
            }
            _matrix_synonym_check!(1, $ldims);

            macro_rules! _matrix_synonym_impl {
                () => (
                    impl From<$lsyn> for _matrix!($ldims, 1) {
                        #[inline]
                        fn from(vec: $lsyn) -> Self {
                            _matrix!($ldims, 1)(vec.into())
                        }
                    }

                    impl From<_matrix!($ldims, 1)> for $lsyn {
                        #[inline]
                        fn from(mat: _matrix!($ldims, 1)) -> Self {
                            <$lsyn as From<[[$scalar; 1]; $ldims]>>::from(mat.0)
                        }
                    }
                );
            }
            _matrix_synonym_check!($ldims, 1);

            macro_rules! _matrix_synonym_impl {
                () => (
                    impl From<$lsyn> for _matrix!($ldims, $ldims) {
                        #[inline]
                        fn from(vec: $lsyn) -> Self {
                            <_matrix!($ldims, $ldims) as From<[$scalar; $ldims]>>::from(vec.into())
                        }
                    }
                );
            }
            _matrix_synonym_check!($ldims, $ldims);

            $(
                _matrix_synonym_space_impl! {
                    @inner
                    $scalar:
                    $lsyn => $ldims
                    $rsyn => $rdims
                    $mats
                }
            )+
        }
    );
    (@inner $scalar:ty: $lsyn:path => $ldims:tt $rsyn:path => $rdims:tt { $($matname:ident => $mrows:tt, $mcols:tt)+ }) => (
        $(
            macro_rules! _matrix_synonym_impl {
                () => (
                    n_pairs_are_eq! {
                        if ($ldims) == ($mcols) && ($mrows) == ($rdims) {
                            impl Mul<$lsyn> for _matrix!($mrows, $mcols) {
                                type Output = $rsyn;

                                #[inline]
                                fn mul(self, rhs: $lsyn) -> $rsyn {
                                    let arr: [[$scalar; 1]; $ldims] = rhs.into();
                                    <$rsyn as From<[[$scalar; 1]; $rdims]>>::from(
                                        _matrix_mul_impl!([$mrows, $mcols],
                                            [$ldims, 1] self, arr, $scalar))
                                }
                            }
                        } else {}
                    }

                    n_pairs_are_eq! {
                        if ($ldims) == ($mrows) && ($mcols) == ($rdims) {
                            impl Mul<_matrix!($mrows, $mcols)> for $lsyn {
                                type Output = $rsyn;

                                #[inline]
                                fn mul(self, rhs: _matrix!($mrows, $mcols)) -> $rsyn {
                                    let arr: [[$scalar; $ldims]; 1] = self.into();
                                    <$rsyn as From<[[$scalar; $rdims]; 1]>>::from(
                                        _matrix_mul_impl!([1, $ldims],
                                            [$mrows, $mcols] arr, rhs, $scalar))
                                }
                            }
                        } else {}
                    }
                );
            }
            _matrix_synonym_check!($mrows, $mcols);
        )+
    );
    ($scalar:ty: $mats:tt $($syns:tt)*) => (
        $(
            _matrix_synonym_space_impl!(@outer $scalar: $syns $syns $mats);
        )*
    );
}

macro_rules! _matrix_traitswitch {
    (ApproxEq = !;) => (
        macro_rules! _matrix_approxeq_call { () => (); }
    );
    ($traitswitch:ident = $arg:tt;) => (
        !! "behemoth! matrices! error: unrecognized trait switch: " $traitswitch = $arg;
    );
}

#[macro_export]
macro_rules! matrices {
    (
        $scalar:ty:
        { $($tyname:ident => $rows:tt, $cols:tt)+ }
        $(
            { $($synonym:path => $dims:tt)+ }
        )*
        $(
            trait $traitswitch:ident = $arg:tt;
        )*
    ) => (
        _behemoth_in_wrapper_check!();

        macro_rules! _matrix_approxeq_call {
            () => (_matrix_approxeq_impl!(););
        }

        $(
            _matrix_traitswitch!($traitswitch = $arg;);
        )*

        macro_rules! _matrix {
            $(
                ($rows, $cols) => ($tyname);
            )+
        }

        macro_rules! _matrix_synonym_check {
            $(
                ($rows, $cols) => (_matrix_synonym_impl!(););
            )+
            (1, 1) => ();
            $(
                ($rows, 1) => ();
                (1, $cols) => ();
                ($rows, $rows) => ();
                ($cols, $cols) => ();
            )+
        }

        macro_rules! _matrix_is_defined {
            $(
                ($rows, $cols) => (_matrix_isdef!(true););
            )+
            (1, 1) => (_matrix_isdef!(false););
            $(
                ($rows, 1) => (_matrix_isdef!(false););
                (1, $cols) => (_matrix_isdef!(false););
                ($rows, $rows) => (_matrix_isdef!(false););
                ($cols, $cols) => (_matrix_isdef!(false););
                ($cols, $rows) => (_matrix_isdef!(false););
            )+
        }

        _matrices_auto_mul_impls!($scalar: $($tyname => $rows $cols)+);
        _matrix_synonym_space_impl!($scalar: { $($tyname => $rows, $cols)+ } $({$($synonym => $dims)+})*);

        $(
            as_items! {
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct $tyname (pub [[$scalar; $cols]; $rows]);

                impl $tyname {
                    const ROWS: usize = $rows;
                    const COLS: usize = $cols;

                    #[inline]
                    #[allow(dead_code)]
                    pub fn new(data: [[$scalar; $cols]; $rows]) -> $tyname { $tyname(data) }
                }

                impl Matrix for $tyname {
                    type Scalar = $scalar;

                    #[inline]
                    fn dimensions(&self) -> (usize, usize) { (Self::ROWS, Self::COLS) }
                }

                macro_rules! _matrix_isdef {
                    (true) => (
                        impl Transpose for $tyname {
                            type Transpose = _matrix!($cols, $rows);

                            #[inline]
                            #[cfg(not(feature = "no_special_cases"))]
                            fn transpose(&self) -> Self::Transpose {
                                _matrix_transpose_impl!($rows, $cols, self)
                            }

                            #[inline]
                            #[cfg(feature = "no_special_cases")]
                            fn transpose(&self) -> Self::Transpose {
                                let mut out = <Self::Transpose as Zero>::ZERO;
                                for (i, row) in self.iter().enumerate() {
                                    for (j, &elem) in row.iter().enumerate() {
                                        out[j][i] = elem;
                                    }
                                }
                                out
                            }
                        }
                    );
                    (false) => ();
                }
                _matrix_is_defined!($cols, $rows);

                impl Deref for $tyname {
                    type Target = [[$scalar; $cols]; $rows];

                    fn deref(&self) -> &[[$scalar; $cols]; $rows] { &self.0 }
                }

                impl DerefMut for $tyname {
                    fn deref_mut(&mut self) -> &mut [[$scalar; $cols]; $rows] { &mut self.0 }
                }

                // FIXME: Special case for matrices nxm with n <= 4, m <= 4
                impl Add for $tyname {
                    type Output = Self;

                    #[inline]
                    fn add(self, rhs: $tyname) -> $tyname {
                        let mut out = self;
                        for (i, row) in rhs.iter().enumerate() {
                            for (j, &elem) in row.iter().enumerate() {
                                out[i][j] += elem;
                            }
                        }
                        out
                    }
                }

                // FIXME: Special case for matrices nxm with n <= 4, m <= 4
                impl AddAssign for $tyname {
                    #[inline]
                    fn add_assign(&mut self, rhs: $tyname) {
                        for (i, row) in rhs.iter().enumerate() {
                            for (j, &elem) in row.iter().enumerate() {
                                self[i][j] += elem;
                            }
                        }
                    }
                }

                // FIXME: Special case for matrices nxm with n <= 4, m <= 4
                impl Sub for $tyname {
                    type Output = Self;

                    #[inline]
                    fn sub(self, rhs: $tyname) -> $tyname {
                        let mut out = self;
                        for (i, row) in rhs.iter().enumerate() {
                            for (j, &elem) in row.iter().enumerate() {
                                out[i][j] -= elem;
                            }
                        }
                        out
                    }
                }

                // FIXME: Special case for matrices nxm with n <= 4, m <= 4
                impl SubAssign for $tyname {
                    #[inline]
                    fn sub_assign(&mut self, rhs: $tyname) {
                        for (i, row) in rhs.iter().enumerate() {
                            for (j, &elem) in row.iter().enumerate() {
                                self[i][j] -= elem;
                            }
                        }
                    }
                }

                // FIXME: Special case for matrices nxm with n <= 4, m <= 4
                impl Mul<$scalar> for $tyname {
                    type Output = Self;

                    #[inline]
                    fn mul(self, rhs: $scalar) -> $tyname {
                        let mut out = self;
                        for row in out.iter_mut() {
                            for elem in row.iter_mut() {
                                *elem *= rhs;
                            }
                        }
                        out
                    }
                }

                // FIXME: Special case for matrices nxm with n <= 4, m <= 4
                impl Mul<$tyname> for $scalar {
                    type Output = $tyname;

                    #[inline]
                    fn mul(self, rhs: $tyname) -> $tyname {
                        let mut out = rhs;
                        for row in out.iter_mut() {
                            for elem in row.iter_mut() {
                                *elem *= self;
                            }
                        }
                        out
                    }
                }

                // FIXME: Special case for matrices nxm with n <= 4, m <= 4
                impl MulAssign<$scalar> for $tyname {
                    #[inline]
                    fn mul_assign(&mut self, rhs: $scalar) {
                        for row in self.iter_mut() {
                            for elem in row.iter_mut() {
                                *elem *= rhs;
                            }
                        }
                    }
                }

                // FIXME: Special case for matrices nxm with n <= 4, m <= 4
                impl Div<$scalar> for $tyname {
                    type Output = Self;

                    #[inline]
                    fn div(self, rhs: $scalar) -> $tyname {
                        let mut out = self;
                        for row in out.iter_mut() {
                            for elem in row.iter_mut() {
                                *elem /= rhs;
                            }
                        }
                        out
                    }
                }

                // FIXME: Special case for matrices nxm with n <= 4, m <= 4
                impl DivAssign<$scalar> for $tyname {
                    #[inline]
                    fn div_assign(&mut self, rhs: $scalar) {
                        for row in self.iter_mut() {
                            for elem in row.iter_mut() {
                                *elem /= rhs;
                            }
                        }
                    }
                }

                // FIXME: Special case for matrices nxm with n <= 4, m <= 4
                impl Neg for $tyname {
                    type Output = Self;

                    #[inline]
                    fn neg(self) -> $tyname {
                        let mut out = self;
                        for row in out.iter_mut() {
                            for elem in row.iter_mut() {
                                *elem = -(*elem);
                            }
                        }
                        out
                    }
                }

                impl Zero for $tyname {
                    const ZERO: $tyname = $tyname(
                        [[<$scalar as Zero>::ZERO; $cols]; $rows]
                    );
                }
            }

            macro_rules! _matrix_approxeq_impl {
                () => (
                    impl ApproxEq for $tyname {
                        fn approx_eq(&self, other: &$tyname) -> bool {
                            for (lrow, rrow) in self.iter().zip(other.iter()) {
                                for (lelem, relem) in lrow.iter().zip(rrow.iter()) {
                                    if lelem.approx_ne(&relem) {
                                        return false;
                                    }
                                }
                            }
                            true
                        }
                    }
                );
            }
            _matrix_approxeq_call!();

            is_eq! {
                if ($rows) == ($cols) {
                    impl Square for $tyname {
                        #[inline]
                        #[cfg(not(feature = "no_special_cases"))]
                        fn identity() -> $tyname {
                            _matrix_id_impl!($rows, $tyname, $scalar)
                        }

                        #[inline]
                        #[cfg(feature = "no_special_cases")]
                        fn identity() -> $tyname {
                            let mut id = <$tyname as Zero>::ZERO;
                            for i in 0..$tyname::ROWS {
                                id[i][i] = <$scalar as One>::ONE;
                            }
                            id
                        }

                        #[inline]
                        #[cfg(not(feature = "no_special_cases"))]
                        fn determinant(&self) -> $scalar {
                            _matrix_det_impl!($rows, self, $scalar)
                        }

                        #[inline]
                        #[cfg(feature = "no_special_cases")]
                        fn determinant(&self) -> $scalar {
                            fn det(data: &Vec<Vec<$scalar>>) -> $scalar {
                                if data.len() == 2 {
                                    data[0][0] * data[1][1] - data[0][1] * data[1][0]
                                } else {
                                    let (top, body) = data.split_first().unwrap();
                                    let mut minor: Vec<Vec<$scalar>>
                                        = body.iter().map(|row| row[1..].to_vec()).collect();
                                    let mut sum = top[0] * det(&minor);
                                    for i in 1..top.len() {
                                        for j in 0..body.len() {
                                            minor[j][i-1] = body[j][i-1];
                                        }
                                        sum += top[i] * det(&minor) * (-((i as isize) & 1) | 1) as f64;
                                    }
                                    sum
                                }
                            }
                            det(&self.iter().map(|row| row.to_vec()).collect())
                        }

                        #[inline]
                        fn transpose_mut(&mut self) {
                            *self = self.transpose();
                        }
                    }

                    impl From<[$scalar; $rows]> for $tyname {
                        #[inline]
                        fn from(diags: [$scalar; $rows]) -> Self {
                            let mut empty = $tyname::ZERO;
                            for i in 0..$rows {
                                empty[i][i] = diags[i];
                            }
                            empty
                        }
                    }

                    impl MulAssign for $tyname {
                        #[inline]
                        fn mul_assign(&mut self, rhs: $tyname) {
                            *self = *self * rhs;
                        }
                    }
                } else {}
            }
        )+
    );
}
