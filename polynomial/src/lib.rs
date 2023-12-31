//! #Polynomial
//! 
//! Polynomial is a crate that provides functionality for working with a ring of polynomials

#[macro_use]
pub mod any_pnm{
    use std::{
        vec,
        ops :: {Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg, Div, Rem},
        fmt::Display,
    };
    use tech::{Field, Ring, AssAdd, ComAdd, AssMul, ComMul, UnRing, IntegralDomain, Meta, Gcd};

    #[derive(PartialEq, Debug, Clone)]
    pub struct Polynomial <T: Field> {
        ratios: Vec<T>,
    } 

    impl<T> Polynomial<T> where T : Field {
        pub fn new(mut ratios: Vec<T>) -> Polynomial<T> {
            ratios.reverse();
            let mut res = Polynomial {ratios};

            res.rm_lead_zero();

            res
        }

        pub fn deg(&self) -> usize {
            let mut i = self.ratios.len() - 1;

            while i > 0 {
                if !self.ratios[i].is_zero() {
                    return i;
                }

                i -= 1;
            }

            0
        }
        
        pub fn rm_lead_zero(&mut self) {
            self.ratios.truncate(self.deg() + 1);
        }
    
        pub fn new_monomial(val: T, idx: usize) -> Polynomial<T>  where T: Clone {
            let mut res = vec![T::zero(); idx + 1];

            res[0] = val;

            Polynomial::new(res)
        }

        pub fn get(&self) -> &[T] {
            &self.ratios[..]
        }

