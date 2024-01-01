//! # tech 
//! 
//! #This a supportive crate for the math workspaces that contains useful Traits

use std::ops::{Add, Sub, Div, Mul, Neg, AddAssign, SubAssign, DivAssign, MulAssign};

/// A macro for implementing traits
macro_rules! implTrait {
    (Ring for $($typ:ty),*) => {
        $(
            impl Ring for $typ {
                fn is_zero(&self) -> bool {
                    self == &(0 as $typ)
                }

                fn zero () -> $typ {
                    0 as $typ
                }
            }
        )*
    };

    (UnRing for $($typ: ty),*) => {
        $(
            impl UnRing for $typ {
                fn is_one(&self) -> bool {
                    self == &(1 as $typ)
                }
            
                fn one () -> $typ {
                    1 as $typ
                }
            }
        )*
    };

    ($i: ident for $($typ: ty),*) => {
        $(
            impl $i for $typ {}
        )*
    }
}

/// Special trait that allows to assume that elements of the type form a field (in algebraic terms). 
/// Types that implement Field are also assumed to be UnRing and an IntegralDomain
pub trait Field 
where Self: UnRing + IntegralDomain + Div<Self, Output = Self> + DivAssign <Self> + Neg<Output = Self> {}

implTrait!(Field for f32, f64);

/// Special trait that allows to assume that elements of the type form a ring (in algebraic terms)
pub trait Ring
where Self: Add<Self, Output = Self> + Sub <Self, Output = Self> + Mul<Self, Output = Self> +
AddAssign<Self> + SubAssign<Self> + MulAssign<Self> + ComAdd + AssAdd + PartialEq + Sized
{
    fn is_zero(&self) -> bool;
    fn zero() -> Self;
}

implTrait!(Ring for f32, f64, i8, i16, i32, i64, i128);

/// Describes a ring with one
pub trait UnRing: Ring {
    fn is_one(&self) -> bool;
    fn one() -> Self;
}

implTrait!(UnRing for f32, f64, i8, i16, i32, i64, i128);

/// Describes an integral domain
pub trait IntegralDomain: Ring + ComMul + AssMul{}

implTrait!(IntegralDomain for f32, f64, i8, i16, i32, i64, i128);
/// Special trate that allows to assume that the type implements Commutative Multiplying
pub trait ComMul : Mul<Self, Output = Self> + MulAssign<Self> + Sized {}

implTrait!(ComMul for f32, f64, i8, i16, i32, i64, i128);

///Special trate that allows to assume that the type implements Associative Multiplying
pub trait AssMul: Mul<Self, Output = Self> + MulAssign<Self> + Sized{}

implTrait!(AssMul for f32, f64, i8, i16, i32, i64, i128);

///Special trate that allows to assume that the type implememts  Commutative Adding
pub trait ComAdd: Add<Self, Output = Self> + AddAssign<Self> + Sized{}

implTrait!(ComAdd for f32, f64, i8, i16, i32, i64, i128);

/// Special trate that allows to assume that the type implements Assoсiative Adding
pub trait AssAdd: Add<Self, Output = Self> + AddAssign<Self> + Sized{}

implTrait!(AssAdd for f32, f64, i8, i16, i32, i64, i128);