#[cfg(feature = "rt-quantum-simulator")]
mod quantum_simulator {
    use crate::{
        rt::{quantum_simulator::QuantumSimulatorRt, SyncQuantoRuntime},
        value::Value,
    };

    pub type Sync = QuantumSimulatorRt;

    #[cfg(feature = "rt-quantum-simulator")]
    pub static SYNC_RT: QuantumSimulatorRt = QuantumSimulatorRt;

    pub fn sync_rt() -> &'static impl SyncQuantoRuntime<Output = Value> {
        &SYNC_RT
    }
}
#[cfg(feature = "rt-quantum-simulator")]
pub use quantum_simulator::*;

#[cfg(not(feature = "rt-quantum-simulator"))]
compile_error!("No runtime was specified, consider using one of `rt-*` features");
