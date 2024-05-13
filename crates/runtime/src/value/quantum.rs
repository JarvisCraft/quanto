use super::{sealed, QasmValue};

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
