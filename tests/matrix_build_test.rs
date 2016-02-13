#![feature(associated_consts)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(type_macros)]
#![feature(log_syntax)]

#[macro_use] extern crate behemoth;

behemoth! {
    matrices! {
        f64:
        {
            Mat3x3 => 3, 3
        }
    }
}
