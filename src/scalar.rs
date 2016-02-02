use super::*;
use traits::{Field};

use std::ops::{Add, Sub, Mul, Div, Neg};

impl<T> Field for T where T: Add<Output=T>
                           + Sub<Output=T>
                           + Mul<Output=T>
                           + Div<Output=T>
                           + Neg<Output=T>
                           + Zero
                           + One {}
