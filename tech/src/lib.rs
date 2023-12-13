//! # tech 
//! 
//! #This a supportive crate for the math workspaces that contains udeful Traits

use std::ops::{Add, Sub, Div, Mul, Neg, AddAssign, SubAssign, DivAssign, MulAssign};

/// Special trait that allows to assume that elements of the type form a field (in algebraic terms)
pub trait Field 
where Self: Add<Self, Output = Self> + Sub<Self, Output = Self> + Div<Self, Output = Self> + Mul<Self, Output = Self> + Neg<Output = Self> +
AddAssign<Self> + SubAssign<Self> + DivAssign<Self> + MulAssign<Self> + PartialEq + Sized,
{
    fn is_zero(&self) -> bool;
    fn is_one(&self) -> bool;
    const ZERO: Self;
    const ONE: Self;
}

impl Field for f32 {
    fn is_zero(&self) -> bool {
        self == &0.0 
    }

    fn is_one(&self) -> bool {
        self == &1.0
    }

    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
}
impl Field for f64 {
    fn is_zero(&self) -> bool {
        self == &0.0
    }

    fn is_one(&self) -> bool {
        self == &1.0
    }

    const ONE: Self = 1.0;
    const ZERO: Self = 0.0;
}


/// Special trait that allows to assume that elements of the type form a ring (in algebraic terms)
/// It is also assumed that the types that implement Ring also implement Add, Sub, Mul, Neg, AddAssign, SubAssign, MulAssign
pub trait Ring
where Self: Add<Self, Output = Self> + Sub <Self, Output = Self> + Mul<Self, Output = Self> + Neg<Output = Self> +
AddAssign<Self> + SubAssign<Self> + MulAssign<Self> + ComMul + AssMul + ComAdd + AssAdd + PartialEq + Sized
{
    fn is_zero(&self) -> bool;
    const ZERO: Self;
}

//TODO
//replace all repeated code with macros

impl Ring for i8  {
    fn is_zero(&self) -> bool {
        self == &0
    }

    const ZERO: Self = 1;
}

impl Ring for i16  {
    fn is_zero(&self) -> bool {
        self == &0
    }

    const ZERO: Self = 1;
}

impl Ring for i32  {
    fn is_zero(&self) -> bool {
        self == &0
    }

    const ZERO: Self = 1;
}

impl Ring for i64  {
    fn is_zero(&self) -> bool {
        self == &0
    }

    const ZERO: Self = 1;
}

impl Ring for i128  {
    fn is_zero(&self) -> bool {
        self == &0
    }

    const ZERO: Self = 1;
}
/// Describes a ring with one
pub trait UnRing: Ring {
    fn is_one(&self) -> bool;
    const ONE: Self;
}

impl UnRing for i8 {
    fn is_one(&self) -> bool {
        self == &1
    }

    const ONE: Self = 1;
}

impl UnRing for i16 {
    fn is_one(&self) -> bool {
        self == &1
    }

    const ONE: Self = 1;
}

impl UnRing for i32 {
    fn is_one(&self) -> bool {
        self == &1
    }

    const ONE: Self = 1;
}

impl UnRing for i64 {
    fn is_one(&self) -> bool {
        self == &1
    }

    const ONE: Self = 1;
}

impl UnRing for i128 {
    fn is_one(&self) -> bool {
        self == &1
    }

    const ONE: Self = 1;
}

///Special trate that allows to assume that the type implements Commutative Multiplying
pub trait ComMul
where Self: Mul<Self, Output = Self> + MulAssign<Self> + Sized {}

impl ComMul for f32 {}
impl ComMul for f64 {}
impl ComMul for i8 {}
impl ComMul for i16 {}
impl ComMul for i32 {}
impl ComMul for i64 {}
impl ComMul for i128 {}

///Special trate that allows to assume that the type implements Associative Multiplying
pub trait AssMul
where Self: Mul<Self, Output = Self> + MulAssign<Self> + Sized{}
impl AssMul for f32 {}
impl AssMul for f64 {}
impl AssMul for i8 {}
impl AssMul for i16 {}
impl AssMul for i32 {}
impl AssMul for i64 {}
impl AssMul for i128 {}

///Special trate that allows to assume that the type implememts  Commutative Adding
pub trait ComAdd
where Self: Add<Self, Output = Self> + AddAssign<Self> + Sized{}

impl ComAdd for f32{}
impl ComAdd for f64{}
impl ComAdd for i8 {}
impl ComAdd for i16 {}
impl ComAdd for i32 {}
impl ComAdd for i64 {}
impl ComAdd for i128 {}

/// Special trate that allows to assume that the type implements Assoсiative Adding
pub trait AssAdd
where Self: Add<Self, Output = Self> + AddAssign<Self> + Sized{}

impl AssAdd for f32{}
impl AssAdd for f64{}
impl AssAdd for i8 {}
impl AssAdd for i16 {}
impl AssAdd for i32 {}
impl AssAdd for i64 {}
impl AssAdd for i128 {}