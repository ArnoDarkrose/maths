//! #Polynomial
//! 
//! Polynomial is a crate that provides functionality for working with a ring of polynomials

//TODO 
//rm int_pnm
pub mod int_pnm {
    use std::{
        ops::{ AddAssign, Add, Mul, MulAssign, Sub, SubAssign},
        cmp::max,
        vec,
        fmt,
        convert::TryInto,
    };

    #[derive(Debug, Clone, PartialEq)]
    pub struct Polynomial {
        ratios: Vec<i32>
    }

    impl Polynomial {
        pub fn new(mut ratios: Vec<i32>) -> Polynomial {
            ratios.reverse();
            let mut res = Polynomial { ratios: (ratios) };

            res.rm_lead_zero();

            res
        }

        pub fn new_monomial(val:i32, idx: usize) -> Polynomial {
            let mut res = vec![0; idx + 1];

            res[0] = val;

            Polynomial::new(res)
        }

        pub fn deg (& self) -> usize {
            let mut i = self.ratios.len() - 1;

            while i > 0 {
                if self.ratios[i] != 0 {
                    return i;
                }

                i -= 1;
            }

            0
        }

        pub fn rm_lead_zero(&mut self){
            self.ratios.truncate(self.deg() + 1);
        }

        pub fn is_zero(& self) -> bool {
            if self.deg() == 0 && self.ratios[0] == 0 {
                return true;
            }

            false
        }

        pub fn get(&self) -> &[i32] {
            &self.ratios[..]
        }

    }

    impl AddAssign for Polynomial {
        fn add_assign(&mut self, rhs: Self){
            let deg1 = self.deg();
            let deg2 = rhs.deg();
            
            if deg2 > deg1 {
                self.ratios.reserve_exact(deg2 - deg1);

                for _ in 0..(deg2 - deg1) {
                    self.ratios.push(0);
                }
            }

            let mut i = 0;
            while i < deg2 + 1 {
                self.ratios[i] += rhs.ratios[i];

                i += 1;
            }

            self.rm_lead_zero();
        }
    }

    impl Add for Polynomial {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            let deg1 = self.deg();
            let deg2 = rhs.deg();

            let mut res = Polynomial {ratios: vec![1; ((max(deg1, deg2) + 1) as i32).try_into().unwrap()]};
            let mut i = 0;

            while i < deg1 + 1 && i < deg2 + 1 {
                res.ratios[i] = self.ratios[i] + rhs.ratios[i];

                i += 1;
            }

            while i < deg1 + 1 {
                res.ratios[i] = self.ratios[i];

                i += 1;
            }

            while i < deg2 + 1 {
                res.ratios[i] = rhs.ratios[i];

                i+=1;
            } 

            res.rm_lead_zero();

