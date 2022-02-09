use std::cmp::Ordering;
use std::ops::{Add, AddAssign, BitXor, Div, Mul, Neg, Shl, ShlAssign, Shr, ShrAssign, Sub};
use num::traits::Pow;
use num::traits::real::Real;

const FRACTIONAL_BITS: i32 = 16;
const FIXED_UNIT: i32 = (1 << FRACTIONAL_BITS);

pub trait ToFixed {
    fn to_fixed(self) -> i32;
}

#[macro_export]
macro_rules! to_fixed_int {
    ($t:ty) => {
            impl ToFixed for $t {
                fn to_fixed(self) -> i32 {
                    (self as i32 * FIXED_UNIT)
                }
            }
    }
}

impl ToFixed for f32 {
    fn to_fixed(self) -> i32 {
        (self * (FIXED_UNIT as f32)).round() as i32
    }
}

to_fixed_int!(i8);
to_fixed_int!(i16);
to_fixed_int!(i32);
to_fixed_int!(i64);

to_fixed_int!(u8);
to_fixed_int!(u16);
to_fixed_int!(u32);
to_fixed_int!(u64);
to_fixed_int!(usize);

#[cfg(not(feature = "floating-point-arithmetics"))]
#[derive(Copy, Clone, Default, PartialOrd, PartialEq, Ord, Eq)]
pub struct RealNumber {
    bits: i32,
}

impl RealNumber {
    pub fn new<T: ToFixed>(num: T) -> Self {
        Self {
            bits: num.to_fixed()
        }
    }

    pub fn new_from_bits(bits: i32) -> Self {
        Self { bits }
    }

    pub fn is_zero(self) -> bool {
        self.bits == 0
    }

    pub fn is_negative(self) -> bool {
        self.is_negative()
    }

    #[deprecated] // TODO: Marking this as deprecated because it *SHOULDNT EXIST*
    pub fn to_bits(self) -> i32 {
        self.bits
    }

    pub fn to_int(self) -> i32 {
        self.bits >> FRACTIONAL_BITS
    }

    pub fn abs(self) -> Self { Self { bits: self.bits.abs() } }
}

impl Div for RealNumber {
    type Output = RealNumber;

    fn div(self, rhs: Self) -> Self::Output {
        if (self.bits.abs() >> 14) >= rhs.bits.abs() {
            Self {
                bits: if self.bits ^ rhs.bits < 0 { i32::MIN } else { i32::MAX }
            }
        } else {
            let c = (self.bits as f64 / rhs.bits as f64) * FIXED_UNIT as f64;

            if c >= 2147483648.0 || c < -2147483648.0 {
                panic!("FixedDiv: divide by zero")
            }

            Self {
                bits: c as i32
            }
        }
    }
}

// TODO Dont know what I think about this...
impl Div<i32> for RealNumber {
    type Output = RealNumber;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            bits: self.bits / rhs
        }
    }
}

impl Mul for RealNumber {
    type Output = RealNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        let bits = (self.bits as i64) * (rhs.bits as i64) >> FRACTIONAL_BITS;
        Self {
            bits: bits as i32
        }
    }
}

impl Sub for RealNumber {
    type Output = RealNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits - rhs.bits
        }
    }
}

impl Add for RealNumber {
    type Output = RealNumber;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits + rhs.bits
        }
    }
}

impl AddAssign for RealNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.bits += rhs.bits;
    }
}

impl BitXor for RealNumber {
    type Output = RealNumber;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits
        }
    }
}

impl Neg for RealNumber {
    type Output = RealNumber;

    fn neg(self) -> Self::Output {
        Self {
            bits: -self.bits
        }
    }
}

impl Shr<i32> for RealNumber {
    type Output = RealNumber;

    fn shr(self, rhs: i32) -> Self::Output {
        Self {
            bits: self.bits >> rhs
        }
    }
}

impl ShrAssign<i32> for RealNumber {
    fn shr_assign(&mut self, rhs: i32) {
        self.bits >>= rhs;
    }
}

impl Shl<i32> for RealNumber {
    type Output = RealNumber;

    fn shl(self, rhs: i32) -> Self::Output {
        Self {
            bits: self.bits << rhs
        }
    }
}

impl ShlAssign<i32> for RealNumber {
    fn shl_assign(&mut self, rhs: i32) {
        self.bits <<= rhs;
    }
}