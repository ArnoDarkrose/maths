
pub mod checked_reducible {
    use crate::tech::*;
    use std::{fmt, 
        ops::{Mul, Add, Div, Sub, Neg, Rem}
    };

    pub trait RedBound = CheckIntegralDomain + CheckGcd + Meta where for <'a> &'a Self: 
    Add<&'a Self, Output = Option<Self>> + Sub<&'a Self, Output = Option<Self>> + Neg<Output = Option<Self>> + 
    Mul<&'a Self, Output = Option<Self>> + Div<&'a Self, Output = Option<Self>> + Rem<&'a Self, Output = Option<Self>>;

    #[derive(Debug, Clone)]
    pub struct CheckRed <T: RedBound> {
        num: T,
        denom: T
    }

    impl<T: RedBound> CheckRed<T> {
        pub fn new(num: T, denom: T) -> CheckRed<T> {
            if denom.is_zero() {
                panic!("Zero denominator");
            }

            let mut res = CheckRed{num, denom};
            
            res.simplify();
            
            res
        }

        pub fn num(&self) -> &T {
            &self.num
        }

        pub fn denom(&self) -> &T {
            &self.denom
        }

        pub fn simplify(&mut self) -> Option<()>{
            let gcd = self.num().gcd(self.denom())?;

            self.num = (self.num()/&gcd).expect("Never fails");

            self.denom = (self.denom()/&gcd).expect("Never fails");

            Some(())
        }

    }

    #[macro_export]
    macro_rules! chrdc {
        ($num:expr, $denom:expr) => {
            CheckRed::new($num, $denom)
        };


        ($typ:ty) => {
            CheckRed::new(<$typ>::non_zero(), <$typ>::non_zero())
        };

        ($num_denom:expr) => {
            CheckRed::new($num_denom.0, $num_denom.1)
        }
    }

    impl<T: RedBound> std::default::Default for CheckRed<T> {
        fn default() -> Self {
            CheckRed {num: T::non_zero(), denom: T::non_zero()}
        }
    }

    impl<T: RedBound + Clone> PartialEq for CheckRed<T> {
        fn eq(&self, other: &Self) -> bool {
            let mut overflowed = false;

            let lhs =  match &self.num * &other.denom {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let rhs = match &self.denom * &other.num {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };
            
            if !overflowed {
                return lhs == rhs;
            }

            let mut new_self = self.clone();
            new_self.simplify();

            let mut other = other.clone();
            other.simplify();

            let denom_gcd = new_self.denom().gcd(other.denom()).unwrap();

            let lhs = match new_self.num() * &((other.denom()/&denom_gcd).expect("Never fails")) {
                Some(val) => val,
                None => panic!("Failed to compare fractions")
            };

            let rhs = match other.num() * &((new_self.denom()/&denom_gcd).expect("Never fails")) {
                Some(val) => val,
                None => panic!("Failed to compare fractions")
            };

            lhs == rhs
        }
    }

    impl<T: RedBound + Clone> Eq for CheckRed<T> {}

    impl<T: RedBound + PartialOrd + Clone> PartialOrd for CheckRed<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let mut overflowed = false;

            let lhs = match &self.num * &other.denom {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let rhs = match &self.denom * &other.num {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };
            
            if !overflowed {
                return lhs.partial_cmp(&rhs)
            }

            let mut new_self = self.clone();
            new_self.simplify();

            let mut other = other.clone();
            other.simplify();

            let denom_gcd = self.denom().gcd(other.denom())?;

            let lhs = (new_self.num() * &((other.denom()/&denom_gcd).expect("Never fails")))?;

            let rhs = (other.num() * &((new_self.denom()/&denom_gcd).expect("Never fails")))?;

            lhs.partial_cmp(&rhs)
        }
    }

    impl<T: RedBound> Mul<Self> for &mut CheckRed<T> {
        type Output = Option<CheckRed<T>>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut overflowed = false;

            let num = match self.num() * rhs.num() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            if !overflowed {
                return Some(CheckRed {num, denom})
            }
        
            self.simplify();
            rhs.simplify();

            let s_num_r_denom_gcd = self.num().gcd(rhs.denom())?;
            let s_denom_r_num_gcd = self.denom().gcd(rhs.num())?;

            self.num = (self.num() / &s_num_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_num_r_denom_gcd).expect("Never fails");

            self.denom = (self.denom() / &s_denom_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_denom_r_num_gcd).expect("Never fails");

            let num = (self.num() * rhs.num())?;

            let denom = (self.denom() * rhs.denom())?;