            res
        }
    }

    impl Mul for Polynomial {
        type Output = Self;

        fn mul(self, rhs:Self) -> Self {
            let deg1 = self.deg();
            let deg2 = rhs.deg();

            let mut res = Polynomial {ratios: vec![0; deg1 + deg2 + 1]};
            for i in 0..(deg1 + 1) {
                for j in 0..(deg2 + 1) {
                    res.ratios[i + j] += self.ratios[i] * rhs.ratios[j];
                }
            }

            res.rm_lead_zero();

            res
        }
    }

    impl Mul<i32> for Polynomial {
        type Output = Polynomial;

        fn mul(self, rhs: i32) -> Self::Output {
            let mut res = Self { ratios: self.ratios.iter().map(|v| v * rhs).collect() };

            res.rm_lead_zero();

            res
        }
    }

    impl MulAssign for Polynomial {
        fn mul_assign(&mut self, rhs: Self) {
            let deg1 = self.deg();
            let deg2 = rhs.deg();
            
            let mut res = Polynomial{ratios: vec![0; deg1 + deg2 + 1]};
            for i in 0..(deg1 + 1) {
                for j in 0..(deg2 + 1) {
                    res.ratios[i + j] += self.ratios[i] * rhs.ratios[j];
                }
            }

            res.rm_lead_zero();

            *self = res;
        }
    }

    impl MulAssign<i32> for Polynomial {
        fn mul_assign(&mut self, rhs: i32) {
            for i in 0..(self.deg() + 1){
                self.ratios[i] *= rhs;
            }

            self.rm_lead_zero();
        }

    }

    impl fmt::Display for Polynomial {
        fn fmt (&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut i = self.deg();
            let mut cur_pow_ratio = self.ratios[i];
            if cur_pow_ratio != 0 {
                if i != 0 && i != 1 {
                    write!(f, "{}x^{} ", if cur_pow_ratio != 1 && cur_pow_ratio != -1  {cur_pow_ratio.to_string()} else {"".to_string()}, i).expect("failed to write polinom");
                } else if i == 1{
                    write!(f, "{}x ", if cur_pow_ratio != 1 && cur_pow_ratio != -1  {cur_pow_ratio.to_string()} else {"".to_string()}).expect("failed to write polinom");
                } else {
                    write!(f, "{} ", cur_pow_ratio).expect("failed to write polinom");
                }
            }
            i-= 1;

            while i > 0 {
                cur_pow_ratio = self.ratios[i];

                if cur_pow_ratio != 0 {
                    if i != 0 && i != 1 {
                        write!(f,
                            "{}{}x^{} ", 
                            if cur_pow_ratio < 0 {"- "} else {"+ "}, 
                            if cur_pow_ratio != 1 && cur_pow_ratio != -1 {cur_pow_ratio.abs().to_string()} else {"".to_string()},
                            i
                        ).expect("failed to write polinom");
                    } else if i == 1 {
                        write!(f,
                            "{}{}x ",
                            if cur_pow_ratio < 0 {"- "} else {"+ "},
                            if cur_pow_ratio != 1 && cur_pow_ratio != -1 {cur_pow_ratio.abs().to_string()} else {"".to_string()}
                        ).expect("failed to write polinom");
                    } else {
                        write!(f,"{}{} ", if cur_pow_ratio < 0 {"- "} else {"+ "}, cur_pow_ratio.abs()).expect("failed to write polinom");
                    }
                }
                i -= 1;
            }

            Ok(())
        }
    }

    impl Sub for Polynomial {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            let deg1 = self.deg();
            let deg2 = rhs.deg();

            let mut res = Polynomial{ratios: vec![1; ((max(deg1, deg2) + 1) as i32).try_into().unwrap()]};
            let mut i = 0;

            while i < deg1 + 1 && i < deg2 + 1 {
                res.ratios[i] = self.ratios[i] - rhs.ratios[i];

                i += 1;
            }

            while i < deg1 + 1 {
                res.ratios[i] = self.ratios[i];
                
                i += 1;
            }

            while i < deg2 + 1 {
                res.ratios[i] = -rhs.ratios[i];

                i += 1;
            } 

            res.rm_lead_zero();

            res
        }
    }
    
    impl SubAssign for Polynomial {
        fn sub_assign(&mut self, rhs: Self) {
            let deg1 = self.deg();
            let deg2 = rhs.deg();
            
            if deg2 > deg1 {
                self.ratios.reserve_exact(deg2 - deg1);

                for _ in 0..(deg2 - deg1) {
                    self.ratios.push(0);
                }
            }

            let mut i = 0;
            while i < deg2 + 1 {
                self.ratios[i] -= rhs.ratios[i];

                i += 1;
            }

            self.rm_lead_zero();
        }
    }
    
    impl Mul for &Polynomial {
        type Output = Polynomial;

        fn mul(self, rhs: Self) -> Self::Output {
            let deg1 = self.deg();
            let deg2 = rhs.deg();

            let mut res = Polynomial {ratios: vec![0; deg1 + deg2 + 1]};
            for i in 0..(deg1 + 1){
                for j in 0..(deg2 + 1) {
                    res.ratios[i + j] += self.ratios[i] * rhs.ratios[j];
                }
            }

            res.rm_lead_zero();

            res
        }
    }

    impl Mul<i32> for &Polynomial {
        type Output = Polynomial;

        fn mul(self, rhs: i32) -> Self::Output {
            let mut res = Self::Output { ratios: self.ratios.iter().map(|v| v * rhs).collect() };

            res.rm_lead_zero();

            res
        }
    }
    
    impl MulAssign<&Polynomial> for &mut Polynomial {
        fn mul_assign(&mut self, rhs: &Polynomial) {
            let deg1 = self.deg();
            let deg2 = rhs.deg();
            
            let mut res = Polynomial{ratios: vec![0; deg1 + deg2 + 1]};
            for i in 0..(deg1 + 1) {
                for j in 0..(deg2 + 1) {
                    res.ratios[i + j] += self.ratios[i] * rhs.ratios[j];
                }
            }

            res.rm_lead_zero();

            **self = res;
        }
    }

    impl MulAssign<i32> for &mut Polynomial {
        fn mul_assign(&mut self, rhs: i32) {
            let mut res = Polynomial {ratios: self.ratios.iter().map(|v| v * rhs).collect()};

            res.rm_lead_zero();

            **self = res;
        }
    }

    impl Sub for &Polynomial {
        type Output = Polynomial;

        fn sub(self, rhs: Self) -> Self::Output {
            let deg1 = self.deg();
            let deg2 = rhs.deg();

            let mut res = Polynomial{ratios: vec![1; ((max(deg1, deg2) + 1) as i32).try_into().unwrap()]};

            let mut i = 0;
            while i < deg1 + 1 && i < deg2 + 1 {
                res.ratios[i] = self.ratios[i] - rhs.ratios[i];

                i += 1;
            }

            while i < deg1 + 1 {
                res.ratios[i] = self.ratios[i];
                
                i += 1;
            }

            while i < deg2 + 1 {
                res.ratios[i] = -rhs.ratios[i];

                i += 1;
            } 

            res.rm_lead_zero();

            res
        }
    }

    impl SubAssign<&Polynomial> for &mut Polynomial {
        fn sub_assign(&mut self, rhs: &Polynomial) {
            let mut res = Polynomial{ratios: vec![1; ((max(self.deg(), rhs.deg()) + 1)as i32).try_into().unwrap()]};

            let mut i = 0;
            while i < self.deg() + 1 && i < rhs.deg() + 1 {
                res.ratios[i] = self.ratios[i] - rhs.ratios[i];

                i += 1;
            }

            while i < self.deg() + 1 {
                res.ratios[i] = self.ratios[i];

                i += 1;
            }

            while i < rhs.deg() + 1 {
                res.ratios[i] = -rhs.ratios[i];

                i += 1;
            } 

            res.rm_lead_zero();

            **self = res;
            
        }
    }

    impl Add for &Polynomial {
        type Output = Polynomial;

        fn add(self, rhs: Self) -> Self::Output {
            let deg1 = self.deg();
            let deg2 = rhs.deg();

            let mut res = Polynomial{ratios: vec![1; ((max(deg1, deg2) + 1) as i32).try_into().unwrap()]};

            let mut i = 0;
            while i < deg1 + 1 && i < deg2 + 1 {
                res.ratios[i] = self.ratios[i] + rhs.ratios[i];

                i += 1;
            }

            while i < deg1 + 1 {
                res.ratios[i] = self.ratios[i];

                i += 1;
            }

            while i < deg2 + 1 {
                res.ratios[i] = rhs.ratios[i];

                i+=1;
            } 

            res.rm_lead_zero();

            res
        }


    }
    
    impl AddAssign<&Polynomial> for &mut Polynomial {
        fn add_assign(&mut self, rhs: &Polynomial) {
            let deg1 = self.deg();
            let deg2 = rhs.deg();
            
            if deg2 > deg1 {
                self.ratios.reserve_exact(deg2 - deg1);

                for _ in 0..(deg2 - deg1) {
                    self.ratios.push(0);
                }
            }

            let mut i = 0;
            while i < deg2 + 1 {
                self.ratios[i] += rhs.ratios[i];

                i += 1;
            }
            
            self.rm_lead_zero();
        }
    }

    pub fn try_div_with_rem(f: & Polynomial, g: & Polynomial) -> Result<(Polynomial, Polynomial), &'static str> {
        let deg_f = f.deg();
        let deg_g = g.deg();

        if g.is_zero() {
            return Err("dividing by zero polynomial");
        }

        if deg_f < deg_g || deg_f == 0{
            return Ok((Polynomial::new(vec![0]), f.clone()));
        }

        if f.ratios[deg_f] % g.ratios[deg_g] != 0 {
            return Err("inaccurate subtraction")
        }

        let f1 = Polynomial::new_monomial(1, deg_f - deg_g);
        let f1 =  (&f1 * g) * (f.ratios[deg_f]/g.ratios[deg_g]);
        let f1 = f - &f1;

        if f1.is_zero() {
            return Ok((Polynomial::new_monomial(1, deg_f - deg_g) * (f.ratios[deg_f]/g.ratios[deg_g]), Polynomial::new(vec![0])));
        }

        let (q1, r1) = try_div_with_rem(&f1, g)?;

        let q = Polynomial::new_monomial(1, deg_f - deg_g) * (f.ratios[deg_f]/g.ratios[deg_g]) + q1;
        let r = r1;
        
        Ok((q, r))
    }
    
    pub fn gcd(f: &Polynomial, g: &Polynomial) -> Result<Polynomial, &'static str> {
        let (_q, r) = try_div_with_rem(f, g)?;
        if r.is_zero() {
            return Ok(g.clone());
        }

       gcd(g, &r)
    }
}

