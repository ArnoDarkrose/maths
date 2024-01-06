//TODO
//create the Rational struct

#[macro_use]
pub mod fract {
    use tech::{IntegralDomain, Meta, AssAdd, AssMul, ComAdd, ComMul, Ring, UnRing, Field, Gcd};

    use std::{
        ops::{Mul, MulAssign, Add, AddAssign, Sub, SubAssign, Div, DivAssign, Neg},
        fmt::Display,
    };

    #[macro_export]
    macro_rules! fract {
        ($num: expr, $denom:expr) => {
            Fraction::new($num, $denom)
        };
    }

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
            fract!(self.num * rhs.denom, self.denom * rhs.num)
        }
    }

    impl<T: IntegralDomain + Meta> DivAssign for Fraction<T> {
        fn div_assign(&mut self, rhs: Self) {
            self.num *= rhs.denom;
            self.denom *= rhs.num;

            if self.denom.is_zero() {
                panic!("Crearing a fraction with zero denom");
            }
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
        (for Reducible : $($i: ident),*) => {
            $(
                impl<T: IntegralDomain + Meta + Clone + DivAssign<T> + Gcd> $i for Reducible<T> where for <'a> &'a T: Mul<&'a T, Output = T> {}
            )*
        };

        ($($i: ident),*) => {
           $(
                impl<T: IntegralDomain + Meta + Clone> $i for Fraction <T> where for <'a> &'a T: Mul<&'a T, Output = T> {}
           )*
        };
    }

    implTrait!(ComAdd, ComMul, AssAdd, AssMul, IntegralDomain, Field);

    impl<T: IntegralDomain + Meta + Clone> Ring for Fraction<T> 
    where for <'a> &'a T: Mul<&'a T, Output = T> {
        fn zero() -> Self {
            Fraction {num: T::zero(), denom: T::non_zero()}
        }
    }

    impl<T: IntegralDomain + Meta + Clone> UnRing for Fraction<T>
    where for <'a> &'a T: Mul<&'a T, Output = T> {
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

    impl<T: IntegralDomain + Meta + Display> Display for Fraction<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}/{}", self.num, self.denom)
        }
    }

    #[derive(Debug, Clone)]
    pub struct Reducible <T: IntegralDomain + Meta + Gcd + DivAssign<T>> {
        fract: Fraction<T>,
        count: i32,
    }

    impl<T> Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone {
        fn simplify(&mut self) {
            if self.count < 4 {
                self.count += 1;

                return ()
            }
            
            let gcd = self.num().gcd(self.denom());

            self.fract.num /= gcd.clone();
            self.fract.denom /= gcd;
            
            self.count = 0;
        }

        pub fn num(&self) -> &T {
            &self.fract.num
        }

        pub fn denom(&self) -> &T {
            &self.fract.denom
        }

        pub fn new(num: T, denom: T) -> Reducible<T> {
            Reducible {fract: fract!(num, denom), count: 0}
        }
    }

    #[macro_export]
    macro_rules! reducible {
        ($num:expr, $denom:expr) => {
            Reducible::new($num, $denom)
        };
    }

    impl<T> PartialEq for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T>,
    for <'a> &'a T: Mul<&'a T, Output = T> {
        fn eq(&self, other: &Self) -> bool {
            self.fract == other.fract
        }
    }

    impl<T> Mul for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone{
        type Output = Reducible<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut res = Reducible {fract: self.fract * rhs.fract, count: self.count + rhs.count};

            res.simplify();

            res
        }
    }

    impl<T> Div for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone {
        type Output = Reducible<T>;

        fn div(self, rhs: Self) -> Self::Output {
            let mut res = Reducible {fract: self.fract/rhs.fract, count: self.count + rhs.count};

            res.simplify();

            res
        }
    }

    impl<T> Add for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone {
        type Output = Reducible<T>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut res = Reducible {fract: self.fract + rhs.fract, count: self.count + rhs.count};

            res.simplify();

            res
        }
    }

    impl<T> Sub for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone {
        type Output = Reducible<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut res = Reducible {fract: self.fract - rhs.fract, count: self.count + rhs.count};

            res.simplify();

            res
        }
    }

    impl<T> Neg for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone {
        type Output = Reducible<T>;

        fn neg(self) -> Self::Output {
            let mut res = Reducible {fract: -self.fract, count: self.count};

            res.simplify();

            res
        }
    }

    impl<T> AddAssign for Reducible<T> 
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone{
        fn add_assign(&mut self, rhs: Self) {
            self.fract += rhs.fract;
            self.count += rhs.count;

            self.simplify();
        }
    }

    impl<T> SubAssign for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone {
        fn sub_assign(&mut self, rhs: Self) {
            self.fract -= rhs.fract;
            self.count += rhs.count;

            self.simplify();
        }
    }
    
    impl<T> MulAssign for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone {
        fn mul_assign(&mut self, rhs: Self) {
            self.fract *= rhs.fract;
            self.count += rhs.count;

            self.simplify();
        }
    }

    impl<T> DivAssign for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone {
        fn div_assign(&mut self, rhs: Self) {
            self.fract /= rhs.fract;
            self.count += rhs.count;

            self.simplify();
        }
    }

    impl<T> Add for &Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T>{
        type Output = Reducible<T>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut res = Reducible {fract: &self.fract + &rhs.fract, count: self.count + rhs.count};

            res.simplify();

            res 
        }
    }

    impl<T> Sub for &Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T>{
        type Output = Reducible<T>;
        
        fn sub(self, rhs: Self) -> Self::Output {
            let mut res = Reducible {fract: &self.fract - &rhs.fract, count: self.count + rhs.count};

            res.simplify();

            res
        }
    }

    impl<T> Mul for &Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Reducible<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut res = Reducible{fract: &self.fract * &rhs.fract, count: self.count + rhs.count};

            res.simplify();

            res
        }
    }

    impl<T> Div for &Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Reducible<T>;
        
        fn div(self, rhs: Self) -> Self::Output {
            let mut res = Reducible{fract: &self.fract / &rhs.fract, count: self.count + rhs.count};

            res.simplify();

            res
        }
    }

    impl<T> Meta for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone {
        fn name () -> String {
            format!("Reducible<{}>", T::name())
        }

        fn non_zero () -> Self {
            Reducible {fract: Fraction::non_zero(), count: 0}
        }
    }

    impl<T> Display for Reducible<T> 
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone + Display{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.fract.fmt(f)
        }
    }


    implTrait!(for Reducible : ComAdd, ComMul, AssAdd, AssMul, IntegralDomain, Field);

    impl<T> Ring for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {
        fn zero() -> Self {
            Reducible {fract: Fraction::<T>::zero(), count: 0}
        }
    }

    impl <T> UnRing for Reducible<T>
    where T: IntegralDomain + Meta + Gcd + DivAssign<T> + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {
        fn one() -> Self {
            Reducible {fract: Fraction::<T>::one(), count: 0}
        }
    }
}
