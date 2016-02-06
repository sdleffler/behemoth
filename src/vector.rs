#[macro_export]
macro_rules! _vector_binop_impl {
    ($name:ident $optrait:ident { $($it:tt)* }) => (
        as_items!(impl $optrait for $name {
            type Output = $name;

            $($it)*
        });
    );
    ($name:ident $optrait:ident $opfunc:ident { $($vn:ident: $vt:ty),* } $binop:tt) => (
        impl $optrait for $name {
            type Output = $name;

            fn $opfunc (self, other: $name) -> $name {
                $name { $($vn: as_expr!(self.$vn $binop other.$vn)),* }
            }
        }
    );
    ($name:ident $optrait:ident $opfunc:ident ( $vt:ty ) $binop:tt) => (
        impl $optrait for $name {
            type Output = $name;

            fn $opfunc (self, other: $name) -> $name {
                $name ( as_expr!(self.0 $binop other.0) )
            }
        }
    );
}

#[macro_export]
macro_rules! _vector_assign_binop_impl {
    ($name:ident $optrait:ident { $($it:tt)* }) => (
        as_items! {
            impl $optrait for $name {
                $($it)*
            }
        }
    );
    ($name:ident $optrait:ident $opfunc:ident { $($vn:ident: $vt:ty),* } $binop:tt) => (
        as_items! {
            impl $optrait for $name {
                fn $opfunc (&mut self, other: $name) {
                    $(self.$vn $binop other.$vn;)*
                }
            }
        }
    );
    ($name:ident $optrait:ident $opfunc:ident ( $vt:ty ) $binop:tt) => (
        as_items! {
            impl $optrait for $name {
                fn $opfunc (&mut self, other: $name) {
                    self.0 $binop other.0;
                }
            }
        }
    );
}

#[macro_export]
macro_rules! _vector_scalar_binop_impl {
    (@commutative $name:ident $scalar:ty, $optrait:ident $opfunc:ident { $($vn:ident: $vt:ty),* } $binop:tt) => (
        impl $optrait<$name> for $scalar {
            type Output = $name;

            fn $opfunc (self, other: $name) -> $name {
                $name { $($vn: as_expr!(self * other.$vn)),* }
            }
        }
    );
    (@commutative $name:ident $scalar:ty, $optrait:ident $opfunc:ident ( $vt:ty ) $binop:tt) => (
        impl $optrait<$name> for $scalar {
            type Output = $name;

            fn $opfunc (self, other: $name) -> $name {
                $name ( as_expr!(self * other.0) )
            }
        }
    );
    (@commutative $name:ident $scalar:ty, $optrait:ident { $($it:tt)* }) => (
        as_items!(
            impl $optrait<$name> for $scalar {
                type Output = $name;

                $($it)*
            }
        );
    );
    ($name:ident $scalar:ty, $optrait:ident $opfunc:ident { $($vn:ident: $vt:ty),* } $binop:tt) => (
        impl $optrait<$scalar> for $name {
            type Output = $name;

            fn $opfunc (self, other: $scalar) -> $name {
                $name { $($vn: as_expr!(self.$vn $binop other)),* }
            }
        }
    );
    ($name:ident $scalar:ty, $optrait:ident $opfunc:ident ( $vt:ty ) $binop:tt) => (
        impl $optrait<$scalar> for $name {
            type Output = $name;

            fn $opfunc (self, other: $scalar) -> $name {
                $name ( as_expr!(self.0 $binop other) )
            }
        }
    );
    ($name:ident $scalar:ty, $optrait:ident { $($it:tt)* }) => (
        as_items!(
            impl $optrait<$scalar> for $name {
                type Output = $name;

                $($it)*
            }
        );
    );
}

#[macro_export]
macro_rules! _vector_scalar_assign_binop_impl {
    ($name:ident $scalar:ty, $optrait:ident $opfunc:ident { $($vn:ident: $vt:ty),* } $binop:tt) => (
        as_items! {
            impl $optrait<$scalar> for $name {
                fn $opfunc (&mut self, other: $scalar) {
                    $(self.$vn $binop other;)*
                }
            }
        }
    );
    ($name:ident $scalar:ty, $optrait:ident $opfunc:ident ( $vt:ty ) $binop:tt) => (
        as_items! {
            impl $optrait<$scalar> for $name {
                fn $opfunc (&mut self, other: $scalar) {
                    self.0 $binop other;
                }
            }
        }
    );
    ($name:ident $scalar:ty, $optrait:ident { $($it:tt)* }) => (
        as_items! {
            impl $optrait<$scalar> for $name {
                $($it)*
            }
        }
    );
}