            Some(CheckRed {num, denom})
        }
    }

    impl<T: RedBound + Clone> Mul<Self> for &CheckRed<T> {
        type Output = Option<CheckRed<T>>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut overflowed = false;

            let num = match self.num() * rhs.num() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            if !overflowed {
                return Some(CheckRed {num, denom})
            }

            let mut new_self = self.clone();
            let mut rhs = rhs.clone();

            new_self.simplify();
            rhs.simplify();

            let s_num_r_denom_gcd = self.num().gcd(rhs.denom())?;
            let s_denom_r_num_gcd = self.denom().gcd(rhs.num())?;

            new_self.num = (self.num() / &s_num_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_num_r_denom_gcd).expect("Never fails");

            new_self.denom = (self.denom() / &s_denom_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_denom_r_num_gcd).expect("Never fails");

            let num = (new_self.num() * rhs.num())?;

            let denom = (new_self.denom() * rhs.denom())?;

            Some(CheckRed {num, denom})
        }
    }

    impl<T: RedBound> Add<Self> for &mut CheckRed<T> {
        type Output = Option<CheckRed<T>>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut already_simplified = false;

            let mut denom_gcd;

            //here and a few lines later i declare a reference that'll be mainly used in calcualtions
            //but when overflow occurs i change multipliers. So second variable 
            //is for storing those values and the first is for taking references to them
            let mut self_mult = rhs.denom();
            let mut self_mult_val;
        
            let mut rhs_mult = self.denom();
            let mut rhs_mult_val;

            let mut new_denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {
                    self.simplify(); 
                    rhs.simplify(); 
                    
                    already_simplified = true; 

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    (self.denom() * self_mult)?
                }
            };

            let new_num_part1 = match self.num() * self_mult {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;
                    
                    (self_mult * self.num())?
                }
            };

            let new_num_part2 = match rhs_mult * rhs.num() {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;

                    (rhs_mult * rhs.num())?
                }
            };

            let new_num = match &new_num_part1 + &new_num_part2 {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None
                    }
                    
                    self.simplify();
                    rhs.simplify();

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;

                    (&(self_mult * self.num())? + &(rhs_mult * rhs.num())?)?
                }
            };

            Some(CheckRed {num: new_num, denom: new_denom})
        }
    }

    impl<T: RedBound + Clone> Add<Self> for &CheckRed<T> {
        type Output = Option<CheckRed<T>>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut new_self = self.clone();
            let mut rhs = rhs.clone();

            &mut new_self + &mut rhs
        }
    }

    impl<T: RedBound> Sub<Self> for &mut CheckRed<T> {
        type Output = Option<CheckRed<T>>;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut already_simplified = false;

            let mut denom_gcd;

            //here and a few lines later i declare a reference that'll be mainly used in calcualtions
            //but when overflow occurs i change multipliers. So the second variable 
            //is for storing those values and the first is for taking references to them
            let mut self_mult = rhs.denom();
            let mut self_mult_val;
        
            let mut rhs_mult = self.denom();
            let mut rhs_mult_val;

            let mut new_denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {
                    self.simplify(); 
                    rhs.simplify(); 
                    
                    already_simplified = true; 

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    (self.denom() * self_mult)?
                }
            };

            let new_num_part1 = match self.num() * self_mult {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;
                    
                    (self_mult * self.num())?
                }
            };

            let new_num_part2 = match rhs_mult * rhs.num() {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;

                    (rhs_mult * rhs.num())?
                }
            };

            let new_num = match &new_num_part1 - &new_num_part2 {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None
                    }
                    
                    self.simplify();
                    rhs.simplify();

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;

                    (&(self_mult * self.num())? - &(rhs_mult * rhs.num())?)?
                }
            };

            Some(CheckRed {num: new_num, denom: new_denom})
        }
    }

    impl<T: RedBound + Clone> Sub<Self> for &CheckRed<T> {
        type Output = Option<CheckRed<T>>;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut new_self = self.clone();
            let mut rhs = rhs.clone();
        
            &mut new_self - &mut rhs
        }
    }

    impl<T: RedBound> Div<Self> for &mut CheckRed<T> {
        type Output = Option<CheckRed<T>>;

        fn div(self, rhs: Self) -> Self::Output {
            if rhs.num.is_zero() {
                panic!("Dividing by zero");
            }

            let mut overflowed = false;

            let num = match self.num() * rhs.denom() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let denom = match self.denom() * rhs.num() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            if !overflowed {
                return Some(CheckRed {num, denom})
            }
        
            self.simplify();
            rhs.simplify();

            let s_num_r_num_gcd = self.num().gcd(rhs.num())?;
            let s_denom_r_denom_gcd = self.denom().gcd(rhs.denom())?;

            self.num = (self.num() / &s_num_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_num_r_num_gcd).expect("Never fails");

            self.denom = (self.denom() / &s_denom_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_denom_r_denom_gcd).expect("Never fails");

            let num = (self.num() * rhs.denom())?;

            let denom = (self.denom() * rhs.num())?;

            Some(CheckRed {num, denom})
        }
    }

    impl<T: RedBound + Clone> Div<Self> for &CheckRed<T> {
        type Output = Option<CheckRed<T>>;

        fn div(self, rhs: Self) -> Self::Output {
            if rhs.num.is_zero() {
                panic!("Dividing by zero");
            }

            let mut overflowed = false;

            let num = match self.num() * rhs.denom() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let denom = match self.denom() * rhs.num() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            if !overflowed {
                return Some(CheckRed {num, denom})
            }
        
            let mut new_self = self.clone();
            let mut rhs = rhs.clone();

            new_self.simplify();
            rhs.simplify();

            let s_num_r_num_gcd = new_self.num().gcd(rhs.num())?;
            let s_denom_r_denom_gcd = new_self.denom().gcd(rhs.denom())?;

            new_self.num = (new_self.num() / &s_num_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_num_r_num_gcd).expect("Never fails");

            new_self.denom = (new_self.denom() / &s_denom_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_denom_r_denom_gcd).expect("Never fails");

            let num = (new_self.num() * rhs.denom())?;

            let denom = (new_self.denom() * rhs.num())?;

            Some(CheckRed {num, denom})
        }
    }

    impl<T: RedBound> Neg for &mut CheckRed<T> {
        type Output = Option<CheckRed<T>>;

        fn neg(self) -> Self::Output {
            &mut CheckRed {num: T::zero(), denom: T::non_zero()} - self
        }
    }

    impl<T: RedBound + Clone> Neg for &CheckRed<T> {
        type Output = Option<CheckRed<T>>;

        fn neg(self) -> Self::Output {
            &CheckRed {num: T::zero(), denom: T::non_zero()} - self
        }
    }

    impl<T: RedBound> Meta for CheckRed<T> {
        fn name() -> String {
            format!("CheckRed<{}>", T::name())
        }

        fn non_zero() ->  Self {
            CheckRed {num: T::non_zero(), denom: T::non_zero()}
        }
    }

    impl<T: RedBound> Ass<AddOp> for CheckRed<T> {}
    impl<T: RedBound> Ass<MulOp> for CheckRed<T> {}
    impl<T: RedBound> Com<AddOp> for CheckRed<T> {}
    impl<T: RedBound> Com<MulOp> for CheckRed<T> {}

    impl<T: RedBound + Clone> Group<AddOp> for CheckRed<T> {
        fn neut() -> Self {
            CheckRed {num: T::zero(), denom: T::non_zero()}
        }
    }

    impl<T: RedBound + Clone> Group<MulOp> for CheckRed<T> {
        fn neut() -> Self {
            CheckRed {num: T::non_zero(), denom:T::non_zero()}
        }
    }

    impl<T: RedBound> Checked for CheckRed<T> {}

    impl<T: RedBound + Clone> CheckAddGroup for CheckRed<T> {
        fn is_zero(&self) -> bool {
            self.num.is_zero()
        }
    }
    impl<T: RedBound + Clone> CheckMulGroup for CheckRed<T> {
        fn is_one(&self) -> bool {
            self.num == self.denom
        }
    }

    impl<T: RedBound + Clone> Abelian<AddOp> for CheckRed<T> {}
    impl<T: RedBound + Clone> Abelian<MulOp> for CheckRed<T> {}

    impl<T: RedBound + Clone> CheckRing for CheckRed<T> {}

    impl<T: RedBound + Clone> CheckIntegralDomain for CheckRed<T> {}

    impl<T: RedBound + Clone> CheckField for CheckRed<T> {}

    impl<T: RedBound + fmt::Display> fmt::Display for CheckRed<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({})/({})", self.num, self.denom)
        }
    }
}

