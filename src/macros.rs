macro_rules! as_expr {
    ($x:expr) => ($x)
}

macro_rules! as_items {
    ($($x:item)*) => ($($x)*)
}

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
                    ($($input)*) => {}
                    $($unsearched)*
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
