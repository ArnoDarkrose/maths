#![feature(trait_alias)]
//! #Maths is the crate for solving varios maths problems
pub mod primitives;

pub mod fraction;

///This the main mod that primarily contains traits that the whole crate depends on
pub mod tech {
    //Notice that the names of the imported traits are changed
    use std::ops::{Add, Sub, Div, Mul, Neg, Rem};

    macro_rules! plainImpl {
        ($typ:ty: $($trait_name: ident $(<$gen_param: ident>)?),*) => {
            $(
                impl $trait_name $(<$gen_param>)? for $typ {}
            )*
        };
    }

    macro_rules! implTechTraitsUpToField {
        ($($typ:ty),*) => {
            $(
                plainImpl!($typ: Com<AddOp>, Ass<AddOp>, Com<MulOp>, Ass<MulOp>, Abelian<AddOp>, Abelian<MulOp>,
                    Ring, MulGroup, AddGroup, IntegralDomain, Field
                );

                impl Group<AddOp> for $typ {
                    fn neut() -> Self {
                        0 as Self
                    }
                }

                impl Group<MulOp> for $typ {
                    fn neut() -> Self {
                        1 as Self
                    }
                }
            )*
        };
    }

    macro_rules! implTechTraitsUpToRing {
        ($($typ:ty),*) => {
            $(
                plainImpl!($typ: Com<AddOp>, Ass<AddOp>, Com<MulOp>, Ass<MulOp>, Abelian<AddOp>, Ring, AddGroup, IntegralDomain);

                impl Group<AddOp> for $typ {
                    fn neut() -> Self {
                        0 as Self
                    }
                }
            )*
        };
    }

    macro_rules! implMeta{
        ($($typ:ty: $name: expr);*) => {
            $(
                impl Meta for $typ {
                    fn non_zero() -> Self {
                        1 as Self
                    }

                    fn name() -> String {
                        String::from($name)
                    }
                }
            )*
        };
    }
    
    macro_rules! implGcd {
        ($($typ:ty),*) => {
            $(
                impl Gcd for $typ {
                    fn gcd(&self, rhs: &Self) -> Self {
                        let rem = self % rhs;

                        if rem == <$typ>::zero() {
                            return rhs.clone()
                        }

                        rhs.gcd(&rem)
                    }
                }
            )*
        };
    }
    ///Indicates that the type makes overflow checks
    pub trait Checked {}
    ///Indicates that the struct that implements it can be considered as a representaion of an operation
    pub trait Op {}

    pub struct MulOp {}
    impl Op for MulOp{}

    pub struct AddOp {}
    impl Op for AddOp{}

    pub trait Com<T: Op> {}
    pub trait Ass<T: Op> {}

    pub trait Group<T: Op> where Self: Ass<T> + std::cmp::PartialEq{
        fn neut() -> Self;
    }

    pub trait AddGroup where Self: 
        Group<AddOp> + Abelian<AddOp> + Add<Self, Output = Self> + Sub<Self, Output = Self> + Neg<Output = Self> + Sized, {
        fn zero() -> Self {
            <Self as Group<AddOp>>::neut()
        }

        fn is_zero(&self) -> bool {
            self == &Self::zero()
        }
    }

    pub trait CheckAddGroup where Self:
        Group<AddOp> + Checked + Abelian<AddOp> + Sized, 
        for <'a> &'a Self: Add<&'a Self, Output = Option<Self>> + Sub<&'a Self, Output = Option<Self>> + Neg<Output = Option<Self>> {

        fn zero() -> Self {
            <Self as Group<AddOp>>::neut()
        }

        fn is_zero(&self) -> bool {
            self == &Self::zero()
        }
    }


    pub trait MulGroup where Self:
        Group<MulOp> + Mul<Self, Output = Self> + Abelian<MulOp> + Div<Self, Output = Self> + Sized{
        fn one() -> Self {
            <Self as Group<MulOp>>::neut()
        }

        fn is_one(&self) -> bool {
            self == &Self::one()
        }
    }

    pub trait CheckMulGroup where Self:
        Group<MulOp> + Checked  + Abelian<MulOp> + Sized, 
        for <'a> &'a Self: Mul<&'a Self, Output = Option<Self>> + Div<&'a Self, Output = Option<Self>> {
        fn one() -> Self {
            <Self as Group<MulOp>>::neut()
        }

        fn is_one(&self) -> bool {
            self == &Self::one()
        }
    }


    pub trait Abelian<T: Op> where Self: Com<T> + Group<T> {}

    pub trait Ring where Self:
        AddGroup + Mul<Self, Output = Self> {}

    pub trait CheckRing where Self: 
        CheckAddGroup, for <'a> &'a Self: 
        Add<&'a Self, Output = Option<Self>> + Sub<&'a Self, Output = Option<Self>> + Neg<Output = Option<Self>> + Mul<&'a Self, Output = Option<Self>>{}

    pub trait IntegralDomain where Self: 
        Ring + Com<MulOp> + Ass<MulOp> {} 

    pub trait CheckIntegralDomain where Self:
        CheckRing + Com<MulOp> + Ass<MulOp>, for <'a> &'a Self: 
        Add<&'a Self, Output = Option<Self>> + Sub<&'a Self, Output = Option<Self>> + Neg<Output = Option<Self>> + Mul<&'a Self, Output = Option<Self>> {}

    pub trait Field where Self: 
        IntegralDomain + MulGroup {}

    pub trait CheckField where Self:
        CheckIntegralDomain + CheckMulGroup, for <'a> &'a Self: 
        Add<&'a Self, Output = Option<Self>> + Sub<&'a Self, Output = Option<Self>> + Neg<Output = Option<Self>> + 
        Mul<&'a Self, Output = Option<Self>> + Div<&'a Self, Output = Option<Self>> {}
    

    pub trait Meta {
        fn non_zero() ->  Self;
        fn name() -> String;
    }
    
    ///Greatest Common Divisor
    pub trait Gcd where for <'a> &'a Self: Rem<&'a Self, Output = Self> {
        fn gcd(&self, rhs: &Self) -> Self;
    }

    pub trait CheckGcd: Checked + Sized where for <'a> &'a Self: Rem<&'a Self, Output = Option<Self>> {
        fn gcd(&self, rhs: &Self) -> Option<Self>;
    }

    implTechTraitsUpToField!(f32, f64);
    
    implTechTraitsUpToRing!(i8, i16, i32, i64, i128);

    implMeta!(
        i8: "i8"; i16: "i16"; i32: "i32"; i64: "i64"; i128: "i128";
        u8: "u8"; u16: "u16"; u32: "u32"; u64: "u64"; u128: "u128"; 
        f32: "f32"; f64: "f64"
    );

    implGcd!(i8, i16, i32, i64, i128);
}

//TODO when rewriting Polynomial make it contain array so that i can make some funcs const and for some other benefits
//To do that i can remove Assign operators as they seem strange for polynoms anyway

//TODO when writing Matrixes I should create an algorithm of memiry compression that turns matrix into a product
//of several matrixes which is represented as Iterator