pub mod panic_reducible {
    use crate::tech::*;
    use std::{fmt, 
        ops::{Mul, Add, Div, Sub, Neg, Rem}
    };
    pub trait RedBound = CheckIntegralDomain + CheckGcd + Meta where for <'a> &'a Self: 
    Add<&'a Self, Output = Option<Self>> + Sub<&'a Self, Output = Option<Self>> + Neg<Output = Option<Self>> + 
    Mul<&'a Self, Output = Option<Self>> + Div<&'a Self, Output = Option<Self>> + Rem<&'a Self, Output = Option<Self>>;

    #[derive(Debug, Clone)]
    pub struct Reducible <T: RedBound> {
        num: T,
        denom: T
    }

    impl<T: RedBound> Reducible<T> {
        pub fn new(num: T, denom: T) -> Reducible<T> {
            if denom.is_zero() {
                panic!("Zero denominator");
            }

            let mut res = Reducible{num, denom};
            
            res.simplify();
            
            res
        }

        pub fn num(&self) -> &T {
            &self.num
        }

        pub fn denom(&self) -> &T {
            &self.denom
        }

        pub fn simplify(&mut self) {
            println!("Simplify");
            let gcd = self.num().gcd(self.denom()).expect("Failed to calculate gcd");

            self.num = (self.num()/&gcd).expect("Never fails");

            self.denom = (self.denom()/&gcd).expect("Never fails");
        }

    }

