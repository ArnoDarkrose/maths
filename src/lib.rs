//! #Maths is the crate for solving varios maths problems
mod primitives;
pub use crate::primitives::checked::*;

pub mod fraction;

///This the main mod that primarily contains traits that the whole crate depends on
mod tech {
    //Notice that the names of the imported traits are changed
    use std::ops::{Add as AddTrait, Sub as SubTrait, Div as DivTrait, Mul as MulTrait, Neg};

    ///Indicates that the type makes overflow checks
    pub trait Checked {}
    ///Indicates that the struct that implements it can be considered as a representaion of an operation
    pub trait Op {}

    pub struct Mul {}
    impl Op for Mul {}

    pub struct Add {}
    impl Op for Add {}

    pub trait Com<T: Op> {}
    pub trait Ass<T: Op> {}

    pub trait Group<T: Op> where Self: Ass<T> + std::cmp::Eq{
        fn neut() -> Self;
    }

    pub trait AddGroup where Self: 
        Group<Add> + AddTrait<Self, Output = Self> + SubTrait<Self, Output = Self> + Neg<Output = Self> + Sized{
        fn zero() -> Self {
            <Self as Group<Add>>::neut()
        }
    }

    pub trait MulGroup where Self: 
        Group<Mul> + MulTrait<Self, Output = Self> + DivTrait<Self, Output = Self> + Sized{
        fn one() -> Self {
            <Self as Group<Mul>>::neut()
        }
    }

    pub trait Abelian<T: Op> where Self: Com<T> {}

    pub trait Ring where Self:
        AddGroup + Abelian<Add> + MulTrait<Self, Output = Self> {}

    pub trait IntegralDomain where Self: 
        Ring + Com<Mul> + Ass<Mul> {} 

    pub trait Field where Self: 
        IntegralDomain + MulGroup {}

    pub trait Meta {
        fn non_zero() ->  Self;
        fn name() -> String;
    }
    //TODO Gcd ?
}

//TODO when rewriting Polynomial make it contain array so that i can make some funcs const and for some other benefits
//To do that i can remove Assign operators as they seem strange for polynoms anyway