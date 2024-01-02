#[macro_use]
pub mod fract {
    use tech::{IntegralDomain, Meta, AssAdd, AssMul, ComAdd, ComMul, Ring, UnRing, Field};

    use std::ops::{Mul, MulAssign, Add, AddAssign, Sub, SubAssign, Div, DivAssign, Neg};

    #[derive(Debug, Clone)]
    pub struct Fraction <T: IntegralDomain + Meta> {
        num: T,
        denom: T,
    }

    impl <T: IntegralDomain + Meta> PartialEq for Fraction<T>
    where for <'a> &'a T: Mul<&'a T, Output = T> {
        fn eq(&self, other: &Self) -> bool {
            &self.num * &other.denom == &self.denom * &other.num
        }
    }

    impl<T: IntegralDomain + Meta> Mul for Fraction<T> {
        type Output = Fraction<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            Fraction {num: self.num * rhs.num, denom: self.denom * rhs.denom}
        }
    }

    impl<T: IntegralDomain + Meta> MulAssign for Fraction<T> {
        fn mul_assign(&mut self, rhs: Self) {
            self.num *= rhs.num;
            self.denom *= rhs.denom;
        }
    }

    impl<T: IntegralDomain + Meta> Div for Fraction<T> {
        type Output = Fraction<T>;

        fn div(self, rhs: Self) -> Self::Output {
            Fraction {num: self.num * rhs.denom, denom: self.denom * rhs.num}            
        }
    }

    impl<T: IntegralDomain + Meta> DivAssign for Fraction<T> {
        fn div_assign(&mut self, rhs: Self) {
            self.num *= rhs.denom;
            self.denom *= rhs.num;
        }
    }

    impl<T: IntegralDomain + Clone + Meta> Add for Fraction<T> {
        type Output = Fraction<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Fraction {num: self.num * rhs.denom.clone() + self.denom.clone() * rhs.num, denom: self.denom * rhs.denom}
        }
    }

    impl<T: IntegralDomain + Clone + Meta> AddAssign for Fraction<T> {
        fn add_assign(&mut self, rhs: Self) {
            let res = self.clone() + rhs;

            self.num = res.num;
            self.denom = res.denom;
        }
    }

    impl<T: IntegralDomain + Clone + Meta> Sub for Fraction<T> {
        type Output = Fraction<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            Fraction {num: self.num * rhs.denom.clone() - self.denom.clone() * rhs.num, denom: self.denom * rhs.denom}
        }
    }

    impl<T: IntegralDomain + Clone + Meta> SubAssign for Fraction<T> {
        fn sub_assign(&mut self, rhs: Self) {
            let res = self.clone() - rhs;

            self.num = res.num;
            self.denom = res.denom;
        }
    }

    impl<T: IntegralDomain + Meta> Neg for Fraction<T> {
        type Output = Fraction<T>;
        
        fn neg(self) -> Self::Output {
            Fraction {num: self.num * (T::zero() - T::non_zero()), denom: self.denom * T::non_zero()}
        }
    }

    impl<T: IntegralDomain + Meta> Add for &Fraction<T>
    where for <'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Fraction<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Fraction {num: &self.num * &rhs.denom + &self.denom * &rhs.num, denom: &self.denom * &rhs.denom}
        }
    }

    impl<T: IntegralDomain + Meta> Sub for &Fraction<T> 
    where for <'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Fraction<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            Fraction {num: &self.num * &rhs.denom - &self.denom * &rhs.num, denom: &self.denom * &rhs.denom}
        } 
    }

    impl<T: IntegralDomain + Meta> Mul for &Fraction<T> 
    where for <'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Fraction<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            Fraction {num: &self.num * &rhs.num, denom: &self.denom * &rhs.denom}
        }
    }

    impl<T: IntegralDomain + Meta> Div for &Fraction<T>
    where for <'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Fraction<T>;
    
        fn div(self, rhs: Self) -> Self::Output {
            Fraction {num: &self.num * &rhs.denom, denom: &self.denom * &rhs.num}
        }
    }

    macro_rules! implTrait {
        ($($i: ident),*) => {
           $(
            impl<T: IntegralDomain + Meta + Clone> $i for Fraction <T> where for <'a> &'a T: Mul<&'a T, Output = T> {}
           )*
        };
    }

    implTrait!(ComAdd, ComMul, AssAdd, AssMul, IntegralDomain, Field);

    impl<T: IntegralDomain + Meta + Clone> Ring for Fraction<T> 
    where for <'a> &'a T: Mul<&'a T, Output = T> {
        fn is_zero(&self) -> bool {
            self.num.is_zero()
        }

        fn zero() -> Self {
            Fraction {num: T::zero(), denom: T::non_zero()}
        }
    }

    impl<T: IntegralDomain + Meta + Clone> UnRing for Fraction<T>
    where for <'a> &'a T: Mul<&'a T, Output = T> {
        fn is_one(&self) -> bool {
            self.num == self.denom
        }

        fn one() -> Self {
            Fraction {num: T::non_zero(), denom: T::non_zero()}
        } 
    }

    impl<T: IntegralDomain + Meta + Clone> Meta for Fraction<T> {
        fn name () -> String {
            format!("Fraction<{}>", T::name())
        }

        fn non_zero () -> Self {
            Fraction {num: T::non_zero(), denom: T::non_zero()}
        }
    }

    impl<T: IntegralDomain + Meta> Fraction<T> {
        pub fn new(num: T, denom: T) -> Fraction<T> {
            if denom.is_zero() {
                panic!("Creating a Fraction with zero denom");
            }

            Fraction {num, denom}
        }

        pub fn denom(&self) -> &T {
            &self.denom
        }

        pub fn num(&self) -> &T {
            &self.num
        }
    }

    #[allow(unused_macros)]
    macro_rules! fract {
        ($num: expr, $denom:expr) => {
            Fraction::new($num, $denom)
        };
    }
}