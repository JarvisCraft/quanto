// The implementation limits float values to `f32` and `f64`.

use core::f64::consts::TAU;

use arbitrary_int::UInt;
use derive_more::From;
use nevermore::FromNever;

use super::{sealed, QasmValue};

/// A scalar OpenQASM value.
#[derive(Debug, PartialEq, PartialOrd, From)]
pub enum Value {
    Bit(Bit),
    UInt8(UnsignedInteger<u8, 8>),
    UInt16(UnsignedInteger<u16, 16>),
    UInt32(UnsignedInteger<u32, 32>),
    UInt64(UnsignedInteger<u64, 64>),
    UInt128(UnsignedInteger<u128, 128>),
    Float32(Float32),
    Float64(Float64),
    Void(Void),
}
impl sealed::Sealed for Value {}
impl QasmValue for Value {}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Float32::from(value).into()
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Float64::from(value).into()
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd, From)]
pub struct Bit(pub bool);
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Bit::from(value).into()
    }
}

// TODO: a more compact implementation
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Bits<const N: usize>([bool; N]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UnsignedInteger<T, const BITS: usize>(UInt<T, BITS>);
macro_rules! impl_unsigned_integer_from {
    ($($from:ident: $size:literal);*) => {$(
        impl From<$from> for UnsignedInteger<$from, $size> {
            fn from(value: $from) -> Self {
                Self::from(UInt::from(value))
            }
        }
        impl From<$from> for Value {
            fn from(value: $from) -> Self {
                UnsignedInteger::<$from, $size>::from(value).into()
            }
        }
    )*};
}
impl_unsigned_integer_from!(u8: 8; u16: 16; u32: 32; u64: 64; u128: 128);

#[derive(Debug, Default, PartialEq, PartialOrd, From)]
pub struct Float32(pub f32);
impl sealed::Sealed for Float32 {}
impl QasmValue for Float32 {}

#[derive(Debug, Default, PartialEq, PartialOrd, From)]
pub struct Float64(pub f64);
impl sealed::Sealed for Float64 {}
impl QasmValue for Float64 {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromNever)]
pub enum Void {}

pub struct Angle<T, const BITS: usize>(UInt<T, BITS>);
impl<T: Copy + Into<u128>, const BITS: usize> Angle<T, BITS> {
    pub fn value(&self) -> f64 {
        2. * TAU * Into::<u128>::into(self.0.value()) as f64 / 2u128.pow(BITS as u32) as f64
    }
}
