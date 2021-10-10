use fixed::types::I16F16;
use serde::Deserialize;
use fixed::prelude::FromFixed;

#[cfg(feature = "floating-point-arithmetics")]
pub type DoomRealNum = f32;

#[cfg(not(feature = "floating-point-arithmetics"))]
pub type DoomRealNum = I16F16;

#[cfg(feature = "floating-point-arithmetics")]
pub fn real<T>(val: T) -> DoomRealNum {
    panic!("not implemented!");
}

#[cfg(not(feature = "floating-point-arithmetics"))]
pub fn real<T: fixed::traits::ToFixed>(val: T) -> DoomRealNum { I16F16::from_num(val) }

#[repr(C)]
#[derive(Deserialize)]
pub struct WadString {
    string: [u8; 8],
}

/*impl Into<String> for WadString {
    fn into(self) -> String {
        String::from_utf8(self.string.into())
            .unwrap()
            .trim_matches(char::from(0))
            .to_string()
    }
}*/

impl From<WadString> for String {
    fn from(ws: WadString) -> Self {
        String::from_utf8(ws.string.into())
            .unwrap()
            .trim_matches(char::from(0))
            .to_string()
    }
}