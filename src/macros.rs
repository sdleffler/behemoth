#[macro_export]
macro_rules! assert_approx_eq {
    ($lhs:expr, $rhs:expr) => (
        if $lhs.approx_ne(&$rhs) {
            panic!(format!("assert_approx_eq failed: {:?} != {:?}", $lhs, $rhs));
        }
    );
}

#[macro_export]
macro_rules! as_expr {
    ($x:expr) => ($x)
}

#[macro_export]
macro_rules! as_items {
    ($($x:item)*) => ($($x)*)
}

#[macro_export]
macro_rules! as_ty {
    ($t:ty) => ($t)
}

#[macro_export]
macro_rules! as_stmts {
    ($($s:stmt)*) => ($($s)*)
}

#[macro_export]
macro_rules! replace {
    ($id:ident, $e:expr) => ($e)
}

#[macro_export]
macro_rules! is_empty {
    (() {$($body:tt)*}) => ($($body:tt)*);
    (($($notempty:tt)+) $body:tt) => ();
}

#[macro_export]
macro_rules! is_not_empty {
    (($($notempty:tt)+) $body:tt) => ($($body:tt)*);
    (() {$($body:tt)*}) => ();
}

#[macro_export]
macro_rules! is_eq {
    (
        if ($($thingA:tt)*) == ($($thingB:tt)*) {
            $($if_true:tt)*
        } else {
            $($if_false:tt)*
        }
    ) => (
        macro_rules! is_eq_test {
            ($($thingA)*, $($thingA)*) => ($($if_true)*);
            ($($thingA)*, $($thingB)*) => ($($if_false)*);
        }

        is_eq_test!($($thingA)*, $($thingB)*);
    );
}

#[macro_export]
macro_rules! n_pairs_are_eq {
    (
        if $(($($thingA:tt)*) $eq:tt ($($thingB:tt)*))&&* {
            $($if_true:tt)*
        } else {
            $($if_false:tt)*
        }
    ) => (
        macro_rules! is_eq_test {
            ($($($thingA)* $eq $($thingA)*)*) => ($($if_true)*);
            ($($($thingA)* $eq $($thingB)*)*) => ($($if_false)*);
        }

        is_eq_test!($($($thingA)* $eq $($thingB)*)*);
    );
}

#[macro_export]
macro_rules! lazy_single_instantiate {
    (@remove
        ($($remove:tt)*) $name:ident
        { $($searched:tt)* }
        { ($($input:tt)*) => { $($body:tt)* } $($unsearched:tt)* }
    ) => (
        is_eq! {
            if ($($remove)*) == ($($input)*) {
                lazy_single_instantiate! {
                    $name:
                    $($searched)*
                    $($unsearched)*
                    ($($input)*) => {}
                }
            } else {
                lazy_single_instantiate! {
                    @remove
                    ($($remove)*)
                    $name
                    { $($searched)* ($($input)*) => { $($body)* } }
                    { $($unsearched)* }
                }
            }
        }
    );
    (@recurse $all:tt { $($build:tt)* } $name:ident ( $($input:tt)* ) => { $($body:tt)* } $($more:tt)*) => (
        lazy_single_instantiate! {
            @recurse
            $all
            {
                $($build)*
                ($($input)*) => (
                    $($body)*
                    lazy_single_instantiate!(@remove ($($input)*) $name {} $all);
                );
            }
            $name
            $($more)*
        }
    );
    (@recurse $all:tt { $($build:tt)* } $name:ident) => (
        macro_rules! $name {
            $($build)*
        }
    );
    ($name:ident: $($more:tt)*) => (
        lazy_single_instantiate!(@recurse { $($more)* } {} $name $($more)*);
    );
}

#[macro_export]
macro_rules! lazy_single_use {
    ($($macname:ident => ($($body:tt)*);)+) => (
        $(
            macro_rules! $macname {
                () => (
                    $($body)*
                    macro_rules! $macname {
                        () => ();
                    }
                );
            }
        )+
    );
}

#[macro_export]
macro_rules! lazy_cycle_instantiate {
    (@remove
        ($($remove:tt)*) $name:ident
        { $($searched:tt)* }
        { ($($input:tt)*) => { $($body:tt)* } $($unsearched:tt)* }
    ) => (
        is_eq! {
            if ($($remove)*) == ($($input)*) {
                lazy_cycle_instantiate! {
                    $name:
                    $($searched)*
                    $($unsearched)*
                    ($($input)*) => { $($body)* }
                }
            } else {
                lazy_cycle_instantiate! {
                    @remove
                    ($($remove)*)
                    $name
                    { $($searched)* ($($input)*) => { $($body)* } }
                    { $($unsearched)* }
                }
            }
        }
    );
    (@recurse $all:tt { $($build:tt)* } $name:ident ( $($input:tt)* ) => { $($body:tt)* } $($more:tt)*) => (
        lazy_cycle_instantiate! {
            @recurse
            $all
            {
                $($build)*
                ($($input)*) => (
                    $($body)*
                    lazy_cycle_instantiate!(@remove ($($input)*) $name {} $all);
                );
            }
            $name
            $($more)*
        }
    );
    (@recurse $all:tt { $($build:tt)* } $name:ident) => (
        macro_rules! $name {
            $($build)*
        }
    );
    ($name:ident: $($more:tt)*) => (
        lazy_cycle_instantiate!(@recurse { $($more)* } {} $name $($more)*);
    );
}