    #[macro_export]
    macro_rules! rdc {
        ($num:expr, $denom:expr) => {
            Reducible::new($num, $denom)
        };

        ($num_denom:expr) => {
            Reducible::new($num_denom.0, $num_denom.1)
        };

        ($typ:ty) => {
            Reducible::new(<$typ>::non_zero(), <$typ>::non_zero())
        }
    }

    impl<T: RedBound> std::default::Default for Reducible<T> {
        fn default() -> Self {
            Reducible {num: T::non_zero(), denom: T::non_zero()}
        }
    }

    impl<T: RedBound + Clone> PartialEq for Reducible<T> {
        fn eq(&self, other: &Self) -> bool {
            let mut overflowed = false;

            let lhs =  match &self.num * &other.denom {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let rhs = match &self.denom * &other.num {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };
            
            if !overflowed {
                return lhs == rhs;
            }

            let mut new_self = self.clone();
            new_self.simplify();

            let mut other = other.clone();
            other.simplify();

            let denom_gcd = new_self.denom().gcd(other.denom()).unwrap();

            let lhs = match new_self.num() * &((other.denom()/&denom_gcd).expect("Never fails")) {
                Some(val) => val,
                None => panic!("Failed to compare fractions")
            };

            let rhs = match other.num() * &((new_self.denom()/&denom_gcd).expect("Never fails")) {
                Some(val) => val,
                None => panic!("Failed to compare fractions")
            };

            lhs == rhs
        }
    }

    impl<T: RedBound + Clone> Eq for Reducible<T> {}

    impl<T: RedBound + PartialOrd + Clone> PartialOrd for Reducible<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let mut overflowed = false;

            let lhs = match &self.num * &other.denom {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let rhs = match &self.denom * &other.num {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };
            
            if !overflowed {
                return lhs.partial_cmp(&rhs)
            }

            let mut new_self = self.clone();
            new_self.simplify();
            
            let mut other = other.clone();
            other.simplify();

            let denom_gcd = self.denom().gcd(other.denom())?;

            let lhs = (new_self.num() * &((other.denom()/&denom_gcd).expect("Never fails")))?;

            let rhs = (other.num() * &((new_self.denom()/&denom_gcd).expect("Never fails")))?;

            lhs.partial_cmp(&rhs)
        }
    }

    impl<T: RedBound> Mul<Self> for &mut Reducible<T> {
        type Output = Reducible<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut overflowed = false;

            let num = match self.num() * rhs.num() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            if !overflowed {
                return Reducible {num, denom}
            }
        
            self.simplify();
            rhs.simplify();

            let s_num_r_denom_gcd = self.num().gcd(rhs.denom()).expect("Failed to calcuate gcd");
            let s_denom_r_num_gcd = self.denom().gcd(rhs.num()).expect("Failed to calcuate gcd");

            self.num = (self.num() / &s_num_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_num_r_denom_gcd).expect("Never fails");

            self.denom = (self.denom() / &s_denom_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_denom_r_num_gcd).expect("Never fails");

            let num = (self.num() * rhs.num()).expect("Failed to multiply");

            let denom = (self.denom() * rhs.denom()).expect("Failed to multiply");

