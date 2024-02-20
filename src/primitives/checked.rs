
//! Provides the functionality for overflow checked computations that are very important for the whole crate
//TODO rewrite so that i could concatanate method name with checked_ to avoid being too verbose
use std::ops::{Add, Mul, Sub, Div, Rem, Neg, Shl, Shr, BitAnd, BitOr, BitXor, Not};
use crate::tech::*;

pub trait IntoChecked <U> {
    fn safe (self) -> U;
}

macro_rules! impl_ops {
    (shift - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident, $meth_name:ident));*) => {
        $(
            impl $op<CheckedU32> for $name {
                type Output = Option<$name>;

                fn $fn_name(self, rhs: CheckedU32) -> Self::Output {
                    if let Some(val) = self.0.$meth_name(rhs.0) {
                        Some(Self(val))
                    } else {
                        None
                    }
                }
            }
        )*
    };

    (without check - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident($($param:ident: $param_typ:ty),*)));*) => {
        $(
            impl $op for $name {
                type Output = Self;

                fn $fn_name(self $(, $param: $param_typ)*) -> Self::Output{
                    Self(self.0.$fn_name($($param.0),*))
                }
            }
        )*
    };

    ($typ:ty, $name:ident: $(($op:ident, $fn_name:ident, $meth_name:ident));*) => {
        $(
            impl $op for $name {
                type Output = Option<$name>;

                fn $fn_name (self, rhs:Self) -> Self::Output {
                    if let Some (val) = self.0.$meth_name(rhs.0) {
                        Some($name(val))
                    } else {
                        None 
                    }
                }
            }
        )*
    };

    (ref - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident, $meth_name:ident (rhs: $rhs_typ:ty)));*) => {
        $(
            impl $op for &$name {
                type Output = Option<$name>;

                fn $fn_name (self, rhs: &$rhs_typ) -> Self::Output {
                    if let Some(val) = self.0.$meth_name(rhs.0) {
                        Some($name(val))
                    } else {
                        None
                    }
                }
            }
        )*
    };

    
    (ref mut - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident, $meth_name:ident (rhs: $rhs_typ:ty)));*) => {
        $(
            impl $op for &mut $name {
                type Output = Option<$name>;

                fn $fn_name (self, rhs: &mut $rhs_typ) -> Self::Output {
                    if let Some(val) = self.0.$meth_name(rhs.0) {
                        Some($name(val))
                    } else {
                        None
                    }
                }
            }
        )*
    };

    (ref shift - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident, $meth_name:ident));*) => {
        $(
            impl $op<&CheckedU32> for &$name {
                type Output = Option<$name>;

                fn $fn_name (self, rhs: &CheckedU32) -> Self::Output {
                    if let Some(val) = self.0.$meth_name(rhs.0) {
                        Some($name(val))
                    } else {
                        None
                    }
                }
            }
        )*
    };

    (ref without check - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident($($param:ident: $param_typ:ty),*)));*) => {
        $(
            impl $op for &$name {
                type Output = $name;

                fn $fn_name(self $(, $param: $param_typ)*) -> Self::Output{
                    $name(self.0.$fn_name($($param.0),*))
                }
            }
        )*
    };

    (ref mut shift - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident, $meth_name:ident));*) => {
        $(
            impl $op<&mut CheckedU32> for &mut $name {
                type Output = Option<$name>;

                fn $fn_name (self, rhs: &mut CheckedU32) -> Self::Output {
                    if let Some(val) = self.0.$meth_name(rhs.0) {
                        Some($name(val))
                    } else {
                        None
                    }
                }
            }
        )*
    };

    (ref mut without check - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident($($param:ident: $param_typ:ty),*)));*) => {
        $(
            impl $op for &mut $name {
                type Output = $name;

                fn $fn_name(self $(, $param: $param_typ)*) -> Self::Output{
                    $name(self.0.$fn_name($($param.0),*))
                }
            }
        )*
    };
}

