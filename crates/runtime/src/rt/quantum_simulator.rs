//! An implementation of runtime based on [Quantum Simulator] project accessible via FFI.
//!
//! [Quantum Simulator]: https://github.com/RKulagin/quantum-simulator/tree/master

use alloc::boxed::Box;
use core::{
    fmt::{self, Formatter},
    mem::MaybeUninit,
};
use std::num::NonZeroU8;

use smallvec::{CollectionAllocErr, SmallVec};

use crate::{
    rt::{QuantoRuntime, SyncQuantoRuntime},
    value::{QuantumValue, ScalarValue, Value},
    BoundProgram, Program,
};

pub struct QuantumSimulatorRt;

impl QuantoRuntime for QuantumSimulatorRt {
    type Output = QuantumValue;
}

#[derive(Debug)]
pub enum Error {
    BuildArgsStack(BuildArgsStackError),
    Failure(NonZeroU8),
}
impl From<BuildArgsStackError> for Error {
    fn from(value: BuildArgsStackError) -> Self {
        Self::BuildArgsStack(value)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuildArgsStack(_) => f.write_str("failed to build arguments stack"),
            Self::Failure(error_code) => write!(f, "execution failed with error code {error_code}"),
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BuildArgsStack(e) => Some(e),
            Self::Failure(_) => None,
        }
    }
}

impl SyncQuantoRuntime for QuantumSimulatorRt {
    type Error = Error;

    fn execute<const N: usize>(
        &self,
        BoundProgram {
            program: Program { src },
            args,
        }: BoundProgram<'_, N>,
    ) -> Result<Self::Output, Self::Error> {
        let (src_ptr, src_len) = (src.as_ptr().cast(), src.len());

        let args = build_args_stack(args)?;
        let (args_ptr, args_len) = (args.as_ptr(), args.len());

        let mut output = MaybeUninit::uninit();
        let output_ptr = output.as_mut_ptr();

        // SAFETY:
        // * `src` is a valid pointer to `src_len` bytes;
        // * `args` is a valid pointer to `args_len` bytes;
        // * ``
        let code = unsafe {
            bindings::quantum_simulator_execute_sync(
                src_ptr, src_len, args_ptr, args_len, output_ptr,
            )
        };

        if let Some(error_code) = NonZeroU8::new(code) {
            Err(Error::Failure(error_code))
        } else {
            // SAFETY: zero code indicates that the operation succeeded and the buffer was filled.
            let result = unsafe { output.assume_init() };
            todo!("Convert QuantumResult into the value")
        }
    }
}

/// The maximal size of arguments stack (in bytes)
/// which can be stored in the caller stack instead of being allocated.
const MAX_ARGS_BYTES_ON_STACK: usize = 512;

/// Builds the stack of arguments passed to the quantum simulator.
///
/// # Example
///
///
fn build_args_stack(values: &[Value]) -> Result<Box<[u8]>, BuildArgsStackError> {
    let mut total_size = 0usize;

    for value in values {
        let size = match value {
            Value::Quantum(value) => match value {
                QuantumValue::VirtualQubit(_) => 1,
                QuantumValue::HardwareQubit(_) => 1,
            },
            Value::Scalar(value) => match value {
                ScalarValue::Float32(_) => 4,
                ScalarValue::Float64(_) => 8,
                ScalarValue::Void(void) => match *void {},
            },
        };

        total_size = total_size
            .checked_add(size)
            .ok_or(BuildArgsStackError::SizeOverflow)?;
    }

    let mut payload = SmallVec::<[u8; MAX_ARGS_BYTES_ON_STACK]>::new();
    payload.try_reserve_exact(total_size)?;

    Ok(payload.into_boxed_slice())
}

#[derive(Debug)]
pub enum BuildArgsStackError {
    SizeOverflow,
    AllocFailure(CollectionAllocErr),
}

impl From<CollectionAllocErr> for BuildArgsStackError {
    fn from(value: CollectionAllocErr) -> Self {
        Self::AllocFailure(value)
    }
}

impl fmt::Display for BuildArgsStackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SizeOverflow => f.write_str("too many arguments cannot fit in memory"),
            Self::AllocFailure(_) => f.write_str("failed to allocate a vector to store arguments"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BuildArgsStackError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SizeOverflow => None,
            // TODO: https://github.com/servo/rust-smallvec/issues/355
            Self::AllocFailure(_e) => None,
        }
    }
}

mod tag {
    const VIRTUAL_QUBIT: [u8; 1] = [0x1];
    const PHYSIAL_QUBIT: [u8; 1] = [0x2];
}

mod bindings {
    use std::ffi::c_char;

    extern "C" {
        pub fn quantum_simulator_execute_sync(
            src: *const c_char,
            src_len: usize,
            args: *const u8,
            arg_count: usize,
            output: *mut QuantumResult,
        ) -> u8;
    }
    #[repr(C)]
    pub struct QuantumResult {
        tag: u8,
        value: QuantumResultValue,
    }

    #[repr(C)]
    pub union QuantumResultValue {
        qubit: bool,
        boolean: bool,
    }
}
