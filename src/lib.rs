//! #Maths is the crate for solving varios maths problems
mod primitives;

use crate::primitives::checked::*;
///This the main mod that primarily contains traits that the whole crate depends on
mod tech {
    use std::ops::{Add, Sub, Div, Mul, Neg, AddAssign, SubAssign, DivAssign, MulAssign};
}