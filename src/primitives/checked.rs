pub mod checked {
    use std::ops::{Add, Mul, Sub, Div, Rem, Neg, Shl, Shr};



    macro_rules! impl_ops {
        (shift - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident, $meth_name:ident));*) => {
            $(
                impl $op<u32> for $name {
                    type Output = Option<$name>;

                    fn $fn_name(self, rhs: u32) -> Self::Output {
                        if let Some(val) = self.val.$meth_name(rhs) {
                            Some(Self{val})
                        } else {
                            None
                        }
                    }
                }
            )*
        };

        ($typ:ty, $name:ident: $(($op:ident, $fn_name:ident, $meth_name:ident));*) => {
            $(
                impl $op for $name {
                    type Output = Option<$name>;

                    fn $fn_name (self, rhs:Self) -> Self::Output {
                        if let Some (val) = self.val.$meth_name(rhs.val) {
                            Some($name {val})
                        } else {
                            None 
                        }
                    }
                }
            )*
        };
    }
    
    macro_rules! define {
        ($($typ:ty, $name:ident, $macro:ident);*) => {
            $(
                #[macro_export]
                macro_rules! $macro {
                    ($val:expr) => {
                        $name::new($val)
                    }
                }

                #[allow(non_camel_case_types)]
                pub struct $name {
                    val: $typ,
                }

                impl $name {
                    #[allow(dead_code)]
                    pub fn new(val: $typ) -> $name {
                        $name {val}
                    }

                    pub const MIN: Self = Self {val: <$typ>::MIN};
                    pub const MAX: Self = Self {val: <$typ>::MAX};
                    pub const BITS: u32 = <$typ>::BITS;
                }

                
                impl_ops!($typ, $name: 
                    (Add, add, checked_add); (Sub, sub, checked_sub); (Div, div, checked_div); (Mul, mul, checked_mul);
                    (Rem, rem, checked_rem)
                );

                impl_ops!(shift - $typ, $name: (Shl, shl, checked_shl); (Shr, shr, checked_shr));

                impl Neg for $name {
                    type Output = Option<$name>;

                    fn neg(self) -> Self::Output{
                        if let Some(val) = self.val.checked_neg() {
                            Some($name {val})
                        } else {
                            None
                        }
                    }
                }
            )*
        };
    }

    define!(i8, Checked_i8, s_i8; u16, Checked_u16, s_u16);

}