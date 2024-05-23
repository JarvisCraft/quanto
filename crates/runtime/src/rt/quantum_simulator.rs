//! An implementation of runtime based on [Quantum Simulator] project accessible via FFI.
//!
//! [Quantum Simulator]: https://github.com/RKulagin/quantum-simulator/tree/master

use alloc::boxed::Box;
use core::{
    fmt::{self, Formatter},
    mem::MaybeUninit,
};
use std::{ffi::c_char, mem::size_of, num::NonZeroU8};

use smallvec::{CollectionAllocErr, SmallVec};

use self::bindings::{QuantumResult, Tag};
use crate::{
    rt::{quantum_simulator, QuantoRuntime, SyncQuantoRuntime},
    value::{quantum::Value as QuantumValue, scalar::Value as ScalarValue, Value},
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
            let QuantumResult { tag, value } = unsafe { output.assume_init() };
            /* Ok(match tag {
                Tag::VirtualQubit => QuantumValue::VirtualQubit(unsafe { value.qubit }),
                Tag::HardwareQubit => QuantumValue::HardwareQubit(unsafe { value.qubit }),
                Tag::Boolean => QuantumValue::from(unsafe { value.boolean }),
                Tag::Uint8 => QuantumValue::from(unsafe { value.uint8 }),
                Tag::Uint16 => QuantumValue::from(unsafe { value.uint16 }),
                Tag::Uint32 => QuantumValue::from(unsafe { value.uint32 }),
                Tag::Uint64 => QuantumValue::from(unsafe { value.uint64 }),
                Tag::Uint128 => QuantumValue::from(unsafe { value.uint128 }),
                Tag::Float32 => QuantumValue::from(unsafe { value.float32 }),
                Tag::Float64 => QuantumValue::from(unsafe { value.float64 }),
            }) */
            Ok(todo!("Unsafe conversion"))
        }
    }
}

/// The maximal size of arguments stack (in bytes)
/// which can be stored in the caller stack instead of being allocated.
const MAX_ARGS_BYTES_ON_STACK: usize = 512;

/// Builds the stack of arguments passed to the quantum simulator.
fn build_args_stack(values: &[(&str, Value)]) -> Result<Box<[u8]>, BuildArgsStackError> {
    let mut total_size = 0usize;

    for (_, value) in values {
        let size = match value {
            Value::Quantum(value) => match value {
                QuantumValue::VirtualQubit(_) => 1,
                QuantumValue::HardwareQubit(_) => 1,
            },
            Value::Scalar(value) => match value {
                ScalarValue::Bit(_) => 1,
                ScalarValue::UInt8(_) => 1,
                ScalarValue::UInt16(_) => 2,
                ScalarValue::UInt32(_) => 4,
                ScalarValue::UInt64(_) => 8,
                ScalarValue::UInt128(_) => 16,
                ScalarValue::Float32(_) => 4,
                ScalarValue::Float64(_) => 8,
                ScalarValue::Void(void) => match *void {},
            },
        };

        total_size = total_size
            .checked_add(size_of::<*const c_char>() + size)
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

mod bindings {
    use std::ffi::c_char;

    use derive_more::From;

    #[repr(u8)]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
    pub enum Tag {
        VirtualQubit = 0x1,
        HardwareQubit = 0x2,
        Boolean = 0x3,
        Uint8 = 0x4,
        Uint16 = 0x5,
        Uint32 = 0x6,
        Uint64 = 0x7,
        Uint128 = 0x8,
        Float32 = 0x9,
        Float64 = 0xA,
    }

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
        pub tag: Tag,
        pub value: QuantumResultValue,
    }

    #[repr(C)]
    pub union QuantumResultValue {
        pub virtual_qubit: bool,
        pub hardware_qubit: bool,
        pub boolean: bool,
        pub uint8: u8,
        pub uint16: u16,
        pub uint32: u32,
        pub uint64: u64,
        pub uint128: u128,
        pub float32: f32,
        pub float64: f64,
    }
}
