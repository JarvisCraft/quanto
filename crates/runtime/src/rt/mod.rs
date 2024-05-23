pub mod quantum_simulator;

use core::{
    fmt::{Debug, Display},
    future::Future,
};

use crate::BoundProgram;

pub trait QuantoRuntime {
    type Output;
}

pub trait SyncQuantoRuntime: QuantoRuntime {
    type Error: Debug + Display;

    fn execute<const N: usize>(
        &self,
        program: BoundProgram<'_, N>,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait AsyncQuantoRuntime: QuantoRuntime {
    type Error;

    fn execute<const N: usize>(
        &self,
        program: BoundProgram<'_, N>,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
