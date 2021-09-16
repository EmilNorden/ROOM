use fixed::types::I16F16;

#[cfg(feature = "floating-point-arithmetics")]
pub type DoomRealNum = f32;

#[cfg(not(feature = "floating-point-arithmetics"))]
pub type DoomRealNum = I16F16;

#[cfg(feature = "floating-point-arithmetics")]
pub fn real<T>(val: T) -> DoomRealNum {
    panic!("not implemented!");
}

#[cfg(not(feature = "floating-point-arithmetics"))]
pub fn real<T: fixed::traits::ToFixed>(val: T) -> DoomRealNum {
    I16F16::from_num(val)
}