macro_rules! impl_method {
    (self. $(($name:ident ($($param_name:ident : $typ:ty),*) -> $ret_typ:ident));*) => {
        $(
            pub const fn $name (self $(, $param_name: $typ)*) -> $ret_typ {
                $ret_typ(self.0.$name($($param_name.0),*))
            }
        )*
    };

    (with Output as Self: self. $(($name:ident ($($param_name:ident : $typ:ty),* ) -> $ret_typ:ty));*) => {
        $(
            pub const fn $name (self $(, $param_name: $typ)*) -> $ret_typ {
                Self(self.0.$name($($param_name.0),*))
            }
        )*
    };

    (return Option: self. $(($name:ident, $meth_name:ident ($($param_name:ident : $typ:ty),* ) -> Option<$ret_typ: ty>));*) => {
        $(
            pub const fn $name (self $(, $param_name: $typ)*) -> Option<$ret_typ> {
                if let Some(val) = self.0.$meth_name($($param_name.0),*) {
                    Some(<$ret_typ>::new(val))
                } else {
                    None
                }
            }
        )*
    };

    (return [Checked_u8; 4]: self. $(($name:ident($($param_name:ident : $typ:ty),* )));*) => {
        $(
            //TODO unconst map issue
        )*
    };

    (to_bytes - self. $(($name:ident($($param_name:ident : $typ:ty),* ) -> [u8; $size:expr]));*) => {
        $(
            pub const fn $name (self $(, $param_name: $typ)*) -> [u8; $size] {
                self.0.$name($($param_name),*)
            }
        )*
    };

    (from_bytes - $(($typ:ty, $name:ident(bytes: [u8; $size:expr]) -> Self));*) => {
        $(
            pub const fn $name(bytes: [u8; $size]) -> Self {
                Self(<$typ>::$name(bytes))
            }
        )*
    }
}

macro_rules! implFormatTrait {
    ($name:ident: $($trait:ident),*) => {
        $(
            impl std::fmt::$trait for $name {
                fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
                    self.0.fmt(f)
                }
            }
        )*
    };
}

macro_rules! implTechTraitsUpToCheckRing {
    ($($typ:ty),*) => {
        $(
            plainImpl!($typ: Com<AddOp>, Ass<AddOp>, Com<MulOp>, Ass<MulOp>, Abelian<AddOp>, Checked, CheckRing, CheckAddGroup, CheckIntegralDomain);

            impl Group<AddOp> for $typ {
                fn neut() -> Self {
                    Self::try_from(0).unwrap()
                }
            }
        )*
    };
}

macro_rules! plainImpl {
    ($typ:ty: $($trait_name: ident $(<$gen_param: ident>)?),*) => {
        $(
            impl $trait_name $(<$gen_param>)? for $typ {}
        )*
    };
}

