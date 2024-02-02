//! #Maths is the crate for solving varios maths problems
mod primitives;

use crate::primitives::checked::*;
///This the main mod that primarily contains traits that the whole crate depends on
mod tech {
    use std::ops::{Add, Sub, Div, Mul, Neg, AddAssign, SubAssign, DivAssign, MulAssign};
}

//TODO when rewriting Polynomial make it contain array so that i can make some funcs const and for some other benefits
//To do that i can remove Assign operators as they seem strange for polynoms anyway