            Reducible {num, denom}
        }
    }

    impl<T: RedBound + Clone> Mul<Self> for &Reducible<T> {
        type Output = Reducible<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut overflowed = false;

            let num = match self.num() * rhs.num() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            if !overflowed {
                return Reducible {num, denom}
            }

            let mut new_self = self.clone();
            let mut rhs = rhs.clone();

            new_self.simplify();
            rhs.simplify();

            let s_num_r_denom_gcd = self.num().gcd(rhs.denom()).expect("Failed to calculate gcd");
            let s_denom_r_num_gcd = self.denom().gcd(rhs.num()).expect("Failed to calculate gcd");

            new_self.num = (self.num() / &s_num_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_num_r_denom_gcd).expect("Never fails");

            new_self.denom = (self.denom() / &s_denom_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_denom_r_num_gcd).expect("Never fails");

            let num = (new_self.num() * rhs.num()).expect("Failed to multiply");

            let denom = (new_self.denom() * rhs.denom()).expect("Failed to multiply");

            Reducible {num, denom}
        }
    }

    impl<T: RedBound> Add<Self> for &mut Reducible<T> {
        type Output = Reducible<T>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut already_simplified = false;

            let mut denom_gcd;

            //here and a few lines later i declare a reference that'll be mainly used in calcualtions
            //but when overflow occurs i change multipliers. So second variable 
            //is for storing those values and the first is for taking references to them
            let mut self_mult = rhs.denom();
            let mut self_mult_val;
        
            let mut rhs_mult = self.denom();
            let mut rhs_mult_val;

            let mut new_denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {
                    self.simplify(); 
                    rhs.simplify(); 
                    
                    already_simplified = true; 

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    (self.denom() * self_mult).expect("Failed to multiply")
                }
            };

            let new_num_part1 = match self.num() * self_mult {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to add reducibles")
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");
                    
                    (self_mult * self.num()).expect("Failed to mutliply")
                }
            };

            let new_num_part2 = match rhs_mult * rhs.num() {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to add reducibles")
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");

                    (rhs_mult * rhs.num()).expect("Failed to multiply")
                }
            };

            let new_num = match &new_num_part1 + &new_num_part2 {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to add reducibles")
                    }
                    
                    self.simplify();
                    rhs.simplify();

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");

                    (&(self_mult * self.num()).expect("Failed to multiply") + &(rhs_mult * rhs.num()).expect("Failed to mutiply")).expect("Failed to multiply")
                }
            };

            Reducible {num: new_num, denom: new_denom}
        }
    }

    impl<T: RedBound + Clone> Add<Self> for &Reducible<T> {
        type Output = Reducible<T>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut new_self = self.clone();
            let mut rhs = rhs.clone();

            &mut new_self + &mut rhs
        }
    }

    impl<T: RedBound> Sub<Self> for &mut Reducible<T> {
        type Output = Reducible<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut already_simplified = false;

            let mut denom_gcd;

            //here and a few lines later i declare a reference that'll be mainly used in calcualtions
            //but when overflow occurs i change multipliers. So the second variable 
            //is for storing those values and the first is for taking references to them
            let mut self_mult = rhs.denom();
            let mut self_mult_val;
        
            let mut rhs_mult = self.denom();
            let mut rhs_mult_val;

            let mut new_denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {
                    self.simplify(); 
                    rhs.simplify(); 
                    
                    already_simplified = true; 

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    (self.denom() * self_mult).expect("Failed to multiply")
                }
            };

            let new_num_part1 = match self.num() * self_mult {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to subtract reducibles")
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");
                    
                    (self_mult * self.num()).expect("Failed to multiply")
                }
            };

            let new_num_part2 = match rhs_mult * rhs.num() {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to subtract reducibles")
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");

                    (rhs_mult * rhs.num()).expect("Failed to multiply")
                }
            };

            let new_num = match &new_num_part1 - &new_num_part2 {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to subtract reducibles")
                    }
                    
                    self.simplify();
                    rhs.simplify();

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");

                    (&(self_mult * self.num()).expect("Failed to multiply") - &(rhs_mult * rhs.num()).expect("Failed to multiply")).expect("Failed to multiply")
                }
            };

            Reducible {num: new_num, denom: new_denom}
        }
    }

    impl<T: RedBound + Clone> Sub<Self> for &Reducible<T> {
        type Output = Reducible<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut new_self = self.clone();
            let mut rhs = rhs.clone();
        
            &mut new_self - &mut rhs
        }
    }

    impl<T: RedBound> Div<Self> for &mut Reducible<T> {
        type Output = Reducible<T>;

        fn div(self, rhs: Self) -> Self::Output {
            if rhs.num.is_zero() {
                panic!("Dividing by zero");
            }

            let mut overflowed = false;

            let num = match self.num() * rhs.denom() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let denom = match self.denom() * rhs.num() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            if !overflowed {
                return Reducible {num, denom}
            }
        
            self.simplify();
            rhs.simplify();

            let s_num_r_num_gcd = self.num().gcd(rhs.num()).expect("Failed to calculate gcd");
            let s_denom_r_denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

            self.num = (self.num() / &s_num_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_num_r_num_gcd).expect("Never fails");

            self.denom = (self.denom() / &s_denom_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_denom_r_denom_gcd).expect("Never fails");

            let num = (self.num() * rhs.denom()).expect("Failed to multiply");

            let denom = (self.denom() * rhs.num()).expect("Failed to multiply");

            Reducible {num, denom}
        }
    }

    impl<T: RedBound + Clone> Div<Self> for &Reducible<T> {
        type Output = Option<Reducible<T>>;

        fn div(self, rhs: Self) -> Self::Output {
            if rhs.num.is_zero() {
                panic!("Dividing by zero");
            }

            let mut overflowed = false;

            let num = match self.num() * rhs.denom() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let denom = match self.denom() * rhs.num() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            if !overflowed {
                return Some(Reducible {num, denom})
            }
        
            let mut new_self = self.clone();
            let mut rhs = rhs.clone();

            new_self.simplify();
            rhs.simplify();

            let s_num_r_num_gcd = new_self.num().gcd(rhs.num())?;
            let s_denom_r_denom_gcd = new_self.denom().gcd(rhs.denom())?;

            new_self.num = (new_self.num() / &s_num_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_num_r_num_gcd).expect("Never fails");

            new_self.denom = (new_self.denom() / &s_denom_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_denom_r_denom_gcd).expect("Never fails");

            let num = (new_self.num() * rhs.denom())?;

            let denom = (new_self.denom() * rhs.num())?;

            Some(Reducible {num, denom})
        }
    }

    impl<T: RedBound> Neg for &mut Reducible<T> {
        type Output = Reducible<T>;

        fn neg(self) -> Self::Output {
            &mut Reducible {num: T::zero(), denom: T::non_zero()} - self
        }
    }

    impl<T: RedBound + Clone> Neg for &Reducible<T> {
        type Output = Reducible<T>;

        fn neg(self) -> Self::Output {
            &Reducible {num: T::zero(), denom: T::non_zero()} - self
        }
    }

    impl<T: RedBound> Mul for Reducible<T> {
        type Output = Reducible<T>;

        fn mul(mut self, mut rhs: Self) -> Self::Output {
            let mut overflowed = false;

            let num = match self.num() * rhs.num() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            if !overflowed {
                return Reducible {num, denom}
            }
        
            self.simplify();
            rhs.simplify();

            let s_num_r_denom_gcd = self.num().gcd(rhs.denom()).expect("Failed to calcuate gcd");
            let s_denom_r_num_gcd = self.denom().gcd(rhs.num()).expect("Failed to calcuate gcd");

            self.num = (self.num() / &s_num_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_num_r_denom_gcd).expect("Never fails");

            self.denom = (self.denom() / &s_denom_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_denom_r_num_gcd).expect("Never fails");

            let num = (self.num() * rhs.num()).expect("Failed to multiply");

            let denom = (self.denom() * rhs.denom()).expect("Failed to multiply");

            Reducible {num, denom}
            
        }
    }

    impl<T: RedBound> Div for Reducible<T> {
        type Output = Reducible<T>;

        fn div(mut self, mut rhs: Self) -> Self::Output {
            if rhs.num.is_zero() {
                panic!("Dividing by zero");
            }

            let mut overflowed = false;

            let num = match self.num() * rhs.denom() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            let denom = match self.denom() * rhs.num() {
                Some(val) => val,
                None => {overflowed = true; T::non_zero()}
            };

            if !overflowed {
                return Reducible {num, denom}
            }
        
            self.simplify();
            rhs.simplify();

            let s_num_r_num_gcd = self.num().gcd(rhs.num()).expect("Failed to calculate gcd");
            let s_denom_r_denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

            self.num = (self.num() / &s_num_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_num_r_num_gcd).expect("Never fails");

            self.denom = (self.denom() / &s_denom_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_denom_r_denom_gcd).expect("Never fails");

            let num = (self.num() * rhs.denom()).expect("Failed to multiply");

            let denom = (self.denom() * rhs.num()).expect("Failed to multiply");

            Reducible {num, denom}
        }
    }

    impl<T: RedBound> Add for Reducible<T> {
        type Output = Reducible<T>;

        fn add(mut self, mut rhs: Self) -> Self::Output {
            let mut already_simplified = false;

            let mut denom_gcd;

            //here and a few lines later i declare a reference that'll be mainly used in calcualtions
            //but when overflow occurs i change multipliers. So second variable 
            //is for storing those values and the first is for taking references to them
            let mut self_mult = rhs.denom();
            let mut self_mult_val;
        
            let mut rhs_mult = self.denom();
            let mut rhs_mult_val;

            let mut new_denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {
                    self.simplify(); 
                    rhs.simplify(); 
                    
                    already_simplified = true; 

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    (self.denom() * self_mult).expect("Failed to multiply")
                }
            };

            let new_num_part1 = match self.num() * self_mult {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to add reducibles")
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");
                    
                    (self_mult * self.num()).expect("Failed to mutliply")
                }
            };

            let new_num_part2 = match rhs_mult * rhs.num() {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to add reducibles")
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");

                    (rhs_mult * rhs.num()).expect("Failed to multiply")
                }
            };

            let new_num = match &new_num_part1 + &new_num_part2 {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to add reducibles")
                    }
                    
                    self.simplify();
                    rhs.simplify();

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");

                    (&(self_mult * self.num()).expect("Failed to multiply") + &(rhs_mult * rhs.num()).expect("Failed to mutiply")).expect("Failed to multiply")
                }
            };

            Reducible {num: new_num, denom: new_denom}
            
        }
    }

    impl<T: RedBound> Sub for Reducible<T> {
        type Output = Reducible<T>;

        fn sub(mut self, mut rhs: Self) -> Self::Output {
            let mut already_simplified = false;

            let mut denom_gcd;

            //here and a few lines later i declare a reference that'll be mainly used in calcualtions
            //but when overflow occurs i change multipliers. So the second variable 
            //is for storing those values and the first is for taking references to them
            let mut self_mult = rhs.denom();
            let mut self_mult_val;
        
            let mut rhs_mult = self.denom();
            let mut rhs_mult_val;

            let mut new_denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {
                    self.simplify(); 
                    rhs.simplify(); 
                    
                    already_simplified = true; 

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    (self.denom() * self_mult).expect("Failed to multiply")
                }
            };

            let new_num_part1 = match self.num() * self_mult {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to subtract reducibles")
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");
                    
                    (self_mult * self.num()).expect("Failed to multiply")
                }
            };

            let new_num_part2 = match rhs_mult * rhs.num() {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to subtract reducibles")
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");

                    (rhs_mult * rhs.num()).expect("Failed to multiply")
                }
            };

            let new_num = match &new_num_part1 - &new_num_part2 {
                Some(val) => val,
                None => {
                    if already_simplified {
                        panic!("Failed to subtract reducibles")
                    }
                    
                    self.simplify();
                    rhs.simplify();

                    denom_gcd = self.denom().gcd(rhs.denom()).expect("Failed to calculate gcd");

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult).expect("Failed to multiply");

                    (&(self_mult * self.num()).expect("Failed to multiply") - &(rhs_mult * rhs.num()).expect("Failed to multiply")).expect("Failed to multiply")
                }
            };

            Reducible {num: new_num, denom: new_denom}
        }
    }

    impl<T: RedBound> Neg for Reducible<T> {
        type Output = Reducible<T>;

        fn neg(self) -> Self::Output {
            Reducible {num: T::zero(), denom: T::non_zero()} - self
        }
    }

    impl<T: RedBound> Meta for Reducible<T> {
        fn name() -> String {
            format!("Reducible<{}>", T::name())
        }

        fn non_zero() ->  Self {
            Reducible {num: T::non_zero(), denom: T::non_zero()}
        }
    }

    impl<T: RedBound> Ass<AddOp> for Reducible<T> {}
    impl<T: RedBound> Ass<MulOp> for Reducible<T> {}
    impl<T: RedBound> Com<AddOp> for Reducible<T> {}
    impl<T: RedBound> Com<MulOp> for Reducible<T> {}

    impl<T: RedBound + Clone> Group<AddOp> for Reducible<T> {
        fn neut() -> Self {
            Reducible {num: T::zero(), denom: T::non_zero()}
        }
    }

    impl<T: RedBound + Clone> Group<MulOp> for Reducible<T> {
        fn neut() -> Self {
            Reducible {num: T::non_zero(), denom:T::non_zero()}
        }
    }

    impl<T: RedBound + Clone> AddGroup for Reducible<T> {
        fn is_zero(&self) -> bool {
            self.num.is_zero()
        }
    }

    impl<T: RedBound + Clone> MulGroup for Reducible<T> {
        fn is_one(&self) -> bool {
            self.num == self.denom
        }
    }

    impl<T: RedBound + Clone> Abelian<AddOp> for Reducible<T> {}
    impl<T: RedBound + Clone> Abelian<MulOp> for Reducible<T> {}

    impl<T: RedBound + Clone> Ring for Reducible<T> {}

    impl<T: RedBound + Clone> IntegralDomain for Reducible<T> {}

    impl<T: RedBound + Clone> Field for Reducible<T> {}

    impl<T: RedBound + fmt::Display> fmt::Display for Reducible<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({})/({})", self.num, self.denom)
        }
    }
}

