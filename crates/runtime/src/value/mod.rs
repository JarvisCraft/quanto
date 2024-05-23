use derive_more::From;

pub mod quantum;
pub mod scalar;

mod sealed {
    pub(super) trait Sealed {}
}

pub trait QasmValue: sealed::Sealed {}

#[derive(Debug, From)]
pub enum Value {
    Quantum(quantum::Value),
    Scalar(scalar::Value),
}

impl sealed::Sealed for Value {}
impl QasmValue for Value {}
