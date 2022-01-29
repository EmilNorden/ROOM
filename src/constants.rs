// Im not fond of having a 'constants' module, but I dont know where else to put these atm.


// These are related to DOOMs use of fixed point arithmetics.
// I should probably avoid these and rely on the fixed crate instead.
// This represents the number of bits used for the fractional part.
pub const FRAC_BITS: i32 = 16;
// This represents fixed-point 1.0. Thus when DOOM wants to convert a number to fixed point, the original source code
// multiplies the number by FRAC_UNIT.
pub const FRAC_UNIT: i32 = 1 << FRAC_BITS;


pub const MAP_BLOCK_SHIFT: i32 = FRAC_BITS + 7;

pub const MAX_RADIUS: i32 = 32;