pub mod classic_reducible {
    use std:: {
        fmt,
        ops::{Add, Sub, Mul, Div, Neg, Rem}
    };

    use crate::tech::*;

    pub trait ClassicRdcBound = IntegralDomain + Gcd + Meta where for <'a> &'a Self: Rem<&'a Self, Output = Self> + Div<&'a Self, Output = Self>;
    
    #[derive(Debug, Clone)]
    pub struct ClassicRdc <T: ClassicRdcBound> {
        num: T,
        denom: T,
    }

    impl<T: ClassicRdcBound> ClassicRdc<T> {
        pub fn new(num: T, denom:T) -> ClassicRdc<T> {
            if num == T::zero() {
                panic!("Zero denominator")
            }

            let mut res = ClassicRdc{num, denom};

            res.simplify();

            res
        }

        pub fn num(&self) -> &T {
            &self.num
        }
    
        pub fn denom(&self) -> &T {
            &self.denom
        }

        fn simplify(&mut self) {
            let gcd = self.num().gcd(self.denom());

            self.num = self.num() / &gcd;
            self.denom = self.denom() / &gcd;
        }
    }

    #[macro_export]
    macro_rules! clrdc{
        ($num:expr, $denom:expr) => {
            ClassicRdc::new($num, $denom)
        };

        ($num_denom:expr) => {
            ClassicRdc::new($num_denom.0, $num_denom.1)
        };

        ($typ:ty) => {
            ClassicRdc::<$typ>::new(<$typ>::non_zero(), <$typ>::non_zero())
        }
    }
    
