use fixed::traits::FromFixed;
use fixed::types::I16F16;
use crate::rendering::types::tables::{TAN_TO_ANGLE, get_sine_table, get_cosine_table};
use float_cmp::approx_eq;
use std::ops::{Sub, Add, Mul, SubAssign, Neg};
use std::cmp::Ordering;
use std::mem::swap;
use num::traits::real::Real;
use crate::constants::FRAC_BITS;
use crate::number::RealNumber;

pub mod tables;

#[derive(Clone)]
pub struct Point2D {
    pub x: RealNumber,
    pub y: RealNumber,
}

impl Point2D {
    pub fn new(x: RealNumber, y: RealNumber) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> RealNumber { self.x }
    pub fn y(&self) -> RealNumber { self.y }

    pub fn distance(&self, other: Point2D) -> RealNumber {
        // TODO: This also needs to be rewritten for floating point
        const SLOPE_BITS: i32 = 11;
        const DBITS: i32 = FRAC_BITS-SLOPE_BITS;
        let mut dx = (self.x - other.x).abs();
        let mut dy = (self.y - other.y).abs();

        if dy > dx {
            swap(&mut dx, &mut dy);
        }

        let angle = (Angle::new(TAN_TO_ANGLE[((dy / dx) >> DBITS).to_bits() as usize]) + Angle::angle90()).fineshift();

        // use as cosine
        let dist = dx / RealNumber::new(get_sine_table()[angle.0 as usize]);

        dist
    }
}

impl From<&Point3D> for Point2D {
    fn from(p: &Point3D) -> Self {
        Self::new(p.x, p.y)
    }
}

#[derive(Clone)]
pub struct Point3D {
    pub x: RealNumber,
    pub y: RealNumber,
    pub z: RealNumber,
}

impl Into<Point2D> for Point3D {
    fn into(self) -> Point2D { // TODO: Is it worth implementing traits so Point3D can be interpreted as a Point2D?
        Point2D::new(self.x, self.y)
    }
}

impl Point3D {
    pub fn new(x: RealNumber, y: RealNumber, z: RealNumber) -> Self {
        Self { x, y, z }
    }
    pub fn x(&self) -> RealNumber { self.x }
    pub fn y(&self) -> RealNumber { self.y }
    pub fn z(&self) -> RealNumber { self.z }
}

#[derive(Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Angle(u32);

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_sub(rhs.0))
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}


impl Mul<Angle> for u32 {
    type Output = Angle;

    fn mul(self, rhs: Angle) -> Self::Output {
        Angle(self * rhs.0)
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0 - rhs.0;
    }
}

impl Neg for Angle {
    type Output = Angle;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl Angle {
    const ANGLE_90: u32 = 0x40000000;
    const ANGLE_180: u32 = 0x80000000;
    const ANGLE_270: u32 = 0xc0000000;
    const ANGLE_TO_FINESHIFT: usize = 19;

    pub fn rotate(&self, angle: Angle) -> Angle {
        *self + angle
    }

    pub const fn angle180() -> Self {
        Self(Angle::ANGLE_180)
    }

    pub const fn angle90() -> Self {
        Self(Angle::ANGLE_90)
    }

    pub fn flip(&self) -> Self {
        Self(Angle::ANGLE_180 ^ self.0)
    }

    pub fn to_u32(&self) -> u32 { self.0 }

    pub fn new(angle: u32) -> Self {
        Self(angle)
    }

    pub fn from_fine_shift(fine_shift: usize) -> Self {
        Self((fine_shift << Self::ANGLE_TO_FINESHIFT) as u32)
    }

    // R_PointToAngle, view[x|y] is parameter b
    pub fn from_points(a: &Point2D, b: &Point2D) -> Self {
        let x = a.x - b.x;
        let y = a.y - b.y;

        if x.is_zero() && y.is_zero() {
            return Angle(0);
        }

        let angle = if x >= RealNumber::new(0) {
            if y >= RealNumber::new(0) {
                if x > y {
                    // octant 0
                    TAN_TO_ANGLE[Angle::slope_div(y.to_bits(), x.to_bits())]
                } else {
                    // octant 1
                    Angle::ANGLE_90 - 1 - TAN_TO_ANGLE[Angle::slope_div(x.to_bits(), y.to_bits())]
                }
            } else {
                // y<0
                let y = -y;
                if x > y {
                    // octant 8
                    // Original code negated the result, but negating an unsigned integer makes no sense.
                    // However, the MSB is the sign bit, so we can just flip it with a XOR.
                    0x80000000 ^ TAN_TO_ANGLE[Angle::slope_div(y.to_bits(), x.to_bits())]
                } else {
                    // octant 7
                    Angle::ANGLE_270 + TAN_TO_ANGLE[Angle::slope_div(x.to_bits(), y.to_bits())]
                }
            }
        } else {
            // x<0
            let x = -x;

            if y >= RealNumber::new(0) {
                if x > y {
                    // octant 3
                    Angle::ANGLE_180 - 1 - TAN_TO_ANGLE[Angle::slope_div(y.to_bits(), x.to_bits())]
                } else {
                    // octant 2
                    Angle::ANGLE_90 + TAN_TO_ANGLE[Angle::slope_div(x.to_bits(), y.to_bits())]
                }
            } else {
                // y<0
                let y = -y;

                if x > y {
                    // octant 4
                    Angle::ANGLE_180 + TAN_TO_ANGLE[Angle::slope_div(y.to_bits(), x.to_bits())]
                } else {
                    // octant 5
                    Angle::ANGLE_270 - 1 - TAN_TO_ANGLE[Angle::slope_div(x.to_bits(), y.to_bits())]
                }
            }
        };

        Angle(angle)
    }

    fn slope_div(num: i32, den: i32) -> usize {
        let num = num as u32;
        let den = den as u32;

        const SLOPE_RANGE: usize = 2048;

        if den < 512 {
            SLOPE_RANGE
        } else {
            let ans = ((num << 3) / (den >> 8)) as usize;
            if ans <= SLOPE_RANGE {
                ans
            } else {
                SLOPE_RANGE
            }
        }
    }

    pub fn fineshift(&self) -> Angle {
        Self(self.0 >> Angle::ANGLE_TO_FINESHIFT)
    }

    pub fn sine(&self) -> RealNumber {
        // #define ANGLETOFINESHIFT	19
        panic!("Fix this!");
        // I16F16::from_bits(get_sine_table()[self.fineshift().0 as usize])
    }

    pub fn cosine(&self) -> RealNumber {
        panic!("Fix this!");
        // I16F16::from_bits(get_cosine_table()[self.fineshift().0 as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_from_points() {
        let p1 = Point2D::new(RealNumber::new(-736), RealNumber::new(-128));
        let p2 = Point2D::new(RealNumber::new(-864), RealNumber::new(-96));
        let angle = Angle::from_points(&p1, &p2);
        assert_eq!(angle.0, 2314942560);

        let p1 = Point2D::new(RealNumber::new(-768), RealNumber::new(-192));
        let angle = Angle::from_points(&p1, &p2);
        assert_eq!(angle.0, 3758096384);
    }
}