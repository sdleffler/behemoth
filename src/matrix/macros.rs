#[macro_export]
macro_rules! _matrix_mul_impl {
    ([$lrows:expr => $mdims:expr => $rcols:expr] $lhs:ident, $rhs:ident, $scalar:ty) => (
        unsafe {
            let mut out = [[<$scalar as Zero>::ZERO; $rcols]; $lrows];
            for i in 0..$lrows {
                for j in 0..$rcols {
                    *out.get_unchecked_mut(i)
                        .get_unchecked_mut(j) = (1..$mdims).fold(
                            $lhs.get_unchecked(i)
                                .get_unchecked(0) *
                            $rhs.get_unchecked(0)
                                .get_unchecked(j),
                            |acc, k| $lhs.get_unchecked(i)
                                         .get_unchecked(k) *
                                     $rhs.get_unchecked(k)
                                         .get_unchecked(j) + acc);
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
                    type Output = DenseMatrix<[[$scalar; $ccols]; $brows], [$scalar; $ccols], $scalar>;

                    #[inline]
                    fn mul(self, rhs: $cty) -> Self::Output {
                        Self::Output::new(
                            _matrix_mul_impl!([$brows => $bcols => $ccols] self, rhs, $scalar))
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
        $s.0[0][0]
    );
    (2, $s:ident, $scalar:ty) => (
        $s.0[0][0] * $s.0[1][1] - $s.0[0][1] * $s.0[1][0]
    );
    (3, $s:ident, $scalar:ty) => (
        $s.0[0][0]*$s.0[1][1]*$s.0[2][2] + $s.0[0][1]*$s.0[1][2]*$s.0[2][0] +
            $s.0[0][2]*$s.0[1][0]*$s.0[2][1] -
        $s.0[0][2]*$s.0[1][1]*$s.0[2][0] - $s.0[0][1]*$s.0[1][0]*$s.0[2][2] -
            $s.0[0][0]*$s.0[1][2]*$s.0[2][1]
    );
    (4, $s:ident, $scalar:ty) => (
        $s.0[0][3]*$s.0[1][2]*$s.0[2][1]*$s.0[3][0] - $s.0[0][2]*$s.0[1][3]*$s.0[2][1]*$s.0[3][0] -
        $s.0[0][3]*$s.0[1][1]*$s.0[2][2]*$s.0[3][0] + $s.0[0][1]*$s.0[1][3]*$s.0[2][2]*$s.0[3][0] +
        $s.0[0][2]*$s.0[1][1]*$s.0[2][3]*$s.0[3][0] - $s.0[0][1]*$s.0[1][2]*$s.0[2][3]*$s.0[3][0] -
        $s.0[0][3]*$s.0[1][2]*$s.0[2][0]*$s.0[3][1] + $s.0[0][2]*$s.0[1][3]*$s.0[2][0]*$s.0[3][1] +
        $s.0[0][3]*$s.0[1][0]*$s.0[2][2]*$s.0[3][1] - $s.0[0][0]*$s.0[1][3]*$s.0[2][2]*$s.0[3][1] -
        $s.0[0][2]*$s.0[1][0]*$s.0[2][3]*$s.0[3][1] + $s.0[0][0]*$s.0[1][2]*$s.0[2][3]*$s.0[3][1] +
        $s.0[0][3]*$s.0[1][1]*$s.0[2][0]*$s.0[3][2] - $s.0[0][1]*$s.0[1][3]*$s.0[2][0]*$s.0[3][2] -
        $s.0[0][3]*$s.0[1][0]*$s.0[2][1]*$s.0[3][2] + $s.0[0][0]*$s.0[1][3]*$s.0[2][1]*$s.0[3][2] +
        $s.0[0][1]*$s.0[1][0]*$s.0[2][3]*$s.0[3][2] - $s.0[0][0]*$s.0[1][1]*$s.0[2][3]*$s.0[3][2] -
        $s.0[0][2]*$s.0[1][1]*$s.0[2][0]*$s.0[3][3] + $s.0[0][1]*$s.0[1][2]*$s.0[2][0]*$s.0[3][3] +
        $s.0[0][2]*$s.0[1][0]*$s.0[2][1]*$s.0[3][3] - $s.0[0][0]*$s.0[1][2]*$s.0[2][1]*$s.0[3][3] -
        $s.0[0][1]*$s.0[1][0]*$s.0[2][2]*$s.0[3][3] + $s.0[0][0]*$s.0[1][1]*$s.0[2][2]*$s.0[3][3]
    );
    ($order:tt, $s:ident, $scalar:ty) => (
        {
            // FIXME: This is absolutely nasty and horrible. Need a better general case determinant
            // implementation.
            fn det(data: &Vec<Vec<$scalar>>) -> $scalar {
                match data.len() {
                    1 => data[0][0],
                    2 => data[0][0] * data[1][1] - data[0][1] * data[1][0],
                    _ => {
                        let (top, body) = data.split_first().unwrap();
                        let mut minor: Vec<Vec<$scalar>>
                            = body.iter().map(|row| row[1..].to_vec()).collect();
                        let mut sum = top[0] * det(&minor);
                        for i in 1..top.len() {
                            for j in 0..body.len() {
                                minor[j][i-1] = body[j][i-1];
                            }
                            sum += top[i] * det(&minor) * if i % 2 == 1 { -1. } else { 1. };
                        }
                        sum
                    }
                }
            }
            det(&$s.iter().map(|row| row.to_vec()).collect())
        }
    );
}

#[macro_export]
macro_rules! _matrix_id_impl {
    (1, $matrix:ident, $scalar:ty) => (
        $matrix::new([[<$scalar as One>::ONE]])
    );
    (2, $matrix:ident, $scalar:ty) => (
        $matrix::new([[<$scalar as One>::ONE, <$scalar as Zero>::ZERO],
                 [<$scalar as Zero>::ZERO, <$scalar as One>::ONE]])
    );
    (3, $matrix:ident, $scalar:ty) => (
        $matrix::new([[<$scalar as One>::ONE,
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
        $matrix::new([[<$scalar as One>::ONE,
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
                id.0[i][i] = <$scalar as One>::ONE;
            }
            id
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
                    impl From<$lsyn> for DenseMatrix<[[$scalar; $ldims]; 1], [$scalar; $ldims], $scalar> {
                        #[inline]
                        fn from(vec: $lsyn) -> Self {
                            Self::new([vec.into()])
                        }
                    }

                    impl From<DenseMatrix<[[$scalar; $ldims]; 1], [$scalar; $ldims], $scalar>> for $lsyn {
                        #[inline]
                        fn from(mat: DenseMatrix<[[$scalar; $ldims]; 1], [$scalar; $ldims], $scalar>) -> Self {
                            <$lsyn as From<[[$scalar; $ldims]; 1]>>::from(mat.0)
                        }
                    }
                );
            }
            _matrix_synonym_check!(1, $ldims);

            macro_rules! _matrix_synonym_impl {
                () => (
                    impl From<$lsyn> for DenseMatrix<[[$scalar; 1]; $ldims], [$scalar; 1], $scalar> {
                        #[inline]
                        fn from(vec: $lsyn) -> Self {
                            Self::new(vec.into())
                        }
                    }

                    impl From<DenseMatrix<[[$scalar; 1]; $ldims], [$scalar; 1], $scalar>> for $lsyn {
                        #[inline]
                        fn from(mat: DenseMatrix<[[$scalar; 1]; $ldims], [$scalar; 1], $scalar>) -> Self {
                            <$lsyn as From<[[$scalar; 1]; $ldims]>>::from(mat.0)
                        }
                    }
                );
            }
            _matrix_synonym_check!($ldims, 1);

            macro_rules! _matrix_synonym_impl {
                () => (
                    impl From<$lsyn> for DenseMatrix<[[$scalar; $ldims]; $ldims], [$scalar; $ldims], $scalar> {
                        #[inline]
                        fn from(vec: $lsyn) -> Self {
                            <Self as From<[$scalar; $ldims]>>::from(vec.into())
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
                            impl Mul<$lsyn> for DenseMatrix<[[$scalar; $mcols]; $mrows], [$scalar; $mcols], $scalar> {
                                type Output = $rsyn;

                                #[inline]
                                fn mul(self, rhs: $lsyn) -> $rsyn {
                                    let arr: [[$scalar; 1]; $ldims] = rhs.into();
                                    <$rsyn as From<[[$scalar; 1]; $rdims]>>::from(
                                        _matrix_mul_impl!([$mrows => $mcols => 1]
                                            self, arr, $scalar))
                                }
                            }
                        } else {}
                    }

                    n_pairs_are_eq! {
                        if ($ldims) == ($mrows) && ($mcols) == ($rdims) {
                            impl Mul<DenseMatrix<[[$scalar; $mcols]; $mrows], [$scalar; $mcols], $scalar>> for $lsyn {
                                type Output = $rsyn;

                                #[inline]
                                fn mul(self, rhs: DenseMatrix<[[$scalar; $mcols]; $mrows], [$scalar; $mcols], $scalar>) -> $rsyn {
                                    let arr: [[$scalar; $ldims]; 1] = self.into();
                                    <$rsyn as From<[[$scalar; $rdims]; 1]>>::from(
                                        _matrix_mul_impl!([1 => $mrows => $mcols]
                                            arr, rhs, $scalar))
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
        !! behemoth! matrices! error: unrecognized trait switch: $traitswitch = $arg;
    );
}

#[macro_export]
macro_rules! _matrix_undefined_error {
    ($rows:tt, $cols:tt) => (
        "Error: Matrix undefined. Perhaps you forgot to request it?"
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
        _matrix_synonym_space_impl!($scalar: { $($tyname => $rows, $cols)+ }
            $({$($synonym => $dims)+})*);

        $(
            as_items! {
                pub type $tyname = DenseMatrix<[[$scalar; $cols]; $rows], [$scalar; $cols], $scalar>;

                impl $tyname {
                    const ROWS: usize = $rows;
                    const COLS: usize = $cols;
                }

                impl AsMathematica for $tyname {
                    fn as_mathematica(&self) -> String {
                        fn push_row(string: &mut String, row: &[$scalar]) {
                            let mut elems = row.iter();
                            string.push_str(&format!("{{ {:?}", elems.next().unwrap()));
                            for e in elems {
                                string.push_str(&format!(", {:?}", e));
                            }
                            string.push_str(" }");
                        }

                        let mut string = String::from("{");
                        let mut rows = self.iter();
                        push_row(&mut string, rows.next().unwrap());
                        for row in rows {
                            string.push_str(", ");
                            push_row(&mut string, row);
                        }
                        string.push_str("}");
                        string
                    }
                }

                impl Matrix for $tyname {
                    type Scalar = $scalar;

                    #[inline]
                    fn dimensions(&self) -> (usize, usize) { (Self::ROWS, Self::COLS) }

                    #[inline]
                    fn row_switch(mut self, i: usize, j: usize) -> Self {
                        self.0.swap(i, j);
                        self
                    }

                    #[inline]
                    fn row_mul(mut self, i: usize, m: Self::Scalar) -> Self {
                        for e in self.0[i].iter_mut() {
                            *e *= m;
                        }
                        self
                    }

                    #[inline]
                    fn row_add(mut self, i:usize, j: usize, m: Self::Scalar) -> Self {
                        for k in 0..Self::COLS {
                            self.0[i][k] += self.0[j][k] * m;
                        }
                        self
                    }

                    #[inline]
                    fn col_switch(mut self, i: usize, j: usize) -> Self {
                        for row in self.iter_mut() {
                            row.swap(i, j);
                        }
                        self
                    }

                    #[inline]
                    fn col_mul(mut self, i: usize, m: Self::Scalar) -> Self {
                        for e in self.iter_mut() {
                            e[i] *= m;
                        }
                        self
                    }

                    #[inline]
                    fn col_add(mut self, i:usize, j: usize, m: Self::Scalar) -> Self {
                        for row in self.iter_mut() {
                            row[i] += row[j] * m;
                        }
                        self
                    }

                    #[inline]
                    fn row_switch_mut(&mut self, i: usize, j: usize) {
                        self.0.swap(i, j);
                    }

                    #[inline]
                    fn row_mul_mut(&mut self, i: usize, m: Self::Scalar) {
                        for e in self.0[i].iter_mut() {
                            *e *= m;
                        }
                    }

                    #[inline]
                    fn row_add_mut(&mut self,  i: usize, j: usize, m: Self::Scalar) {
                        for k in 0..Self::COLS {
                            self.0[i][k] += self.0[j][k] * m;
                        }
                    }

                    #[inline]
                    fn col_switch_mut(&mut self, i: usize, j: usize) {
                        for row in self.iter_mut() {
                            row.swap(i, j);
                        }
                    }

                    #[inline]
                    fn col_mul_mut(&mut self, i: usize, m: Self::Scalar) {
                        for e in self.iter_mut() {
                            e[i] *= m;
                        }
                    }

                    #[inline]
                    fn col_add_mut(&mut self, i:usize, j: usize, m: Self::Scalar) {
                        for row in self.iter_mut() {
                            row[i] += row[j] * m;
                        }
                    }
                }

                macro_rules! _matrix_isdef {
                    (true) => (
                        impl Transpose for $tyname {
                            type Transpose = DenseMatrix<[[$scalar; $rows]; $cols], [$scalar; $rows], $scalar>;

                            #[inline]
                            fn transpose(&self) -> Self::Transpose {
                                let mut out = <Self::Transpose as Zero>::ZERO;
                                for (i, row) in self.iter().enumerate() {
                                    for (j, elem) in row.iter().enumerate() {
                                        out.0[j][i] = *elem;
                                    }
                                }
                                out
                            }
                        }
                    );
                    (false) => ();
                }
                _matrix_is_defined!($cols, $rows);

                // FIXME: Special case for matrices nxm with n <= 4, m <= 4
                impl Add for $tyname {
                    type Output = Self;

                    #[inline]
                    fn add(self, rhs: $tyname) -> $tyname {
                        let mut out = self;
                        for (i, row) in rhs.iter().enumerate() {
                            for (j, &elem) in row.iter().enumerate() {
                                out.0[i][j] += elem;
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
                                self.0[i][j] += elem;
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
                                out.0[i][j] -= elem;
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
                                self.0[i][j] -= elem;
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
                    const ZERO: $tyname =
                        DenseMatrix::<[[$scalar; $cols]; $rows], [$scalar; $cols], $scalar>(
                            [[<$scalar as Zero>::ZERO; $cols]; $rows], PhantomData);
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
                        fn order(&self) -> usize {
                            $rows
                        }

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
                                id.0[i][i] = <$scalar as One>::ONE;
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
                                match data.len() {
                                    1 => data[0][0],
                                    2 => data[0][0] * data[1][1] - data[0][1] * data[1][0],
                                    _ => {
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
                                empty.0[i][i] = diags[i];
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
