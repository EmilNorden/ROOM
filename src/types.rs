use fixed::types::I16F16;
use serde::Deserialize;
use fixed::prelude::FromFixed;
use crate::number::RealNumber;
/*
#[cfg(feature = "floating-point-arithmetics")]
pub type DoomRealNum = f32;

#[cfg(not(feature = "floating-point-arithmetics"))]
pub type DoomRealNum = I16F16;

#[cfg(feature = "floating-point-arithmetics")]
pub fn real_from_bits<T>(val: T) -> DoomRealNum {
    unimplemented!()
}

#[cfg(not(feature = "floating-point-arithmetics"))]
pub fn real_from_bits(val: i32) -> DoomRealNum { I16F16::from_bits(val) }

#[cfg(feature = "floating-point-arithmetics")]
pub fn real<T>(val: T) -> DoomRealNum {
    unimplemented!()
}

#[cfg(not(feature = "floating-point-arithmetics"))]
pub fn real<T: fixed::traits::ToFixed>(val: T) -> DoomRealNum {
    I16F16::from_num(val)
}

#[cfg(not(feature = "floating-point-arithmetics"))]
pub fn real_unchecked(val: i32) -> DoomRealNum {
    I16F16::from_bits(val << 16)
}

#[cfg(feature = "floating-point-arithmetics")]
pub fn real_to_int(num: DoomRealNum) -> i32 {
    unimplemented!()
}

#[cfg(not(feature = "floating-point-arithmetics"))]
pub fn real_to_int(num: DoomRealNum) -> i32 {
    num.to_bits() >> 16
}

#[cfg(feature = "floating-point-arithmetics")]
pub fn real_to_bits(num: DoomRealNum) -> i32 {
    unimplemented!()
}

#[cfg(not(feature = "floating-point-arithmetics"))]
pub fn real_to_bits(num: DoomRealNum) -> i32 {
    num.to_bits()
}
*/

#[repr(C)]
#[derive(Deserialize)]
pub struct WadString {
    string: [u8; 8],
}
/*
impl Into<String> for WadString {
    fn into(self) -> String {
        String::from_utf8(self.string.into())
            .unwrap()
            .trim_matches(char::from(0))
            .to_string()
    }
}
*/
impl From<WadString> for String {
    fn from(ws: WadString) -> Self {
        String::from_utf8(ws.string.into())
            .unwrap()
            .trim_matches(char::from(0))
            .to_string()
    }
}

#[derive(Default)]
pub struct Vector3 {
    x: RealNumber,
    y: RealNumber,
    z: RealNumber,
}

impl Vector3 {
    pub fn new(x: RealNumber, y: RealNumber, z: RealNumber) -> Self {
        Self { x, y, z }
    }
}