#[macro_export]
macro_rules! _vector_unop_impl {
    ($name:ident $optrait:ident $opfunc:ident { $($vn:ident: $vt:ty),* } $unop:tt) => (
        impl $optrait for $name {
            type Output = $name;

            fn $opfunc (self) -> $name {
                $name { $($vn: as_expr!($unop self.$vn)),* }
            }
        }
    );
    ($name:ident $optrait:ident $opfunc:ident ( $vt:ty ) $unop:tt) => (
        impl $optrait for $name {
            type Output = $name;

            fn $opfunc (self) -> $name {
                $name ( as_expr!($unop self.0) )
            }
        }
    );
    ($name:ident $optrait:ident { $($it:tt)* }) => (
        as_items!(
            impl $optrait for $name {
                type Output = $name;

                $($it)*
            }
        );
    );
}

#[macro_export]
macro_rules! _vector_zero_impl {
    ($name:ident { $($vn:ident: $vt:ty),* }) => (
        impl Zero for $name {
            const ZERO: $name = $name { $($vn: <$vt as Zero>::ZERO),* };
        }
    );
    ($name:ident ( $vt:ty )) => (
        impl Zero for $name {
            const ZERO: $name = $name ( <$vt as Zero>::ZERO );
        }
    );
    ($name:ident { $($it:tt)* }) => (
        impl Zero for $name {
            $($it:tt)*
        }
    );
}

#[macro_export]
macro_rules! _vector_trait_impl {
    (Add $name:ident $scalar:ty, $body:tt _) => (
        _vector_binop_impl!($name Add add $body +);
    );
    (Add $name:ident $scalar:ty, $body:tt { $($it:tt)* }) => (
        _vector_binop_impl!($name Add { $($it)* });
    );

    (AddAssign $name:ident $scalar:ty, $body:tt _) => (
        _vector_assign_binop_impl!($name AddAssign add_assign $body +=);
    );
    (AddAssign $name:ident $scalar:ty, $body:tt { $($it:tt)* }) => (
        _vector_assign_binop_impl!($name AddAssign { $($it)* });
    );

    (Sub $name:ident $scalar:ty, $body:tt _) => (
        _vector_binop_impl!($name Sub sub $body -);
    );
    (Sub $name:ident $scalar:ty, $body:tt { $($it:tt)* }) => (
        _vector_binop_impl!($name Sub { $($it)* });
    );

    (SubAssign $name:ident $scalar:ty, $body:tt _) => (
        _vector_assign_binop_impl!($name SubAssign sub_assign $body -=);
    );
    (SubAssign $name:ident $scalar:ty, $body:tt { $($it:tt)* }) => (
        _vector_assign_binop_impl!($name SubAssign { $($it)* });
    );

    (Mul $name:ident $scalar:ty, $body:tt _) => (
        _vector_scalar_binop_impl!($name $scalar, Mul mul $body *);
        _vector_scalar_binop_impl!(@commutative $name $scalar, Mul mul $body *);
    );
    (Mul $name:ident $scalar:ty, $body:tt { { $($it_a:tt)* } { $($it_b:tt)* } }) => (
        _vector_scalar_binop_impl!($name $scalar, Mul { $($it_a)* });
        _vector_scalar_binop_impl!(@commutative $name $scalar, Mul { $($it_b)* });
    );

    (MulAssign $name:ident $scalar:ty, $body:tt _) => (
        _vector_scalar_assign_binop_impl!($name $scalar, MulAssign mul_assign $body *=);
    );
    (MulAssign $name:ident $scalar:ty, $body:tt { { $($it_a:tt)* } { $($it_b:tt)* } }) => (
        _vector_scalar_assign_binop_impl!($name $scalar, MulAssign { $($it_a)* });
    );

    (Div $name:ident $scalar:ty, $body:tt _) => (
        _vector_scalar_binop_impl!($name $scalar, Div div $body /);
    );
    (Div $name:ident $scalar:ty, $body:tt { $($it:tt)* }) => (
        _vector_scalar_binop_impl!($name $scalar, Div { $($it)* });
    );

    (DivAssign $name:ident $scalar:ty, $body:tt _) => (
        _vector_scalar_assign_binop_impl!($name $scalar, DivAssign div_assign $body /=);
    );
    (DivAssign $name:ident $scalar:ty, $body:tt { $($it:tt)* }) => (
        _vector_scalar_assign_binop_impl!($name $scalar, DivAssign { $($it)* });
    );

    (Neg $name:ident $scalar:ty, $body:tt _) => (
        _vector_unop_impl!($name Neg neg $body -);
    );
    (Neg $name:ident $scalar:ty, $body:tt { $($it:tt)* }) => (
        _vector_unop_impl!($name Neg { $($it)* });
    );

    (Zero $name:ident $scalar:ty, $body:tt _) => (
        _vector_zero_impl!($name $body);
    );
    (Zero $name:ident $scalar:ty, $body:tt { $($it:tt)* }) => (
        _vector_zero_impl!($name { $($it:tt)* });
    );
}

