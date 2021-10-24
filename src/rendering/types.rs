use crate::types::{DoomRealNum, real};
use fixed::traits::FromFixed;
use fixed::types::I16F16;
use crate::rendering::types::tables::TAN_TO_ANGLE;

mod tables;

pub struct Point {
    x: DoomRealNum,
    y: DoomRealNum,
}

impl Point {
    pub fn new(x: DoomRealNum, y: DoomRealNum) -> Self {
        Self { x, y }
    }
}

pub struct Angle(DoomRealNum);

impl Angle {
    const ANGLE_90: u32 = 0x40000000;
    const ANGLE_180: u32 = 0x80000000;
    const ANGLE_270: u32 = 0xc0000000;
    // R_PointToAngle, view[x|y] is parameter b
    pub fn from_points(a: &Point, b: &Point) -> Self {
        let x = a.x - b.x;
        let y = a.y - b.y;

        if x.is_zero() && y.is_zero() {
            return Angle(real(0));
        }

        let angle = if x >= 0 {
            if y >= 0 {
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
                    0x80000000 ^ TAN_TO_ANGLE[Angle::slope_div(y.to_bits(), x.to_bits())]
                } else {
                    // octant 7
                    Angle::ANGLE_270 + TAN_TO_ANGLE[Angle::slope_div(x.to_bits(), y.to_bits())]
                }
            }
        } else {
            // x<0
            let x = -x;

            if y >= 0 {
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

        Angle(I16F16::from_bits(angle as i32))
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let p1 = Point::new(real(-960), real(-192));
        let p2 = Point::new(real(-864), real(-96));
        let angle = Angle::from_points(&p1, &p2);
        assert_eq!(angle.0, 0x60000001);
    }

}