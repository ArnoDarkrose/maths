
pub mod fract {
    use tech::*;
    use std::{
        cmp,
        ops::{Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign, Neg},
    };
    /*

    impl<T: IntegralDomain + Clone> Add<&Fraction<T>> for &Fraction<T>
    where for<'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Fraction<T>;

        fn add(self, rhs: &Fraction<T>) -> Self::Output {
            Fraction {num: &self.num * &rhs.denom + &self.denom * &rhs.num, denom: &self.denom * &rhs.denom}
        }
    }


    */

    #[derive(Debug, Clone)]
    pub struct Fraction <T: IntegralDomain> {
        num: T,
        denom: T,
    }

    macro_rules! defineFract {
        ($($val: tt : $typ: ty),*) => {
            $(
                impl cmp::PartialEq for Fraction<$typ>{
                    fn eq(&self, other: &Self) -> bool {
                        &self.num * &other.denom == &self.denom * &other.num
                    }
                }

                impl Mul for Fraction<$typ> {
                    type Output = Fraction<$typ>;

                    fn mul(self, rhs: Self) -> Self::Output {
                        Fraction {num: self.num * rhs.num, denom: self.denom * rhs.denom}
                    }
                }

                impl MulAssign for Fraction<$typ> {
                    fn mul_assign(&mut self, rhs: Self) {
                        self.num *= rhs.num;
                        self.denom *= rhs.denom;
                    }  
                }


                impl Div for Fraction<$typ> {
                    type Output = Fraction<$typ>;

                    fn div(self, rhs: Self) -> Self::Output {
                        Fraction {num: self.num * rhs.denom, denom: self.denom * rhs.num}            
                    }
                }

                impl DivAssign for Fraction<$typ> {
                    fn div_assign(&mut self, rhs: Self) {
                        self.num *= rhs.denom;
                        self.denom *= rhs.num;
                    }
                }

                impl Add for Fraction<$typ> {
                    type Output = Fraction<$typ>;

                    fn add(self, rhs: Self) -> Self::Output {
                        Fraction {num: self.num * rhs.denom.clone() + self.denom.clone() * rhs.num, denom: self.denom * rhs.denom}
                    }
                }

                impl AddAssign for Fraction<$typ>{
                    fn add_assign(&mut self, rhs: Self) {
                        let res = self.clone() + rhs;

                        self.num = res.num;
                        self.denom = res.denom;
                    } 
                }

                impl Sub for Fraction<$typ> {
                    type Output = Self;

                    fn sub(self, rhs: Self) -> Self::Output {
                        Fraction {num: self.num * rhs.denom.clone() - self.denom.clone() * rhs.num, denom: self.denom * rhs.denom}
                    }
                }

                impl SubAssign for Fraction<$typ> {
                    fn sub_assign(&mut self, rhs: Self) {
                        let res = self.clone() - rhs;

                        self.num = res.num;
                        self.denom = res.denom;
                    }
                }

                impl Neg for Fraction<$typ> {
                    type Output = Fraction<$typ>;

                    fn neg(self) -> Self::Output {
                        Fraction {num: self.num * (-$val), denom: self.denom * (-$val)}              
                    }
                }

                //TODO
                //impl & ops
                //make a concise way to initialize a fraction like a fract! macro

                impl ComAdd for Fraction <$typ> {}
                impl AssAdd for Fraction <$typ> {}
                impl ComMul for Fraction <$typ> {}
                impl AssMul for Fraction <$typ> {}

                impl Ring for Fraction <$typ> {
                    fn is_zero(&self) -> bool {
                        self.num.is_zero()
                    }

                    const ZERO: Fraction<$typ> = Fraction {num: <$typ>::ZERO, denom: $val};
                }

                impl UnRing for Fraction <$typ> {
                    fn is_one(&self) -> bool {
                        self.num == self.denom
                    }

                    const ONE: Fraction<$typ> = Fraction {num: $val, denom: $val};
                }

                impl IntegralDomain for Fraction <$typ> {}

                impl Field for Fraction <$typ> {}

                
                impl Fraction <$typ> {
                    pub fn new(num: $typ, denom: $typ) ->Fraction<$typ> {
                        Fraction {num, denom}
                    }
                }
            )*
        };
    }

    defineFract!(1.0 : f32, 1.0 : f64, 1: i8, 1 : i16, 1: i32, 1 : i64, 1 : i128);

}

#[cfg(test)]
mod test {
    use tech::UnRing;

    use crate::fract::*;
    #[test]
    fn test_i32() {
        
    }
}