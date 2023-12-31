pub mod imgn {
    use tech::{AssAdd, AssMul, ComAdd, ComMul, Ring, UnRing, IntegralDomain, Field, Meta};
    use std::{
        fmt::Display,
        ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg}
    };
    
    pub trait ToImaginary {
        fn to_imaginary(self) -> Imaginary;
    }

    impl ToImaginary for f64 {
        fn to_imaginary(self) -> Imaginary {
            Imaginary {real: self, imaginary: 0.0}
        }
    }

    impl ToImaginary for f32 {
        fn to_imaginary(self) -> Imaginary {
            Imaginary {real: self as f64, imaginary: 0.0}
        }
    }
    
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Imaginary {
        real: f64,
        imaginary: f64,
    }

    impl Imaginary {
        pub fn abs(&self) -> f64 {
            (self.real * self.real + self.imaginary * self.imaginary).sqrt()
        }

        pub fn new(real: f64, imaginary: f64) -> Imaginary {
            Imaginary {real, imaginary}
        }

        pub fn conjugated(&self) -> Imaginary {
            Imaginary {real:self.real, imaginary: -self.imaginary}
        }

        pub fn into_polar(self) -> (f64, f64) {
            let r = self.abs();
            let angle = (self.imaginary/self.real).atan();

            (r, angle)
        }

        pub fn root(&self, n: i32) -> Vec<Imaginary> {
            let (r, angle) = self.clone().into_polar();
            let r_root = r.powf(1.0/(n as f64));

            let mut res  = Vec::new();

            for k in 0..n {
                res.push(Imaginary::from_polar(r_root, (angle + 2.0 * std::f64::consts::PI * (k as f64))/(n as f64)));
            }

            res
        }

        pub fn from_polar(abs: f64, angle: f64) -> Imaginary {
            Imaginary {real: abs * angle.cos(), imaginary: abs * angle.sin()}
        } 
    }

    impl Display for Imaginary {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({} {} {}i)", self.real, if self.imaginary > 0.0 {"+"} else {"-"}, self.imaginary.abs())
        }
    }

    impl Add for Imaginary {
        type Output = Imaginary;

        fn add(self, rhs: Self) -> Self::Output {
            let mut res = self;
            res += rhs;

            res
        }
    }

    impl AddAssign for Imaginary {
        fn add_assign(&mut self, rhs: Self) {
            self.real += rhs.real;
            self.imaginary += rhs.imaginary;
        }
    }
    
    impl Sub for Imaginary {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut res = self;

            res -= rhs;

            res
        }
    } 

    impl SubAssign for Imaginary {
        fn sub_assign(&mut self, rhs: Self) {
            self.real -= rhs.real;
            self.imaginary -= rhs.imaginary;
        }
    }

    impl Mul for Imaginary {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut res = self;

            res *= rhs;

            res
        } 
    }

    impl MulAssign for Imaginary {
        fn mul_assign(&mut self, rhs: Self) {
            let s_r = self.real;
            let s_i = self.imaginary;

            let r_r = rhs.real;
            let r_i = rhs.imaginary;

            self.real = s_r * r_r - s_i * r_i;
            self.imaginary = s_r * r_i + s_i * r_r;
        }
    }

    impl Div for Imaginary {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            let mut res = self;

            res /= rhs;

            res
        }
    }

    impl DivAssign for Imaginary {
        fn div_assign(&mut self, rhs: Self) {
            let s_r = self.real;
            let s_i = self.imaginary;

            let r_r = rhs.real;
            let r_i = rhs.imaginary;

            let denominator = r_r * r_r + r_i * r_i;

            self.real = (s_r * r_r + s_i*r_i)/denominator;
            self.imaginary = (s_i * r_r - s_r * r_i)/denominator;
        }
    }

    impl Neg for Imaginary {
        type Output = Self;

        fn neg(self) -> Self::Output {
            self * (-1.0).to_imaginary()
        }
    }

    impl Mul for &Imaginary {
        type Output = Imaginary;

        fn mul(self, rhs: Self) -> Self::Output {
            let s_r = self.real;
            let s_i = self.imaginary;

            let r_r = rhs.real;
            let r_i = rhs.imaginary;

            Imaginary {real: s_r * r_r - s_i * r_i, imaginary: s_r * r_i + s_i * r_r}
        }
    }

    impl Div for &Imaginary {
        type Output = Imaginary;

        fn div(self, rhs: Self) -> Self::Output {
            let s_r = self.real;
            let s_i = self.imaginary;

            let r_r = rhs.real;
            let r_i = rhs.imaginary;


            let denominator = r_r * r_r + r_i * r_i;

            Imaginary {real: (s_r * r_r + s_i * r_i)/denominator, imaginary: (s_i * r_r - s_r * r_i)/denominator}
        }
    }

    impl Add for &Imaginary {
        type Output = Imaginary;

        fn add(self, rhs: Self) -> Self::Output {
            Imaginary {real: self.real + rhs.real, imaginary: self.imaginary + rhs.imaginary}
        }
    }

    impl Sub for &Imaginary {
        type Output = Imaginary;

        fn sub(self, rhs: Self) -> Self::Output {
            Imaginary {real: self.real - rhs.real, imaginary: self.imaginary - rhs.imaginary}
        }
    }

    impl AssAdd for Imaginary {}
    impl ComAdd for Imaginary {}
    impl AssMul for Imaginary {}
    impl ComMul for Imaginary {}

    impl Ring for Imaginary {
        fn zero() -> Imaginary {
            Imaginary {real: 0.0, imaginary: 0.0}
        }
    }

    impl UnRing for Imaginary {
        fn one() -> Imaginary {
            Imaginary {real: 1.0, imaginary: 0.0}
        }
    }

    impl IntegralDomain for Imaginary {}
    impl Field for Imaginary {}

    impl Meta for Imaginary {
        fn non_zero () -> Self {
            Imaginary::one()
        }

        fn name () -> String {
            "Imaginary".to_string()
        }
    }

    //TODO
    //impl Gcd
}

