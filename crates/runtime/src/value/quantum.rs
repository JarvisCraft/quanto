use derive_more::From;

use super::{sealed, QasmValue};

/// A quantum OpenQASM value.
#[derive(Debug, From)]
pub enum Value {
    VirtualQubit(VirtualQubit),
    HardwareQubit(HardwareQubit),
}
impl sealed::Sealed for Value {}
impl QasmValue for Value {}

#[derive(Debug, Default)]
pub enum VirtualQubit {
    #[default]
    Undefined,
    Zero,
    One,
}
impl sealed::Sealed for VirtualQubit {}
impl QasmValue for VirtualQubit {}

#[derive(Debug, Default)]
pub enum HardwareQubit {
    #[default]
    Undefined,
    Zero,
    One,
}
impl sealed::Sealed for HardwareQubit {}
impl QasmValue for HardwareQubit {}
