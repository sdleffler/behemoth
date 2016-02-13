#![feature(associated_consts)]
#![feature(augmented_assignments)]
#![feature(op_assign_traits)]
#![feature(type_macros)]

#[macro_use]
extern crate behemoth;

behemoth! {
    matrices! {
        f64: {
            Mat3x3 => 3, 3
        }
    }
}

use behemoth::*;

fn main() {
    let u = Mat3x3::identity();
    for k in 0..u.order() {
        let r = u.rows()
                 .enumerate()
                 .fold((k, u[(k, k)]),
                     |(j, max), (i, row)| {
                         if row[k] > max {
                             (i, row[k])
                         } else {
                             (j, max)
                         }
                     }
                 ).0;
        println!("Row with largest element at index {:?}: {:?}", k, r);
    }
}