#[macro_export]
macro_rules! _vector_structure {
    ($name:ident { $($vn:ident: $vt:ty),* }) => (

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $name { $(pub $vn: $vt),* }

        #[allow(dead_code)]
        impl $name {
            pub fn new($($vn: $vt),*) -> $name {
                $name { $($vn: $vn),* }
            }
        }
    );
    ($name:ident ( $vt:ty )) => (
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $name ( pub $vt );

        #[allow(dead_code)]
        impl $name {
            pub fn new(v: $vt) -> $name {
                $name ( v )
            }
        }
    );
}

#[macro_export]
macro_rules! _vector_space_recurse {
    ($tr:ident { $($unimpl:ident)* }, $($rest:tt)*) =>
        (_vector_space_recurse!($tr { $($unimpl)* } {} $($rest)*););

    (Add { Add $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Add { $($tail)* } { $($checked)* }
            $($rest)*););
    (Add { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Add { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (AddAssign { AddAssign $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(AddAssign { $($tail)* } { $($checked)* }
            $($rest)*););
    (AddAssign { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(AddAssign { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (Sub { Sub $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Sub { $($tail)* } { $($checked)* }
            $($rest)*););
    (Sub { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Sub { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (SubAssign { SubAssign $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(SubAssign { $($tail)* } { $($checked)* }
            $($rest)*););
    (SubAssign { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(SubAssign { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (Mul { Mul $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Mul { $($tail)* } { $($checked)* }
            $($rest)*););
    (Mul { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Mul { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (MulAssign { MulAssign $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(MulAssign { $($tail)* } { $($checked)* }
            $($rest)*););
    (MulAssign { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(MulAssign { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (Div { Div $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Div { $($tail)* } { $($checked)* }
            $($rest)*););
    (Div { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Div { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (DivAssign { DivAssign $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(DivAssign { $($tail)* } { $($checked)* }
            $($rest)*););
    (DivAssign { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(DivAssign { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (Neg { Neg $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Neg { $($tail)* } { $($checked)* }
            $($rest)*););
    (Neg { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Neg { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (Zero { Zero $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Zero { $($tail)* } { $($checked)* }
            $($rest)*););
    (Zero { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Zero { $($tail)* } { $head $($checked)* }
            $($rest)*););

    ($tr:ident {} { $($unimpl:ident)* } $($rest:tt)*) =>
        (vector_space!({ $($unimpl)* } $($rest)*););
}

#[macro_export]
macro_rules! vector_space {
    (
        { $($unimpl:ident)* }
        $name:ident $scalar:ty, $body:tt
        $tr:ident = $trimpl:tt;
        $( $rest:tt )*
    ) => (
           _vector_trait_impl!($tr $name $scalar, $body $trimpl);
           _vector_space_recurse!($tr { $($unimpl)* }, $name $scalar, $body $( $rest )*);
    );
    (
        { $($unimpl:ident)* }
        $name:ident $scalar:ty, $body:tt
    ) => (
        $(_vector_trait_impl!($unimpl $name $scalar, $body _);)*
    );
    (
        $name:ident {
            $body:tt

            Scalar = $scalar:ty;
        }

        $( $rest:tt )*
    ) => (
        _use_Add!();
        _use_AddAssign!();
        _use_Sub!();
        _use_SubAssign!();
        _use_Mul!();
        _use_MulAssign!();
        _use_Div!();
        _use_DivAssign!();
        _use_Neg!();

        _use_Vector!();
        _use_Zero!();

        _vector_structure!($name $body);

        impl Vector for $name {
            type Scalar = $scalar;
        }

        vector_space! {
            {
                Add
                AddAssign
                Sub
                SubAssign
                Mul
                MulAssign
                Div
                DivAssign
                Neg
                Zero
            }
            $name $scalar, $body
            $( $rest )*
        }
    );
}

#[macro_export]
macro_rules! _sum {
    ($head:expr, $($body:expr),*) => ($head + _sum!($($body),*));
    ($tail:expr) => ($tail);
}

#[macro_export]
macro_rules! _try_dimension_specific_op {
    ($name:ident $t:ty, $x:ident $y:ident) => (
        _use_Cross!();

        impl Cross for $name {
            type Perpendicular = $t;

            fn cross(self, other: $name) -> $t {
                self.$x * other.$y - self.$y * other.$x
            }
        }

        impl Cross<$t> for $name {
            type Perpendicular = $name;

            fn cross(self, other: $t) -> $name {
                $name { $x:  self.$y * other,
                        $y: -self.$x * other }
            }
        }

        impl Cross<$name> for $t {
            type Perpendicular = $name;

            fn cross(self, other: $name) -> $name {
                $name { $x: -self * other.$y,
                        $y:  self * other.$x }
            }
        }
    );
    ($name:ident $t:ty, $x:ident $y:ident $z:ident) => (
        _use_Cross!();

        impl Cross for $name {
            type Perpendicular = $name;

            fn cross(self, other: $name) -> $name {
                $name { $x: self.$y * other.$z - self.$z * other.$y,
                        $y: self.$z * other.$x - self.$x * other.$z,
                        $z: self.$x * other.$y - self.$y * other.$x }
            }
        }
    );
    ($name:ident $t:ty, $($else_:ident)*) => ();
}

#[macro_export]
macro_rules! _zip_fold_consts {
    ($acc:expr, $fid:ident $(,$id:ident)*) => (
        const $fid: usize = $acc;
        _zip_fold_consts!(($fid + 1), $($id),*);
    );
    ($acc:expr, ) => ();
}

#[macro_export]
macro_rules! ntuple {
    (
        $name:ident: $t:ty {
            $($e:ident),*
        }
    ) => (
        vector_space! {
            $name {
                { $($e: $t),* }

                Scalar = $t;
            }
        }

        _use_Index!();
        _use_IndexMut!();

        impl From<$name> for [$t; _sum!($(replace!($e, 1)),*)] {
            #[inline]
            fn from(t: $name) -> Self {
                [ $(t.$e),* ]
            }
        }

        impl From<$name> for [[$t; _sum!($(replace!($e, 1)),*)]; 1] {
            #[inline]
            fn from(t: $name) -> Self {
                [[ $(t.$e),* ]]
            }
        }

        impl From<$name> for [[$t; 1]; _sum!($(replace!($e, 1)),*)] {
            #[inline]
            fn from(t: $name) -> Self {
                [ $([t.$e]),* ]
            }
        }

        impl From<[$t; _sum!($(replace!($e, 1)),*)]> for $name {
            #[inline]
            #[allow(non_upper_case_globals)]
            fn from(arr: [$t; _sum!($(replace!($e, 1)),*)]) -> Self {
                _zip_fold_consts!(0usize, $($e),*);
                $name { $($e: arr[$e]),* }
            }
        }

        impl From<[[$t; _sum!($(replace!($e, 1)),*)]; 1]> for $name {
            #[inline]
            #[allow(non_upper_case_globals)]
            fn from(arr: [[$t; _sum!($(replace!($e, 1)),*)]; 1]) -> Self {
                _zip_fold_consts!(0usize, $($e),*);
                $name { $($e: arr[0][$e]),* }
            }
        }

        impl From<[[$t; 1]; _sum!($(replace!($e, 1)),*)]> for $name {
            #[inline]
            #[allow(non_upper_case_globals)]
            fn from(arr: [[$t; 1]; _sum!($(replace!($e, 1)),*)]) -> Self {
                _zip_fold_consts!(0usize, $($e),*);
                $name { $($e: arr[$e][0]),* }
            }
        }

        impl Index<usize> for $name {
            type Output = $t;

            #[allow(non_upper_case_globals)]
            fn index(&self, index: usize) -> &$t {
                _zip_fold_consts!(0usize, $($e),*);
                match index {
                    $(
                        $e => &self.$e,
                    )*
                    _ => panic!("n-tuple vector index out of bounds!"),
                }
            }
        }

        impl IndexMut<usize> for $name {
            #[allow(non_upper_case_globals)]
            fn index_mut(&mut self, index: usize) -> &mut $t {
                _zip_fold_consts!(0usize, $($e),*);
                match index {
                    $(
                        $e => &mut self.$e,
                    )*
                    _ => panic!("n-tuple vector index out of bounds!"),
                }
            }
        }
    );
}

#[macro_export]
macro_rules! euclidean {
    (
        $($name:ident: $t:ty {
            $($e:ident),*
        })+
    ) => (
        $(
            ntuple! {
                $name: $t {
                    $($e),*
                }
            }

            _use_InnerProduct!();
            _use_Norm!();
            _use_Metric!();

            impl InnerProduct for $name {
                fn dot(self, other: $name) -> $t {
                    _sum!($(self.$e * other.$e),*)
                }
            }

            impl Norm for $name {
                type Length = $t;

                fn length(self) -> $t {
                    _sum!($( (self.$e * self.$e) ),*).sqrt()
                }
            }

            impl Metric for $name {
                type Distance = $t;

                fn distance(self, other: $name) -> $t {
                    (self - other).length()
                }
            }

            _try_dimension_specific_op!($name $t, $($e)*);
        )+
    );
}
