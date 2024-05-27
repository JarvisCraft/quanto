#[cfg(feature = "rt-quantum-simulator")]
pub mod quantum_simulator;

use core::{
    fmt::{Debug, Display},
    future::Future,
};

use crate::BoundProgram;

pub trait QuantoRuntime {
    type Output;
    type Error: Debug + Display;
}

pub trait SyncQuantoRuntime: QuantoRuntime {
    fn execute<const N: usize>(
        &self,
        program: BoundProgram<'_, N>,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait AsyncQuantoRuntime: QuantoRuntime {
    fn execute<const N: usize>(
        &self,
        program: BoundProgram<'_, N>,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
