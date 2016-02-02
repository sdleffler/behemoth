use super::{Zero};
use traits::{Cross, InnerProduct, Metric, Norm, Vector};

use std::ops::{Add, Sub, Mul, Div, Neg};

macro_rules! as_expr {
    ($x:expr) => ($x)
}

macro_rules! as_items {
    ($($x:item)*) => ($($x)*)
}

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

macro_rules! _vector_unop_impl {
    ($name:ident $optrait:ident $opfunc:ident { $($vn:ident: $vt:ty),* } $binop:tt) => (
        impl $optrait for $name {
            type Output = $name;

            fn $opfunc (self) -> $name {
                $name { $($vn: as_expr!($binop self.$vn)),* }
            }
        }
    );
    ($name:ident $optrait:ident $opfunc:ident ( $vt:ty ) $binop:tt) => (
        impl $optrait for $name {
            type Output = $name;

            fn $opfunc (self) -> $name {
                $name ( as_expr!($binop self.0) )
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

macro_rules! _vector_trait_impl {
    (Add $name:ident $scalar:ty, $body:tt _) => (
        _vector_binop_impl!($name Add add $body +);
    );
    (Add $name:ident $scalar:ty, $body:tt { $($it:tt)* }) => (
        _vector_binop_impl!($name Add { $($it)* });
    );

    (Sub $name:ident $scalar:ty, $body:tt _) => (
        _vector_binop_impl!($name Sub sub $body -);
    );
    (Sub $name:ident $scalar:ty, $body:tt { $($it:tt)* }) => (
        _vector_binop_impl!($name Sub { $($it)* });
    );

    (Mul $name:ident $scalar:ty, $body:tt _) => (
        _vector_scalar_binop_impl!($name $scalar, Mul mul $body *);
        _vector_scalar_binop_impl!(@commutative $name $scalar, Mul mul $body *);
    );
    (Mul $name:ident $scalar:ty, $body:tt { { $($it_a:tt)* } { $($it_b:tt)* } }) => (
        _vector_scalar_binop_impl!($name $scalar, Mul { $($it_a)* });
        _vector_scalar_binop_impl!(@commutative $name $scalar, Mul { $($it_b)* });
    );

    (Div $name:ident $scalar:ty, $body:tt _) => (
        _vector_scalar_binop_impl!($name $scalar, Div div $body /);
    );
    (Div $name:ident $scalar:ty, $body:tt { $($it:tt)* }) => (
        _vector_scalar_binop_impl!($name $scalar, Div { $($it)* });
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

macro_rules! _vector_structure {
    ($name:ident { $($vn:ident: $vt:ty),* }) => (

        #[derive(Copy, Clone, Debug)]
        pub struct $name { $($vn: $vt),* }

        impl $name {
            pub fn new($($vn: $vt),*) -> $name {
                $name { $($vn: $vn),* }
            }
        }
    );
    ($name:ident ( $vt:ty )) => (
        #[derive(Copy, Clone, Debug)]
        pub struct $name ( $vt );

        impl $name {
            pub fn new($v: $vt) {
                $name ( $v )
            }
        }
    );
}

macro_rules! _vector_space_recurse {
    ($tr:ident { $($unimpl:ident)* }, $($rest:tt)*) =>
        (_vector_space_recurse!($tr { $($unimpl)* } {} $($rest)*););

    (Add { Add $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Add { $($tail)* } { $($checked)* }
            $($rest)*););
    (Add { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Add { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (Sub { Sub $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Sub { $($tail)* } { $($checked)* }
            $($rest)*););
    (Sub { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Sub { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (Mul { Mul $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Mul { $($tail)* } { $($checked)* }
            $($rest)*););
    (Mul { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Mul { $($tail)* } { $head $($checked)* }
            $($rest)*););

    (Div { Div $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Div { $($tail)* } { $($checked)* }
            $($rest)*););
    (Div { $head:ident $($tail:ident)* } { $($checked:ident)* } $($rest:tt)*) =>
        (_vector_space_recurse!(Div { $($tail)* } { $head $($checked)* }
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
        _vector_structure!($name $body);

        impl Vector for $name {
            type Scalar = $scalar;
        }

        vector_space!({ Add Sub Mul Div Neg Zero } $name $scalar, $body
            $( $rest )*);
    );
}

macro_rules! _sum {
    ($head:expr, $($body:expr),*) => ($head + _sum!($($body),*));
    ($tail:expr) => ($tail);
}

macro_rules! _try_dimension_specific_op {
    ($name:ident $t:ty, $x:ident $y:ident) => (
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
macro_rules! ntuple_space {
    (
        $name:ident: $t:ty {
            $($e:ident),*
        }

        $( $rest:tt )*
    ) => (
        vector_space! {
            $name {
                { $($e: $t),* }

                Scalar = $t;
            }
        }
    );
}

macro_rules! euclidean_space {
    (
        $name:ident: $t:ty {
            $($e:ident),*
        }

        $( $rest:tt )*
    ) => (
        ntuple_space! {
            $name: $t {
                $($e)*
            }
        }

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
    );
}

//trace_macros!(true);

// vector_space! {
//     Vec3f {
//         { x: f64, y: f64, z: f64 }

//         Scalar = f64;
//     }
// }

euclidean_space! {
    Vec2f: f64 { x, y }
}

euclidean_space! {
    Vec3f: f64 { x, y, z }
}

//trace_macros!(false);
