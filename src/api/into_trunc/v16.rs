//! `FromTrunc` and `IntoTrunc` implementations for portable 16-bit wide vectors
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::*;

impl_from_trunc!(
    i8x2: u8x2, m8x2, i16x2, u16x2, m16x2, i32x2, u32x2, f32x2, m32x2,
    i64x2, u64x2, f64x2, m64x2, i128x2, u128x2, m128x2
);
impl_from_trunc!(
    u8x2: i8x2, m8x2, i16x2, u16x2, m16x2, i32x2, u32x2, f32x2, m32x2,
    i64x2, u64x2, f64x2, m64x2, i128x2, u128x2, m128x2
);
impl_from_trunc_mask!(
    m8x2: i8x2, u8x2, i16x2, u16x2, m16x2, i32x2, u32x2, f32x2, m32x2,
        i64x2, u64x2, f64x2, m64x2, i128x2, u128x2, m128x2
);
