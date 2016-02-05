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
macro_rules! matrices {
    ($scalar:ty: { $($tyname:ident => $rows:tt, $cols:tt)+ } $($synonym:path => $dims:tt)*) => (
        _behemoth_in_wrapper_check!();

        _use_Matrix!();
        _use_One!();
        _use_Zero!();

        _use_Add!();
        _use_AddAssign!();
        _use_Sub!();
        _use_SubAssign!();
        _use_Mul!();
        _use_MulAssign!();
        _use_Div!();
        _use_DivAssign!();
        _use_Neg!();
        _use_Deref!();
        _use_DerefMut!();

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

        _matrices_auto_mul_impls!($scalar: $($tyname => $rows $cols)+);

        $(
            as_items! {
                macro_rules! _matrix_synonym_impl {
                    () => (
                        log_syntax!("Generating From: " $synonym -> (1, $dims));

                        impl From<$synonym> for _matrix!(1, $dims) {
                            #[inline]
                            fn from(vec: $synonym) -> Self {
                                _matrix!(1, $dims)([vec.into()])
                            }
                        }

                        impl From<_matrix!(1, $dims)> for $synonym {
                            #[inline]
                            fn from(mat: _matrix!(1, $dims)) -> Self {
                                $synonym::from(mat.0);
                            }
                        }
                    );
                }
                _matrix_synonym_check!(1, $dims);

                macro_rules! _matrix_synonym_impl {
                    () => (
                        log_syntax!("Generating From: " $synonym -> ($dims, 1));

                        impl From<$synonym> for _matrix!($dims, 1) {
                            #[inline]
                            fn from(vec: $synonym) -> Self {
                                _matrix!($dims, 1)(vec.into())
                            }
                        }

                        impl From<_matrix!($dims, 1)> for $synonym {
                            #[inline]
                            fn from(mat: _matrix!($dims, 1)) -> Self {
                                $synonym::from(mat.0);
                            }
                        }
                    );
                }
                _matrix_synonym_check!($dims, 1);

                macro_rules! _matrix_synonym_impl {
                    () => (
                        log_syntax!("Generating From: " $synonym -> ($dims, $dims));

                        impl From<$synonym> for _matrix!($dims, $dims) {
                            #[inline]
                            fn from(vec: $synonym) -> Self {
                                <_matrix!($dims, $dims) as From<[$scalar; $dims]>>::from(vec.into())
                            }
                        }
                    );
                }
                _matrix_synonym_check!($dims, $dims);

                macro_rules! _matrix_synonym_impl {
                    () => (
                        macro_rules! _matrix_synonym_impl {
                            () => (
                                macro_rules! _matrix_synonym_impl {
                                    () => (
                                        impl Mul<$synonym> for _matrix!($dims, 1) {
                                            type Output = _matrix!($dims, $dims);

                                            #[inline]
                                            fn mul(self, rhs: $synonym) -> _matrix!($dims, $dims) {
                                                self * <_matrix!(1, $dims) as From<$synonym>>::from(rhs)
                                            }
                                        }

                                        impl Mul<_matrix!(1, $dims)> for $synonym {
                                            type Output = _matrix!($dims, $dims);

                                            #[inline]
                                            fn mul(self, rhs: _matrix!(1, $dims)) -> _matrix!($dims, $dims) {
                                                <_matrix!($dims, 1) as From<$synonym>>::from(self) * rhs
                                            }
                                        }
                                    );
                                }
                                _matrix_synonym_check!(1, $dims);
                            );
                        }
                        _matrix_synonym_check!($dims, 1);

                        log_syntax!("Generating Mul: " $synonym * [$dims, $dims]);

                        impl Mul<_matrix!($dims, $dims)> for $synonym {
                            type Output = $synonym;

                            #[inline]
                            fn mul(self, rhs: _matrix!($dims, $dims)) -> $synonym {
                                let arr = <Self as Into<[[$scalar; $dims]; 1]>>::into(self);
                                <Self as From<[[$scalar; $dims]; 1]>>::from(
                                    _matrix_mul_impl!([1, $dims], [$dims, $dims] arr, rhs, $scalar)
                                )
                            }
                        }

                        log_syntax!("Generating Mul: " [$dims, $dims] * $synonym);

                        impl Mul<$synonym> for _matrix!($dims, $dims) {
                            type Output = $synonym;

                            #[inline]
                            fn mul(self, rhs: $synonym) -> $synonym {
                                let arr = <$synonym as Into<[[$scalar; 1]; $dims]>>::into(rhs);
                                <$synonym as From<[[$scalar; 1]; $dims]>>::from(
                                    _matrix_mul_impl!([$dims, $dims], [$dims, 1] self, arr, $scalar)
                                )
                            }
                        }
                    );
                }
                _matrix_synonym_check!($dims, $dims);

                macro_rules! _matrix_synonym_impl {
                    () => (
                        macro_rules! _matrix_synonym_impl {
                            () => (
                                macro_rules! _matrix_synonym_impl {
                                    () => (
                                        impl Mul<_matrix!($dims, 1)> for $synonym {
                                            type Output = _matrix!(1, 1);

                                            #[inline]
                                            fn mul(self, rhs: _matrix!($dims, 1)) -> _matrix!(1, 1) {
                                                <_matrix!(1, $dims) as From<$synonym>>::from(self) * rhs
                                            }
                                        }

                                        impl Mul<$synonym> for _matrix!(1, $dims) {
                                            type Output = _matrix!(1, 1);

                                            #[inline]
                                            fn mul(self, rhs: $synonym) -> _matrix!(1, 1) {
                                                self * <_matrix!($dims, 1) as From<$synonym>>::from(rhs)
                                            }
                                        }
                                    );
                                }
                                _matrix_synonym_check!(1, $dims);
                            );
                        }
                        _matrix_synonym_check!($dims, 1);
                    );
                }
                _matrix_synonym_check!(1, 1);
            }
        )*

        $(
            as_items! {
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct $tyname (pub [[$scalar; $cols]; $rows]);

                impl $tyname {
                    const ROWS: usize = $rows;
                    const COLS: usize = $cols;
                }

                impl Matrix for $tyname {
                    type Scalar = $scalar;
                    type Transpose = _matrix!($cols, $rows);

                    #[inline]
                    fn dimensions(&self) -> (usize, usize) { (Self::ROWS, Self::COLS) }

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

            is_eq! {
                if ($rows) == ($cols) {
                    _use_Square!();

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
