use crate::tech::*;

pub struct Reducible <T: IntegralDomain + Meta + Checked> {
    num: T,
    denom: T
}

impl<T: IntegralDomain + Meta + Checked> Reducible<T> {
    pub fn new(num: T, denom: T) -> Reducible<T> {
        if denom == T::zero() {
            panic!("Zero denominator");
        }

        Reducible{num, denom}
    }

    pub fn num(&self) -> &T {
        &self.num
    }

    pub fn denom(&self) -> &T {
        &self.denom
    }
}