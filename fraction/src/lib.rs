pub mod fract {
    use tech::*;
    use std::{
        fmt::Display,
        ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg},
    };

    trait IntoFraction 
    where Self: Field {
        fn into_fraction(self)->Fraction<Self>;
    }

    impl<T: Field> IntoFraction for T {
        fn into_fraction(self)->Fraction<Self> {
            Fraction {num: self, denom:Self::ONE}
        }
    }
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

}