        //is not overflow safe
        pub fn evaluate(self, val: T) -> T 
        where T: std::iter::Sum + Clone,
        for <'a> &'a T: Mul<&'a T, Output = T>{
            self.ratios.into_iter().enumerate().map(
                |v| {
                    fn fast_pow<T>(n: &T, pow: usize) -> T 
                    where T: Field + Clone,
                    for <'a> &'a T: Mul<&'a T, Output = T>{
                        let mut cur_n = n.clone();

                        let mut cur_pow = 1;
                        while cur_pow < pow {
                            if cur_pow + 1 == pow {
                                cur_n *= n.clone();
                                cur_pow += 1;
                            } else if cur_pow * 2 <= pow {
                                cur_n = &cur_n * &cur_n;

                                cur_pow *= 2;
                            } else {
                                cur_n = fast_pow::<T>(n, pow - cur_pow);

                                cur_pow = pow;
                            }
                        }

                        cur_n
                    }
                    
                    let(pow, cur_ratio) = v;

                    if cur_ratio != T::zero() {
                        cur_ratio * fast_pow::<T>(&val, pow)
                    } else {
                        T::zero()
                    }
                }
            ).sum()
        }
    }

    #[macro_export]
    macro_rules! polynom {
        ($($rat: expr),*) => {
            {
                let ratios = vec![$($rat),*];

                Polynomial::new(ratios)
            }
        };
    }

    #[macro_export]
    macro_rules! monom {
        ($val:expr ; $idx:expr) => {
            Polynomial::new_monomial($val, $idx)
        };
    }

    impl<T> Add for Polynomial<T> where T: Field + Clone{
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output{
            let mut res = self;

            res += rhs;

            res
        }
    }

    impl<T> AddAssign for Polynomial<T> where T: Field + Clone{
        fn add_assign(&mut self, rhs: Self) {
            let s_deg = self.deg();
            let r_deg = rhs.deg();

            if s_deg < r_deg {
                self.ratios.extend_from_slice(&vec![T::zero(); r_deg - s_deg]);
            }

            let r_iter = rhs.ratios.into_iter();

            for (i, val) in r_iter.enumerate() {
                self.ratios[i] += val;
            }

            self.rm_lead_zero();
        }
    }
    
    impl<T> Sub for Polynomial<T> where T: Field + Clone {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut res = self;

            res -= rhs;

            res
        }
    }

    impl<T> SubAssign for Polynomial<T> where T: Field + Clone {
        fn sub_assign(&mut self, rhs: Self) {
            let deg1 = self.deg();
            let deg2 = rhs.deg();

            if deg1 < deg2 {
                self.ratios.extend_from_slice(&vec![T::zero(); deg2 - deg1]);
            }

            let r_iter = rhs.ratios.into_iter();

            for(i, val) in r_iter.enumerate() {
                self.ratios[i] -= val;
            }

            self.rm_lead_zero();
        }
    }

    impl<T> Mul<T> for Polynomial<T> where T: Field + Clone,
    for<'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Self;

        fn mul(self, rhs: T) -> Self::Output {
            let mut res = self;
            res *= rhs;

            res
        }
    }

    impl<T> MulAssign<T> for Polynomial<T> 
    where T: Field + Clone,
    for<'a> &'a T: Mul<&'a T, Output = T> {
        fn mul_assign(&mut self, rhs: T) {
            self.ratios = self.ratios.iter().map(|v| v * &rhs).collect();
        }
    }

    impl<T> Neg for Polynomial<T> where T: Field + Clone,
    for<'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            self * (-T::one())
        }
    }

    //I dont like that i have to be able to multiply T refs in order to multiply Polynomial::<T>
    impl<T> Mul for Polynomial<T> 
    where T: Field + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Polynomial<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut res = self;
            res *= rhs;

            res
        }
    }

    impl<T> MulAssign for Polynomial<T>
    where T: Field + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {
        fn mul_assign(&mut self, rhs: Self) {
            let s_deg = self.deg();
            let r_deg = rhs.deg();

            let mut ratios = vec![T::zero(); s_deg + r_deg + 1];

            for i in 0..(s_deg + 1) {
                for j in 0..(r_deg + 1) {
                    ratios[i + j] += &self.ratios[i] * &rhs.ratios[j];
                }
            }

            let mut res = Polynomial {ratios};
            res.rm_lead_zero();

            *self = res; 
        }
    }
    
    impl<T> Add for &Polynomial<T>
    where T: Field + Clone,
    for <'a> &'a T: Add<&'a T, Output = T>{
        type Output = Polynomial<T>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut r_iter = rhs.ratios.iter();

            let mut ratios: Vec<_> = self.ratios.iter().map(
                |v| {
                    if let Some(r_val) = r_iter.next() {
                        v + r_val
                    } else {
                        v.clone()
                    }
                }
            ).collect();

            for r_val in r_iter {
                ratios.push(r_val.clone());
            }

            
            let mut res = Polynomial{ratios};
            res.rm_lead_zero();

            res
        }
    }

    impl<T> Sub for &Polynomial<T>
    where T: Field + Clone,
    for <'a> &'a T: Sub<&'a T, Output = T> + Neg<Output = T> {
        type Output = Polynomial<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut r_iter = rhs.ratios.iter();

            let mut ratios:Vec<_> = self.ratios.iter().map(
                    |v| {
                        if let Some(r_val) = r_iter.next() {
                            v - r_val
                        } else {
                            v.clone()
                        }
                    }
            ).collect();


            for r_val in r_iter {
                ratios.push(-r_val);
            }

            let mut res = Polynomial {ratios};
            res.rm_lead_zero();

            res
        }
    }

    impl<T> Mul for &Polynomial<T>
    where T: Field + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Polynomial<T>;
        
        fn mul(self, rhs: Self) -> Self::Output {
            let mut ratios = vec![T::zero(); self.deg() + rhs.deg() + 1];

            for (i, s_val) in self.ratios.iter().enumerate() {
                for (j, r_val) in rhs.ratios.iter().enumerate() {
                    ratios[i + j] += s_val * r_val;
                }
            }

            let mut res = Polynomial {ratios};
            res.rm_lead_zero();

            res
        }
    } 

    impl<T> Mul<&T> for &Polynomial<T>
    where T: Field + Clone,
    for<'a> &'a T: Mul<&'a T, Output = T> {
        type Output = Polynomial<T>;

        fn mul(self, rhs: &T) -> Self::Output {
            Polynomial {ratios: self.ratios.iter().map(|v| v * rhs).collect()}
        }
    }

    impl<T> Display for Polynomial<T> 
    where T: Field,
    for <'a> &'a T: Display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut ratios_it = self.ratios.iter().enumerate().rev(); 
            let (i, cur_pow_ratio) = ratios_it.next().expect("the iterator returned None while displaying polynomial");
            
            if *cur_pow_ratio != T::zero() {
                if i != 0 && i != 1 {
                    write!(
                        f, 
                        "{}x^{} ", 
                        if *cur_pow_ratio == T::one() {"".to_string()} else {cur_pow_ratio.to_string()},
                        i
                    )?;
                } else if i == 1 {
                    write!(
                        f, 
                        "{}x ", 
                        if *cur_pow_ratio == T::one() {"".to_string()} else {cur_pow_ratio.to_string()},
                    )?;
                } else {
                    write!(
                        f,
                        "{} ",
                        cur_pow_ratio
                    )?;
                }
            }
            
            for(i, cur_pow_ratio) in ratios_it {
                if *cur_pow_ratio != T::zero() {
                    if i != 0 && i != 1 {
                        write!(
                            f,
                            "+ {}x^{} ",
                            if *cur_pow_ratio == T::one() {"".to_string()} else {cur_pow_ratio.to_string()},
                            i
                        )?;
                    } else if i == 1 {
                        write!(
                            f,
                            "+ {}x ",
                            if *cur_pow_ratio == T::one() {"".to_string()} else {cur_pow_ratio.to_string()},
                        )?;
                    } else {
                        write!(
                            f,
                            "+ {} ",
                            cur_pow_ratio
                        )?;
                    }
                }
            }

            Ok(())
        }
    }

    impl<T: Field + Clone> Polynomial<T>
    where for <'a> &'a T: Mul<&'a T, Output = T> +  Div<&'a T, Output = T> + Neg<Output = T> + Sub<&'a T, Output = T>{
        pub fn div_with_rem(&self, rhs: &Polynomial<T>) -> (Polynomial<T>, Polynomial<T>) {
            if rhs.is_zero() {
                panic!("dividing by zero Polynomial");
            }

            let deg_s = self.deg();
            let deg_r = rhs.deg();

            if deg_s < deg_r {
                return (polynom!(T::zero()), self.clone());
            }

            let sub_ratio = &self.ratios[deg_s]/&rhs.ratios[deg_r];

            let f1 = monom!(T::one(); deg_s - deg_r);
            let f1 = &(&f1 * rhs) * (&sub_ratio);
            let f1 = self - &f1;

            if f1.is_zero() {
                return (&monom!(T::one(); deg_s - deg_r) * (&sub_ratio), polynom!(T::zero()));
            }

            let (q1, r1) = f1.div_with_rem(rhs);

            let q = monom!(T::one(); deg_s - deg_r) * (sub_ratio) + q1;
            let r = r1;

            (q, r)
                
            }
    }

    impl<T: Field + Clone> Rem<&Polynomial<T>> for &Polynomial<T>
    where for <'a> &'a T: Mul<&'a T, Output = T> + Div<&'a T, Output = T> + Neg<Output = T> + Sub<&'a T, Output = T> {
        type Output = Polynomial<T>;

        fn rem(self, rhs: &Polynomial<T>) -> Self::Output {
            let (_, r) = self.div_with_rem(&rhs);

            r
        }
    }

    impl<T: Field + Clone> Gcd for Polynomial<T>
    where for <'a> &'a T: Mul<&'a T, Output = T> +  Div<&'a T, Output = T> + Neg<Output = T> + Sub<&'a T, Output = T>{
        fn gcd(&self, rhs: &Self) -> Self{
            let r = self % rhs;

            if r.is_zero() {
                return rhs.clone();
            }

            r.gcd(rhs)
        }
    }

    impl<T: Field + Clone> AssAdd for Polynomial<T> {}
    impl<T: Field + Clone> ComAdd for Polynomial<T> {}

    impl<T> Ring for Polynomial<T>
    where T: Field + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {
        fn zero() -> Polynomial<T> {
            Polynomial::new(vec![T::zero()])
        }
    }

    impl<T> UnRing for Polynomial<T>
    where T: Field + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T>{
       fn one() -> Self {
           Polynomial::new(vec![T::one()])
       }
    }

    impl<T> AssMul for Polynomial<T> 
    where T: Field + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {}

    impl<T> ComMul for Polynomial<T>
    where T: Field + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {}

    impl<T> IntegralDomain for Polynomial<T>
    where T: Field + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {}

    impl<T> Meta for Polynomial<T>
    where T: Field + Meta + Clone,
    for <'a> &'a T: Mul<&'a T, Output = T> {
        fn non_zero () -> Self {
            Polynomial::<T>::one()
        }

        fn name () -> String {
            format!("Polynomial<{}>", T::name())
        }
    }

}
