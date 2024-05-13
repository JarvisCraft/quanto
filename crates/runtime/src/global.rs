use crate::rt::{quantum_simulator::QuantumSimulatorRt, SyncQuantoRuntime};

// TODO: customization via a feature
static SYNC_RT: QuantumSimulatorRt = QuantumSimulatorRt;

pub fn sync_rt() -> &'static impl SyncQuantoRuntime {
    &SYNC_RT
}