    impl<T: ClassicRdcBound> std::default::Default for ClassicRdc<T> {
        fn default() -> Self {
            ClassicRdc {num: T::non_zero(), denom: T::non_zero()}
        }
    }
    impl<T: ClassicRdcBound> Mul for ClassicRdc<T> {
        type Output = Self;
    
        fn mul(self, rhs: Self) -> Self::Output {
            let mut res = Self {num: self.num * rhs.num, denom: self.denom * rhs.denom};

            res.simplify();

            res
        }
    }

    impl<T: ClassicRdcBound> Div for ClassicRdc<T> {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            Self::new(self.num * rhs.denom, self.denom * rhs.num)
        }
    }

    impl<T: ClassicRdcBound> Add for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let denom = &self.denom * &rhs.denom;
            
            let mut res = ClassicRdc {num: self.num * rhs.denom + self.denom * rhs.num, denom};

            res.simplify();

            res
        }
    }

    impl<T: ClassicRdcBound> Sub for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            let denom = &self.denom * &rhs.denom;

            let mut res = ClassicRdc {num: self.num * rhs.denom - self.denom * rhs.num, denom};

            res.simplify();

            res
        }
    }

    impl<T: ClassicRdcBound> Neg for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            ClassicRdc {num: T::zero(), denom: T::non_zero()} - self
        }
    }

    impl<T: ClassicRdcBound> PartialEq for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {
        fn eq(&self, other: &Self) -> bool {
            &self.num * &other.denom == &self.denom * &other.num
        }
    }

    impl<T: ClassicRdcBound> Eq for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {}

    impl<T: ClassicRdcBound + PartialOrd> PartialOrd for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            (&self.num * &other.denom).partial_cmp(&(&other.num * &self.denom))
        }
    }

    impl<T: ClassicRdcBound + Ord> Ord for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            (&self.num * &other.denom).cmp(&(&other.num * &self.denom))
        }
    }

    impl<T: ClassicRdcBound> Meta for ClassicRdc<T> {
        fn name() -> String {
            format!("ClassicRdc<{}>", T::name())
        }

        fn non_zero() ->  Self {
            ClassicRdc {num: T::non_zero(), denom: T::non_zero()}
        }
    }

    impl<T: ClassicRdcBound + fmt::Display> fmt::Display for ClassicRdc<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({})/({})", self.num, self.denom)
        }
    }

    impl<T: ClassicRdcBound> Com<AddOp> for ClassicRdc<T> {}
    impl<T: ClassicRdcBound> Com<MulOp> for ClassicRdc<T> {}
    impl<T: ClassicRdcBound> Ass<AddOp> for ClassicRdc<T> {}
    impl<T: ClassicRdcBound> Ass<MulOp> for ClassicRdc<T> {}

    impl<T: ClassicRdcBound> Group<MulOp> for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {
        fn neut() -> Self {
            ClassicRdc {num: T::non_zero(), denom: T::non_zero()}
        }
    }

    impl<T: ClassicRdcBound> Group<AddOp> for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T>{
        fn neut() -> Self {
            ClassicRdc {num: T::zero(), denom: T::non_zero()}
        }
    }

    impl<T: ClassicRdcBound> Abelian<MulOp> for ClassicRdc<T>where for <'a> &'a T: Mul<&'a T, Output = T> {}
    impl<T: ClassicRdcBound> Abelian<AddOp> for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T>{}

    impl<T: ClassicRdcBound> MulGroup for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T>{
        fn is_one(&self) -> bool {
            self.num == self.denom
        }
    }

    impl<T: ClassicRdcBound> AddGroup for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {
        fn is_zero(&self) -> bool {
            self.num.is_zero()
        }
    }

    impl<T: ClassicRdcBound> Ring for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {}

    impl<T: ClassicRdcBound> IntegralDomain for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {}

    impl<T: ClassicRdcBound> Field for ClassicRdc<T> where for <'a> &'a T: Mul<&'a T, Output = T> {}

}