pub mod any_pnm{
    use std::{
        vec,
        ops :: {Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg},
        fmt::Display,
    };
    use tech::Field;

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
        
        pub fn is_zero(&self) -> bool {
            self.ratios[0].is_zero() && self.deg() == 0 
        }

        pub fn rm_lead_zero(&mut self) {
            self.ratios.truncate(self.deg() + 1);
        }
    
        pub fn new_monomial(val: T, idx: usize) -> Polynomial<T>  where T: Clone {
            let mut res = vec![T::ZERO; idx + 1];

            res[0] = val;

            Polynomial::new(res)
        }

        pub fn get(&self) -> &[T] {
            &self.ratios[..]
        }
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
                self.ratios.extend_from_slice(&vec![T::ZERO; r_deg - s_deg]);
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
                self.ratios.extend_from_slice(&vec![T::ZERO; deg2 - deg1]);
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
            self * (-T::ONE)
        }
    }

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

            let mut ratios = vec![T::ZERO; s_deg + r_deg + 1];

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
            let mut ratios = vec![T::ZERO; self.deg() + rhs.deg() + 1];

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
            Polynomial::new(self.ratios.iter().map(|v| v * rhs).collect())
        }
    }

    //TODO
    impl<T> Display for Polynomial<T>
    where T: Field + Display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
           Ok(()) 
        }
    } 

}

#[cfg(test)]
mod test {
    use crate::any_pnm::Polynomial;

    #[test]
    fn test_mul() {
        let a = Polynomial::new(vec![1.0, 1.0, 0.0]);
        let b = Polynomial::new(vec![1.0, 0.0]);

        let c = a * b;

        assert_eq!(c, Polynomial::new(vec![1.0, 1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_ref_sub () {
        let a = Polynomial::new(vec![1.0, 2.0, 3.0]);

        let b = Polynomial::new(vec![1.0, 0.0]);

        assert_eq!(&a - &b, Polynomial::new(vec![1.0, 1.0, 3.0]));
    }

    #[test]
    fn test_ref_add2() {
        let a = Polynomial::new(vec![5.0, 2.0, 3.0]);

        let b = Polynomial::new(vec![1.0]);
        
        assert_eq!(&a + &b, Polynomial::new(vec![5.0, 2.0, 4.0]));
    }
}


