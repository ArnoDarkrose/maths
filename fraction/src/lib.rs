pub mod fract {
    use tech::*;
    use std::fmt::Display

    #[derive(Debug, Clone)]
    pub struct Fraction <T: Ring> {
        num: T,
        denom: T,
    }

    impl<T: Ring> Fraction<T> {
        pub fn new (num: T, denom: T) -> Fraction<T> {
            Fraction {num, denom}
        }
    }

    impl<T> Display for Fraction<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            
        }
    }
}