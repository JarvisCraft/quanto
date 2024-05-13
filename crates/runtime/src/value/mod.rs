mod quantum;
mod scalar;

mod sealed {
    pub(super) trait Sealed {}
}

pub trait QasmValue: sealed::Sealed {}

pub enum Value {
    Quantum(QuantumValue),
    Scalar(ScalarValue),
}

impl sealed::Sealed for Value {}
impl QasmValue for Value {}

/// A quantum OpenQASM value.
pub enum QuantumValue {
    VirtualQubit(quantum::VirtualQubit),
    HardwareQubit(quantum::HardwareQubit),
}
impl sealed::Sealed for QuantumValue {}
impl QasmValue for QuantumValue {}

/// A scalar OpenQASM value.
pub enum ScalarValue {
    Float32(scalar::Float32),
    Float64(scalar::Float64),
    Void(scalar::Void),
}
impl sealed::Sealed for ScalarValue {}
impl QasmValue for ScalarValue {}
