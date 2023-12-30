pub mod fract {
    use tech::*;
    use std::{
        fmt::Display,
        ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg},
        cmp,
    };

    trait IntoFraction 
    where Self: Field {
        fn into_fraction(self)->Fraction<Self>;
    }

    //TODO
    //make an efficient macros implementation of an IntoFraction trait for all Fields except for Fraction

    #[derive(Debug, Clone)]
    pub struct Fraction <T: IntegralDomain> {
        num: T,
        denom: T,
    }

    impl<T: IntegralDomain> Fraction<T> {
        pub fn new (num: T, denom: T) -> Fraction<T> {
            if denom.is_zero() {panic!("zero denominator")};

            Fraction {num, denom}
        }
    }

    impl<T: IntegralDomain + Display> Display for Fraction<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}/{}", self.num, self.denom)
        }
    }

    /*
    impl<T: IntegralDomain, U:IntegralDomain> Mul<Fraction<U>> for Fraction<T> {
        type Output = ;

    } 
    */

    impl<T, U> PartialEq<Fraction<U>> for Fraction<T> 
    where T:IntegralDomain + IntoFraction,
    U: IntegralDomain + IntoFraction {
        fn eq(&self, other: &Fraction<U>) -> bool {
            self.num.into_fraction() * other.denom.into_fraction() == self.denom.into_fraction() * other.num.into_fraction()
        }
    }
}