use crate::tech::{IntegralDomain, Meta, Com, Ass, AddGroup, AddOp, MulGroup, MulOp, Group, Abelian, Ring, Field};
use std::{
    fmt,
    ops::{Add, Sub, Mul, Div, Neg}
};
#[derive(Debug, Clone)]
pub struct GrowFract<T: IntegralDomain + Meta> {
    num: T,
    denom: T
}

impl<T: IntegralDomain + Meta> GrowFract<T> {
    pub fn new(num: T, denom: T) -> Self {
        if denom.is_zero() {
            panic!("Zero denominator");
        }

        GrowFract {num, denom}
    }

    pub fn num(&self) -> &T {
        &self.num
    }

    pub fn denom(&self) -> &T {
        &self.denom
    }
}

#[macro_export]
macro_rules! fract {
    ($num: expr, $denom : expr) => {
       GrowFract::new($num, $denom) 
    };

    ($num_denom: expr) => {
        GrowFract::new($num_denom.0, $num_denom.1)
    };

    ($typ: ty) => {
        GrowFract{num: $typ::non_zero(), denom: $typ::non_zero()}
    }
}

impl<T: IntegralDomain + Meta> std::default::Default for GrowFract<T> {
    fn default() -> Self {
        GrowFract{num: T::non_zero(), denom: T::non_zero()}
    }
}

impl<T: IntegralDomain + Meta> Mul for GrowFract<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        GrowFract {num: self.num * rhs.num, denom: self.denom * rhs.denom}
    }
}

impl<T: IntegralDomain + Meta> Div for GrowFract<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.num.is_zero() {
            panic!("Dividing by zero")
        }

        GrowFract {num: self.num * rhs.denom, denom: self.denom * rhs.num}
    }
}

impl<T: IntegralDomain + Meta> Add for GrowFract<T> 
where for <'a> &'a T: Mul<&'a T, Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let denom = &self.denom * &rhs.denom;

        GrowFract {num: self.num * rhs.denom + rhs.num * self.denom, denom}
    }
}

impl<T: IntegralDomain + Meta> Sub for GrowFract<T> 
where for <'a> &'a T: Mul<&'a T, Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let denom = &self.denom * &rhs.denom;

        GrowFract {num: self.num * rhs.denom - rhs.num * self.denom, denom}
    }
}

impl<T: IntegralDomain + Meta> Neg for GrowFract<T> 
where for <'a> &'a T: Mul<&'a T, Output = T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::zero() - self
    }
}

impl<T: IntegralDomain + Meta> PartialEq for GrowFract<T> 
where for <'a> &'a T: Mul<&'a T, Output = T> {
    fn eq(&self, other: &Self) -> bool {
        self.num() * other.denom() == self.denom() * other.num()
    }
}

impl<T: IntegralDomain + Meta> Eq for GrowFract<T> where for <'a> &'a T: Mul<&'a T, Output = T> {}

impl<T: IntegralDomain + Meta + PartialOrd> PartialOrd for GrowFract<T> 
where for <'a> &'a T: Mul<&'a T, Output = T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.num() * other.denom()).partial_cmp(&(other.num() * self.denom()))
    }
}

impl<T: IntegralDomain + Meta + Ord> Ord for GrowFract<T> 
where for <'a> &'a T: Mul<&'a T, Output = T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering{
        (self.num() * other.denom()).cmp(&(other.num() * self.denom()))
    }
}

impl<T: IntegralDomain + Meta> Meta for GrowFract<T> {
    fn name() -> String {
        format!("GrowFract<{}>", T::name())
    }

    fn non_zero() ->  Self {
        GrowFract {num: T::non_zero(), denom: T::non_zero()}
    }
}

impl<T: IntegralDomain + Meta + fmt::Display> fmt::Display for GrowFract<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})/({})", self.num, self.denom)
    }
}

impl<T: IntegralDomain + Meta> Com<AddOp> for GrowFract<T> {}
impl<T: IntegralDomain + Meta> Ass<AddOp> for GrowFract<T> {}
impl<T: IntegralDomain + Meta> Com<MulOp> for GrowFract<T> {}
impl<T: IntegralDomain + Meta> Ass<MulOp> for GrowFract<T> {}

impl<T: IntegralDomain + Meta> Group<MulOp> for GrowFract<T> where for <'a> &'a T: Mul<&'a T, Output = T>{
    fn neut() -> Self {
        GrowFract {num: T::non_zero(), denom: T::non_zero()}
    }
}

impl<T: IntegralDomain + Meta> Group<AddOp> for GrowFract<T> where for <'a> &'a T: Mul<&'a T, Output = T>{
    fn neut() -> Self {
        GrowFract {num: T::zero(), denom: T::non_zero()}
    }
}

impl<T: IntegralDomain + Meta> Abelian<MulOp> for GrowFract<T> where for <'a> &'a T: Mul<&'a T, Output = T> {}
impl<T: IntegralDomain + Meta> Abelian<AddOp> for GrowFract<T> where for <'a> &'a T: Mul<&'a T, Output = T> {}

impl<T: IntegralDomain + Meta> MulGroup for GrowFract<T> where for <'a> &'a T: Mul<&'a T, Output = T> {
    fn is_one(&self) -> bool {
        self.num == self.denom
    }
}

impl<T: IntegralDomain + Meta> AddGroup for GrowFract<T> where for <'a> &'a T: Mul<&'a T, Output = T> {
    fn is_zero(&self) -> bool {
        self.num.is_zero()
    }
}

impl<T: IntegralDomain + Meta> Ring for GrowFract<T> where for <'a> &'a T: Mul<&'a T, Output = T> {}

impl<T: IntegralDomain + Meta> IntegralDomain for GrowFract<T> where for <'a> &'a T: Mul<&'a T, Output = T> {}

impl<T: IntegralDomain + Meta> Field for GrowFract<T> where for <'a> &'a T: Mul<&'a T, Output = T> {}
