// The implementation limits float values to `f32` and `f64`.

use core::f64::consts::TAU;

use arbitrary_int::UInt;

use crate::value::{sealed, QasmValue};

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Bit(pub bool);

// TODO: a more compact implementation
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bits<const N: usize>([bool; N]);

pub struct UnsignedInteger<T, const BITS: usize>(UInt<T, BITS>);

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Float32(pub f32);
impl sealed::Sealed for Float32 {}
impl QasmValue for Float32 {}

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Float64(pub f64);
impl sealed::Sealed for Float64 {}
impl QasmValue for Float64 {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Void {}

pub struct Angle<T, const BITS: usize>(UInt<T, BITS>);
impl<T: Copy + Into<u128>, const BITS: usize> Angle<T, BITS> {
    pub fn value(&self) -> f64 {
        2. * TAU * Into::<u128>::into(self.0.value()) as f64 / 2u128.pow(BITS as u32) as f64
    }
}