macro_rules! define {
    ($($typ:ty, $name:ident);*) => {
        $(
            ///Main struct for working with checked primitives. 
            /// It is very close to std primitives in its functionality
            #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub struct $name ($typ);

            impl $name {
                #[allow(dead_code)]
                pub const fn new(val: $typ) -> $name {
                    $name (val)
                }

                pub const MIN: Self = Self(<$typ>::MIN);
                pub const MAX: Self = Self(<$typ>::MAX);
                pub const BITS: u32 = <$typ>::BITS;

                impl_method!(self.
                    (count_zeros() -> CheckedU32); (leading_zeros() -> CheckedU32); (count_ones() -> CheckedU32); (trailing_zeros() -> CheckedU32);
                    (leading_ones() -> CheckedU32); (trailing_ones() -> CheckedU32)
                );

                impl_method!(with Output as Self: self.
                    (rotate_left(n: CheckedU32) -> Self); (rotate_right(n: CheckedU32) -> Self); (swap_bytes() -> Self); (reverse_bits() -> Self);
                    (to_be() -> Self); (to_le() -> Self)
                );

                impl_method!(return Option: self.
                    (div_euclid, checked_div_euclid(rhs: Self) -> Option<Self>); (rem_euclid, checked_rem_euclid(rhs: Self) -> Option<Self>);
                    (pow, checked_pow(exp: CheckedU32) -> Option<Self>); (ilog, checked_ilog(base: Self) -> Option<CheckedU32>);
                    (ilog2, checked_ilog2() -> Option<CheckedU32>); (ilog10, checked_ilog10() -> Option<CheckedU32>)
                );


                pub fn from_str_radix(src: &str, radix: CheckedU32) -> Result<Self, std::num::ParseIntError> {
                    let res = <$typ>::from_str_radix(src, radix.0);
                    match res {
                        Ok(val) => Ok(Self(val)),
                        Err(val) => Err(val),
                    }
                }

                pub const fn from_be(x: Self) -> Self {
                    Self(<$typ>::from_be(x.0))
                }

                pub const fn from_le(x: Self) -> Self {
                    Self(<$typ>::from_le(x.0))
                }

                //TODO change this functions to return array of CheckedU8 while remaining const
                impl_method!(to_bytes - self. 
                    (to_be_bytes() -> [u8; (0 as $typ).to_be_bytes().len()]);
                    (to_le_bytes() -> [u8; (0 as $typ).to_le_bytes().len()]);
                    (to_ne_bytes() -> [u8; (0 as $typ).to_ne_bytes().len()])
                );

                //TODO Similar to the above
                impl_method!(from_bytes - 
                    ($typ, from_be_bytes(bytes: [u8; (0 as $typ).to_be_bytes().len()]) -> Self);
                    ($typ, from_le_bytes(bytes: [u8; (0 as $typ).to_le_bytes().len()]) -> Self);
                    ($typ, from_ne_bytes(bytes: [u8; (0 as $typ).to_ne_bytes().len()]) -> Self)
                );

                fn gcd_sign_unsafe(&self, rhs: &Self) -> Option<Self> {
                    let rem = (self % rhs)?;

                    if rem == Self::try_from(0).unwrap() {
                        return Some(rhs.clone())
                    } 

                    rhs.gcd_sign_unsafe(&rem)
                }
            }

            impl IntoChecked<$name> for $typ {
                fn safe(self) -> $name {
                    $name(self)
                }
            }

            impl_ops!($typ, $name: 
                (Add, add, checked_add); (Sub, sub, checked_sub); (Div, div, checked_div); (Mul, mul, checked_mul);
                (Rem, rem, checked_rem)
            );

            impl_ops!(shift - $typ, $name: (Shl, shl, checked_shl); (Shr, shr, checked_shr));

            impl Neg for $name {
                type Output = Option<$name>;

                fn neg(self) -> Self::Output{
                    if let Some(val) = self.0.checked_neg() {
                        Some($name(val))
                    } else {
                        None
                    }
                }
            }

            implFormatTrait!($name: Display, Binary, LowerExp, LowerHex, Octal, UpperExp, UpperHex);

            impl std::str::FromStr for $name {
                type Err = std::num::ParseIntError;

                fn from_str(src: &str) -> Result<Self, std::num::ParseIntError> {
                    match <$typ>::from_str(src) {
                        Ok(val) => Ok(Self(val)),
                        Err(e) => Err(e)
                    }
                }
            }

            impl_ops!(without check - $typ, $name: 
                (BitAnd, bitand(rhs: $name)); (BitOr, bitor(rhs: $name)); (BitXor, bitxor(rhs: $name)); (Not, not())
            );

            impl_ops!(ref - $typ, $name:
                (Add, add, checked_add(rhs: $name)); (Sub, sub, checked_sub(rhs: $name)); (Mul, mul, checked_mul(rhs: $name)); 
                (Div, div, checked_div(rhs: $name)); (Rem, rem, checked_rem(rhs: $name))
            );

            impl_ops!(ref without check - $typ, $name:
                (BitAnd, bitand(rhs: &$name)); (BitOr, bitor(rhs: &$name)); (BitXor, bitxor(rhs: &$name)); (Not, not())
            );

            impl_ops!(ref shift - $typ, $name: 
                (Shl, shl, checked_shl); (Shr, shr, checked_shr)
            );

            impl Neg for &$name {
                type Output = Option<$name>;

                fn neg(self) -> Self::Output {
                    if let Some(val) = self.0.checked_neg() {
                        Some($name(val))
                    } else {
                        None
                    }
                }
            }

            impl_ops!(ref mut - $typ, $name: 
                (Add, add, checked_add(rhs: $name)); (Sub, sub, checked_sub(rhs: $name)); (Mul, mul, checked_mul(rhs: $name)); 
                (Div, div, checked_div(rhs: $name)); (Rem, rem, checked_rem(rhs: $name))
            );

            impl_ops!(ref mut shift - $typ, $name:
                (Shl, shl, checked_shl); (Shr, shr, checked_shr)
            );

            impl_ops!(ref mut without check - $typ, $name:
                (BitAnd, bitand(rhs: &mut $name)); (BitOr, bitor(rhs: &mut $name)); (BitXor, bitxor(rhs: &mut $name)); (Not, not())
            );

            impl Neg for &mut $name {
                type Output = Option<$name>;

                fn neg(self) -> Self::Output {
                    if let Some(val) = self.0.checked_neg() {
                        Some($name(val))
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

macro_rules! define_unsigned {
    ($($typ:ty, $name:ident);*) => {
        $(
            impl $name {
                #[allow(dead_code)]
                pub const fn div_ceil(self, rhs: Self) -> Self {
                    Self(self.0.div_ceil(rhs.0))
                }

                pub const fn next_multiple_of(self, rhs: Self) -> Option<Self> {
                    if let Some(val) = self.0.checked_next_multiple_of(rhs.0) {
                        Some(Self(val))
                    } else {
                        None
                    }
                }

                pub const fn is_power_of_two(self) -> bool {
                    self.0.is_power_of_two()
                }

                pub const fn next_power_of_two(self) -> Option<Self> {
                    if let Some(val) = self.0.checked_next_power_of_two() {
                        Some(Self(val))
                    } else {
                        None
                    }
                }

                pub const fn abs_diff(self, other: Self) -> Self {
                    Self(self.0.abs_diff(other.0))
                }
            }

            impl Checked for $name{}

            impl Ass<AddOp> for $name {}
            impl Ass<MulOp> for $name {}
            impl Com<MulOp> for $name {}
            impl Com<AddOp> for $name {}

            impl CheckGcd for $name {
                fn gcd(&self, rhs: &Self) -> Option<Self> {
                    self.gcd_sign_unsafe(rhs)
                }
            }
        )*
    };
}

macro_rules! define_signed {
    ($($typ:ty, $name:ident - unsigned: $utyp:ty, $uname:ident);*) => {
        $(
            impl $name {
                #[allow(dead_code)]
                pub const fn is_positive(self) -> bool {
                    self.0.is_positive()
                }

                #[allow(dead_code)]
                pub const fn is_negative(self) -> bool {
                    self.0.is_negative()
                }

                pub const fn abs_diff(self, other: Self) -> $uname {
                    $uname(self.0.abs_diff(other.0))
                }

                pub const fn abs(self) -> Option<Self> {
                    if let Some(val) = self.0.checked_abs() {
                        Some(Self(val))
                    } else {
                        None
                    }
                }

            }

            implTechTraitsUpToCheckRing!($name);

            impl CheckGcd for $name {
                fn gcd(&self, rhs: &Self) -> Option<Self> {
                    self.abs()?.gcd_sign_unsafe(&rhs.abs()?)
                }
            }
        )*
    };
}

macro_rules! defineFromInt {
    ($($typ:ty, $name:ident: ($($from_typ:ty, $from_name:ident);*));*) =>  {
        $(
            $(
                impl From<$from_name> for $name {
                    fn from(value: $from_name) -> Self {
                        Self(value.0 as $typ)
                    }
                }

                impl From<$from_typ> for $name { fn from(value: $from_typ) -> Self { Self(value as $typ) } }
            )*
        )*
    }
}

macro_rules! defineFrom {
    ($($typ: ty, $name:ident: ($($directly_from:ty),*));*) => {
        $(
            $(
                impl From<$directly_from> for $name {
                    fn from(value: $directly_from) -> Self {
                        Self(<$typ>::from(value))
                    }
                }
            )*
        )*
    };
}

macro_rules! defineTryFromChecked {
    ($($typ:ty, $name:ident: $(($from_name:ident, $from_typ:ty)),*);*) => {
        $(
            $(
                impl TryFrom<$from_name> for $name {
                    type Error = <$typ as TryFrom<$from_typ>>::Error;

                    fn try_from(u: $from_name) -> Result<$name, Self::Error> {
                        match <$typ>::try_from(u.0) {
                            Ok(val) => Ok(Self(val)),
                            Err(e) => Err(e)
                        }
                    }
                }
            )*
        )*
    }
}

macro_rules! defineTryFromInt {
    ($($typ:ty, $name:ident: $($from_typ:ty),*);*) => {
        $(
            $(
                impl TryFrom<$from_typ> for $name {
                    type Error = <$typ as TryFrom<$from_typ>>::Error;

                    fn try_from(u: $from_typ) -> Result<$name, Self::Error> {
                        match <$typ>::try_from(u) {
                            Ok(val) => Ok(Self(val)),
                            Err(e) => Err(e)
                        }
                    }
                }
            )*
        )*
    }
}

macro_rules! implMeta {
    ($($name:ident, $name_str:expr);*) => {
        $(
            impl Meta for $name {
                fn name() -> String {
                    String::from($name_str)
                }

                fn non_zero() -> Self {
                    Self::try_from(1).unwrap()
                }
            }
        )*
    };
}
define!(
    u32, CheckedU32; 
    i8, CheckedI8; 
    i16, CheckedI16; 
    i32, CheckedI32; 
    i64, CheckedI64; 
    i128, CheckedI128; 
    u8, CheckedU8; 
    u16, CheckedU16; 
    u64, CheckedU64; 
    u128, CheckedU128;
    usize, CheckedUsize; 
    isize, CheckedIsize
);

define_unsigned!(
    u8, CheckedU8; 
    u16, CheckedU16; 
    u32, CheckedU32; 
    u64, CheckedU64; 
    u128, CheckedU128; 
    usize, CheckedUsize
);
define_signed!(
    i8, CheckedI8 - unsigned: u8, CheckedU8; 
    i16, CheckedI16 - unsigned: u16, CheckedU16; 
    i32, CheckedI32 - unsigned: u32, CheckedU32; 
    i64, CheckedI64 - unsigned: u64, CheckedU64; 
    i128, CheckedI128 - unsigned: u128, CheckedU128; 
    isize, CheckedIsize - unsigned: usize, CheckedUsize
);

defineFromInt!(
    u16, CheckedU16: (u8, CheckedU8);
    i16, CheckedI16: (i8, CheckedI8);
    u32, CheckedU32: (u8, CheckedU8; u16, CheckedU16);
    i32, CheckedI32: (i8, CheckedI8; i16, CheckedI16);
    u64, CheckedU64: (u8, CheckedU8; u16, CheckedU16; u32, CheckedU32);
    i64, CheckedI64: (i8, CheckedI8; i16, CheckedI16; i32, CheckedI32);
    u128, CheckedU128: (u8, CheckedU8; u16, CheckedU16; u32, CheckedU32; u64, CheckedU64);
    i128, CheckedI128: (i8, CheckedI8; i16, CheckedI16; i32, CheckedI32; i64, CheckedI64);
    usize, CheckedUsize: (u8, CheckedU8; u16, CheckedU16);
    isize, CheckedIsize: (i8, CheckedI8; u8, CheckedU8; i16, CheckedI16)
);

defineFrom!(
    u8, CheckedU8: (std::num::NonZeroU8, bool);
    i8, CheckedI8: (std::num::NonZeroI8, bool);
    u16, CheckedU16: (std::num::NonZeroU16, bool);
    i16, CheckedI16: (std::num::NonZeroI16, bool);
    u32, CheckedU32: (std::num::NonZeroU32, bool, std::net::Ipv4Addr, char);
    i32, CheckedI32: (std::num::NonZeroI32, bool);
    u64, CheckedU64: (std::num::NonZeroU64, bool, char);
    i64, CheckedI64: (std::num::NonZeroI64, bool);
    u128, CheckedU128: (std::num::NonZeroU128, bool, std::net::Ipv6Addr, char);
    i128, CheckedI128: (std::num::NonZeroI128, bool);
    usize, CheckedUsize: (std::num::NonZeroUsize, bool);
    isize, CheckedIsize: (std::num::NonZeroIsize, bool)
);

defineTryFromInt!(
    u8, CheckedU8: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize;
    i8, CheckedI8: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize;
    u16, CheckedU16: i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize;
    i16, CheckedI16: u8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize;
    u32, CheckedU32: i8, i16, u32, i32, u64, i64, u128, i128, usize, isize;
    i32, CheckedI32: u8, u16, u32, i32, u64, i64, u128, i128, usize, isize;
    u64, CheckedU64: i8, i16, i32, u64, i64, u128, i128, usize, isize;
    i64, CheckedI64: u8, u16, u32, u64, i64, u128, i128, usize, isize;
    u128, CheckedU128: i8, i16, i32, i64, u128, i128, usize, isize;
    i128, CheckedI128: u8, u16, u32, u64, u128, i128, usize, isize;
    usize, CheckedUsize: i8, i16, u32, i32, u64, i64, i128, u128, usize, isize;
    isize, CheckedIsize: u16, u32, i32, u64, i64, u128, i128, usize, isize
);

defineTryFromChecked!(
    u8, CheckedU8: 
        (CheckedI8, i8), (CheckedI16, i16), (CheckedI32, i32), (CheckedI64, i64), (CheckedI128, i128),
        (CheckedU16, u16), (CheckedU32, u32), (CheckedU64, u64), (CheckedU128, u128),
        (CheckedUsize, usize), (CheckedIsize, isize);

    i8, CheckedI8:
        (CheckedU8, u8), (CheckedU16, u16), (CheckedU32, u32), (CheckedU64, u64), (CheckedU128, u128),
        (CheckedI16, i16), (CheckedI32, i32), (CheckedI64, i64), (CheckedI128, i128),
        (CheckedUsize, usize), (CheckedIsize, isize);

    u16, CheckedU16: 
        (CheckedI8, i8), (CheckedI16, i16), (CheckedI32, i32), (CheckedI64, i64), (CheckedI128, i128),
        (CheckedU32, u32), (CheckedU64, u64), (CheckedU128, u128),
        (CheckedUsize, usize), (CheckedIsize, isize);

    i16, CheckedI16: 
        (CheckedU8, u8), (CheckedU16, u16), (CheckedU32, u32), (CheckedU64, u64), (CheckedU128, u128),
        (CheckedI32, i32), (CheckedI64, i64), (CheckedI128, i128),
        (CheckedUsize, usize), (CheckedIsize, isize);

    u32, CheckedU32:
        (CheckedI8, i8), (CheckedI16, i16), (CheckedI32, i32), (CheckedI64, i64), (CheckedI128, i128),
        (CheckedU64, u64), (CheckedU128, u128),
        (CheckedUsize, usize), (CheckedIsize, isize);

    i32, CheckedI32:
        (CheckedU8, u8), (CheckedU16, u16), (CheckedU32, u32), (CheckedU64, u64), (CheckedU128, u128),
        (CheckedI64, i64), (CheckedI128, i128),
        (CheckedUsize, usize), (CheckedIsize, isize);

    u64, CheckedU64:
        (CheckedI8, i8), (CheckedI16, i16), (CheckedI32, i32), (CheckedI64, i64), (CheckedI128, i128),
        (CheckedU128, u128),
        (CheckedUsize, usize), (CheckedIsize, isize);

    i64, CheckedI64:
        (CheckedU8, u8), (CheckedU16, u16), (CheckedU32, u32), (CheckedU64, u64), (CheckedU128, u128),
        (CheckedI128, i128),
        (CheckedUsize, usize), (CheckedIsize, isize);

    u128, CheckedU128:
        (CheckedI8, i8), (CheckedI16, i16), (CheckedI32, i32), (CheckedI64, i64), (CheckedI128, i128),
        (CheckedUsize, usize), (CheckedIsize, isize);

    i128, CheckedI128:
        (CheckedU8, u8), (CheckedU16, u16), (CheckedU32, u32), (CheckedU64, u64), (CheckedU128, u128),
        (CheckedUsize, usize), (CheckedIsize, isize);
    
    usize, CheckedUsize:
        (CheckedI8, i8), (CheckedI16, i16), (CheckedI32, i32), (CheckedI64, i64), (CheckedI128, i128),
        (CheckedU32, u32), (CheckedU64, u64), (CheckedU128, u128),
        (CheckedIsize, isize);
    
    isize, CheckedIsize:
        (CheckedU16, u16), (CheckedU32, u32), (CheckedU64, u64), (CheckedU128, u128),
        (CheckedI32, i32), (CheckedI64, i64), (CheckedI128, i128),
        (CheckedUsize, usize)
    );

implMeta!(
    CheckedU8, "CheckedU8"; CheckedU16, "CheckedU16"; CheckedU32, "CheckedU32"; CheckedU64, "CheckedU64"; CheckedU128, "CheckedU128";
    CheckedI8, "CheckedI8"; CheckedI16, "CheckedI16"; CheckedI32, "CheckedI32"; CheckedI64, "CheckedI64"; CheckedI128, "CheckedI128"
);
