//! #Maths is the crate for solving varios maths problems
mod primitives;
pub use crate::primitives::checked;
///This the main mod that primarily contains traits that the whole crate depends on
mod tech {
    //Notice that the names of the imported traits are changed
    use std::ops::{Add as AddTrait, Sub as SubTrait, Div as DivTrait, Mul as MulTrait, Neg};

    ///Indicates that the struct that implements it can be considered as a representaion of an operation
    pub trait Op {}

    pub struct Mul {}
    impl Op for Mul {}

    pub struct Add {}
    impl Op for Add {}

    pub trait Com<T: Op> {}
    pub trait Ass<T: Op> {}

    pub trait Group<T: Op> where Self: Ass<T>{
        fn neut() -> Self;
    }

    pub trait Abelian<T: Op> where Self: Com<T> {}

    pub trait Ring where Self: 
        Group<Add> + Abelian<Add> + AddTrait<Self, Output = Self> + MulTrait<Self, Output = Self> +
        SubTrait<Self, Output = Self> + Neg<Output = Self> + Sized {}

    pub trait UnRing where Self: Ring {
        fn one() -> Self;
    }

    pub trait IntegralDomain where Self: Ring + Com<Mul> + Ass<Mul> {} 

    pub trait Field where Self: UnRing + IntegralDomain + DivTrait<Self, Output = Self> {
        fn zero() -> Self {
            <Self as Group<Add>>::neut()
        }
    }

    //TODO Meta, Gcd ?
}

//TODO when rewriting Polynomial make it contain array so that i can make some funcs const and for some other benefits
//To do that i can remove Assign operators as they seem strange for polynoms anyway